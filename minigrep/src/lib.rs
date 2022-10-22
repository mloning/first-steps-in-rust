use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // println!("Searching for: {0} in file: {1}", config.query, config.file);
    let contents = fs::read_to_string(config.file).expect("Error reading file: {file}");
    // println!("File content:\n{content}");
    for line in search(&config.query, &contents) {
        print!("{line}");
    }
    Ok(())
}

pub struct Config {
    query: String,
    file: String,
}

impl Config {
    pub fn parse_args(args: &[String]) -> Result<Config, &'static str> {
        let n_args = args.len();
        if n_args != 3 {
            panic!("Requires 3 arguments, but found: {n_args}");
        }

        let query = args[1].clone();
        let file = args[2].clone();
        Ok(Config { query, file })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
