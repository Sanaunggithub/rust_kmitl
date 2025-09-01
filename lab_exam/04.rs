use std::io::Read;
use std::io::Write;
use std::fs::File;

fn main() {
    let mut file = File::create("Hehe.txt").unwrap();
    file.write_all(b"HEELLLLLLOOOOOO").unwrap();

    let mut contents = String::new();
    let mut file2 = File::open("data.txt").unwrap();
    file2.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}