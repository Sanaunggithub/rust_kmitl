use std::mem::*;

fn main(){
    println!("Size of i32 is {}",size_of::<i32>());
    println!("Size of f64 is {}",size_of::<f64>());
    println!("Size of the value of 1000 is {}",size_of_val(&1000));
}