mod asciify;
mod control_count;
mod uni_chars;
mod ws_align;
mod ws_chars;
mod ws_control;
mod ws_dot_cmd;
mod ws_emphasis;
mod ws_mappings;
mod ws_special;
mod ws_string;
mod ws_wrappers;

use control_count::ControlCount;
use std::io::{self, Seek, SeekFrom};

// TESTING CODE HERE FOR NOW...

use std::io::{BufRead, BufReader, BufWriter, Read, Write}; // + self

fn transform_file(input: &mut impl Read, output: &mut impl Write) -> io::Result<()> {
    let mut original_counts = ControlCount::new("Original".to_string());
    let mut post_dot_counts = ControlCount::new("Dot cmds".to_string());
    let mut alignment_counts = ControlCount::new("Re-align".to_string());
    let mut emphasis_counts = ControlCount::new("Emphasis".to_string());
    let mut special_counts = ControlCount::new("Specials".to_string());
    let mut wrappers_counts = ControlCount::new("Wrappers".to_string());
    let mut de_ctrl_counts = ControlCount::new("Controls".to_string());

    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);
    let mut wrappers = ws_wrappers::Wrappers::new();

    for line in reader.lines() {
        let mut line = line?;
        original_counts.scan(&line);

        if let Some(replacement) = ws_dot_cmd::process(&line) {
            match &replacement[..] {
                "" => continue,
                _ => line = replacement,
            }
        }
        post_dot_counts.scan(&line);

        if let Some(replacement) = ws_align::process(&line) {
            line = replacement;
        }
        alignment_counts.scan(&line);

        if let Some(replacement) = ws_emphasis::process(&line) {
            line = replacement;
        }
        emphasis_counts.scan(&line);

        if let Some(replacement) = ws_special::process(&line) {
            line = replacement;
        }
        special_counts.scan(&line);

        if let Some(replacement) = wrappers.process(&line) {
            line = replacement;
        }
        wrappers_counts.scan(&line);

        if let Some(replacement) = ws_control::process(&line, true) {
            line = replacement;
        }
        de_ctrl_counts.scan(&line);

        writeln!(writer, "{}", line)?;
    }
    writer.flush()?;

    eprintln!("{}", original_counts);
    eprintln!("{}", post_dot_counts);
    eprintln!("{}", alignment_counts);
    eprintln!("{}", emphasis_counts);
    eprintln!("{}", special_counts);
    eprintln!("{}", wrappers_counts);
    eprintln!("{}", de_ctrl_counts);
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
