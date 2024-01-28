use std::ffi::OsString;
use std::path;

#[derive(Copy, Clone)]
pub struct Filter {
    include_symlinks: bool,
    include_dot_paths: bool,
}

const DOT_PREFIX: &str = ".";

pub enum MatchReason {
    IsSymlink,
    IsDotPath,
}

impl Filter {
    pub fn new(include_symlinks: bool, include_dot_paths: bool) -> Filter {
        return Filter {
            include_symlinks,
            include_dot_paths,
        };
    }

    pub fn matches(&self, p: &path::Path, name: &OsString) -> Option<MatchReason> {
        if !self.include_symlinks && p.is_symlink() {
            return Some(MatchReason::IsSymlink);
        }
        if !self.include_dot_paths
            && name
                .to_str()
                .map(|n| n.starts_with(DOT_PREFIX))
                .unwrap_or(false)
        {
            return Some(MatchReason::IsDotPath);
        }
        return None;
    }
}
