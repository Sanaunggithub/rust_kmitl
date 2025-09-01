use std::io;

fn main() {
    // Create an empty String to hold the user input
    let mut input = String::new();
    
    // Print a prompt for the user
    println!("Enter some text:");

    // Read a line of input from the console and append it to the 'input' String
    let bytes_read = io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    // Output the number of bytes read and the input itself
    print!("You entered: {}", input);
    println!("Number of bytes read: {}", bytes_read);
}