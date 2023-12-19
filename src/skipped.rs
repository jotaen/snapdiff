use crate::stats::Count;
use std::ffi::OsString;
use std::path;

#[derive(Copy, Clone)]
pub struct Skipped {
    pub files: Count,
    pub folders: Count,
    symlinks: bool,
    dot_paths: bool,
}

const DOT_PREFIX: &str = ".";

impl Skipped {
    pub fn new(skip_symlinks: bool, skip_dots_paths: bool) -> Skipped {
        return Skipped {
            symlinks: skip_symlinks,
            dot_paths: skip_dots_paths,
            files: Count::new(),
            folders: Count::new(),
        };
    }

    pub fn record_if_match(&mut self, p: &path::Path, name: &OsString) -> bool {
        if self.symlinks && p.is_symlink() {
            self.record(p);
            return true;
        }
        if self.dot_paths
            && name
                .to_str()
                .map(|n| n.starts_with(DOT_PREFIX))
                .unwrap_or(false)
        {
            self.record(p);
            return true;
        }
        return false;
    }

    pub fn record_file(&mut self) {
        self.files.files += 1;
    }

    pub fn record_folder(&mut self) {
        self.folders.files += 1;
    }

    fn record(&mut self, p: &path::Path) {
        if p.is_dir() {
            self.record_folder();
        } else {
            self.record_file();
        }
    }
}
