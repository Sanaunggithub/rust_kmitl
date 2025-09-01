fn get_substring(s:&String,start:usize,end:usize) -> &str {
    let result ="".to_string();
    let result = &s[start..end];
    return result
}
fn main() {
    let s = String::from("Hello, World!");
    let result = get_substring(&s, 7, 12);
    println!("{}", result); // World
}