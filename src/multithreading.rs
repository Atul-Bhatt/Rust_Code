use std::thread;
use std::time::Duration;

pub fn run_multithreading() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Number {} from spawned thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("Number {} from run function.", i);
        thread::sleep(Duration::from_millis(1));
    }
}