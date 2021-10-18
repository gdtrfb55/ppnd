mod options;
mod netdev;
mod ifregex;
mod ifstats;
mod bytescale;
mod timestamp;

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

    match run() {
        Ok(_) => {
            process::exit(SUCCESS)
        },
        Err(e) => { 
            eprintln!("\n{}: {}.", WHOAMI, e);
            process::exit(FAILURE)
        }
    }
}
