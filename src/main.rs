mod asciify;
mod control_count;

use std::io::{self, Seek, SeekFrom};
use tempfile;
use control_count::ControlCount;

// TESTING CODE HERE FOR NOW...

use std::io::{Read, Write, BufRead, BufReader, BufWriter};  // + self
use std::char;

fn transform_line(input: &str, counts: &mut ControlCount) -> String {
    let mut output = String::with_capacity(input.len()*2);
    for c in input.chars() {
        if c.is_ascii_control() {
            counts.up(c);
            output.push('^');
            output.push(match char::from_u32(c as u32 + '@' as u32) {
                Some(ch) => ch,
                None => '*'
            });
        } else {
            output.push(c);
        }
    }
    output
}

fn transform_file(input: &mut impl Read, output: &mut impl Write,
    counts: &mut ControlCount) -> io::Result<()> {
    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    for line in reader.lines() {
        let line = line?;
        writeln!(writer, "{}", transform_line(&line, counts))?;
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
    let mut counts = ControlCount::new("Counts".to_string());
    transform_file(&mut input, &mut output, &mut counts).unwrap();
    println!("{}", counts);
}