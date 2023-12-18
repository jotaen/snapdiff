use crate::error::Error;
use crate::file::SizeBytes;
use crate::progress::Progress;
use crate::report::ScanStats;
use crate::snapper::{open_file, CHUNK_SIZE};
use std::cmp::Ordering;
use std::{fs, path};

pub struct DirIterator {
    large_files: PathList,
    small_files: PathList,
    pub root: path::PathBuf,
    pub scan_stats: ScanStats,
    num_workers: usize,
}

impl DirIterator {
    pub fn scan(
        num_workers: usize,
        root: &path::Path,
        progress: &mut Progress,
    ) -> Result<DirIterator, Error> {
        progress.scan_start();
        let mut dir_it = DirIterator {
            root: root.to_path_buf(),
            large_files: PathList::new(),
            small_files: PathList::new(),
            scan_stats: ScanStats {
                scheduled_files_count: 0,
                scheduled_size: 0,
                skipped_folders: 0,
                skipped_files: 0,
            },
            num_workers,
        };
        dir_it = scan_dir(dir_it, root)?;
        dir_it.large_files.paths.sort_by(|(_, s1), (_, s2)| {
            return if s1 > s2 {
                Ordering::Less
            } else if s1 < s2 {
                Ordering::Greater
            } else {
                Ordering::Equal
            };
        });
        progress.scan_done(&dir_it.scan_stats);
        return Ok(dir_it);
    }

    fn push(&mut self, p: path::PathBuf, size: SizeBytes) {
        self.scan_stats.scheduled_files_count += 1;
        self.scan_stats.scheduled_size += size;
        if size > CHUNK_SIZE && self.num_workers > 1 {
            self.large_files.paths.push((p.to_path_buf(), size));
        } else {
            self.small_files.paths.push((p.to_path_buf(), size));
        }
    }

    fn skipped_file(&mut self) {
        self.scan_stats.skipped_folders += 1;
    }

    fn skipped_folder(&mut self) {
        self.scan_stats.skipped_files += 1;
    }

    pub fn next_file(&mut self) -> Option<path::PathBuf> {
        return self.large_files.next().or_else(|| self.small_files.next());
    }
}

fn scan_dir(mut dir_it: DirIterator, path: &path::Path) -> Result<DirIterator, Error> {
    if !path.is_dir() {
        return Ok(dir_it);
    }
    let read_dir_result = fs::read_dir(path).map_err(|e| {
        dir_it.skipped_file();
        return Error::from(
            format!("cannot read directory: {}", path.display()),
            e.to_string(),
        );
    });
    if !read_dir_result.is_ok() {
        return Ok(dir_it);
    }
    for read_res in read_dir_result? {
        let p = read_res
            .map_err(|e| {
                return Error::from(
                    format!("cannot inspect files in directory: {}", path.display()),
                    e.to_string(),
                );
            })
            .map(|r| r.path())?;
        if p.is_dir() {
            dir_it = scan_dir(dir_it, &p)?;
        } else {
            open_file(&p)
                .map(|f| {
                    let m = f.metadata().expect("failed to query file metadata");
                    dir_it.push(p, m.len());
                })
                .unwrap_or_else(|_| {
                    dir_it.skipped_folder();
                });
        }
    }
    return Ok(dir_it);
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
