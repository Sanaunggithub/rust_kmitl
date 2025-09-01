fn main() {
    let value =((1,2),(3,4));
    let ((a,b),(c,d)) = value;
    println!("{} {} {} {}",a,b,c,d);
}