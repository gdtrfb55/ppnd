extern crate getopts;

use std::env;
use getopts::Options;
use std::time::Duration;
use crate::bytescale::Scale;

pub struct CLOptions {
    pub minime: String,
    pub show_lo: bool,
    pub scale: Scale,
    pub precision: usize,
    pub repeat: u16,
    pub delay: Duration
}

fn basename(argv_name: &str) -> Result<String, String> {
    use std::path::Path;

    if let Some(n) = Path::new(argv_name).file_name() {
        if let Some(b) = n.to_str() {
            return Ok(b.to_string())
        };
    };
    Err("could not obtain program name from argv".to_string())
}

pub const MAX_PRECISION: usize = 8;

fn precision_opt_help() -> String {
    format!("precision of scaled byte count (0-{})\n(default: 3)", MAX_PRECISION)
}

const MAX_REPEAT: u16 = 60;

fn repeat_opt_help() -> String {
    format!("query /proc/net/dev COUNT times (1-{})\n(default: 1)", MAX_REPEAT) 
}

const MAX_DELAY: u64 = 60;

fn delay_opt_help() -> String {
    format!("delay between queries in SECONDS (1-{})\n(default: 5)", MAX_DELAY)
}

fn show_help_and_exit(opts: &Options) {
    const SCALE_PARAMS: &str = "
Valid parameters for SCALE are:
    
'raw' = raw byte count

'dyn10' = dynamic power-of-10 scaling (kB = 1000 bytes)
'kb', 'mb', 'gb', 'tb', or 'pb' = fixed power-of-10 scaling

'dyn2' = dynamic power-of-2 scaling (KiB = 1024 bytes)
'kib', 'mib', 'gib', 'tib' or 'pib' = fixed power-of-2 scaling
    ";

    print!("{}", opts.usage("\nppnd 0.9.9 -- a prettier /proc/net/dev"));
    print!("{}", SCALE_PARAMS);
    std::process::exit(0);
}

fn valid_scale(s: String) -> Result<Scale, String> {
    match s.as_str() {
        "dyn10" => Ok(Scale::Dyn10),
        "dyn2" => Ok(Scale::Dyn2),
        "raw" => Ok(Scale::Raw),
        "kb" => Ok(Scale::Kilo),
        "mb" => Ok(Scale::Mega),
        "gb" => Ok(Scale::Giga),
        "tb" => Ok(Scale::Tera),
        "pb" => Ok(Scale::Peta),
        "kib" => Ok(Scale::Kibi),
        "mib" => Ok(Scale::Mebi),
        "gib" => Ok(Scale::Gibi),
        "tib" => Ok(Scale::Tebi),
        "pib" => Ok(Scale::Pebi),
        _ => Err(format!("'{}' is not a valid scale value", s))
    }
}

fn valid_precision(s: String) -> Result<usize, String> {
    if let Ok(p) = s.parse() {
        if p <= MAX_PRECISION { return Ok(p) }
    };
    Err(format!("precision must be an integer value from 0 to {}", MAX_PRECISION))
}

fn valid_repeat(s: String) -> Result<u16, String> {
    if let Ok(p) = s.parse() {
        if (p > 0) && (p <= MAX_REPEAT) { return Ok(p) }
    };
    Err(format!("repeat must be an integer value from 1 to {}", MAX_REPEAT))
}

fn valid_delay(s: String) -> Result<u64, String> {
    if let Ok(p) = s.parse() {
        if (p > 0) && (p <= MAX_DELAY) { return Ok(p) }
    };
    Err(format!("delay must be an integer value from 1 to {}", MAX_DELAY))
}

pub fn get() -> Result<CLOptions, String> {
    let mut args: Vec<String> = env::args().collect();
    let minime = basename(&args.remove(0))?;
    let mut opts = Options::new();
    opts.optflag("l", "show-lo", "show loopback interface in list\n(default: hide loopback)");
    opts.optopt("s", "scale", "scaling factor for byte count\n(default: dyn10)", "SCALE");
    opts.optopt("p", "precision", &precision_opt_help(), "PRECISION");
    opts.optopt("r", "repeat", &repeat_opt_help(), "COUNT");
    opts.optopt("d", "delay", &delay_opt_help(), "SECONDS");
    opts.optflag("h", "help", "show this help");
    let matches = match opts.parse(&args) {
        Ok(m) => { m }
        Err(e) => return Err(e.to_string())
    };
    if matches.opt_present("h") { show_help_and_exit(&opts) };
    let show_lo = matches.opt_present("l");
    let scale = match matches.opt_str("s") {
        Some(s) => valid_scale(s)?,
        None => Scale::Dyn10
    };
    let precision = match matches.opt_str("p") {
        Some(p) => valid_precision(p)?,
        None => 3
    };
    let repeat = match matches.opt_str("r") {
        Some(p) => valid_repeat(p)?,
        None => 1
    };
    let delay = match matches.opt_str("d") {
        Some(p) => Duration::from_secs(valid_delay(p)?),
        None => Duration::from_secs(5)
    };
    Ok(CLOptions { minime, show_lo, scale, precision, repeat, delay })
}