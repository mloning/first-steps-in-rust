use std::env;
use std::fs;
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
    println!("Searching for: {0} in file: {1}", config.query, config.file);

    // read file
    let content = fs::read_to_string(config.file).expect("Error reading file: {file}");
    println!("File content:\n{content}");
}

struct Config {
    query: String,
    file: String,
}

impl Config {
    fn parse_args(args: &[String]) -> Result<Config, &'static str> {
        let n_args = args.len();
        if n_args != 3 {
            panic!("Requires 3 arguments, but found: {n_args}");
        }

        let query = args[1].clone();
        let file = args[2].clone();
        Ok(Config { query, file })
    }
}
