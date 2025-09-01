fn main() {
    //by using to_string
    let int = 45.to_string();
    let float = 4.5.to_string();
    let bool_str = true.to_string();

    println!("{}",int);
    println!("{}",float);
    println!("{}", bool_str);

    //by using format macro

    let int2 = format!("{}",500);
    let bool2 = format!("{}",false);

    println!("{}",int2);
    println!("{}",bool2);
}