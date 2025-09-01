use std::io::{Read, Write};

fn main(){
    let mut command_line: std::env::Args = std::env::args();
    command_line.next().unwrap(); // skips program name
    let source = command_line.next().unwrap(); // source file path 
    let destination = command_line.next().unwrap(); // destination file path
    let mut file_in = std::fs::File::open(source).unwrap();
    let mut file_out = std::fs::File::create(destination).unwrap();
    let mut buffer = [0u8; 4096];/*This defines a buffer of size 4096 bytes (4KB) to temporarily hold data read from the source file. 
                                  This buffer will be reused in the loop for reading and writing chunks of data.*/


    loop {
    let nbytes = file_in.read(&mut buffer).unwrap();//This reads up to 4096 bytes from the source file into the buffer. The read function returns the number of bytes that were actually read (nbytes).
    file_out.write_all(&buffer[..nbytes]).unwrap(); //This writes the nbytes number of bytes (the amount just read) from the buffer to the destination file.
                                                    //&buffer[..nbytes] creates a slice of the buffer containing only the portion that was filled by the read operation.
    
    if nbytes < buffer.len() { 
        break;//If fewer bytes were read than the buffer size, it indicates that the end of the file has been reached, so the loop is terminated with break.
    }
    }
}