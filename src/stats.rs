use file::{File, SizeBytes};
use crate::file;

#[derive(Debug, Copy, Clone)]
pub struct Stats {
    files_count: u64,
    size: SizeBytes,
}

impl Stats {
    pub fn new() -> Stats {
        return Stats { files_count: 0, size: 0 };
    }

    pub fn record_file(&mut self, f: &File) {
        self.record(f.size_bytes);
    }

    pub fn record(&mut self, s: SizeBytes) {
        self.size += s;
        self.files_count += 1;
    }

    pub fn size(&self) -> SizeBytes {
        return self.size;
    }

    pub fn files_count(&self) -> u64 {
        return self.files_count;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StatsDelta {
    files_count: u64,
    gain: SizeBytes,
    loss: SizeBytes,
}

impl StatsDelta {
    pub fn new() -> StatsDelta {
        return StatsDelta { files_count: 0, gain: 0, loss: 0 };
    }

    pub fn record(&mut self, f1: &File, f2: &File) {
        let delta: i128 = (f1.size_bytes as i64 - f2.size_bytes as i64) as i128;
        if delta <= 0 {
            self.loss += (-1 * delta) as SizeBytes;
        } else {
            self.gain += delta as SizeBytes;
        }
        self.files_count += 1;
    }

    pub fn diff(&self) -> i128 {
        return self.gain as i128 - self.loss as i128;
    }

    pub fn gain(&self) -> SizeBytes {
        return self.gain;
    }

    pub fn loss(&self) -> SizeBytes {
        return self.loss;
    }

    pub fn files_count(&self) -> u64 {
        return self.files_count;
    }
}

#[cfg(test)]
mod tests {
    use stats::Stats;
    use crate::stats;
    use crate::file;

    #[test]
    fn new_stats_is_empty() {
        let r = Stats::new();
        assert_eq!(r.size(), 0);
        assert_eq!(r.files_count(), 0);
    }

    #[test]
    fn stats_records_file() {
        let mut r = Stats::new();
        r.record_file(&file::from_strings("/tmp/x", "Foo"));
        assert_eq!(r.size(), 3);
        assert_eq!(r.files_count(), 1);
    }
}
