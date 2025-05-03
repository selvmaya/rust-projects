use std::error::Error;
use std::path::PathBuf;

pub struct Config {
    pub query: String,
    pub file_path: PathBuf,
    pub ignore_case: bool,
}
impl Config {
    pub fn build(args: &mut impl Iterator<Item = String>) -> Result<Self, &'static str> {
        Ok(Config {
            query: args.next().ok_or("first argument missing")?,
            file_path: PathBuf::from(args.next().ok_or("second argument missing")?),
            ignore_case: std::env::var("IGNORE_CASE").is_ok()
        })
    }
}

pub fn run(c: Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(c.file_path)?;
    
    let results = if c.ignore_case {
        search_insensitive(c.query.as_str(), contents.as_str())
    } else {
        search(c.query.as_str(), contents.as_str())
    };
    
    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|l| l.contains(query)).collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|l| l.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
RUST:
safe, fast, productive
Pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(query, contents))
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

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
