fn square(vec: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for item in vec {
        result.push(item * item);
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let result = square(&numbers);
    println!("The squared values are: {:?}", result);
}
