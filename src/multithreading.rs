use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn run_multithreading() {

    let v = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || {
        for i in v {
            println!("Number {} from spawned thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("Number {} from run function.", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

pub fn run_channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let movie = String::from("Lives of Other People.");
        tx.send(movie).unwrap();
    });

    let movie = rx.recv().unwrap();
    println!("{}", movie);
}