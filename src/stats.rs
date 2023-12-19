use crate::file;
use file::{File, SizeBytes};

#[derive(Debug, Copy, Clone)]
pub struct Count {
    pub files: u64,
    pub size: SizeBytes,
}

impl Count {
    pub fn new() -> Count {
        return Count { files: 0, size: 0 };
    }

    pub fn add(&mut self, files: u64, size: SizeBytes) {
        self.size += size;
        self.files += files;
    }
}

#[derive(Debug)]
pub struct Stats {
    pub count: Count,
    files: Vec<File>,
    shall_store_files: bool,
}

impl Stats {
    pub fn new() -> Stats {
        return Stats {
            count: Count::new(),
            shall_store_files: false,
            files: vec![],
        };
    }

    pub fn new_with_file_storage() -> Stats {
        return Stats {
            count: Count::new(),
            shall_store_files: true,
            files: vec![],
        };
    }

    pub fn record(&mut self, f: &File) {
        self.count.add(1, f.size);
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
        assert_eq!(r.count.size, 0);
        assert_eq!(r.count.files, 0);
    }

    #[test]
    fn stats_records_file() {
        let mut r = Stats::new();
        r.record(&File::from_strings("/tmp/x", "Foo"));
        assert_eq!(r.count.size, 3);
        assert_eq!(r.count.files, 1);
    }
}
