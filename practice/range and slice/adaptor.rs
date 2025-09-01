fn main() {
    //map()
    let numbers = vec![1,2,3,4,5];
    let squared:Vec<i32> = numbers.iter().map(|x| x*x).collect();

    println!("{:?}",squared);

    //enumerate()
    let characters = vec!['a','b','c'];
    for (index, ch) in characters.iter().enumerate() {
        println!("Index {} : {}",index,ch);
    }

    //zip()
    let numbers = vec![1,2,3];
    let letters = vec!['a','b','c'];
    let pairs:Vec<(i32,char)> = numbers.into_iter().zip(letters.into_iter()).collect();
    println!("{:?}",pairs);

    //chaining
    let result:Vec<i32> = (1..100)
    .filter(|x| x % 3 == 0)
    .map(|x| x * 2)
    .take(5)
    .collect();

    println!("The result is {:?}",result);
}