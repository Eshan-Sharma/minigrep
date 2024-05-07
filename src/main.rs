use minigrep::Config;
use std::env;
use std::process;
fn main() {
    // str::env::args returns an iterator
    //iterator produces a series of values which are collected by collect()
    //this line reads any command line argument passed to it and collects the values into a vector
    //let args: Vec<String> = env::args().collect(); //future optimization use args_os
    //dbg!(args);

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application returned error: {e}");
        process::exit(1);
    }
}
