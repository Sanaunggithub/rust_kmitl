trait Iterator{
    type Item; //associated type

    fn next(&mut self) -> Self::Item;

}

struct Counter {
    count:i32,
}

impl Iterator for Counter{
    type Item = i32;

    fn next(&mut self) -> i32 {
        self.count +=1;
        self.count
    }
}
fn main() {
    let mut counter = Counter{count:0};
    for _ in 0..10{
        println!("Conuter: {}", counter.next());
    }
}