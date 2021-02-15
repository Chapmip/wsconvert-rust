mod asciify;
mod control_count;

use control_count::ControlCount;
use std::io::{self, Seek, SeekFrom};

// TESTING CODE HERE FOR NOW...

use std::char;
use std::io::{BufRead, BufReader, BufWriter, Read, Write}; // + self

fn transform_line(input: &str) -> String {
    let mut output = String::with_capacity(input.len() * 2);
    for c in input.chars() {
        if c.is_ascii_control() {
            output.push('^');
            output.push(match c as u32 {
                u @ 0..=0x1F => char::from_u32(u + '@' as u32).unwrap_or('*'),
                0x7F => '#',
                _ => '?',
            });
        } else {
            output.push(c);
        }
    }
    output
}

fn transform_file(
    input: &mut impl Read,
    output: &mut impl Write,
    counts: &mut ControlCount,
) -> io::Result<()> {
    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    for line in reader.lines() {
        let line = line?;
        counts.scan(&line);
        writeln!(writer, "{}", transform_line(&line))?;
    }

    writer.flush()?;
    Ok(())
}

fn main() {
    let mut input = io::stdin();
    let mut output = tempfile::tempfile().expect("Cannot open temp file");
    asciify::convert_file(&mut input, &mut output).unwrap();

    let mut input = output;
    input.seek(SeekFrom::Start(0)).unwrap();
    let mut output = io::stdout();
    let mut counts = ControlCount::new("Counts".to_string());
    transform_file(&mut input, &mut output, &mut counts).unwrap();
    println!("{}", counts);
}
