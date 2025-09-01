use std::cmp::Ordering;

fn main(){
    let mut arr = [4,8,10,2,7,9,10];
    arr.sort_by(|a,b| {
        if a < b{
            std::cmp::Ordering::Greater
        } else if a > b {
            std::cmp::Ordering::Less
        } else {std::cmp::Ordering::Equal}
    });

    println!("{:?}",arr);
}