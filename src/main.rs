
#[derive(Debug)]

/*
    printing rectangle in debug mode. You can't do it in normal mode because
    your rectangle struct doesn't implement 'std::fmt::Display'
*/
struct Rectangle(i32, i32);

fn main() {
    
    let rect: Rectangle = Rectangle(10, 40);

    println!("rect is {:#?}", rect);
}