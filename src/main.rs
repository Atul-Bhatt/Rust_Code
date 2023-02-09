
// Calculating the area of a rectangle using a struct

struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    
    let rect: Rectangle = build_rectangle(10, 25);

    println!("Area of the rectangle is: {}",
        calc_rec_area(rect)
    );
}

// This function takes input width and height and returns an instance of Rectangle Struct 

fn build_rectangle(width: i32, height: i32) -> Rectangle {
    Rectangle {
        width,
        height,
    }
}

// This function takes input a Rectangle instance and returns the area of the rectangle

fn calc_rec_area(rect: Rectangle) -> i32{
    rect.width * rect.height
}