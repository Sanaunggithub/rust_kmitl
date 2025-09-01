use std::env::args;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main(){
    let args:Vec<String> = args().collect();

    if args.len() !=2 {
        println!("Please provide the file path as a comment-line argument");
        return;
    }

    let file_path = &args[1];
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => {
            println!("Error file not found");
            return;
        }
    };

    let reader = BufReader::new(file);

    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        line_count += 1;
        word_count += line.split_whitespace().count();
        char_count += line.chars().count();
    }

    println!("Lines: {}",line_count);
    println!("Words: {}",word_count);
    println!("Characters: {}",char_count);
}