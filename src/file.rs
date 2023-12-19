use crate::checksum::{CheckSum, CheckSummer};
use std::path;

pub type SizeBytes = u64;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct File {
    pub path: path::PathBuf,
    pub size: SizeBytes,
    pub check_sum: CheckSum,
}

impl File {
    pub fn new(path: path::PathBuf, size_bytes: SizeBytes, check_sum: CheckSum) -> File {
        return File {
            path,
            size: size_bytes,
            check_sum,
        };
    }

    pub fn equals(&self, f2: &File) -> bool {
        return self.check_sum == f2.check_sum && self.size == f2.size;
    }

    #[allow(dead_code)]
    pub fn from_strings(path: &str, contents: &str) -> File {
        return File {
            path: path::Path::new(path).to_path_buf(),
            size: contents.len() as SizeBytes,
            check_sum: CheckSummer::new().consume(contents.as_bytes()).finalize(),
        };
    }
}
