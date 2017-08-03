extern crate greprs;

use std::env;
use std::process;
use std::io::prelude::Write;

use greprs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // > stderr
    let mut cerr = std::io::stderr();

    let config = Config::new(&args).unwrap_or_else(|err| {
        writeln!(
            &mut cerr,
            "Problem parsing arguments: {}", err
        ).expect("Could not write to stderr");

        process::exit(1);
    });

//    println!("Searching for {}", config.query);
//    println!("In file {}", config.filename);

    if let Err(e) = greprs::run(config) {
        writeln!(
            &mut cerr,
            "Application error: {}", e
        ).expect("Could not write to stderr");

        process::exit(1);
    }
}


