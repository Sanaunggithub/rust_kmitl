use std::io::Write;
use std::io::Read;

fn main() {
    //file write
    let mut file = std::fs::File::create("hehe.txt").unwrap();
    file.write(b"I am san aung").unwrap();

    //file read
    let mut file2 = std::fs::File::open("data.txt").unwrap();
    let mut content = String::new();
    file2.read_to_string(&mut content).unwrap();

    println!("{}",content);
}