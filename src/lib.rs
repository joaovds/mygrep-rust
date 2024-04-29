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
}
