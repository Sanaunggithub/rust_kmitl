fn main() {
    let numbers = [0,1,2,3,4,5,6,7,8,9];
    let range = 2..8;

    println!("Range: {:?}",&numbers[range]);

    //iterating using step_by()

    for i in (2..8).step_by(2){
        println!("{}",numbers[i]);
    }
}