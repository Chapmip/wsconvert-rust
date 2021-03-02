//! Module to process any command line arguments supplied to `wsconvert`

use clap::{App, Arg};

/// Holds the values obtained by processing command line arguments
#[derive(Debug)]
pub struct Args {
    pub infile: String,
    pub outfile: String,
}

/// Returns an `Args` structure containing the processed arguments (if any)
/// from the command line input
///
impl Args {
    pub fn parse() -> Self {
        let matches = App::new("wsconvert")
            .arg(
                Arg::with_name("infile")
                    .short("i")
                    .long("infile")
                    .takes_value(true)
                    .help("Read from a file instead of stdin"),
            )
            .arg(
                Arg::with_name("outfile")
                    .short("o")
                    .long("outfile")
                    .takes_value(true)
                    .help("Write to a file instead of stdout"),
            )
            .get_matches();

        let infile = matches.value_of("infile").unwrap_or_default().to_string();
        let outfile = matches.value_of("outfile").unwrap_or_default().to_string();

        Self { infile, outfile }
    }
}
