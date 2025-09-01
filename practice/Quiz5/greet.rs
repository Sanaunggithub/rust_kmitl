fn greet(name:&String){
    println!("Hello {}", name);
}

fn goodbye(name: &String) -> String {
    format!("Goodbye {}",name)
}

fn main(){
    let name = String::from("Alice");
    greet(&name);
    let message = goodbye(&name);
    println!("{}",message);
}

