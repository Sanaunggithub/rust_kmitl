fn calculate(n1:u32,n2:u32) -> (u32,u32) {
    let mut sum = 0;
    let mut product = 0;

    sum = n1 + n2;
    product = n1 * n2;

    return (sum,product)
}
fn main() {
    let result = calculate(3, 4);
    println!("Sum: {}, Product: {}", result.0, result.1);
}