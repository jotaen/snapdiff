use crate::checksum::CheckSummer;
use crate::cli::CtrlCSignal;
use crate::dir_iter::DirIterator;
use crate::progress::Progress;
use crate::snapshot::Snapshot;
use crate::{file, Error};
use file::File;
use std::io::BufRead;
use std::ops::DerefMut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{fs, io, thread};

pub const CHUNK_SIZE: u64 = 1024 * 1024 * 10; // ~10MB

pub struct Snapper {
    name: &'static str,
    num_workers: usize,
    ctrl_csignal: CtrlCSignal,
}

impl Snapper {
    pub fn new(name: &'static str, num_workers: usize, ctrl_csignal: CtrlCSignal) -> Snapper {
        return Snapper {
            name,
            num_workers,
            ctrl_csignal,
        };
    }

    pub fn process<S>(&self, dir_it: DirIterator, snap: S) -> Result<S, Error>
    where
        S: Snapshot + std::fmt::Debug + Send + 'static,
    {
        let mut progress = Progress::new(self.name);
        progress.scan_start();
        progress.scan_done(&dir_it.scan_stats);

        let dir_it_arc = Arc::new(Mutex::new(dir_it));
        let snap_arc = Arc::new(Mutex::new(snap));
        let progress_arc = Arc::new(Mutex::new(progress));
        let mut handles = vec![];

        for _i in 0..self.num_workers {
            let handle = spawn_worker(
                Arc::clone(&dir_it_arc),
                Arc::clone(&snap_arc),
                Arc::clone(&progress_arc),
                Arc::clone(&self.ctrl_csignal),
            );
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap()?;
        }

        let snap = Arc::try_unwrap(snap_arc).unwrap().into_inner().unwrap();
        Arc::try_unwrap(progress_arc)
            .unwrap()
            .into_inner()
            .unwrap()
            .process_done();
        return Ok(snap);
    }
}

fn spawn_worker<S>(
    dir_it_mtx: Arc<Mutex<DirIterator>>,
    snap_mtx: Arc<Mutex<S>>,
    progress_mtx: Arc<Mutex<Progress>>,
    ctrlc_mtx: Arc<AtomicBool>,
) -> JoinHandle<Result<(), Error>>
where
    S: Snapshot + std::fmt::Debug + Send + 'static,
{
    return thread::spawn(move || -> Result<(), Error> {
        {
            let mut p = progress_mtx.lock().unwrap();
            p.process_inc(0, 0 as file::SizeBytes);
        }
        loop {
            let (p, root) = {
                let (entry, root) = {
                    let mut di_mtx = dir_it_mtx.lock().unwrap();
                    let di = di_mtx.deref_mut();
                    (di.next_file(), di.root.to_path_buf())
                };
                if entry.is_none() {
                    break;
                }
                (entry.unwrap(), root)
            };

            let disk_file = fs::File::options().read(true).open(&p).map_err(|e| {
                return Error::from(format!("cannot open file: {}", p.display()), e.to_string());
            })?;
            let mut reader = io::BufReader::with_capacity(CHUNK_SIZE as usize, disk_file);
            let mut size_bytes: file::SizeBytes = 0;
            let mut checksummer = CheckSummer::new();
            loop {
                if ctrlc_mtx.load(Ordering::SeqCst) {
                    println!();
                    std::process::exit(255);
                }
                let buffer = reader.fill_buf().map_err(|e| {
                    return Error::from(
                        format!("failed to read from file: {}", p.display()),
                        e.to_string(),
                    );
                })?;
                let length = buffer.len();
                if length == 0 {
                    break;
                }
                checksummer.consume(&buffer);
                size_bytes += length as file::SizeBytes;
                reader.consume(length);
                {
                    let mut p = progress_mtx.lock().unwrap();
                    p.process_inc(0, length as file::SizeBytes);
                }
            }

            let rel_path = p.strip_prefix(&root).unwrap().to_path_buf();
            let f = File::new(rel_path, size_bytes, checksummer.finalize());

            {
                let mut s = snap_mtx.lock().unwrap();
                s.deref_mut().add(f);
            }
            {
                let mut p = progress_mtx.lock().unwrap();
                p.process_inc(1, 0);
            }
        }
        Ok(())
    });
}
