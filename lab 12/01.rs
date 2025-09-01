use std::env;

fn main() {
    let args:Vec<String> = args().collect();
    
    if args.len() == 4{
        println!("Arguments provided: {} {} {}", args[1], args[2],args[3]);
    } else {
        println!("Exactly 3 arguments are required.You provided 1.");
    }
}