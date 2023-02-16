use std::env;
use std::process;

use practice_project::Config;
/*
    Creating grep like command line application
*/

fn main() {

    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Reading Arguments Failed: {err}");
        process::exit(1);
    });

    if let Err(e) = practice_project::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}