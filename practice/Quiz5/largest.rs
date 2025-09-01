fn find_longest_word(sentence: &str) -> &str {
    let mut longest_word = "";
    let mut max_length = 0;

    for word in sentence.split_whitespace() {
        if word.len() > max_length {
            longest_word = word;
            max_length = word.len();
        }
    }

    longest_word
}

fn main() {
    let sentence = "Rust is a systems programming language with a focus on safety and performance";
    let longest_word = find_longest_word(&sentence);
    println!("The longest word is: {}", longest_word);
}
