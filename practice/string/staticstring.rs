use std::mem::*;

fn main() {
    let a = ""; //static string (&str).They cannot change in size after being defined.Cannot be modified directly.

    let b = "0123456789";
    let c = "abcde";

    print!("{} {} {}",
        size_of_val(&a),
        size_of_val(&b),
        size_of_val(&c),
            );
    
    println!();

    println!("With &&");
    print!("{} {} {}",
        size_of_val(&&a),
        size_of_val(&&b),
        size_of_val(&&c),
            );


}