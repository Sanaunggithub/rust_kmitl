enum Number {
    I32(i32),
    U32(u32),
}

fn get_number (input :i32) -> Number {
    match input.is_positive() {
        true => Number::U32(input as u32),
        _ => Number::I32(input)
    }
}

fn main() {
    let my_vec = vec![get_number(-800), get_number(8)];
    use Number::*;

    for item in my_vec {
        match item {
            U32(n) => println!("It's u32 {}",n),
            I32(n) => println!("It's i32 {}",n),
        }
    }
}