fn main(){
    let multiply = |a, b| a * b;

    println!("{:?}",multiply(5,9));

    let multiply2: fn(i32, i32) -> i32 = |a, b| a * b;
    println!("4 * 5 = {}", multiply2(4, 5));
}