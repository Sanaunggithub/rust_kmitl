fn main() {
    
    let result: Result<i32, &str> = Err("Something went wrong");
    let value = result.expect("Failed to retrieve value");//this will panic
    println!("Value: {}", value);  // This will not be printed

    let result2: Result<i32, &str> = Ok(32);
    let value2 = result2.expect("Failed to retrieve value");
    println!("Value2: {}", value2); //The panic immediately halts the execution of the program, meaning none of the code after the panic will run.
                                    // So, the second Result (result2) and the corresponding println! statements will never be executed.
}
