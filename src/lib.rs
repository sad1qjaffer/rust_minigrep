use std::{
    fs,error::Error
};

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config {query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for _line in search(&config.query, &contents){
        //println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn correct_config() {
        let args:Vec<String> = vec![String::from("callingprogram"),String::from("query"),String::from("filename")];
        let result = Config::new(&args);

        assert_eq!(result.is_ok(),true);
    }
    #[test]
    fn incorrect_config() {
        let args:Vec<String> = vec![String::from("callingprogram"),String::from("query")];
        if let Err(error) = Config::new(&args) {
            assert_eq!(error, String::from("Not enough arguments"));
        } else {
            panic!("Config returned OK instead of Error");
        }
    }
    #[test]
    fn file_open_success() {
        let args:Vec<String> = vec![String::from("target\\debug\\rust_minigrep.exe"),String::from("body"),String::from("poem.txt")];
        let config = Config::new(&args);
        let result = run(config.unwrap());
        assert_eq!(result.is_ok(), true);
    }
    #[test]
    fn file_not_found() {
        let args:Vec<String> = vec![String::from("target\\debug\\rust_minigrep.exe"),String::from("body"),String::from("poem.tx")];
        let config = Config::new(&args);
        assert_eq!(run(config.unwrap()).is_err(), true);
    }
}