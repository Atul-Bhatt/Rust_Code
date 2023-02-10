use std::io::{self, Read};

/*
    Conversting a string to pig latin
*/

fn main() {
    println!("Enter a string: ");

    let mut s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read string.");

    let mut pig_s = String::new();

    let vowels = "aeiouAEIOU";
    
    for word in s.split_whitespace() {
        let first_char = word.chars().nth(0).unwrap(); 
        if(vowels.contains(first_char)) {
            pig_s = format!("{pig_s} {word}-hay");
        }
        else {
            pig_s = format!("{pig_s} {word}-{first_char}ay");
        }
    }

    println!("{}", pig_s);
    
}