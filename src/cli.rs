use crate::filter::Filter;
use crate::printer::{FilePrinter, TerminalPrinter};
use crate::Error;
use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::{fs, path};

pub struct Cli {
    pub snap1_root: path::PathBuf,
    pub snap2_root: path::PathBuf,
    pub filters: Filter,
    pub workers1: usize,
    pub workers2: usize,
    pub terminal_printer: TerminalPrinter,
    pub file_printer: Option<FilePrinter>,
    pub ctrl_c: CtrlCSignal,
}

pub struct CtrlCSignal(Arc<AtomicBool>);

impl Clone for CtrlCSignal {
    fn clone(&self) -> Self {
        return CtrlCSignal(Arc::clone(&self.0));
    }
}

impl CtrlCSignal {
    pub fn has_triggered(&self) -> bool {
        return self.0.load(Ordering::SeqCst);
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    snap1_path: String,
    snap2_path: String,

    #[arg(
        long = "report-file",
        alias = "report",
        short = 'r',
        help = "Print detailed report to file"
    )]
    report_file: Option<String>,

    #[arg(
        long = "include-dot-paths",
        default_value_t = false,
        help = "Ignore files or folders that start with a dot"
    )]
    include_dot_paths: bool,

    #[arg(
        long = "include-symlinks",
        default_value_t = false,
        help = "Ignore paths that are symlinks"
    )]
    include_symlinks: bool,

    #[arg(
        long = "workers",
        alias = "worker",
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
    pub fn new_from_env() -> Result<Cli, Error> {
        let args = Args::parse();
        let (workers1, workers2) = num_workers(args.workers);
        return Ok(Cli {
            snap1_root: get_snap(&args.snap1_path)?,
            snap2_root: get_snap(&args.snap2_path)?,
            filters: Filter::new(args.include_symlinks, args.include_dot_paths),
            workers1,
            workers2,
            terminal_printer: if args.no_color {
                TerminalPrinter::new_plain()
            } else {
                TerminalPrinter::new()
            },
            file_printer: if args.report_file.is_none() {
                None
            } else {
                let f = &args.report_file.unwrap();
                let p = path::Path::new(f);
                if p.exists() {
                    return Err(Error::new(format!(
                        "report file already exists: {}",
                        p.display()
                    )));
                }
                Some(FilePrinter::new(p)?)
            },
            ctrl_c: {
                let ctrl_c = Arc::new(AtomicBool::new(false));
                let c_arc = Arc::clone(&ctrl_c);
                ctrlc::set_handler(move || {
                    c_arc.store(true, Ordering::SeqCst);
                })
                .map_err(|e| {
                    Error::from("failed to register ^C handler".to_string(), e.to_string())
                })?;
                CtrlCSignal(ctrl_c)
            },
        });
    }
}

fn num_workers(ws: Option<Vec<usize>>) -> (usize, usize) {
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
