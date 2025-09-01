fn main() {
    let vec1 = vec!["hello", "world"];
    println!("The first one is {}",vec1[0]);

    let mut vec2 = Vec::new();
    vec2.push(0);
    println!("{:?}",vec2);

    let mut vec:Vec<i32> = vec![1,2,3];
    vec = vec![];

    println!("{:?}",vec);

    let arr = [1,2,3,4,5];
    let vec = Vec::from(arr);
    println!("{:?}",vec);

    let vec2 = arr.to_vec();
    println!("{:?}",vec2);
}