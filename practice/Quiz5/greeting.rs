fn greet(n: String) -> String {

    format!("Hello {}",n)
}
fn main() {
    let greeting = greet("Alice".to_string());
    println!("{}", greeting);
}