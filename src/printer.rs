use std::io;
use std::io::Write;

pub const SNP1: &str = "Snap 1";
pub const SNP2: &str = "Snap 2";

#[derive(Debug)]
pub struct Printer {
    pub blank: &'static str,
    pub reset: &'static str,
    pub bold: &'static str,
    pub light: &'static str,
    pub gray: &'static str,
    pub dark: &'static str,
    pub blue: &'static str,
    pub green: &'static str,
    pub red: &'static str,
    pub yellow: &'static str,
    pub brown: &'static str,
}

impl Printer {
    pub fn new() -> Printer {
        return Printer {
            blank: "",
            reset: "\x1b[0m",
            bold: "\x1b[1m",
            light: "\x1b[38;5;253m",
            gray: "\x1b[38;5;246m",
            dark: "\x1b[38;5;237m",
            blue: "\x1b[38;5;039m",
            green: "\x1b[38;5;082m",
            red: "\x1b[38;5;202m",
            yellow: "\x1b[38;5;220m",
            brown: "\x1b[38;5;094m",
        };
    }

    pub fn new_plain() -> Printer {
        return Printer {
            blank: "",
            reset: "",
            bold: "",
            light: "",
            gray: "",
            dark: "",
            blue: "",
            green: "",
            red: "",
            yellow: "",
            brown: "",
        };
    }

    pub fn print(&mut self, text: String) {
        print!("{}", text);
        io::stdout().flush().unwrap();
    }
}
