//  This file is part of the ppnd distribution, which can be found at 
//  <https://github.com/gdtrfb55/ppnd>.
//
//  Copyright (C) 2021 Jack Browning.
//
//  This program is FREE SOFTWARE: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.

static PROGNAME: &str = "ppnd";
static PROGVERS: &str = "0.9.9";
static YADAYADA: &str = "
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
";

mod options;
mod ifstats;
mod bytescale;

mod ifregex {
    use regex::Regex;

    pub fn build() -> Result<Regex, String> {
        const IF_LINE_RE: &str = r"^\s*[[:alnum:]]+:(?:\s+\d+){16}$";

        if let Ok(r) = Regex::new(IF_LINE_RE) { return Ok(r) };
        Err("regular expression is invalid".to_string())
    }
}

mod netdev {
    pub fn read() -> Result<String, String> {
        use std::fs;

        const PATH: &str = r"/proc/net/dev";
        
        if let Ok(s) = fs::read_to_string(PATH) { return Ok(s) };
        Err(format!("could not read {}", PATH))
    }
}

mod timestamp {
    extern crate chrono;

    pub fn print() {
        use chrono::prelude::*;

        let local = Local::now();
        println!("\n{}", local.format("=== %H:%M:%S ===").to_string());
    }
}

use crate::options::CLOptions;

fn print_interface_stats(line: &str, opts: &CLOptions) -> Result<(), String> {
    const LOCAL_IF: &str = "lo:";

    let stats = ifstats::new(&line)?;
    let printable = (stats.name != LOCAL_IF) || opts.show_lo;
    
    if printable { stats.print(&opts.scale, opts.precision); }

    Ok(())
}

fn run() -> Result<(), String> {
    let opts = options::get()?;
    let interface_line = ifregex::build()?;
    let mut count = opts.repeat;
    let repeating = count > 1;
 
    loop {
        count -= 1;
        if repeating { timestamp::print() };
        for line in netdev::read()?.lines() {
            if interface_line.is_match(line) {
                print_interface_stats(line, &opts)?;
            };
        };
        if count == 0 { break };
        std::thread::sleep(opts.delay);
    };

    Ok(())
}

fn main() {
    use std::process;

    const SUCCESS: i32 = 0;
    const FAILURE: i32 = 1;

    if let Err(e) = run() {
        eprintln!("\n{}: {}.", PROGNAME, e);
        process::exit(FAILURE);
    };

    process::exit(SUCCESS);
}
