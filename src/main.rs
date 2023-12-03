use std::env;

use std::process;
// use std::error::Error;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });    
    
    
    if let Err(err) = minigrep::run(config) {
        eprintln!("application error: {}", err);
        process::exit(1);
    }
    // println!("searching for {}", config.filename);
    // print!("searching for {}", config.query);
}

