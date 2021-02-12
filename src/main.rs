use std::io::{self, Seek, SeekFrom};
use tempfile;

mod asciify;

// TESTING CODE HERE FOR NOW...

use std::io::{Read, Write, BufRead, BufReader, BufWriter};  // + self

fn transform_line(input: &str) -> String {
    let mut output = String::with_capacity(input.len()*2);
    for c in input.chars() {
        output.push(c.to_ascii_uppercase());
        output.push(c.to_ascii_lowercase());
    }
    output
}

fn transform_file(input: &mut impl Read, output: &mut impl Write)
    -> io::Result<()> {
    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);
    
    for line in reader.lines() {
        let line = line?;
        writeln!(writer, "{}", transform_line(&line))?;
    }
    
    writer.flush()?;
    Ok(())
}

fn main() {
    let mut input = io::stdin();
    let mut output = tempfile::tempfile().expect("Cannot open temp file");
    asciify::convert_file(&mut input, &mut output)
        .unwrap();

    let mut input = output;
    input.seek(SeekFrom::Start(0)).unwrap();
    let mut output = io::stdout();
    transform_file(&mut input, &mut output).unwrap();
}