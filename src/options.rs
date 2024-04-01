/*
    This file is part of the ppnd distribution, which can be found at
    <https://github.com/gdtrfb55/ppnd>.

    Copyright (C) 2021 Jack Browning.

    This program is FREE SOFTWARE: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program. If not, see <https://www.gnu.org/licenses/>.
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

use core::ops::RangeInclusive;

pub struct Precision;

impl Precision {
    const DEFAULT: usize = 3;
    pub const MAX: usize = 8;
    const VALID: RangeInclusive<usize> = 0..=Self::MAX;

    fn from_string(s: String) -> Result<usize, String> {
        if let Ok(p) = s.parse() {
            if Self::VALID.contains(&p) { return Ok(p) }
        };
        Err(format!("precision must be an integer value from 0 to {}", Self::MAX))
    }

    fn opt_help() -> String {
        format!("precision of scaled byte count (0-{})\n(default: 3)", Self::MAX)
    }
}

struct Repeat;

impl Repeat {
    const DEFAULT: u16 = 1;
    const MAX: u16 = 60;
    const VALID: RangeInclusive<u16> = 1..=Self::MAX;

    fn from_string(s: String) -> Result<u16, String> {
        if let Ok(r) = s.parse() {
            if Self::VALID.contains(&r) { return Ok(r) }
        };
        Err(format!("repeat must be an integer value from 1 to {}", Self::MAX))
    }

    fn opt_help() -> String {
        format!("query /proc/net/dev COUNT times (1-{})\n(default: 1)", Self::MAX)
    }
}

struct Delay;

impl Delay {
    const DEFAULT: Duration = Duration::from_secs(5);
    const MAX: u64 = 60;
    const VALID: RangeInclusive<u64> = 1..=Self::MAX;

    fn from_string(s: String) -> Result<Duration, String> {
        if let Ok(d) = s.parse() {
            if Self::VALID.contains(&d) { return Ok(Duration::from_secs(d)) }
        };
        Err(format!("delay must be an integer value from 1 to {}", Self::MAX))
    }

    fn opt_help() -> String {
        format!("delay between queries in SECONDS (1-{})\n(default: 5)", Self::MAX)
    }
}

fn show_help_and_exit(opts: &Options) {
    use crate::PROGNAME;

    const SCALE_PARAMS: &str = "
Valid parameters for SCALE are:

'raw' = raw byte count

'dyn' or 'dyn10' = dynamic power-of-10 scaling (kB = 1000 bytes)
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

pub fn get() -> Result<CLOptions, String> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optflag("l", "show-lo", "show loopback interface in list\n(default: hide loopback)");
    opts.optopt("s", "scale", "scaling factor for byte count\n(default: dyn)", "SCALE");
    opts.optopt("p", "precision", &Precision::opt_help(), "PRECISION");
    opts.optopt("r", "repeat", &Repeat::opt_help(), "COUNT");
    opts.optopt("d", "delay", &Delay::opt_help(), "SECONDS");
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
        Some(s) => Scale::from_string(s)?,
        None => Scale::DEFAULT
    };
    let precision = match matches.opt_str("p") {
        Some(p) => Precision::from_string(p)?,
        None => Precision::DEFAULT
    };
    let repeat = match matches.opt_str("r") {
        Some(r) => Repeat::from_string(r)?,
        None => Repeat::DEFAULT
    };
    let delay = match matches.opt_str("d") {
        Some(d) => Delay::from_string(d)?,
        None => Delay::DEFAULT
    };

    Ok(CLOptions { show_lo, scale, precision, repeat, delay })
}
