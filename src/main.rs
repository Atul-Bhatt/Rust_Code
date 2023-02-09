
#[derive(Debug)]

/*
    Implementing a can_hold() method for Rectangle struct
*/
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10, 
        height: 20,
    };
    let rect2 = Rectangle {
        width: 30,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 20, 
        height: 20,
    };

    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1: {}", rect2.can_hold(&rect1));
}