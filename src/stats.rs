use crate::file;
use file::{File, SizeBytes};

#[derive(Debug)]
pub struct Stats {
    pub files_count: u64,
    pub size: SizeBytes,
    files: Vec<File>,
    shall_store_files: bool,
}

impl Stats {
    pub fn new() -> Stats {
        return Stats {
            files_count: 0,
            size: 0,
            shall_store_files: false,
            files: vec![],
        };
    }

    pub fn new_with_file_storage() -> Stats {
        return Stats {
            files_count: 0,
            size: 0,
            shall_store_files: true,
            files: vec![],
        };
    }

    pub fn record(&mut self, f: &File) {
        self.size += f.size_bytes;
        self.files_count += 1;
        if self.shall_store_files {
            self.files.push(f.clone());
        }
    }

    pub fn files(&self) -> Option<&Vec<File>> {
        if !self.shall_store_files {
            return None;
        }
        return Some(&self.files);
    }
}

#[cfg(test)]
mod tests {
    use crate::file::File;
    use crate::stats;
    use stats::Stats;

    #[test]
    fn new_stats_is_empty() {
        let r = Stats::new();
        assert_eq!(r.size, 0);
        assert_eq!(r.files_count, 0);
    }

    #[test]
    fn stats_records_file() {
        let mut r = Stats::new();
        r.record(&File::from_strings("/tmp/x", "Foo"));
        assert_eq!(r.size, 3);
        assert_eq!(r.files_count, 1);
    }
}
