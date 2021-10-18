extern crate chrono;

pub fn print() {
    use chrono::prelude::*;

    let local = Local::now();
    println!("\n{}", local.format("=== %H:%M:%S ===").to_string());
}