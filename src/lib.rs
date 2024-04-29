use std::{error::Error, fs};

pub struct Config {
    pub pattern: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let pattern = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { pattern, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.pattern, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(pattern) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let pattern = pattern.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&pattern) {
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
        let pattern = "abc";
        let contents = "\
Test contents
hello abc
this is test cba
        ";

        assert_eq!(vec!["hello abc"], search(pattern, contents));
    }

    #[test]
    fn case_sensitive() {
        let pattern = "M";
        let contents = "\
Far over the Misty Mountains cold,
To dungeons deep and caverns old,
We must away, ere break of day,
To seek our pale enchanted gold.
        ";

        let expected_result = vec!["Far over the Misty Mountains cold,"];

        assert_eq!(expected_result, search(pattern, contents));
    }

    #[test]
    fn case_insensitive() {
        let pattern = "M";
        let contents = "\
Far over the Misty Mountains cold,
To dungeons deep and caverns old,
We must away, ere break of day,
To seek our pale enchanted gold.
        ";

        let expected_result = vec![
            "Far over the Misty Mountains cold,",
            "We must away, ere break of day,",
        ];

        assert_eq!(expected_result, search_case_insensitive(pattern, contents));
    }
}
