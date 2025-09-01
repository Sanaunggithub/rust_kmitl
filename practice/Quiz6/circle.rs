struct Circle {
    radius:f64
}

impl Circle {
    fn new(radius:f64) -> Self {
       Self{
        radius
       }
    }
}
fn main() {
    let circle = Circle::new(10.0);
    println!("Circle with radius: {}", circle.radius);
}