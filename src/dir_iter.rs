use crate::error::Error;
use crate::stats::Stats;
use std::cmp::Ordering;
use std::{fs, path};

pub struct DirIterator {
    small_file_threshold: u64,
    large_files: PathList,
    small_files: PathList,
    scan_stats: ScanStats,
}

impl DirIterator {
    pub fn new(small_file_threshold: u64) -> DirIterator {
        return DirIterator {
            small_file_threshold,
            large_files: PathList::new(),
            small_files: PathList::new(),
            scan_stats: ScanStats {
                scheduled_files: Stats::new(),
                skipped_folders: 0,
                skipped_files: 0,
            },
        };
    }

    pub fn scan(&mut self, root: &path::Path) -> Result<ScanStats, Error> {
        self.scan_dir(root)?;
        self.large_files.paths.sort_by(|(_, s1), (_, s2)| {
            return if s1 > s2 {
                Ordering::Less
            } else if s1 < s2 {
                Ordering::Greater
            } else {
                Ordering::Equal
            };
        });
        return Ok(self.scan_stats);
    }

    fn scan_dir(&mut self, path: &path::Path) -> Result<(), Error> {
        if !path.is_dir() {
            return Ok(());
        }
        for read_res in fs::read_dir(path).map_err(|e| {
            self.scan_stats.skipped_folders += 1;
            return Error::from(
                format!("cannot read directory: {}", path.display()),
                e.to_string(),
            );
        })? {
            let p = read_res
                .map_err(|e| {
                    return Error::from(
                        format!("cannot inspect files in directory: {}", path.display()),
                        e.to_string(),
                    );
                })
                .map(|r| r.path())?;
            if p.is_dir() {
                self.scan_dir(&p)?;
            } else {
                fs::metadata(&p)
                    .map(|m| {
                        let size = m.len();
                        self.scan_stats.scheduled_files.record(size);
                        if size > self.small_file_threshold - 1 {
                            self.large_files.paths.push((p.to_path_buf(), size));
                        } else {
                            self.small_files.paths.push((p.to_path_buf(), size));
                        }
                    })
                    .unwrap_or_else(|_| {
                        self.scan_stats.skipped_files += 1;
                    });
            }
        }
        return Ok(());
    }

    pub fn next_file(&mut self) -> Option<path::PathBuf> {
        return self.large_files.next().or_else(|| self.small_files.next());
    }
}

#[derive(Copy, Clone)]
pub struct ScanStats {
    pub scheduled_files: Stats,
    pub skipped_folders: u64,
    pub skipped_files: u64,
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
