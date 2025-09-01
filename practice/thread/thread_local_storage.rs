use std::cell::RefCell;
use std::thread;

thread_local!{
    static COUNTER:RefCell<u32> = RefCell::new(0);
}

fn main() {
    thread::spawn(||{
        COUNTER.with(|c|{
            *c.borrow_mut() += 1;
             println!("Thread1:{}",*c.borrow());
        });
    }).join().unwrap();

    COUNTER.with(|c| {
        *c.borrow_mut() += 2;
        println!("Main thread: {}",*c.borrow());
    });
}