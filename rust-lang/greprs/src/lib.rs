use std::fs::File;
use std::error::Error;
//use std::io::prelude::*;
use std::io::prelude::Read;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query, filename, case_sensitive
        })

//        if args.len() < 3 {
//            return Err("not enough arguments")
//        }

//        let query = args[1].clone();
//        let filename = args[2].clone();
//        // > case sensitive if CASE_INSENSITIVE is not set
//        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
//
//        Ok(Config{query, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let vec = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in vec {
        println!("{}", line);
    }
//    println!("With text:\n{}", content);

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let it = content.lines();
    it.filter(|line|{ line.contains(query)})
        .collect()

//    let mut ans = Vec::new();
//
//    for line in content.lines() {
//        if line.contains(query) {
//            ans.push(line);
//        }
//    }
//
//    ans
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut ans = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            ans.push(line);
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        )
    }
}