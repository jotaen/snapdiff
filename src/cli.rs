use crate::printer::Printer;
use crate::Error;
use clap::Parser;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::{fs, path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    snap1_path: String,
    snap2_path: String,

    #[arg(
        long = "workers",
        alias = "worker",
        short = 'w',
        value_delimiter = ':',
        help = "Number of CPU cores to utilise"
    )]
    workers: Option<Vec<usize>>,

    #[arg(
        long = "no-color",
        alias = "no-colour",
        default_value_t = false,
        help = "Disable colouring of output"
    )]
    no_color: bool,
}

impl Cli {
    pub fn snap1(&self) -> Result<&path::Path, Error> {
        return self.get_snap(&self.snap1_path);
    }

    pub fn snap2(&self) -> Result<&path::Path, Error> {
        return self.get_snap(&self.snap2_path);
    }

    pub fn num_workers(&self) -> (usize, usize) {
        let cores = thread::available_parallelism().unwrap().get();
        return self
            .workers
            .as_ref()
            .map(|ws| {
                if ws.len() == 1 {
                    return (ws[0], ws[0]);
                }
                return (ws[0], ws[1]);
            })
            .map(|(mut w1, mut w2)| {
                if w1 == 0 {
                    w1 = cores;
                }
                if w2 == 0 {
                    w2 = cores;
                }
                return (w1, w2);
            })
            .unwrap_or_else(|| {
                return (cores, cores);
            });
    }

    fn get_snap<'a>(&'a self, s: &'a String) -> Result<&path::Path, Error> {
        let m = fs::metadata(s).map_err(|e| {
            return Error::from(format!("cannot open directory: {}", s), e.to_string());
        })?;
        if !m.is_dir() {
            return Err(Error::new(format!("not a directory: {}", s)));
        }
        return Ok(path::Path::new(s));
    }

    pub fn report_file(&self) -> PathBuf {
        return tempfile::NamedTempFile::new()
            .map(|f| {
                return f.path().to_path_buf();
            })
            .unwrap();
    }

    pub fn printer(&self) -> Printer {
        if self.no_color {
            return Printer::new_plain();
        }
        return Printer::new();
    }
}

pub type CtrlCSignal = Arc<AtomicBool>;

pub fn handle_ctrl_c() -> CtrlCSignal {
    let ctrlc_arc = Arc::new(AtomicBool::new(false));
    let r = ctrlc_arc.clone();
    ctrlc::set_handler(move || {
        r.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    return ctrlc_arc;
}
