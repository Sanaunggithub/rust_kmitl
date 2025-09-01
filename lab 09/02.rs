fn main() {
    let mut vec1:Vec<i32> = Vec::new();
    println!("Vector1: Initial length {}" ,vec1.len());
    println!("Vector1: Capacity of the vector {}",vec1.capacity());
    
   for i in 1..=5{
        vec1.push(i);
   }

    println!("Vector1: Initial length {}" ,vec1.len());
    println!("Vector1: Capacity of the vector {}",vec1.capacity());

    let mut vec2:Vec<i32> = Vec::with_capacity(10);

    for i in 1..=15 {
        vec2.push(i);
    }
 
    println!("Vector2:Initial length {}" ,vec2.len());
    println!("Vector2: Capacity of the vector {}",vec2.capacity());

}