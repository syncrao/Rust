use std::error::Error;
use std::fs; 
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.quary, &content)
    } else {
        search(&config.quary, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())

}

pub struct Config{
   pub quary:String,
   pub file_path:String,
   pub ignore_case:bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let quary = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config{quary,file_path, ignore_case})
    }
}

pub fn search<'a>(quary: &str, contents: &'a str) -> Vec<&'a str> {
 contents.lines().filter(|line| line.contains(quary)).collect()
}

pub fn search_case_insensitive<'a>(
    quary: &str,
    contents: &'a str,
) -> Vec<&'a str> {
     let quary = quary.to_lowercase();
     contents.lines().filter(|line| line.to_lowercase().contains(&quary)).collect()
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let quary = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(quary, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}