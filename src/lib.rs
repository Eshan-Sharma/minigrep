use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //Box dyn Error is dynamic error trait
    let content = fs::read_to_string(config.file_path)?;
    //Takes in a path and opens that file, returns a std::io::Result<String> of the file content
    println!("With text:\n{content}");
    Ok(())
}
