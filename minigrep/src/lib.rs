use std::{env, error::Error, fs};

pub enum Case {
    Sensitive,
    Insensitive,
}

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub sensitivity: Case,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Self {
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Self {
            query: &args[1],
            file_path: &args[2],
            sensitivity: if ignore_case {
                Case::Sensitive
            } else {
                Case::Insensitive
            },
        }
    }

    pub fn build(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() != 3 {
            return Err("usage: minigrep <PATTERN> <FILE>");
        }
        Ok(Self::new(args))
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results;
    match config.sensitivity {
        Case::Sensitive => results = search(config.query, &contents),
        Case::Insensitive => results = search_insensitive(config.query, &contents),
    }
    results.iter().for_each(|line| println!("{}", line));
    Ok(())
}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
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
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_insensitive(query, contents)
        );
    }
}
