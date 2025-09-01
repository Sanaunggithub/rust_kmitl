fn check_five(item:i32) -> Result<i32,String> {
    match item {
        5 => Ok(item),
        _ => Err("Sorry it's not five".to_string()),

    }
}

fn main() {
    let mut vec = Vec::new();

    for i in 2..7 {
        vec.push(check_five(i));
    }

    println!("{:?}",vec)
}