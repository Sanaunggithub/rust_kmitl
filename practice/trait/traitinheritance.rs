trait Squareroot {
    fn sqrt(self) -> Self;
}

impl Squareroot for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Squareroot for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

trait Abs {
    fn abs(self) -> Self;
}

impl Abs for f32 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Abs for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
}

trait SqrtAndAbs: Squareroot + Abs {}

impl SqrtAndAbs for f32 {}
impl SqrtAndAbs for f64 {}


fn abs_square_root<Number>(x: Number) -> Number
where 
    Number: SqrtAndAbs,
{
    x.abs().sqrt().sqrt()
}

fn main() {
    println!("{} {}", 
        abs_square_root(-100f64), 
        abs_square_root(-100f32)
    );
}
