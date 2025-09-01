fn reverse_string(s:String) -> String {
    return s.chars().rev().collect()
}

fn main() {
    let name = String::from("Sam");
    println!("Reverse is {}",reverse_string(name));
}