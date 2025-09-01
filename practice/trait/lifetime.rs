fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { //'a is generic lifetime    
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    //same lifetime
    let string1 = String::from("long string"); //string1 and string2 can have different lifetimes that'why they need a lifetime specifier.
    let string2 = String::from("short");

    let result = longest(&string1, &string2);
    println!("The longest string is: {}", result);
}
