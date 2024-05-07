use std::env;
use std::fs;
use std::process;
fn main() {
    // str::env::args returns an iterator
    //iterator produces a series of values which are collected by collect()
    //this line reads any command line argument passed to it and collects the values into a vector
    let args: Vec<String> = env::args().collect(); //future optimization use args_os
                                                   //dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let content =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file"); //Takes in a path and opens that file, returns a std::io::Result<String> of the file content
    println!("With text:\n{content}");
}

// Problems with the above code
// 1. Long main function with a lot of variables, the more the variables, harder to keep track of the purpose of each
// 2. Long function is difficult to reason with, harder to test and harder to change
// 3. Uses 'expect' to print error message but does not give us the real reason why reading failed

struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
