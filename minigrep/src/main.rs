use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // collect command-line arguments
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    // extract arguments
    let config = Config::parse_args(&args).unwrap_or_else(|err| {
        println!("Parsing command-line arguments failed: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
