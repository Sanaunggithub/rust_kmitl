fn main(){
    let fullname = "Saw,San,Phone";
    let tokens:Vec<&str>= fullname.split(",").collect();
    println!("firstName is {}",tokens[0]);
    println!("lastname is {}",tokens[1]);
    println!("company is {}",tokens[2]);
 
}