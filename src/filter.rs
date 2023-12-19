use crate::stats::Count;
use std::ffi::OsString;
use std::path;

#[derive(Copy, Clone)]
pub struct Filter {
    pub skipped_files: Count,
    pub skipped_folders: Count,
    include_symlinks: bool,
    include_dot_paths: bool,
}

const DOT_PREFIX: &str = ".";

impl Filter {
    pub fn new(include_symlinks: bool, include_dot_paths: bool) -> Filter {
        return Filter {
            include_symlinks,
            include_dot_paths,
            skipped_files: Count::new(),
            skipped_folders: Count::new(),
        };
    }

    pub fn is_filtered(&self, p: &path::Path, name: &OsString) -> bool {
        if !self.include_symlinks && p.is_symlink() {
            return true;
        }
        if !self.include_dot_paths
            && name
                .to_str()
                .map(|n| n.starts_with(DOT_PREFIX))
                .unwrap_or(false)
        {
            return true;
        }
        return false;
    }

    pub fn track_skipped_file(&mut self, increment: u64) {
        self.skipped_files.files += increment;
    }

    pub fn track_skipped_folder(&mut self, increment: u64) {
        self.skipped_folders.files += increment;
    }

    pub fn track_skipped(&mut self, p: &path::Path) {
        if p.is_dir() {
            self.track_skipped_folder(1);
        } else {
            self.track_skipped_file(1);
        }
    }
}
