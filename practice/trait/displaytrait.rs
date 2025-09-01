use std::fmt::Display;

fn print_value<T:Display>(v: T){
    println!("The value is {}",v)
}
fn main(){
    let value = "Hehe";
    print_value(value);
}