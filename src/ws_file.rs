//! Module to process input file to output file via temporary file

use crate::asciify;
use crate::ws_filters;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};

/// Attempts to convert a WordStar file from the input filename
/// (or `stdin` if empty) to a new Unicode based text file at the
/// output filename (or `stdout` if empty) via a temporary file
///
/// Returns `()` on success or a `std::io::Error` type on failure
///
/// Note: If an output filename is specified then an error will be
/// returned and no further action taken if the file already exists
///
/// # Arguments
///
/// * `infile` - Path to input file (or "" to use `stdin`)
/// * `outfile` - Path to output file (or "" to use `stdout`)
///
/// # Examples
/// ```
/// let excludes: ws_filters::Excludes = {...};
/// ws_file::process("input.ws", "output.txt", &excludes).unwrap();
/// ```
pub fn process(infile: &str, outfile: &str, excludes: &ws_filters::Excludes) -> io::Result<()> {
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(outfile)?,
        ))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    let mut intermediate = tempfile::tempfile()?;

    asciify::convert_file(&mut reader, &mut intermediate)?;
    intermediate.seek(SeekFrom::Start(0))?;
    ws_filters::transform_file(&mut intermediate, &mut writer, &excludes)?;
    Ok(())
}
