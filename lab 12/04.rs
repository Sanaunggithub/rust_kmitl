use std::fs::{File,OpenOptions}; //OpenOptions is use for truncate,write functionality
use std::io::{BufRead,Write,BufReader};
use std::path::Path;
use std::io::*;

fn main() {
    let mut input_lines = Vec::new();
    println!("Enter lines of text (press Enter  to stop)");

    loop{
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");// read a line of input

        if input.trim().is_empty() {
            break;
        }

        input_lines.push(input);
    }

    let path = Path::new("output.txt");
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(&path).expect("Unable to open or create file");//write(true): Enables writing to the file.create(true): Creates the file if it doesn't exist.truncate(true): Clears the file if it already exists.open(&path): Opens or creates the file at the specified path.

    for line in &input_lines{
        file.write_all(line.as_bytes()).expect("Failed to write a file");
    }

    let file = File::open(path).expect("Failed to open the file for reading");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        println!("{}",line.to_uppercase());
    }
}