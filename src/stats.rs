use crate::file;
use file::{File, SizeBytes};

#[derive(Debug)]
pub struct Stats {
    files_count: u64,
    size: SizeBytes,
}

impl Stats {
    pub fn new() -> Stats {
        return Stats {
            files_count: 0,
            size: 0,
        };
    }

    pub fn new_with_file_storage() -> Stats {
        return Stats {
            files_count: 0,
            size: 0,
        };
    }

    pub fn record(&mut self, f: &File) {
        self.size += f.size_bytes;
        self.files_count += 1;
    }

    pub fn size(&self) -> SizeBytes {
        return self.size;
    }

    pub fn files_count(&self) -> u64 {
        return self.files_count;
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
        assert_eq!(r.size(), 0);
        assert_eq!(r.files_count(), 0);
    }

    #[test]
    fn stats_records_file() {
        let mut r = Stats::new();
        r.record(&File::from_strings("/tmp/x", "Foo"));
        assert_eq!(r.size(), 3);
        assert_eq!(r.files_count(), 1);
    }
}
