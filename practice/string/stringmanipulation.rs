fn main() {
    let mut s = String::from("hello");
    
    // push: Add a single character
    s.push('!');
    println!("After push: {}", s); // Output: hello!

    // push_str: Append another string
    s.push_str(" world");
    println!("After push_str: {}", s); // Output: hello! world

    // insert: Add a character at a specified index
    s.insert(5, ',');
    println!("After insert: {}", s); // Output: hello,! world

    // remove: Remove a character by index
    s.remove(5);
    println!("After remove: {}", s); // Output: hello! world

    // pop: Remove and return the last character
    let last_char = s.pop();
    println!("After pop: {}, removed: {:?}", s, last_char); // Output: hello! worl, removed: Some('d')

    // split: Split a string into an iterator of string slices based on a delimiter
    let parts: Vec<&str> = s.split(' ').collect();
    println!("After split: {:?}", parts); // Output: ["hello!", "worl"]

    // replace: Replace all matches of a pattern with another string
    let replaced = s.replace("world", "Rust");
    println!("After replace: {}", replaced); // Output: hello! worl (since "world" isn't found in current `s`)

    // trim: Remove whitespace from the beginning and end of a string
    let trimmed = String::from("   hello! worl   ").trim().to_string();
    println!("After trim: '{}'", trimmed); // Output: 'hello! worl'
}
