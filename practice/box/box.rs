fn main() {
    let my_box = Box::new(1);
    let my_num = *my_box;
    println!("{:?}",my_box);
    println!("{:?}",my_num);
    println!("{:?}",my_box);

}