fn main() {
    let x = 99;
    match x {
        heha @ 10..=100 => {
            println!("found {} number between 10 to 100!", heha);
            }
        _ => println!("hehe"),
    }
}
    