use std::fs::File;
use std::error::Error;
//use std::io::prelude::*;
use std::io::prelude::Read;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    for line in search(&config.query, &content) {
        println!("{}", line);
    }
//    println!("With text:\n{}", content);

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ans = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            ans.push(line);
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        );
    }
}