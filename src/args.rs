//! Module to process any command line arguments supplied to `wsconvert`

use crate::ws_filters::Excludes;
use clap::{crate_version, App, Arg};

// Log output settings

const LOG_OFF: &str = "off";
const LOG_ERROR: &str = "error";
const LOG_WARN: &str = "warn";
const LOG_INFO: &str = "info";
const LOG_DEBUG: &str = "debug";
const LOG_TRACE: &str = "trace";

const LOG_VALUES: [&str; 6] = [LOG_OFF, LOG_ERROR, LOG_WARN, LOG_INFO, LOG_DEBUG, LOG_TRACE];

// Exclude filter settings

const EXCLUDE_DOT_CMDS: &str = "dot-cmds";
const EXCLUDE_RE_ALIGN: &str = "re-align";
const EXCLUDE_SPECIALS: &str = "specials";
const EXCLUDE_OVERLINE: &str = "overline";
const EXCLUDE_WRAPPERS: &str = "wrappers";
const EXCLUDE_CONTROLS: &str = "controls";

const EXCLUDE_VALUES: [&str; 6] = [
    EXCLUDE_DOT_CMDS,
    EXCLUDE_RE_ALIGN,
    EXCLUDE_SPECIALS,
    EXCLUDE_OVERLINE,
    EXCLUDE_WRAPPERS,
    EXCLUDE_CONTROLS,
];

/// Holds the values obtained by processing command line arguments
#[derive(Debug)]
pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub log_level: log::LevelFilter,
    pub excludes: Excludes,
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
                Arg::with_name("log-level")
                    .short("l")
                    .long("log-level")
                    .takes_value(true)
                    .possible_values(&LOG_VALUES)
                    .case_insensitive(true)
                    .help("Logging level"),
            )
            .arg(
                Arg::with_name("x-names")
                    .short("x")
                    .long("exclude")
                    .takes_value(true)
                    .possible_values(&EXCLUDE_VALUES)
                    .multiple(true)
                    .use_delimiter(true)
                    .case_insensitive(true)
                    .help("Filters to exclude"),
            )
            .get_matches();

        let infile = matches.value_of("infile").unwrap_or_default().to_string();
        let outfile = matches.value_of("outfile").unwrap_or_default().to_string();
        let log_str = matches.value_of("log-level").unwrap_or_default();
        let exclude_vec: Vec<&str> = matches.values_of("x-names").unwrap_or_default().collect();

        let log_level = get_log_level(&log_str);
        let excludes = get_excludes(&exclude_vec);

        Self {
            infile,
            outfile,
            log_level,
            excludes,
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
/// * `log_str` - Desired log level as text slice
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

/// Returns `Excludes` struct corresponding to one or more exclusions
/// specified in command line, or default of no exclusions (no flags
/// set) if none are specified
///
/// # Arguments
///
/// * `exclude_strs` - List of exclusions as Vector of text slices
///
/// # Examples
/// ```
/// assert_eq!(get_excludes(&vec!("specials")), Excludes::SPECIALS);
/// ```
fn get_excludes(exclude_strs: &[&str]) -> Excludes {
    let mut excludes = Excludes::NONE;
    for exclude_str in exclude_strs {
        match exclude_str.to_lowercase().as_str() {
            EXCLUDE_DOT_CMDS => excludes.insert(Excludes::DOT_CMDS),
            EXCLUDE_RE_ALIGN => excludes.insert(Excludes::RE_ALIGN),
            EXCLUDE_SPECIALS => excludes.insert(Excludes::SPECIALS),
            EXCLUDE_OVERLINE => excludes.insert(Excludes::OVERLINE),
            EXCLUDE_WRAPPERS => excludes.insert(Excludes::WRAPPERS),
            EXCLUDE_CONTROLS => excludes.insert(Excludes::CONTROLS),
            _ => {}
        }
    }
    excludes
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

    #[test]
    fn test_get_excludes() {
        assert_eq!(get_excludes(&vec!("specials")), Excludes::SPECIALS);
        assert_eq!(
            get_excludes(&vec!("OverLINE", "WRAPPERS")),
            Excludes::OVERLINE | Excludes::WRAPPERS
        );
        assert_eq!(get_excludes(&vec!("")), Excludes::NONE);
    }
}
