fn main() {
    let factor = 2;

    let multiply = |a| a * factor;  
    
    let multiply_ref = &multiply;

    println!("{}", (*multiply_ref)(13));

    print!("{}",(|a| a * factor)(29));
}
