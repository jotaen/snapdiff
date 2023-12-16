use std::sync::{Arc, Mutex};
use std::{fs, io, path, thread};
use std::io::{BufRead, Write};
use crate::dir_iter::{DirIterator};
use std::ops::{DerefMut};
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
        let dir_it_arc = Arc::new(Mutex::new(DirIterator::new(root)));
        let snap_arc = Arc::new(Mutex::new(snap));
        let progress_arc = Arc::new(Mutex::new(Progress::new(name)));

        let mut handles = vec![];

        for _i in 0..self.config.worker {
            let dir_it_mtx = Arc::clone(&dir_it_arc);
            let snap_mtx = Arc::clone(&snap_arc);
            let progress_mtx = Arc::clone(&progress_arc);
            let root = root.to_path_buf();
            let chunk_size = self.config.chunk_size;
            let handle = thread::spawn(move || {
                loop {
                    let entry = {
                        let mut di = dir_it_mtx.lock().unwrap();
                        di.deref_mut().next()
                    };
                    let has_processed = entry.map(|e| {
                        retrieve_file(chunk_size, root.to_path_buf(), e).map(|f| {
                            let mut s = snap_mtx.lock().unwrap();
                            io::stdout().flush().unwrap();
                            s.deref_mut().add(f);
                            let mut p = progress_mtx.lock().unwrap();
                            p.update(s.total());
                        });
                        return true;
                    }).unwrap_or(false);
                    if !has_processed {
                        break;
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let snap = Arc::try_unwrap(snap_arc).unwrap().into_inner().unwrap();
        Arc::try_unwrap(progress_arc).unwrap().into_inner().unwrap().done(snap.total());
        return snap;
    }
}

fn retrieve_file(chunk_size: usize, root: path::PathBuf, path: path::PathBuf) -> Option<File> {
    if !path.is_file() {
        return None;
    }

    let disk_file = fs::File::options()
        .read(true).open(&path).ok()?;
    let mut reader = io::BufReader::with_capacity(chunk_size, disk_file);
    let mut size_bytes: file::SizeBytes = 0;
    let mut checksum_context = md5::Context::new();
    loop {
        let buffer = reader.fill_buf().ok()?;
        checksum_context.consume(buffer);
        let length = buffer.len();
        size_bytes += length as file::SizeBytes;
        if length == 0 {
            break;
        }
        reader.consume(length);
    }
    let rel_path = path.strip_prefix(root).unwrap().to_path_buf();
    return Some(File::new(rel_path, size_bytes, checksum_context.compute()));
}
