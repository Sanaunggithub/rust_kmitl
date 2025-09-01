use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("No file path provided");
        return;
    }

    let file_path = &args[1];
    let checker = if args.len() > 2 {
        args[2].clone()
    } else {
        "--".to_string()
    };

    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => {
            println!("File Not Found: {}", file_path);
            return;
        }
    };

    let mut word_count = 0;
    let mut line_count = 0;
    let reader = BufReader::new(file);

    if checker == "-w" {
        for line in reader.lines() {
            let line = line.unwrap();
            word_count += line.split_whitespace().count();
        }
        println!("Words: {}", word_count);
    } else if checker == "-l" {
        for line in reader.lines() {
            let line = line.unwrap();
            line_count += 1;
        }
        println!("Lines: {}", line_count);
        
    } else if args.len() == 2 {
        for line in reader.lines() {
            let line = line.unwrap();
            line_count += 1;
            word_count += line.split_whitespace().count();
        }
        println!("Words: {}", word_count);
        println!("Lines: {}", line_count);
    }

    else {
        println!("Command is not correct");
    }
}
