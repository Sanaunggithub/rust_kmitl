use std::mem::{size_of,size_of_val};

struct Point{
    x:i32,
    y:i32,
}

enum Shape {
    Circle(Point,f32),
    Rectangle(Point,Point),
}

fn main() {
    let point = Point { x:0, y:0 };
    let circle = Shape::Circle(Point{x:1,y:2},5.5);
    let rectangle = Shape::Rectangle(Point {x:1,y:2}, Point {x:5,y:10 });

    println!("Size of Point: {} bytes", size_of::<Point>());
    println!("Size of Shape: {} bytes", size_of::<Shape>());
    println!("Size of Circle: {} bytes", size_of_val(&circle));
    println!("Size of Rectangle: {} bytes", size_of_val(&rectangle));
}