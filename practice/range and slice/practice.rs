fn main(){
    let i:Vec<_> = (1..10).step_by(2).collect();
    println!("{:?}",i);

    let k:Vec<_> = (2..20).rev().step_by(2).collect();
    println!("{:?}",k);

    let numbers = vec![11,22,33,44,55,66,77,88,99];
    let range = 2..8;
    for j in &numbers[range]{
        println!("{} ",j);
    }

    let mut arr = [1,2,3,4,5];
    let sl = &mut arr[1..4];

    for i in sl{
        print!("{} ",i);
    }
    println!();

    let my = [1,2,3,4,5,6,7];
    let s1 = &my[1..];
    let s2 = &my[..3];

    println!("Open-ended: {:?}",s1);
    println!("close-ended: {:?}",s2);

    //iterator

    let numbers = vec![1,2,3,4,5,6];
    let result:i32 = numbers
    .iter()
    .filter(|&x| x % 2 == 0)
    .map(|&x|x * x)
    .sum();

    println!("The number is {}",result);

}