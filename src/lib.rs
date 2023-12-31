use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query,&content)
    }else {
        case_insensitive(&config.query, &content)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}



pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() <3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Config { query, filename, case_sensitive })
    }
    
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn case_insensitive<'a>(
    query: &str,
    content: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in content.lines(){
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}



#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn case_sensitive() {
        let query  =  "duct";
        let content = "/
Rust:
safe, fast, productive.
pick three.
Duct tape,";
    

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_nosensitive() {
        let query  =  "rUsT";
        let content = "/
Rust:
safe, fast, productive.
pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            case_insensitive(query, content)
        );
    }


}
