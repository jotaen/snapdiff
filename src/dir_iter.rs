use crate::error::Error;
use crate::file::SizeBytes;
use crate::filter::{Filter, MatchReason};
use crate::printer::TerminalPrinter;
use crate::progress::Progress;
use crate::snapper::{open_file, CHUNK_SIZE};
use crate::stats::Stats;
use std::cmp::Ordering;
use std::{fs, path};

pub struct DirIterator {
    large_files: PathList,
    small_files: PathList,
    pub root: path::PathBuf,
    pub scheduled: Stats,
    filters: Filter,
    skipped: SkippedStats,
    num_workers: usize,
}

impl DirIterator {
    // Traverses the `root` directory recursively, and collects all
    // encountered files (except the ones that are filtered out).
    pub fn scan(
        num_workers: usize,
        root: &path::Path,
        filters: Filter,
        progress: &mut Progress<TerminalPrinter>,
    ) -> Result<DirIterator, Error> {
        progress.scan_start();
        let mut dir_it = DirIterator {
            root: root.to_path_buf(),
            large_files: PathList::new(),
            small_files: PathList::new(),
            scheduled: Stats::new(),
            filters,
            skipped: SkippedStats::new(),
            num_workers,
        };
        dir_it.scan_dir(root)?;

        // Only sort the “large” files, because for the “small” ones
        // the order doesn’t matter (as they fit into one chunk anyway).
        dir_it.large_files.paths.sort_by(|(_, s1), (_, s2)| {
            return if s1 > s2 {
                Ordering::Less
            } else if s1 < s2 {
                Ordering::Greater
            } else {
                Ordering::Equal
            };
        });
        progress.scan_done(dir_it.scheduled.count, dir_it.skipped);
        return Ok(dir_it);
    }

    fn scan_dir(&mut self, path: &path::Path) -> Result<(), Error> {
        if !path.is_dir() {
            return Err(Error::new(format!("not a directory: {}", path.display())));
        }
        let read_dir_result = fs::read_dir(path).map_err(|e| {
            self.skipped.no_opener += 1;
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
            let shall_skip = self
                .filters
                .matches(&p, &name)
                .map(|r| match r {
                    MatchReason::IsSymlink => self.skipped.symlinks += 1,
                    MatchReason::IsDotPath => self.skipped.dot_paths += 1,
                })
                .map(|_| true)
                .unwrap_or(false);
            if shall_skip {
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
                        self.skipped.no_opener += 1;
                    });
            } else if p.is_symlink() {
                self.push(p, 0);
            }
        }
        return Ok(());
    }

    fn push(&mut self, p: path::PathBuf, size: SizeBytes) {
        self.scheduled.count.add(1, size);

        // Sort into “small” and “large” internally files. That way, the “large”
        // files are consumed (hashed) first. This avoids the scenario, where one
        // worker is left over hashing a large file towards the end, when there are
        // no files left for other workers to pick up anymore.
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

// An iterable list of file paths.
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

#[derive(Debug, Copy, Clone)]
pub struct SkippedStats {
    pub dot_paths: u64,
    pub symlinks: u64,
    pub no_opener: u64,
}

impl SkippedStats {
    pub fn new() -> SkippedStats {
        return SkippedStats {
            dot_paths: 0,
            symlinks: 0,
            no_opener: 0,
        };
    }
}
