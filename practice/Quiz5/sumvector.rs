fn sum(vec: &Vec<u32>) -> u32 {
    let mut sum = 0;
    for i in vec{
        sum += i;
    }
    return sum
}
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let result = sum(&numbers);
    println!("The sum is: {}", result);
}