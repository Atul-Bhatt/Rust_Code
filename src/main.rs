
// Calculating the area of a rectangle using a tuple struct

// Tuple struct implementation doesn't specify which one is width and which one is height
struct Rectangle(i32, i32);

fn main() {
    
    let rect: Rectangle = Rectangle(10, 40);

    println!("Area of the rectangle is: {}",
        calc_rec_area(rect)
    );
}

// This function takes input a Rectangle instance and returns the area of the rectangle

fn calc_rec_area(rect: Rectangle) -> i32{
    rect.0 * rect.1
}