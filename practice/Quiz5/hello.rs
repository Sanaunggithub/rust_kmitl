fn greet_optional(name: Option<&str>) {
    match name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello, World!"),
    }
}

fn main() {
    greet_optional(Some("Alice"));  
    greet_optional(None); 
}
