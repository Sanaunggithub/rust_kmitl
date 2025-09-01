fn main() {
    let mut counter = 0;
    let mut counter2 = 0;

    'first:loop {
        counter += 1;
        println!("The counter is {}",counter);
        if counter > 9 {
            println!("We are going into second loop");
            'second:loop {
                println!("The second counter is {}",counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first;
                }
            }
        }
    }
}