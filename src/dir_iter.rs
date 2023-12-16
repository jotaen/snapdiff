use std::{fs, path};
use std::cmp::Ordering;

pub struct DirIterator {
    small_file_threshold: u64,
    large_files: PathList,
    small_files: PathList,
    scan_stats: ScanStats,
}

impl DirIterator {
    pub fn new(small_file_threshold: u64) -> DirIterator {
        return DirIterator{
            small_file_threshold,
            large_files: PathList::new(),
            small_files: PathList::new(),
            scan_stats: ScanStats{
                scheduled_files: 0,
                skipped_folders: 0,
                skipped_files: 0,
            }
        }
    }

    pub fn scan(&mut self, root: &path::Path) -> ScanStats {
        self.scan_dir(root);
        self.large_files.paths.sort_by(|(_, s1), (_, s2)| {
            return if s1 > s2 {
                Ordering::Less
            } else if s1 < s2 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        return self.scan_stats;
    }

    fn scan_dir(&mut self, path: &path::Path) {
        if !path.is_dir() {
            return;
        }
        if fs::read_dir(path).is_err() {
            self.scan_stats.skipped_folders += 1;
            return;
        }
        for read_res in fs::read_dir(path).unwrap() {
            let p = read_res.expect("Cannot determine path").path();
            if p.is_dir() {
                self.scan_dir(&p);
            } else {
                fs::metadata(&p).map(|m| {
                    let size = m.len();
                    self.scan_stats.scheduled_files += 1;
                    if size > self.small_file_threshold - 1 {
                        self.large_files.paths.push((p.to_path_buf(), size));
                    } else {
                        self.small_files.paths.push((p.to_path_buf(), size));
                    }
                }).unwrap_or_else(|_| {
                    self.scan_stats.skipped_files += 1;
                });
            }
        }
    }

    pub fn next_file(&mut self) -> Option<path::PathBuf> {
        return self.large_files.next().or_else(|| self.small_files.next());
    }
}

#[derive(Copy, Clone)]
pub struct ScanStats {
    pub scheduled_files: u64,
    pub skipped_folders: u64,
    pub skipped_files: u64,
}

struct PathList {
    paths: Vec<(path::PathBuf, u64)>,
    it: usize,
}

impl PathList {
    fn new() -> PathList {
        return PathList{
            paths: vec![],
            it: 0,
        }
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
