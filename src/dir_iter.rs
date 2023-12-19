use crate::error::Error;
use crate::file::SizeBytes;
use crate::printer::TerminalPrinter;
use crate::progress::Progress;
use crate::skipped::Skipped;
use crate::snapper::{open_file, CHUNK_SIZE};
use crate::stats::Stats;
use std::cmp::Ordering;
use std::{fs, path};

pub struct DirIterator {
    large_files: PathList,
    small_files: PathList,
    pub root: path::PathBuf,
    pub scheduled: Stats,
    skipped: Skipped,
    num_workers: usize,
}

impl DirIterator {
    pub fn scan(
        num_workers: usize,
        root: &path::Path,
        skipped: Skipped,
        progress: &mut Progress<TerminalPrinter>,
    ) -> Result<DirIterator, Error> {
        progress.scan_start();
        let mut dir_it = DirIterator {
            root: root.to_path_buf(),
            large_files: PathList::new(),
            small_files: PathList::new(),
            scheduled: Stats::new(),
            skipped,
            num_workers,
        };
        dir_it.scan_dir(root)?;
        dir_it.large_files.paths.sort_by(|(_, s1), (_, s2)| {
            return if s1 > s2 {
                Ordering::Less
            } else if s1 < s2 {
                Ordering::Greater
            } else {
                Ordering::Equal
            };
        });
        progress.scan_done(
            dir_it.scheduled.count,
            dir_it.skipped.files,
            dir_it.skipped.folders,
        );
        return Ok(dir_it);
    }

    fn scan_dir(&mut self, path: &path::Path) -> Result<(), Error> {
        if !path.is_dir() {
            return Err(Error::new(format!("not a directory: {}", path.display())));
        }
        let read_dir_result = fs::read_dir(path).map_err(|e| {
            self.skipped.record_file();
            return Error::from(
                format!("cannot read directory: {}", path.display()),
                e.to_string(),
            );
        });
        if !read_dir_result.is_ok() {
            return Ok(());
        }
        for read_res in read_dir_result? {
            let (p, name) = read_res
                .map_err(|e| {
                    return Error::from(
                        format!("cannot inspect files in directory: {}", path.display()),
                        e.to_string(),
                    );
                })
                .map(|r| (r.path(), r.file_name()))?;
            if self.skipped.record_if_match(&p, &name) {
                continue;
            }
            if p.is_dir() {
                self.scan_dir(&p)?;
            } else if p.is_file() {
                open_file(&p)
                    .map(|f| {
                        let m = f.metadata().expect("failed to query file metadata");
                        self.push(p, m.len());
                    })
                    .unwrap_or_else(|_| {
                        self.skipped.record_folder();
                    });
            }
        }
        return Ok(());
    }

    fn push(&mut self, p: path::PathBuf, size: SizeBytes) {
        self.scheduled.count.add(1, size);
        if size > CHUNK_SIZE && self.num_workers > 1 {
            self.large_files.paths.push((p.to_path_buf(), size));
        } else {
            self.small_files.paths.push((p.to_path_buf(), size));
        }
    }

    pub fn next_file(&mut self) -> Option<path::PathBuf> {
        return self.large_files.next().or_else(|| self.small_files.next());
    }
}

struct PathList {
    paths: Vec<(path::PathBuf, u64)>,
    it: usize,
}

impl PathList {
    fn new() -> PathList {
        return PathList {
            paths: vec![],
            it: 0,
        };
    }

    fn next(&mut self) -> Option<path::PathBuf> {
        if self.it >= self.paths.len() {
            return None;
        }
        let (p, _) = &self.paths[self.it];
        self.it += 1;
        return Some(p.to_path_buf());
    }
}
