fn factorial(n:u32) -> u32{
    if n == 1 {
        return 1
    } else {
        return n * factorial(n-1)
    }
}

fn main() {
    let result = factorial(5);
    println!("The factorial of 5 is {}", result);
}