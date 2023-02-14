use std::env;
use std::fs;
use std::process;
use std::error::Error;

/*
    Creating grep like command line application
*/

fn main() {
    // use std::evn::args to take input from command line and collect to turn iterator into collection.
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Reading Arguments Failed: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Open the file given in filepath
    let file_content = fs::read_to_string(config.file_path)?;

    println!("{file_content}");
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}