use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 4{
        println!("You need to provie 3 arguments: two numbers and one operator (+,-,*,/)");
    }

    let num1:f64 = match args[1].parse(){
        Ok(n) => n,
        Err(_) => {
            println!("The first argument is not a number");
            return;
        }
    };

    let num2:f64 =  match args[3].parse(){
        Ok(n) => n,
        Err(_) => {
            println!("The second argument is not a number");
            return;
        }
    };

    let result =  match args[2].as_str(){
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
        if num2 == 0.0 {
            println!("Division by zero");
            return
        }
        num1 / num2
        }
        _ => {println!("Unsupported operator error");
             return;
            }
    };

    println!("The result is {}", result);
}