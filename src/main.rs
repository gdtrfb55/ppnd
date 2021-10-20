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

fn run() -> Result<(), String> {
    use crate::ifstats::IFStats;
    
    const IF_LO: &str = r"lo:";

    let opts = options::get()?;
    let if_line = ifregex::build()?;
    let mut count = opts.repeat;
    let show_time = count > 2;
    let mut stats: IFStats;
 
    loop {
        count -= 1;
        if show_time { timestamp::print() };
        for netdev_line in netdev::read()?.lines() {
            if if_line.is_match(netdev_line) {
                stats = ifstats::new(netdev_line)?;
                if (stats.name != IF_LO) || (opts.show_lo == true) {
                    stats.print(&opts.scale, opts.precision);
                };
            };
        };
        if count == 0 { break };
        std::thread::sleep(opts.delay);
    };

    Ok(())
}

fn main() {
    use std::process;

    const WHOAMI: &str = "ppnd";
    const SUCCESS: i32 = 0;
    const FAILURE: i32 = 1;

    if let Err(e) = run() {
        eprintln!("\n{}: {}.", WHOAMI, e);
        process::exit(FAILURE);
    };

    process::exit(SUCCESS);
}
