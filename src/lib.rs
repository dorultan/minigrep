use std::{error::Error, fs, vec};
#[derive(Debug)]
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a Vec<String>) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided to config");
        }
        let query = &args[1];
        let file_path = &args[2];

        Ok(Self { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let lines = search(&config.query, &contents);

    for line in lines {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn match_a_line() {
        let contents = "\
            It is a beautiful day
            For me and you
            For you and me
            To get higher
            And higher
        ";
        let search_word = "beautiful";

        assert_eq!(vec!["It is a beautiful day"], search(search_word, contents))
    }
}
