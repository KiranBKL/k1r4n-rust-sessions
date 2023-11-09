use std::{error::Error, fs};
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config,&str> {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        //arg[0] is binary path
        let query = args[1].clone();
        let file_path = args[2].clone();//here it it will do copy we handle this in upcoming sessions
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        //we gonna return a Result instead of COnfig

        Ok(Config { query, file_path, ignore_case})
    }
}

pub fn run(config:Config) -> Result<(),Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    // println!("{}", content);
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
        } else {
            search(&config.query, &content)
        };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            vec.push(line);
        }
    }
    vec
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {//line wont change change as it is not binded so line stays unchanged
            results.push(line);
        }
    }
    println!("dkgfgfgf {query}");//here when query is to but initially we passed To it has been chenged to "to"
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Trust me.";
        assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query, contents))
    }
}

//-------------------------lets make it using environment variable-------------------------------------------------------
//export export IGNORE_CASE=true
//unset IGNORE_CASE
//we can save std output using > command eg: cargo run -- To kepler452.txt > output.txt