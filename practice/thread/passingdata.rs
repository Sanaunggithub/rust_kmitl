use std::thread::*;

fn main() {
    let data = vec![1,2,3];

    let handle = spawn(move || {
        println!("thread data: {:?}",data);
    });

    handle.join().unwrap();
}