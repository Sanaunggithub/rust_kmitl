fn modify_by_value(mut x: i32) {
    x += 10;
    println!("X in modify_by_value is {}", x);
}

fn modify_by_ref(x: &mut i32) {
    *x += 10;
}

fn main() {
    let mut num = 10;

    modify_by_value(num); // Pass by value

    modify_by_ref(&mut num); // Pass by mutable reference

    println!("num after modify_by_ref is {}", num); // Print the modified value
}
