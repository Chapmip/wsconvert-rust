use std::io::{self, Read, Write};

mod asciify;

const CHUNK_SIZE: usize = 16 * 1024;

fn main() {
    let mut buffer = [0; CHUNK_SIZE];
    let mut total_input = 0;
    let mut total_output = 0;
    
    loop {
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,             // No further input
            Ok(x) => x,
            Err(_) => break,            // Some kind of error
        };
        total_input += num_read;
        dbg!(num_read);
        
        let conv = asciify::convert_chunk(&mut buffer[..num_read]);
        let num_conv = conv.len();
        total_output += num_conv;
        dbg!(num_conv);
        
        io::stdout().write_all(&buffer[..num_conv]).unwrap();
        
        if num_conv < num_read {
            break;                      // EOF encountered
        }
    }
    eprintln!("total input bytes: {}", total_input);
    eprintln!("total output bytes: {}", total_output);
}
