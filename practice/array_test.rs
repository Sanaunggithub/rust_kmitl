fn main() {
    let array1 = [1,2,3,4,5];//array literal

    let array1:[i32;5] = [1,2,3,4,5];

    println!("{:?}",array1);

    let array1 = [0;5];
    println!("{:?}",array1);

    //nested array

    let array1:[[i32;3];2] = [[1,2,3],[4,5,6]];

    println!("{:?}",array1);

    let array2 = [7,8,9];
    match array2.first() {
        Some(one) => println!("the first one is {}",one),
        None => println!("Oh no"),
    }
    
}
