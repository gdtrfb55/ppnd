/*
*   This file is part of the ppnd distribution, which can be found at 
*   <https://github.com/gdtrfb55/ppnd>.
*
*   Copyright (C) 2021 Jack Browning.
*
*   This program is FREE SOFTWARE: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   This program is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

extern crate getopts;

use std::env;
use getopts::Options;
use std::time::Duration;
use crate::bytescale::Scale;

pub struct CLOptions {
    pub show_lo: bool,
    pub scale: Scale,
    pub precision: usize,
    pub repeat: u16,
    pub delay: Duration
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
    use crate::PROGNAME;

    const SCALE_PARAMS: &str = "
Valid parameters for SCALE are:
    
'raw' = raw byte count

'dyn10' = dynamic power-of-10 scaling (kB = 1000 bytes)
'kb', 'mb', 'gb', 'tb', or 'pb' = fixed power-of-10 scaling

'dyn2' = dynamic power-of-2 scaling (KiB = 1024 bytes)
'kib', 'mib', 'gib', 'tib' or 'pib' = fixed power-of-2 scaling
    ";

    print!("{}", opts.usage(&format!("\nUsage: {} [OPTION]...", PROGNAME)));
    print!("{}", SCALE_PARAMS);
    std::process::exit(0);
}

fn show_version_and_exit() {
    use crate::{PROGNAME, PROGVERS, YADAYADA};

    println!("\n{} {} -- a prettier /proc/net/dev", PROGNAME, PROGVERS);
    print!("{}", YADAYADA);
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
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("l", "show-lo", "show loopback interface in list\n(default: hide loopback)");
    opts.optopt("s", "scale", "scaling factor for byte count\n(default: dyn10)", "SCALE");
    opts.optopt("p", "precision", &precision_opt_help(), "PRECISION");
    opts.optopt("r", "repeat", &repeat_opt_help(), "COUNT");
    opts.optopt("d", "delay", &delay_opt_help(), "SECONDS");
    opts.optflag("h", "help", "show this help and exit");
    opts.optflag("v", "version", "show version information and exit");
    let matches = match opts.parse(&args) {
        Ok(m) => { m }
        Err(e) => return Err(e.to_string())
    };
    if matches.opt_present("h") { show_help_and_exit(&opts) };
    if matches.opt_present("v") { show_version_and_exit() }
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
    Ok(CLOptions { show_lo, scale, precision, repeat, delay })
}