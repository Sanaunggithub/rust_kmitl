use std::fmt::*;

trait MyTrait<T> {
    fn process(&self, value: T);
}

struct MyStruct;

impl<T: Display> MyTrait<T> for MyStruct {
    fn process(&self, value: T) {
        println!("Processing value: {}", value);
    }
}

fn main() {
    let my_struct = MyStruct;
    my_struct.process(10); 
    my_struct.process("Hello, World!"); 
}
