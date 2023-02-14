use std::env;
use std::fs;

/*
    Creating grep like command line application
*/

fn main() {
    // use std::evn::args to take input from command line and collect to turn iterator into collection.
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    // Open the file given in filepath
    let file_content = fs::read_to_string(file_path)
            .expect("Cannot read file.");
    println!("{file_content}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}