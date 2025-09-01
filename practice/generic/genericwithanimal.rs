use std::fmt::Debug;
#[derive(Debug)]

struct Animal {
    age: u32,
    name: String,
}

fn print_item<T: Debug>(item: T) {
    println!("Here's your item {:?}",item);
}

fn main() {
    let charile = Animal {
        name: "charile".to_string(),
        age: 10,
    };

    let number = 55;
    print_item(charile);
    print_item(number);
}