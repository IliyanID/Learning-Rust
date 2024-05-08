use std::error::Error;
use std::fs;
use std::env;


pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}
impl Config{
    pub fn new(args: &mut Vec<String>)->Config{
        if args.len() != 3{
            panic!("Expected two arguments. <query> <file path>. Received {} argument.",args.len()-1);
        }
        let query = args.pop().expect("Missing query");
        let file_path = args.pop().expect("Missing arg");
    
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Config{query,file_path,ignore_case}
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {} in {}",config.query,config.file_path);
    let contents = fs::read_to_string(&config.file_path).expect(&format!("Unable to open file {}",&config.file_path));

    let results = if config.ignore_case {
        search_case_insensitive(&contents,&config.query)
    } else {
        search(&contents,&config.query)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    let contents: Vec<&str> = contents.split("\n").collect();
    let mut result:Vec<&str> = Vec::new();
    for line in contents {
        if line.contains(query){
            result.push(line);
        }
    }
    result
}
pub fn search_case_insensitive<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    let contents: Vec<&str> = contents.split("\n").collect();
    let mut result:Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents {
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }
    result
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

        assert_eq!(vec!["safe, fast, productive."], search(contents,query));
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
            search_case_insensitive(contents,query)
        );
    }
}
