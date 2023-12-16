use std::{fs, path};
use std::cmp::Ordering;

pub struct DirIterator {
    small_file_threshold: u64,
    large_files: PathList,
    small_files: PathList,
}

impl DirIterator {
    pub fn new(small_file_threshold: u64) -> DirIterator {
        return DirIterator{
            small_file_threshold,
            large_files: PathList::new(),
            small_files: PathList::new(),
        }
    }

    pub fn scan(&mut self, root: &path::Path) {
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
    }

    fn scan_dir(&mut self, path: &path::Path) {
        if !path.is_dir() {
            return;
        }
        for entry in fs::read_dir(path).expect("Cannot read directory") {
            let p = entry.expect("Cannot determine path").path();
            if p.is_dir() {
                self.scan_dir(&p);
            } else {
                let size = fs::metadata(&p).unwrap().len();
                if size > self.small_file_threshold - 1 {
                    self.large_files.paths.push((p.to_path_buf(), size));
                } else {
                    self.small_files.paths.push((p.to_path_buf(), size));
                }
            }
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
