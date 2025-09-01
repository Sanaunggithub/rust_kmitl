use std::thread::*;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx,rx) = mpsc::channel();

    let handle = spawn(move|| {
        let result = expensive_computation();
        tx.send(result).unwrap();
    }
    );

    let result = rx.recv().unwrap();
    println!("Received: {}",result);

    handle.join().unwrap();
}

fn expensive_computation() -> i32{
    sleep(Duration::from_secs(2));
    42
}