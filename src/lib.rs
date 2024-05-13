//! # L:itegrep
//! The purpose of this crate ⚙️ is to learn rust features, and this crate will not be maintained 🚮.

use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query = args.next().ok_or("Didn't get a query string")?;

        let file_path = args.next().ok_or("Didn't get a file path")?;

        let mut ignore_case: bool = env::var("IGNORE_CASE")
            .unwrap_or("false".to_string())
            .parse()
            .expect("The variable IGNORE_CASE must be a boolean");
        let collected_args: Vec<String> = args.collect();

        if collected_args[collected_args.len() - 1] == "--ignore-case" {
            ignore_case = true;
        }

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// This function takes a config as parameter and is being used in main to initialize the program.
/// Is designed to return either nothing or a dynamic error.
///
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // let lines = search(&config.query, &contents);
    let results = {
        if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        } else {
            search(&config.query, &contents)
        }
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

/// This function searches thru given contents.
/// # Example:
/// ```
/// let contents = "\
/// May I have your attention please?
/// May I have your attention please?
/// Will the real Slim Shady please stand up?
/// I repeat, will the real Slim Shady please stand up?
/// We're gonna have a problem here
/// ";
/// let lines = litegrep::search("the real Slim Shady", contents);
/// assert_eq!(lines, ["Will the real Slim Shady please stand up?", "I repeat, will the real Slim Shady please stand up?"]);
/// ```
///
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// This search function searches within given contents and returns lines of text regardless of casing.
/// # Example:
/// ```
/// let contents = "\
/// May I have your attention please?
/// May I have your attention please?
/// Will the real Slim Shady please stand up?
/// I repeat, will the real Slim Shady please stand up?
/// We're gonna have a problem here
/// ";
/// let lines = litegrep::search_case_insensitive("SLiM ShadY", contents);
/// assert_eq!(lines, ["Will the real Slim Shady please stand up?", "I repeat, will the real Slim Shady please stand up?"]);
/// ```
///
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let contents = "\
            It is a beautiful day
            For me and you
            For you and me
            To get higher
            And higher
        ";
        let query = "beautiful";

        assert_eq!(vec!["It is a beautiful day"], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let contents = "\
            It is a beautiful day
            For me and you
            For you and me
            To get higher
            And higher
        ";
        let query = "BeAuTiFuL";
        assert_eq!(
            vec!["It is a beautiful day"],
            search_case_insensitive(query, contents)
        )
    }
}
