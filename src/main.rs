//! Main module of WordStar conversion command line utility

mod args;
mod asciify;
mod control_count;
mod uni_chars;
mod ws_align;
mod ws_chars;
mod ws_control;
mod ws_dot_cmd;
mod ws_file;
mod ws_filters;
mod ws_mappings;
mod ws_overline;
mod ws_special;
mod ws_string;
mod ws_wrappers;

use crate::args::Args;
use std::io;

/// Reads command line parameters, sets up logging and then calls
/// `ws_file::process()` with any supplied parameters
///
fn main() -> io::Result<()> {
    let args = Args::parse();

    env_logger::builder()
        .format_timestamp(None)
        .filter_level(args.log_level)
        .init();

    ws_file::process(&args.infile, &args.outfile, &args.excludes)
}
