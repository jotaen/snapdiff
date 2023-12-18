use crate::printer::{FilePrinter, TerminalPrinter};
use crate::Error;
use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::{fs, path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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
        long = "report",
        alias = "report-file",
        short = 'r',
        help = "Print detailed report to file"
    )]
    report_file: String,

    #[arg(
        long = "no-color",
        alias = "no-colour",
        default_value_t = false,
        help = "Disable colouring of output"
    )]
    no_color: bool,
}

pub struct Cli {
    pub snap1: path::PathBuf,
    pub snap2: path::PathBuf,
    pub workers1: usize,
    pub workers2: usize,
    pub terminal_printer: TerminalPrinter,
    pub file_printer: FilePrinter,
    pub ctrl_c: CtrlCSignal,
}

pub type CtrlCSignal = Arc<AtomicBool>;

impl Cli {
    pub fn new_from_env() -> Result<Cli, Error> {
        let args = Args::parse();
        let (workers1, workers2) = num_workers(args.workers);
        return Ok(Cli {
            snap1: get_snap(&args.snap1_path)?,
            snap2: get_snap(&args.snap2_path)?,
            workers1,
            workers2,
            terminal_printer: if args.no_color {
                TerminalPrinter::new_plain()
            } else {
                TerminalPrinter::new()
            },
            file_printer: {
                let p = path::Path::new(&args.report_file);
                if p.exists() {
                    return Err(Error::new(format!(
                        "report file already exists: {}",
                        p.display()
                    )));
                }
                FilePrinter::new(p)?
            },
            ctrl_c: {
                let ctrlc_arc = Arc::new(AtomicBool::new(false));
                let r = ctrlc_arc.clone();
                ctrlc::set_handler(move || {
                    r.store(true, Ordering::SeqCst);
                })
                .map_err(|e| {
                    Error::from("failed to register ^C handler".to_string(), e.to_string())
                })?;
                ctrlc_arc
            },
        });
    }
}

pub fn num_workers(ws: Option<Vec<usize>>) -> (usize, usize) {
    let cores = thread::available_parallelism().unwrap().get();
    return ws
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

fn get_snap(s: &String) -> Result<path::PathBuf, Error> {
    let m = fs::metadata(s).map_err(|e| {
        return Error::from(format!("cannot open directory: {}", s), e.to_string());
    })?;
    if !m.is_dir() {
        return Err(Error::new(format!("not a directory: {}", s)));
    }
    return Ok(path::Path::new(s).to_path_buf());
}
