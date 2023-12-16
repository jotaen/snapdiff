use std::sync::{Arc, Mutex};
use std::{fs, io, path, thread};
use std::io::{BufRead};
use crate::dir_iter::{DirIterator};
use std::ops::{DerefMut};
use std::thread::JoinHandle;
use file::File;
use crate::file;
use crate::progress::Progress;
use crate::snapshot::Snapshot;

pub struct Config {
    pub worker: usize,
    pub chunk_size: usize,
}

pub struct FsScan {
    config: Config,
}

impl FsScan {
    pub fn new(config: Config) -> FsScan {
        return FsScan{
            config,
        }
    }

    pub fn traverse<S>(&self, name: &'static str, root: &path::Path, snap: S) -> S
    where S: Snapshot + std::fmt::Debug + Send + 'static
    {
        let progress = Progress::new(name);
        progress.scan();
        let mut dir_it = DirIterator::new(self.config.chunk_size as u64);
        dir_it.scan(root);

        let dir_it_arc = Arc::new(Mutex::new(dir_it));
        let snap_arc = Arc::new(Mutex::new(snap));
        let progress_arc = Arc::new(Mutex::new(progress));
        let mut handles = vec![];

        for _i in 0..self.config.worker {
            let handle = spawn_worker(
                Arc::clone(&dir_it_arc),
                Arc::clone(&snap_arc),
                Arc::clone(&progress_arc),
                root.to_path_buf(),
                self.config.chunk_size,
            );
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let snap = Arc::try_unwrap(snap_arc).unwrap().into_inner().unwrap();
        Arc::try_unwrap(progress_arc).unwrap().into_inner().unwrap().done();
        return snap;
    }
}

fn spawn_worker<S>(
    dir_it_mtx: Arc<Mutex<DirIterator>>,
    snap_mtx: Arc<Mutex<S>>,
    progress_mtx: Arc<Mutex<Progress>>,
    root: path::PathBuf,
    chunk_size: usize,
) -> JoinHandle<()>
where S: Snapshot + std::fmt::Debug + Send + 'static
{
    return thread::spawn(move || {
        {
            let mut p = progress_mtx.lock().unwrap();
            p.increment(0, 0 as file::SizeBytes);
        }
        loop {
            let p = {
                let entry = {
                    let mut di = dir_it_mtx.lock().unwrap();
                    di.deref_mut().next_file()
                };
                if entry.is_none() {
                    break;
                }
                entry.unwrap()
            };

            let disk_file = fs::File::options()
                .read(true).open(&p).expect("Failed to open file");
            let mut reader = io::BufReader::with_capacity(chunk_size, disk_file);
            let mut size_bytes: file::SizeBytes = 0;
            let mut checksum_context = md5::Context::new();
            loop {
                let buffer = reader.fill_buf().expect("Failed to read file");
                let length = buffer.len();
                if length == 0 {
                    break;
                }
                checksum_context.consume(buffer);
                size_bytes += length as file::SizeBytes;
                reader.consume(length);
                {
                    let mut p = progress_mtx.lock().unwrap();
                    p.increment(0, length as file::SizeBytes);
                }
            }

            let rel_path = p.strip_prefix(&root).unwrap().to_path_buf();
            let f = File::new(rel_path, size_bytes, checksum_context.compute());

            {
                let mut s = snap_mtx.lock().unwrap();
                s.deref_mut().add(f);
            }
            {
                let mut p = progress_mtx.lock().unwrap();
                p.increment(1, 0);
            }
        }
    });
}
