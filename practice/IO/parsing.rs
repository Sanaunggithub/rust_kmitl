fn main(){
    println!("{:?}", "true".parse::<bool>().unwrap());
    println!("{:?}", "1.234".parse::<f32>().unwrap());
    println!("{:?}","false".parse::<bool>().unwrap());
}