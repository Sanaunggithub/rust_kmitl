fn concat_strings(s1:&str,s2: &str) -> String {
    let mut c_string = "".to_string();
    c_string.push_str(s1);
    c_string.push_str(s2);
    return c_string
}
fn main() {
    let s1 = "Hello";
    let s2 = "World";
    let result = concat_strings(&s1, &s2);
    println!("{}", result);
}

fn get_substring(s:&str,start:u32,end:u32) -> String {
    s.to_string();
    return s[start..end]
}
fn main() {
    let s = String::from("Hello, World!");
    let result = get_substring(&s, 7, 12);
    println!("{}", result); // World
}