use std::thread;

fn main() {
    let builder = thread::Builder::new()
    .name("custom thread".into())
    .stack_size(32 * 1024);

    let handle = builder.spawn(||{
        println!("Hello from {}", thread::current().name().unwrap() );// this unwrap extracts thread current name
    }).unwrap();// this uses unwrap because builder.spawn() returns Result<>s

    handle.join().unwrap();
}