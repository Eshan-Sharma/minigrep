use std::env;

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
}
