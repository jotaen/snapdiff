use std::io;
use std::io::Write;
use std::time::{Instant};
use crate::file::SizeBytes;
use crate::format::{dec, duration_human, size_human};
use crate::format::term::*;

#[derive(Debug)]
pub struct Progress {
    prefix: &'static str,
    initialised: Instant,
    last_trigger: Instant,
    files_count: u64,
    size: SizeBytes,
}

impl Progress {
    pub fn new(prefix: &'static str) -> Progress {
        let init = Instant::now();
        return Progress{
            prefix,
            initialised: init,
            last_trigger: init,
            files_count: 0,
            size: 0,
        }
    }

    pub fn scan(&self) {
        print!("Scanning {}...", self.prefix);
        io::stdout().flush().unwrap();
    }

    pub fn increment(&mut self, files_added: u64, bytes_added: SizeBytes) {
        self.files_count += files_added;
        self.size += bytes_added;
        if self.initialised != self.last_trigger && self.last_trigger.elapsed().as_millis() < 500 {
            return;
        }
        self.print(true);
        self.last_trigger = Instant::now();
    }

    pub fn done(&self) {
        self.print(false);
        print!("\n");
    }

    fn print(&self, is_ongoing: bool) {
        let mut rate = "".to_string();
        let elapsed_ms = self.initialised.elapsed().as_millis();
        if !is_ongoing {
            rate = "[Complete.]".to_string()
        } else if elapsed_ms != 0 {
            let s = (1000 * self.size as u128 / elapsed_ms) as SizeBytes;
            rate = format!("[{: >7}/s]", size_human(s));
        }
        print!(
            "\r{GRY}Processing {}:   {: >3} files   {: >7}   {: >4}   {} {RST}",
            self.prefix,
            dec(self.files_count as i128),
            size_human(self.size),
            duration_human(self.initialised.elapsed().as_secs()),
            rate,
        );
        io::stdout().flush().unwrap();
    }
}
