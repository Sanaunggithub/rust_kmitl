fn create_operator(operator: char, factor: i32)-> impl Fn(i32) -> i32 {
    move |x| match operator{
        '+' => x + factor,
        '-' => x - factor,
        '*' => x * factor,
        '/' => x / factor,
        _ => x
    }
}

fn main() {
    let add_ten = create_operator('+',10);
    let multiply_by_five = create_operator('*',5);

    println!("15 + 10 = {}",add_ten(15));
    println!("6 * 5 = {}",multiply_by_five(6));
}