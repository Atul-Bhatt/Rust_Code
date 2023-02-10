mod math;
use math::{Addition, Subtraction};


fn main() {
    let x = 10;
    let y = 20;
    
    println!("Sum is {}", Addition::add(x, y));
    println!("Difference is {}", Subtraction::sub(x, y));
}