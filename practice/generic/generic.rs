use std::fmt::Debug;
fn return_number<T:Debug>(number: T) {
    println!("{:?}",number);
}

fn main() {
    return_number(5);
    
}