//! Module to process any command line arguments supplied to `wsconvert`

use clap::{crate_version, App, Arg};

// Log output settings

const LOG_OFF: &str = "off";
const LOG_ERROR: &str = "error";
const LOG_WARN: &str = "warn";
const LOG_INFO: &str = "info";
const LOG_DEBUG: &str = "debug";
const LOG_TRACE: &str = "trace";

const LOG_VALUES: [&str; 6] = [LOG_OFF, LOG_ERROR, LOG_WARN, LOG_INFO, LOG_DEBUG, LOG_TRACE];

/// Holds the values obtained by processing command line arguments
#[derive(Debug)]
pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub log_level: log::LevelFilter,
}

/// Returns an `Args` structure containing the processed arguments (if any)
/// from the command line input
///
impl Args {
    pub fn parse() -> Self {
        let matches = App::new("wsconvert")
            .about("Converts old WordStar files into readable format")
            .version(crate_version!())
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
            .arg(
                Arg::with_name("log_level")
                    .short("l")
                    .long("log_level")
                    .takes_value(true)
                    .possible_values(&LOG_VALUES)
                    .case_insensitive(true)
                    .help("Set log output level"),
            )
            .get_matches();

        let infile = matches.value_of("infile").unwrap_or_default().to_string();
        let outfile = matches.value_of("outfile").unwrap_or_default().to_string();
        let log_str = matches.value_of("log_level").unwrap_or_default();

        let log_level = get_log_level(log_str);

        Self {
            infile,
            outfile,
            log_level,
        }
    }
}

// PRIVATE HELPER FUNCTIONS

/// Returns `log::LevelFilter` enum value corresponding to input text slice
/// or default of `log::LevelFilter::Error` if text slice is empty or not
/// recognised
///
/// # Arguments
///
/// * `log_str` - Desired log level specified as text slice
///
/// # Examples
/// ```
/// assert_eq!(get_log_level("INFO"), log::LevelFilter::Info);
/// ```

fn get_log_level(log_str: &str) -> log::LevelFilter {
    match log_str.to_lowercase().as_str() {
        LOG_OFF => log::LevelFilter::Off,
        LOG_ERROR => log::LevelFilter::Error,
        LOG_WARN => log::LevelFilter::Warn,
        LOG_INFO => log::LevelFilter::Info,
        LOG_DEBUG => log::LevelFilter::Debug,
        LOG_TRACE => log::LevelFilter::Trace,
        _ => log::LevelFilter::Error, // Default setting
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_log_level() {
        assert_eq!(get_log_level("info"), log::LevelFilter::Info);
        assert_eq!(get_log_level("INFO"), log::LevelFilter::Info);
    }
}
