//! Main module of WordStar conversion command line utility

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

use std::io;

/// Sets up parameters and then calls `process()` in `ws_file` module
///
fn main() -> io::Result<()> {
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .init();

    let infile = String::from("");
    let outfile = String::from("");
    ws_file::process(&infile, &outfile)
}
