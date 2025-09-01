fn main() {
    let mut days = vec![1,2,3,4,5,6,7];

    for day in days{
        match day {
            1 => print!("{} {}\t",day,"Monday".to_string()),
            2 => print!("{} {}\t",day,"Tuesday".to_string()),
            3 => print!("{} {}\t",day,"Wednesday".to_string()),
            4 => print!("{} {}\t",day,"Thursday".to_string()),
            5 => print!("{} {}\t",day,"Friday".to_string()),
            6 => println!("{} {}",day,"Saturday".to_string()),
            7 => print!("{} {}",day,"Sunday".to_string()),
            _ => print!("None"),

        }
    }
}