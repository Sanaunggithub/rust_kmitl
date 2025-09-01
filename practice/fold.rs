fn main(){
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().fold(10, |acc, x| acc + x);
    println!("Sum of numbers: {}", sum);
}