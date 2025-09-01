fn main() {
    for i in 0..12{
        print!("{} ",i);
    }

    println!();
    print!("The dozen is ");
    let dozen = 0..5;

    for i in dozen{
        print!("{} ",i);
    }
    println!();
    let range:std::ops::Range<usize> = 3..8;
    println!("{:?} {} {} {}",range,range.start,range.end,range.len());
}