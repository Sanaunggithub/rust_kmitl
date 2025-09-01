use std::fmt::Display;
use std::cmp::PartialOrd;

fn compare <T,U> (statement: T, num1: U, num2: U)
where
    T: Display,
    U: Display + PartialOrd
{
    println!("{}!Is {} greater than {}? {}", statement,num1,num2,num1>num2);
}

fn main() {
    compare("Yo",79,77);
}