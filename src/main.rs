use std::io;

mod asciify;

fn main() {
    let mut input = io::stdin();
    let mut output = io::stdout();
    
    asciify::convert_file(&mut input, &mut output)
        .unwrap();
}