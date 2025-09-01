fn main() {
    let mut data = ("Rust",1.23,true);
    match data {
       (name,number,boolean) => println!("{} {} {}",name,number,boolean)
    }
}