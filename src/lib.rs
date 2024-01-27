use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = false;
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(cfg.file_path).expect("Should have been able to read the file");
    // println!("Contents: {}", contents);
    let result = if cfg.ignore_case {
        search_caseinsentivie(&cfg.query, &contents)
    } else {
        search_casesensitive(&cfg.query, &contents)
    };
    for l in result {
        println!("{l}");
    }
    Ok(())
}

fn search_caseinsentivie<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let q = query.to_uppercase();
    for a in content.lines() {
        if a.to_uppercase().contains(&q) {
            result.push(a);
        }
    }
    result
}

fn search_casesensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for a in content.lines() {
        if a.contains(query) {
            result.push(a);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_casesensitive(query, content)
        );
    }

    #[test]
    fn multiple_result() {
        let query = "apple";
        let content = "\
Apple a day keeps the doctor away.
But keep the apple away to keep everyone away.
Something about me is good";
        assert_eq!(
            vec!["But keep the apple away to keep everyone away."],
            search_casesensitive(query, content)
        );
    }

    #[test]
    fn multiple_result_caseinsensitive() {
        let query = "apple";
        let content = "\
Apple a day keeps the doctor away.
But keep the apple away to keep everyone away.
Something about me is good";
        assert_eq!(
            vec![
                "Apple a day keeps the doctor away.",
                "But keep the apple away to keep everyone away."
            ],
            search_caseinsentivie(query, content)
        );
    }
}
