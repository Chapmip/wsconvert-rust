mod asciify;
mod control_count;
mod ws_chars;
mod ws_dot_cmd;
mod ws_emphasis;
mod ws_special;

use control_count::ControlCount;
use std::io::{self, Seek, SeekFrom};

use std::io::{BufRead, BufReader, BufWriter, Read, Write}; // + self

// TESTING CODE HERE FOR NOW...

use std::char;

fn process_control(input: &str, escape: bool) -> String {
    let mut output = String::with_capacity(input.len() * 2);
    for c in input.chars() {
        if c.is_ascii_control() {
            let substitute = match c {
                ws_chars::PHANTOM_SPACE => "???",  // %% TO BE ADDED %%
                ws_chars::PHANTOM_RUBOUT => "???", // %% TO BE ADDED %%
                ws_chars::NON_BREAKING_SPACE => "\u{00A0}",
                ws_chars::INACTIVE_SOFT_HYPHEN => "\u{2010}",
                ws_chars::ACTIVE_SOFT_HYPHEN => "\u{2010}",
                ws_chars::DELETE => "???", // %% TO BE ADDED %%
                _ => "",
            };
            if !substitute.is_empty() {
                output.push_str(substitute);
            } else if escape {
                output.push('^');
                output.push(match c as u32 {
                    u @ 0..=0x1F => char::from_u32(u + '@' as u32).unwrap_or('*'),
                    0x7F => '#',
                    _ => '?',
                });
            } else {
                output.push(c);
            }
        } else {
            output.push(c);
        }
    }
    output
}

fn transform_file(input: &mut impl Read, output: &mut impl Write) -> io::Result<()> {
    let mut original_counts = ControlCount::new("Original   ".to_string());
    let mut no_dot_counts = ControlCount::new("No dot cmd ".to_string());
    let mut emphasis_counts = ControlCount::new("Emphasis   ".to_string());
    let mut special_counts = ControlCount::new("Special    ".to_string());
    let mut final_counts = ControlCount::new("Final      ".to_string());

    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    for line in reader.lines() {
        let mut line = line?;
        original_counts.scan(&line);

        if let Some(replacement) = ws_dot_cmd::process_dot_cmd(&line) {
            match &replacement[..] {
                "" => continue,
                _ => line = replacement,
            }
        }
        no_dot_counts.scan(&line);

        if let Some(replacement) = ws_emphasis::process_emphasis(&line) {
            line = replacement;
        }
        emphasis_counts.scan(&line);

        if let Some(replacement) = ws_special::process_special(&line) {
            line = replacement;
        }
        special_counts.scan(&line);

        line = process_control(&line, true);
        final_counts.scan(&line);

        writeln!(writer, "{}", line)?;
    }
    writer.flush()?;

    eprintln!("{}", original_counts);
    eprintln!("{}", no_dot_counts);
    eprintln!("{}", emphasis_counts);
    eprintln!("{}", special_counts);
    eprintln!("{}", final_counts);
    Ok(())
}

fn main() {
    let mut input = io::stdin();
    let mut output = tempfile::tempfile().expect("Cannot open temp file");
    asciify::convert_file(&mut input, &mut output).unwrap();

    let mut input = output;
    input.seek(SeekFrom::Start(0)).unwrap();
    let mut output = io::stdout();
    transform_file(&mut input, &mut output).unwrap();
}
