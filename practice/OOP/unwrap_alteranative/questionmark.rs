fn main() {
    if let Err(e) = perform_division() {
        println!("Error occurred: {}", e);
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Cannot divide by zero".to_string());
    }
    Ok(a / b)
}

fn perform_division() -> Result<(), String> {
    let result = divide(10, 2)?; // Propagate the error if it occurs.If divide(10, 0) returns an Err,the error is immediately returned from perform_division too.
    println!("Result: {}", result);
    Ok(())// it returns Ok() if division is successful  
}