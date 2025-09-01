trait SqrtAndAbs{
    fn sqrt(self)-> Self;
    fn abs(self) -> Self;
}

impl SqrtAndAbs for f32{
    fn sqrt(self)-> Self{
        self.sqrt()
    }

    fn abs(self) -> Self{
        self.abs()
    }
}
fn main(){
    println!("{}",100.00.sqrt());
    println!("{}",-100.00.abs());
}