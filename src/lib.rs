use std::fs;
use std::error::Error;
use std::env;


#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub file: String,
    pub case_sensitive: bool,
}


impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing the 'pattern' argument"),
        };
        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing the 'file' argument"),
        };

        let case_sensitive = env::var("MINIGREP_CASE_INSENSITIVE").is_err();

        Ok(Self {pattern, file, case_sensitive})
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.file)?;

    println!("Result of search:");
    if config.case_sensitive {
       for line in search_case_sensitive(&config.pattern, &text) {
            println!("{}", line);
        }
    } else {
        for line in search_case_insensitive(&config.pattern, &text) {
            println!("{}", line);
        }
    }
    
    Ok(())
}


pub fn search_case_sensitive<'a>(pattern: &str, text: &'a str) -> Vec<&'a str> {
    text
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

pub fn search_case_insensitive<'a>(pattern: &str, text: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();
    text
        .lines()
        .filter(|line| line.to_lowercase().contains(&pattern))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_sensitive() {
        let pattern = "duct";
        let text = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique.";

        assert_eq!(
            vec!["sécurité, rapidité, productivité."],
            search_case_sensitive(pattern, text)
        );
    }

    #[test]
    fn test_case_insensitive() {
        let pattern = "rUsT";
        let text = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique.";

        assert_eq!(
            vec!["Rust:", "C'est pas rustique."],
            search_case_insensitive(pattern, text)
        );
    }
}