use std::{path};
use walkdir::{IntoIter, WalkDir};

pub struct DirIterator {
    it: IntoIter,
}

impl DirIterator {
    pub fn new(root: &path::Path) -> DirIterator {
        return DirIterator{
            it: WalkDir::new(root).into_iter(),
        }
    }

    pub fn next(&mut self) -> Option<path::PathBuf> {
        return self.it.next().map(|r| {
            r.unwrap().path().to_path_buf()
        });
    }
}
