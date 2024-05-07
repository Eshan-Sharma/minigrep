use std::env;
use std::fs;
fn main() {
    // str::env::args returns an iterator
    //iterator produces a series of values which are collected by collect()
    //this line reads any command line argument passed to it and collects the values into a vector
    let args: Vec<String> = env::args().collect(); //future optimization use args_os
                                                   //dbg!(args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let content = fs::read_to_string(file_path).expect("Should have been able to read the file"); //Takes in a path and opens that file, returns a std::io::Result<String> of the file content
    println!("With text:\n{content}");
}
