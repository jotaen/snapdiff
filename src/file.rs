use std::path;

pub(crate) type ContentsHash = md5::Digest;

pub(crate) type SizeBytes = u64;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct File {
    pub path: path::PathBuf,
    pub size_bytes: SizeBytes,
    pub checksum: ContentsHash,
}

impl File {
    pub fn new(path: path::PathBuf, size_bytes: SizeBytes, checksum: ContentsHash) -> File {
        return File {
            path,
            size_bytes,
            checksum,
        };
    }
}

#[allow(dead_code)]
pub fn from_strings(path: &str, contents: &str) -> File {
    return File {
        path: path::Path::new(path).to_path_buf(),
        size_bytes: contents.len() as SizeBytes,
        checksum: md5::compute(contents),
    };
}
