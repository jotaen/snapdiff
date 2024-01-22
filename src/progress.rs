use crate::file::SizeBytes;
use crate::format::{dec, duration_human, percent, size_human};
use crate::printer::{Colours, Printer};
use crate::stats::Count;
use std::fmt::Debug;
use std::io;
use std::io::Write;
use std::time::Instant;

#[derive(Debug)]
pub struct Progress<P: Printer> {
    printer: P,
    display_name: &'static str,
    initialised: Instant,
    last_trigger: Instant,
    bytes_since_last_trigger: SizeBytes,
    current: Count,
    expected: Count,
    previous_files_count: Option<Count>,
}

impl<P: Printer> Progress<P> {
    pub fn new(
        printer: P,
        display_name: &'static str,
        previous_files_count: Option<Count>,
    ) -> Progress<P> {
        let init = Instant::now();
        return Progress {
            printer,
            display_name,
            initialised: init,
            last_trigger: init,
            bytes_since_last_trigger: 0,
            current: Count::new(),
            expected: Count::new(),
            previous_files_count,
        };
    }

    pub fn scan_start(&mut self) {
        let Colours {
            gray: gry,
            reset: rst,
            ..
        } = self.printer.colours();
        self.printer
            .print(format!("{gry}{}: Indexing...{rst}", self.display_name));
    }

    pub fn scan_done(&mut self, scheduled: Count, skipped_files: Count, skipped_folders: Count) {
        self.expected = scheduled;
        let skipped_info = if skipped_files.files > 0 || skipped_folders.files > 0 {
            format!(
                "   (skipped {} files, {} dirs)",
                skipped_files.files, skipped_folders.files,
            )
        } else {
            "".to_string()
        };
        let Colours {
            gray: gry,
            reset: rst,
            ..
        } = self.printer.colours();
        self.printer.print(format!(
            "\r{gry}{}: Indexed:     {: >f$} files  {: >7}{}{rst}\n",
            self.display_name,
            dec(self.expected.files as i128),
            size_human(self.expected.size),
            skipped_info,
            f = self.files_display_length(),
        ));
        io::stdout().flush().unwrap();
    }

    pub fn process_inc(&mut self, files_added: u64, bytes_added: SizeBytes) {
        self.current.files += files_added;
        self.current.size += bytes_added;
        self.bytes_since_last_trigger += bytes_added;
        let elapsed_ms = self.last_trigger.elapsed().as_millis();
        if self.initialised != self.last_trigger && elapsed_ms < 666 {
            return;
        }
        let rate = if elapsed_ms != 0 {
            let s = (1000 * self.bytes_since_last_trigger as u128 / elapsed_ms) as SizeBytes;
            format!("[{: >7}/s]", size_human(s))
        } else {
            "".to_string()
        };
        self.print_process(rate);
        self.last_trigger = Instant::now();
        self.bytes_since_last_trigger = 0;
    }

    pub fn process_done(&mut self) {
        self.print_process("           ".to_string());
        self.printer.print("\n".to_string());
    }

    fn print_process(&mut self, rate: String) {
        let indent = " ".repeat(self.display_name.len() + 2);
        let Colours {
            gray: gry,
            reset: rst,
            ..
        } = self.printer.colours();
        self.printer.print(format!(
            "\r{}{gry}Processing:  {: >f$} files  {: >7}   {: >5}    {: >3}   {}{rst}",
            indent,
            dec(self.current.files as i128),
            size_human(self.current.size),
            percent(self.current.size, self.expected.size),
            duration_human(self.initialised.elapsed().as_secs()),
            rate,
            f = self.files_display_length(),
        ));
        io::stdout().flush().unwrap();
    }

    fn files_display_length(&self) -> usize {
        let extra_padding = 1000;
        let count = self
            .previous_files_count
            .map(|c| c.files + extra_padding)
            .unwrap_or(self.expected.files + extra_padding);
        return dec(count as i128).len();
    }
}

#[cfg(test)]
mod tests {
    use crate::printer::MockPrinter;
    use crate::progress::Progress;
    use crate::stats::Count;
    use std::ops::Sub;
    use std::time::{Duration, Instant};

    #[test]
    fn print_process() {
        let p = MockPrinter::new();
        let mut progress = Progress::new(p, "Snap", None);
        progress.scan_start();
        assert_eq!(progress.printer.flush(), "Snap: Indexing...");
        let count = {
            let mut c = Count::new();
            c.add(61772, 57718293);
            c
        };
        progress.scan_done(count, Count::new(), Count::new());
        assert_eq!(
            progress.printer.flush(),
            "\rSnap: Indexed:     61,772 files   57.7 M\n"
        );
        progress.process_inc(182, 75913);
        assert_eq!(
            progress.printer.flush(),
            "\r      Processing:     182 files   75.9 K     0 %     0s   "
        );
        progress.last_trigger = Instant::now().sub(Duration::from_millis(600));
        assert_eq!(progress.printer.flush(), "");
        progress.last_trigger = Instant::now().sub(Duration::from_millis(700));
        progress.process_inc(8172, 1312425);
        assert_eq!(
            progress.printer.flush(),
            "\r      Processing:   8,354 files    1.3 M     2 %     0s   [  1.8 M/s]"
        );
        progress.process_inc(53418, 56329955);
        progress.process_done();
        assert_eq!(
            progress.printer.flush(),
            "\r      Processing:  61,772 files   57.7 M   100 %     0s              \n"
        );
    }

    #[test]
    fn print_process_with_alignment() {
        let p = MockPrinter::new();
        let previous = {
            let mut c = Count::new();
            c.add(158176, 1902);
            c
        };
        let mut progress = Progress::new(p, "Snap", Some(previous));
        progress.scan_start();
        progress.printer.flush(); // Indexing...
        let count = {
            let mut c = Count::new();
            c.add(3, 910);
            c
        };
        progress.scan_done(count, Count::new(), Count::new());
        assert_eq!(
            progress.printer.flush(),
            "\rSnap: Indexed:           3 files    910 B\n"
        );
    }
}
