fn main() {
    let vec = vec![1,2,3,4,5];
    let squared:Vec<i32> = vec.into_iter().map(|x| x * x).collect();
    println!("Squared Vector is {:?}",squared);

    let vec2 = vec![1,2,3,4,5];
    let sum = vec2.into_iter().fold(0,|acc,x| acc+x);
    print!("Sum is {}",sum);

    let vec3 = vec![1,2,3,4,5];
    let total:i32= vec3.iter().sum();
    println!("The total is {}", total);

    let product:i32 = vec3.iter().product();
    println!("The product is {}",product);

    let vec4 = vec![10,20,30,-2,230];
    let max= vec4.iter().max().unwrap();
    let min = vec4.iter().min().unwrap();
    println!("The maximum value is {}",max);
    println!("The minimum value is {}",min);

    let vec5 = vec![1,2,3,4,5,6,7,8,9,10];
    let positive = vec5.iter().all(|x| *x > 0);
    let even = vec5.iter().any(|x| *x % 2 == 0);
    println!("Is all number positive? {}",positive);
    println!("Is any number included? {}",even);

}