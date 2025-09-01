use std::env::args;
use std::io::{BufRead,BufReader};
use std::fs::File;

fn main() {

    let args: Vec<String> = args().collect();
    if args.len() != 2{
        println!("Please provide the file path as argument");
    }

    let file_path = &args[1];
    let file = match File::open(file_path){
        Ok(f) => f,
        Err(_) => {
            println!("File not found");
            return;
        }   
    };

    let reader = BufReader::new(file);
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;

    for line in reader.lines(){
        let line = line.unwrap();
        line_count += 1;
        word_count += line.split_whitespace().count();
        char_count += line.chars().count();
    }

    println!("Line count: {}",line_count);
    println!("Word count: {}",word_count);
    println!("Character count: {}",char_count);
}