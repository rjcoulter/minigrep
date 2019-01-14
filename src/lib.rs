use console::style;
use indicatif::ProgressBar;
use std::error::Error;
use std::{env, fs};

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    if result.len() == 0 {
        println!("{}", "No matches found");
        return result;
    } else {
        return result;
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let result: Vec<&'a str> = contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
    if result.len() == 0 {
        println!("{}", "No matches found");
        return result;
    } else {
        return result;
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Output a message to the terminal
    println!("{}", style("Searching file for matches...").cyan());

    let contents = fs::read_to_string(config.filename)?;
    /*
        let pb = ProgressBar::new(100);
        pb.inc(100);
    */
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string inputted"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No file inputted"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
