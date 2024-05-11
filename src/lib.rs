use std::{env, error::Error, fs, vec};
#[derive(Debug)]
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a Vec<String>) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided to config");
        }
        let query = &args[1];
        let file_path = &args[2];

        let mut ignore_case: bool = env::var("IGNORE_CASE")
            .unwrap_or("false".to_string())
            .parse()
            .expect("The variable IGNORE_CASE must be a boolean");

        if args[args.len() - 1] == "--ignore-case" {
            ignore_case = true;
        }

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // let lines = search(&config.query, &contents);
    let results = {
        if config.ignore_case {
            search_case_insensitive(config.query, &contents)
        } else {
            search(config.query, &contents)
        }
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut store = vec![];
    for line in content.lines() {
        if line.contains(query) {
            store.push(line);
        }
    }
    store
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
