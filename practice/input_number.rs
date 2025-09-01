use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Expected to fail");

    let integer:i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("It's not valid");
            return;
        }
    };

    println!("{:?}",integer);
}