fn main() {
    let mut b = 5;

    let a = &b;
    let c = &b;
    println!("a: {} b: {} c: {}",a,b,c);

    /*this will cause an error 
    let d = 10;
    let h = &mut d;//
    let y = &mut d;
    println!("d: {} h: {} y:{}",d,h,y);
    */
    let d = 10;
    let h = &d;
    let y = &d;
    println!("d: {} h: {} y:{}",d,h,y);
    
}