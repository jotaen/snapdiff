use std::io;
use std::io::Write;
use std::time::SystemTime;
use stats::Stats;
use crate::{stats};
use crate::util::dec_format;

#[derive(Debug)]
pub struct Progress {
    prefix: &'static str,
    initialised: SystemTime,
    last_trigger: SystemTime,
}

impl Progress {
    pub fn new(prefix: &'static str) -> Progress {
        return Progress{
            prefix,
            initialised: SystemTime::now(),
            last_trigger: SystemTime::now(),
        }
    }

    pub fn update(&mut self, s: Stats) {
        if self.last_trigger.elapsed().unwrap().as_millis() < 800 {
            return;
        }
        self.last_trigger = SystemTime::now();
        self.print(s);
    }

    pub fn done(&mut self, s: Stats) {
        self.print(s);
        print!("  Done.\n");
    }

    fn print(&mut self, s: Stats) {
        print!(
            "\rProcessing {}: {} files, {} bytes ({}s)",
            self.prefix,
            dec_format(s.files_count() as i128),
            dec_format(s.size() as i128),
            self.initialised.elapsed().unwrap().as_secs()
        );
        io::stdout().flush().unwrap();
    }
}
