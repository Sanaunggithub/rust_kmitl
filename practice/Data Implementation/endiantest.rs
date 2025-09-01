fn main(){
    let num =  0x12345678u32;
    let big_endian = num.to_be();
    let little_endian = num.to_le();

    println!("{}",big_endian);
    println!("{}",little_endian);   
}