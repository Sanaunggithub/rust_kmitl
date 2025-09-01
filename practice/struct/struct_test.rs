#[derive(Debug)]
struct Color (u32,u32,u32);
#[derive(Debug)]
struct SizeAndColor {
    size:u32,
    color:Color,
}

fn main() {
    let my_color = Color(250,100,50);

    let size_color = SizeAndColor {
        size :100,
        color : my_color,
    };  

    println!("{:?}",size_color);
}