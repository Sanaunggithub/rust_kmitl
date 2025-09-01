fn main(){
    let factor = 2;
    let multiply = |x| x * factor;

    let result = multiply(5);
    println!("The result is {}",result);

    let my_closure = || println!("This is my closure");
    my_closure();
}