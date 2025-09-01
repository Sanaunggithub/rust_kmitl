fn main(){
    let a = [10,20,40,50];

    let mut iter = a.iter();
    println!("{:?}",iter);

    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
}