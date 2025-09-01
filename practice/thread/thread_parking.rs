use std::thread::*;
use std::time::Duration;

fn main() {
    let parked_thread = spawn(||{
        println!("Parking a thread");
        park();
    });

    sleep(Duration::from_secs(2));

    println!("Unparking Thread");
    parked_thread.thread().unpark();
    println!("thread Unparked");

    parked_thread.join().unwrap();
}