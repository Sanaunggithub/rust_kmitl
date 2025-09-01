use std::sync::Mutex;

fn main(){
    let my_mutex = Mutex::new(5);
    let mut num = my_mutex.lock().unwrap();

    println!("Initial value: {}",num);
    *num+=1; //dereferencing is required here because num is smart pointer(mutex)
    println!("New Value: {}",num);
}