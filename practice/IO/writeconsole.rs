use std::io::Write;

fn main(){
    std::io::stdout().write("Hello ".as_bytes()).unwrap();
    std::io::stdout().write(String::from("world ").as_bytes()).unwrap();
    std::io::stdout().write(b"Manchester United").unwrap();

}