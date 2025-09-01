fn divide(x:i32,y:i32) -> Result<i32,String> {
    if y == 0{
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(x /y)
    }
}

fn main(){
    let result = divide(10,2);
    match result {
        Ok(answer) => println!("The answer is {}",answer),
        Err(err) => println!("Error {}",err),
    }
}