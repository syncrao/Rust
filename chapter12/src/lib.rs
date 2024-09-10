use std::fs;
use std::error::Error;
use std::env;




pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filename = config.filename;
    let contents = fs::read_to_string(filename)?;
    let mut count = 1;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("Query line ({}) : {}", count, line);
        count += 1;
    }

    Ok(())

}


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("not enough arguments "),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("not enough arguments "),
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config {query: query, filename: filename, case_sensitive: case_sensitive})
    }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
pick three
Trust me";

        assert_eq!(vec!["Rust:", "Trust me"], search_case_insensitive(query, contents));
    }

}