fn main() {
    let mut count = 0;
    let mut increment = ||{
        count +=1;
        println!("Now the count is {}",count);
    };
    increment();
    increment();
}