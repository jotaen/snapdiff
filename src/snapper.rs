use crate::dir_iter::DirIterator;
use crate::file;
use crate::progress::Progress;
use crate::snapshot::Snapshot;
use file::File;
use std::io::BufRead;
use std::ops::DerefMut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{fs, io, path, thread};

pub struct Config {
    pub worker: usize,
    pub chunk_size: usize,
}

pub struct Snapper {
    config: Config,
    ctrlc_arc: Arc<AtomicBool>,
}

impl Snapper {
    pub fn new(config: Config) -> Snapper {
        let ctrlc_arc = Arc::new(AtomicBool::new(false));
        let r = ctrlc_arc.clone();
        ctrlc::set_handler(move || {
            r.store(true, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        return Snapper { config, ctrlc_arc };
    }

    pub fn process<S>(&self, name: &'static str, root: &path::Path, snap: S) -> S
    where
        S: Snapshot + std::fmt::Debug + Send + 'static,
    {
        let mut progress = Progress::new(name);
        progress.scan_start();
        let mut dir_it = DirIterator::new(self.config.chunk_size as u64);
        let s = dir_it.scan(root);
        progress.scan_done(s);

        let dir_it_arc = Arc::new(Mutex::new(dir_it));
        let snap_arc = Arc::new(Mutex::new(snap));
        let progress_arc = Arc::new(Mutex::new(progress));
        let mut handles = vec![];

        for _i in 0..self.config.worker {
            let handle = spawn_worker(
                Arc::clone(&dir_it_arc),
                Arc::clone(&snap_arc),
                Arc::clone(&progress_arc),
                Arc::clone(&self.ctrlc_arc),
                root.to_path_buf(),
                self.config.chunk_size,
            );
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let snap = Arc::try_unwrap(snap_arc).unwrap().into_inner().unwrap();
        Arc::try_unwrap(progress_arc)
            .unwrap()
            .into_inner()
            .unwrap()
            .process_done();
        return snap;
    }
}

fn spawn_worker<S>(
    dir_it_mtx: Arc<Mutex<DirIterator>>,
    snap_mtx: Arc<Mutex<S>>,
    progress_mtx: Arc<Mutex<Progress>>,
    ctrlc_mtx: Arc<AtomicBool>,
    root: path::PathBuf,
    chunk_size: usize,
) -> JoinHandle<()>
where
    S: Snapshot + std::fmt::Debug + Send + 'static,
{
    return thread::spawn(move || {
        {
            let mut p = progress_mtx.lock().unwrap();
            p.process_inc(0, 0 as file::SizeBytes);
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
                .read(true)
                .open(&p)
                .expect("Failed to open file");
            let mut reader = io::BufReader::with_capacity(chunk_size, disk_file);
            let mut size_bytes: file::SizeBytes = 0;
            let mut checksum_context = md5::Context::new();
            loop {
                if ctrlc_mtx.load(Ordering::SeqCst) {
                    println!();
                    std::process::exit(255);
                }
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
                    p.process_inc(0, length as file::SizeBytes);
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
                p.process_inc(1, 0);
            }
        }
    });
}
