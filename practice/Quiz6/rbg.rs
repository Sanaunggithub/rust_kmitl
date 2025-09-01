struct Colour (u8,u8,u8);

fn main() {
    let rbg = Colour (250,0,0);
    println!("Red: {}",rbg.0);
    println!("Green: {}",rbg.1);
    println!("Blue: {}",rbg.2);

}