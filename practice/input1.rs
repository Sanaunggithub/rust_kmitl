use std::io;

fn main() {
    println!("Enter your name: ");

    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("Hello,{}",name.trim()),
        Err(error) => println!("Error")
    }
}