//! Main module of WordStar conversion command line utility

mod asciify;
mod control_count;
mod uni_chars;
mod ws_align;
mod ws_chars;
mod ws_control;
mod ws_dot_cmd;
mod ws_filters;
mod ws_mappings;
mod ws_overline;
mod ws_special;
mod ws_string;
mod ws_wrappers;

use std::io::{self, Seek, SeekFrom};

/// Attempts to convert a WordStar file presented to `stdin` into a
/// Unicode based text file on `stdout` via a temporary file
///
fn main() {
    let mut input = io::stdin();
    let mut output = tempfile::tempfile().expect("Cannot open temp file");
    asciify::convert_file(&mut input, &mut output).unwrap();

    let mut input = output;
    input.seek(SeekFrom::Start(0)).unwrap();
    let mut output = io::stdout();
    ws_filters::transform_file(&mut input, &mut output).unwrap();
}
