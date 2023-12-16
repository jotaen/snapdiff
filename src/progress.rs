use std::io;
use std::io::Write;
use std::time::{Instant};
use crate::dir_iter::ScanStats;
use crate::file::SizeBytes;
use crate::format::{dec, duration_human, size_human};
use crate::format::term::*;

#[derive(Debug)]
pub struct Progress {
    display_name: &'static str,
    initialised: Instant,
    last_trigger: Instant,
    files_count: u64,
    size: SizeBytes,
}

impl Progress {
    pub fn new(display_name: &'static str) -> Progress {
        let init = Instant::now();
        return Progress{
            display_name,
            initialised: init,
            last_trigger: init,
            files_count: 0,
            size: 0,
        }
    }

    pub fn scan_start(&self) {
        print!("{GRY}{}:  Scanning...{RST}", self.display_name);
        io::stdout().flush().unwrap();
    }

    pub fn scan_done(&self, s: ScanStats) {
        print!(
            "{GRY} {} files scheduled (skipped {} files and {} folders){RST}\n",
            s.scheduled_files, s.skipped_files, s.skipped_folders
        );
        io::stdout().flush().unwrap();
    }

    pub fn process_inc(&mut self, files_added: u64, bytes_added: SizeBytes) {
        self.files_count += files_added;
        self.size += bytes_added;
        if self.initialised != self.last_trigger && self.last_trigger.elapsed().as_millis() < 500 {
            return;
        }
        self.print_progress(true);
        self.last_trigger = Instant::now();
    }

    pub fn process_done(&self) {
        self.print_progress(false);
        print!("\n");
    }

    fn print_progress(&self, is_ongoing: bool) {
        let mut rate = "".to_string();
        let elapsed_ms = self.initialised.elapsed().as_millis();
        if !is_ongoing {
            rate = "[Complete.]".to_string()
        } else if elapsed_ms != 0 {
            let s = (1000 * self.size as u128 / elapsed_ms) as SizeBytes;
            rate = format!("[{: >7}/s]", size_human(s));
        }
        let indent = " ".repeat(self.display_name.len()+3);
        print!(
            "\r{}{GRY}Processing...   {: >3} files   {: >7}   {: >4}   {} {RST}",
            indent,
            dec(self.files_count as i128),
            size_human(self.size),
            duration_human(self.initialised.elapsed().as_secs()),
            rate,
        );
        io::stdout().flush().unwrap();
    }
}
