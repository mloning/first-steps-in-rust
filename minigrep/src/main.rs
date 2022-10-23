use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // collect command-line arguments
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Parsing command-line arguments failed: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
