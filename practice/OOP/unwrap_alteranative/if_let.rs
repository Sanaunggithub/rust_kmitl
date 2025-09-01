fn main(){
    let result:Result<i32,&str> = Ok(42);

    if let Ok(value) = result{
        println!("The value is {}",value)
    } else {
        println!("Error occured")
    }
}