trait Squareroot {
    fn sqrt(self) -> Self;
}

impl Squareroot for f32{
    fn sqrt(self) -> Self{
        self.sqrt()
    }
}

impl Squareroot for f64{
    fn sqrt(self) -> Self{
        self.sqrt()
    }
}

trait Abs{
    fn abs(self) -> Self;
}

impl Abs for f32{
    fn abs(self) -> Self{
        self.abs()
    }
}

impl Abs for f64{
    fn abs(self) -> Self{
        self.abs()
    }
}

fn abs_square_root<Hehe>(x:Hehe) -> Hehe
where 
    Hehe: Squareroot + Abs,
{
    x.abs().sqrt().sqrt()

}

fn main(){
    println!("{} {}",
    abs_square_root(-100f64),
    abs_square_root(-100f32)
);
}