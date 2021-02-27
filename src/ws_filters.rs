//! Module to invoke WordStar format filters from input to output stream

// It may be more efficient for the various "process" filter functions
// to return `Cow<'_, str>` instead of `Option<String>`, but I'm still
// figuring that out!  One advantage of returning `Option<String>` is
// that the filter functions can use the Rust `?` operator as a terse
// way to exit immediately with a `None` result.

use crate::control_count::ControlCount;
use crate::ws_align;
use crate::ws_control;
use crate::ws_dot_cmd;
use crate::ws_overline;
use crate::ws_special;
use crate::ws_wrappers;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};

/// Transforms a line-formatted stream of 7-bit ASCII input characters
/// (e.g. from `asciify::convert_file`) into a line-formatted stream of
/// Unicode output characters that implement conversions of WordStar dot
/// commands, wrapper control characters and other special sequences
///
/// Returns `()` on success or a `std::io::Error` type on failure
///
/// # Arguments
///
/// * `input` - Source of bytes that implements `Read` trait
/// * `output` - Destination for bytes that implements `Write` trait
///
/// # Examples
/// ```
/// use std::io;
/// use ws_filters::transform_file;
///
/// let mut input = io::stdin();
/// let mut output = io::stdout();
/// transform_file(&mut input, &mut output).unwrap();
/// ```
pub fn transform_file(input: &mut impl Read, output: &mut impl Write) -> io::Result<()> {
    let mut original_counts = ControlCount::new("To ASCII".to_string());
    let mut post_dot_counts = ControlCount::new("Dot cmds".to_string());
    let mut alignment_counts = ControlCount::new("Re-align".to_string());
    let mut special_counts = ControlCount::new("Specials".to_string());
    let mut overline_counts = ControlCount::new("Overline".to_string());
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
                "" => continue, // Remove line from output
                _ => line = replacement,
            }
        }
        post_dot_counts.scan(&line);

        line = ws_align::process(&line).unwrap_or(line);
        alignment_counts.scan(&line);

        line = ws_special::process(&line).unwrap_or(line);
        special_counts.scan(&line);

        line = ws_overline::process(&line).unwrap_or(line);
        overline_counts.scan(&line);

        line = wrappers.process(&line).unwrap_or(line);
        wrappers_counts.scan(&line);

        line = ws_control::process(&line, true).unwrap_or(line);
        de_ctrl_counts.scan(&line);

        writeln!(writer, "{}", line)?;
    }
    writer.flush()?;

    eprintln!("Control characters after processing:");
    eprintln!("{}", original_counts);
    eprintln!("{}", post_dot_counts);
    eprintln!("{}", alignment_counts);
    eprintln!("{}", special_counts);
    eprintln!("{}", overline_counts);
    eprintln!("{}", wrappers_counts);
    eprintln!("{}", de_ctrl_counts);
    Ok(())
}
