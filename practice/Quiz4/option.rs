fn main() {
    let mut num = Some(5);
        match num {
            Some(num) => println!("Found the value {}",num),
            None => println!("Sorry")
        }
}
