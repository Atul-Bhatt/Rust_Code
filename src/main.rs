use std::env;
use std::process;

use multithreading::run_channels;
use multithreading::run_multithreading;
use practice_project::Config;

mod inventory;
mod multithreading;

/*
    Creating grep like command line application
*/

fn main() {

    // let config: Config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("Reading Arguments Failed: {err}");
    //     process::exit(1);
    // });

    // if let Err(e) = practice_project::run(config) {
    //     println!("Application error: {e}");
    //     process::exit(1);
    // }

    // inventory::run_inventory();

    run_channels();
}