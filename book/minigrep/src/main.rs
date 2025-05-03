use minigrep::*;
use std::process;

fn main() {
    let c = Config::build(&mut std::env::args().skip(1)).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {e}");
        process::exit(1)
    });

    if let Err(e) = run(c) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}
