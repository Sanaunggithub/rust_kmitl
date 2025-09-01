fn main() {
    let text = "Hello, world";

    //iterating using chars()
    for ch in text.chars(){
        println!("{}",ch);
    }
    //iterating using bytes()
    for bytes in text.bytes(){
        println!("{}", bytes);
    }

}