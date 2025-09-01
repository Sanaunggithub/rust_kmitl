use std::thread;

fn main(){
    let mut v = Vec::new();
    for i in 1..=5 {
        let handle = thread::spawn(move||{
            println!("Thread {}: Result {}",i, i * i);
        });
        v.push(handle);
    }

    for handle in v{
        handle.join().unwrap();
    }
}