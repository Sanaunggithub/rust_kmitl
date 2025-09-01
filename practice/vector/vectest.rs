fn main() {
    let vec:Vec<i32> = (1..10).collect();
    println!("{:?}",vec);

    let arr =[1,2,3];
    let vec2:Vec<i32> = arr.iter().cloned().collect();
    println!("{:?}",vec2);

    let mut vec3 = vec![1,2,3,4,5];
    for i in &mut vec3{
        *i *= 2;
    }
    println!("{:?}",vec3);
    
    let mut vec4 = vec![12,13,14,15];
    for i in &mut vec4 {
        *i *= 2;
    }
    println!("{:?}",vec4);   
}