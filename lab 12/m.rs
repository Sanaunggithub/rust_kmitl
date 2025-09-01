use std::thread::*;
use std::time::Duration;

fn main() {
   let handle = spawn(||{
        println!("Parking a thread");
        park()
   });

   sleep(Duration::from_secs(3));
   println!("Unparking a thread");
   handle.thread().unpark();

   handle.join().unwrap();
}