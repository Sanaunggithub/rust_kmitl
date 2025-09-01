trait Root{
    fn root(self) -> Self; // trait declaration
}

impl Root for f64 {
    fn root(self) -> Self {
        self.sqrt()
    }
}

impl Root for f32{
    fn root(self) -> f32 {
        self.sqrt()
    }
}
fn main() {
    println!("{}",4.0.root());
    //won't work this call println!("{}",root(4.99));
}