use std::env::args;

fn main() {
    let v:Vec<String> = args().collect();

    if v.len()!= 4 {
        println!("Exaclty 3 arguments are required.You provided 1.");
    }

    print!("argument provided: ");
    for i in 1..=3{
        print!("{} ",v[i]);
    }
    println!();
}