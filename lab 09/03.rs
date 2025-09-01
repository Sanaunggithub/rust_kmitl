use std::mem::*;
use std::slice::from_raw_parts;

struct Data(i32,char,bool);

fn main() {
    
    let data1 = Data(5,'a',true);
    let bytes:&[u8] = unsafe {from_raw_parts(&data1 as *const Data as *const u8, size_of::<Data>())};

    print!("Byte representation of Data Struct:");

    for byte in bytes {
        print!("{:02x} ",byte);
    }
    println!();

}