#[derive(Debug)]
struct Rectangle {
    width: u32,
    height:u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        return self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 10, height: 20 };
    println!("Area: {:?}", rect.area());
}