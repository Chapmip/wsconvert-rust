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

/// Holds a set of flags to specify filters to be excluded
#[derive(Debug, Default, PartialEq)]
pub struct Excludes {
    pub dot_cmds: bool,
    pub re_align: bool,
    pub specials: bool,
    pub overline: bool,
    pub wrappers: bool,
    pub controls: bool,
}

/// Transforms a line-formatted stream of 7-bit ASCII input characters
/// (e.g. from `asciify::convert_file`) into a line-formatted stream of
/// Unicode output characters that implement conversions of WordStar dot
/// commands, wrapper control characters and other special sequences,
/// optionally excluding a set of `Excludes` filters
///
/// Returns `()` on success or a `std::io::Error` type on failure
///
/// # Arguments
///
/// * `input` - Source of bytes that implements `Read` trait
/// * `output` - Destination for bytes that implements `Write` trait
/// * `excludes` - Optional set of flags to specify filters to exclude
///
/// # Examples
/// ```
/// use std::io;
/// use ws_filters::transform_file;
///
/// let mut input = io::stdin();
/// let mut output = io::stdout();
/// transform_file(&mut input, &mut output, None).unwrap();
/// ```
pub fn transform_file(
    input: &mut dyn Read,
    output: &mut dyn Write,
    excludes: Option<Excludes>,
) -> io::Result<()> {
    let mut dot_cmds_replaced = 0u32;
    let mut dot_cmds_removed = 0u32;
    let mut original_counts = ControlCount::new("To ASCII".to_string());
    let mut dot_cmds_counts = ControlCount::new("Dot-cmds".to_string());
    let mut re_align_counts = ControlCount::new("Re-align".to_string());
    let mut specials_counts = ControlCount::new("Specials".to_string());
    let mut overline_counts = ControlCount::new("Overline".to_string());
    let mut wrappers_counts = ControlCount::new("Wrappers".to_string());
    let mut controls_counts = ControlCount::new("Controls".to_string());

    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);
    let excludes = excludes.unwrap_or_default();
    let mut wrappers = ws_wrappers::Wrappers::new();

    for line in reader.lines() {
        let mut line = line?;
        original_counts.scan(&line);

        if !excludes.dot_cmds {
            if let Some(replacement) = ws_dot_cmd::process(&line) {
                match &replacement[..] {
                    "" => {
                        dot_cmds_removed += 1;
                        continue; // Remove line from output
                    }
                    _ => {
                        dot_cmds_replaced += 1;
                        line = replacement;
                    }
                }
            }
            dot_cmds_counts.scan(&line);
        }

        if !excludes.re_align {
            line = ws_align::process(&line).unwrap_or(line);
            re_align_counts.scan(&line);
        }

        if !excludes.specials {
            line = ws_special::process(&line).unwrap_or(line);
            specials_counts.scan(&line);
        }

        if !excludes.overline {
            line = ws_overline::process(&line).unwrap_or(line);
            overline_counts.scan(&line);
        }

        if !excludes.wrappers {
            line = wrappers.process(&line).unwrap_or(line);
            wrappers_counts.scan(&line);
        }

        if !excludes.controls {
            line = ws_control::process(&line, true).unwrap_or(line);
            controls_counts.scan(&line);
        }

        writeln!(writer, "{}", line)?;
    }
    writer.flush()?;

    eprintln!("Dot commands after processing:");
    eprintln!("Replaced: {}", dot_cmds_replaced);
    eprintln!("Removed:  {}", dot_cmds_removed);

    eprintln!("Control characters after processing:");
    eprintln!("{}", original_counts);
    eprintln!("{}", dot_cmds_counts);
    eprintln!("{}", re_align_counts);
    eprintln!("{}", specials_counts);
    eprintln!("{}", overline_counts);
    eprintln!("{}", wrappers_counts);
    eprintln!("{}", controls_counts);
    Ok(())
}
