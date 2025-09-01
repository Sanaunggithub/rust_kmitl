use std::thread;

fn main() {
    let data = vec![1,2,3];

    thread::scope(|s| {
       s.spawn(|| {
        println!("Length: {}",data.len());
       });

       s.spawn(||{
        for item in &data{
            println!("Data: {}",item);
        }
       });

       s.spawn(||{
        for item in &data{
            println!("Data: {}",item * 2);
        }
       });
    });
}