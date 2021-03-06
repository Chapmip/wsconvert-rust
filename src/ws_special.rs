//! Module to process special WordStar sequences (e.g. simple fractions)

// Written as an exercise in Rust regular expression parsing, as an alternative
// approach to the direct string processing used in the ws_emphasis module.

// Need to build regular expression raw strings dynamically as Rust does not
// yet support compile-time concatenation of constants, as opposed to literal
// expressions (so `concat!` is not an option).  This activity is constrained
// to occur only once (first time round) by using the `lazy_static!` macro.

use crate::uni_chars;
use crate::ws_chars;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

// PRIVATE HELPER FUNCTIONS

/// Returns `Some(replacement)` if the given text slice contains one or more special
/// sequences that have been converted to degree symbols, otherwise `None`
///
/// A degree symbol is indicated a pair of `ws_chars::SUPERSCRIPT` wrapper characters
/// with a single lower-case 'o' between them.  This sequence is converted to the
/// corresponding Unicode "degree" symbol.
///
/// # Arguments
///
/// * `s` - Slice of text to be processed
///
/// # Examples
/// ```
/// assert_eq!(transform_degrees("-40\x14o\x14C"), Some("-40\u{00B0}C".to_string()));
/// ```
fn transform_degrees(before: &str) -> Option<String> {
    lazy_static! {
        static ref REGEX_DEGREE: Regex = {
            let mut re = String::with_capacity(3);  // Can't calculate statically
            re.push(ws_chars::SUPERSCRIPT);
            re.push('o');
            re.push(ws_chars::SUPERSCRIPT);
            Regex::new(&re).unwrap()
        };
    }
    if let Cow::Owned(after) = REGEX_DEGREE.replace_all(before, uni_chars::DEGREE) {
        Some(after)
    } else {
        None
    }
}

/// Returns text slice containing Unicode fraction symbol corresponding to the "1"
/// or "3" numerator passed in the first captured parameter and the "2" or "4"
/// denominator passed in the second captured parameter, or `U+FFFD REPLACEMENT
/// CHARACTER` for an invalid combination.
///
/// # Arguments
///
/// * `caps` - Reference to group of captured strings for a regular expression match
///
fn get_fraction(caps: &regex::Captures) -> &'static str {
    match (&caps[1], &caps[2]) {
        ("1", "2") => uni_chars::HALF,
        ("1", "4") => uni_chars::ONE_QUARTER,
        ("3", "4") => uni_chars::THREE_QUARTERS,
        _ => uni_chars::REPLACEMENT, // Unable to map 3/2
    }
}

/// Returns `Some(replacement)` if the given text slice contains one or more special
/// sequences that have been converted to Unicode fraction symbols (1/2, 1/4 or 3/4),
/// otherwise `None`
///
/// A special fraction sequence is a pair of `ws_chars::UNDERLINE` wrapper characters
/// surrounding a pair of `ws_chars::SUPERSCRIPT` wrapper characters surrounding in
/// turn a '1' or '3' (as appropriate), followed by a `ws_chars::OVERPRINT` character
/// and then a pair of `ws_chars::SUBSCRIPT` wrapper characters surrounding a '2' or
/// '4' (as appropriate).  This sequence is converted to the corresponding Unicode
/// "one half", one quarter" or "three quarters" symbol.  A '3' followed by a '2' is
/// converted to a Unicode `U+FFFD REPLACEMENT CHARACTER` as there is no valid symbol
/// for this unexpected combination.
///
/// Note: Each special sequence can only be detected correctly if the input text has
/// not previously been processed with the `ws_wrappers` module, as otherwise the
/// underlined numerator of the fraction will be unrecognisable as it has been
/// converted to a new sequence using the Unicode underline combiner character.
///
/// # Arguments
///
/// * `s` - Slice of text to be processed
///
/// # Examples
/// ```
/// let before = "\x13\x141\x14\x13\x08\x164\x16";
/// assert_eq!(transform_quarter(before), Some("\u{00BE}".to_string()));
/// ```
fn transform_fraction(before: &str) -> Option<String> {
    lazy_static! {
        static ref REGEX_FRACTION: Regex = {
            let mut re = String::with_capacity(19);  // Can't calculate statically
            re.push(ws_chars::UNDERLINE);
            re.push(ws_chars::SUPERSCRIPT);
            re.push_str(r"([13])");
            re.push(ws_chars::SUPERSCRIPT);
            re.push(ws_chars::UNDERLINE);
            re.push(ws_chars::OVERPRINT);
            re.push(ws_chars::SUBSCRIPT);
            re.push_str(r"([24])");
            re.push(ws_chars::SUBSCRIPT);
            Regex::new(&re).unwrap()
        };
    }
    if let Cow::Owned(after) =
        REGEX_FRACTION.replace_all(before, |caps: &regex::Captures| get_fraction(caps))
    {
        Some(after)
    } else {
        None
    }
}

// EXTERNAL PUBLIC FUNCTION

/// Returns `Some(replacement)` if the given text slice contains any of the
/// special sequences and therefore needs to be replaced, otherwise `None`
///
/// # Arguments
///
/// * `s` - Slice of text to be processed
///
/// # Examples
/// ```
/// let before = "6\x141\x14\x08\x162\x16";
/// assert_eq!(process(before), Some("6\u{00BD}".to_string()));
/// ```
pub fn process(s: &str) -> Option<String> {
    let mut result: Option<String> = None;
    let mut line = s;

    result = transform_degrees(line).or(result);
    line = result.as_deref().unwrap_or(s);

    result = transform_fraction(line).or(result);

    result
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_degrees() {
        assert_eq!(
            transform_degrees("-40\x14o\x14C is -40\x14o\x14F"),
            Some("-40\u{00B0}C is -40\u{00B0}F".to_string())
        );
        assert_eq!(transform_degrees("abcd"), None);
        assert_eq!(transform_degrees(""), None);
    }

    #[test]
    fn test_transform_fraction() {
        assert_eq!(
            transform_fraction(
                "6\x13\x141\x14\x13\x08\x162\x16 has \x13\x141\x14\x13\x08\x162\x16!"
            ),
            Some("6\u{00BD} has \u{00BD}!".to_string())
        );
        assert_eq!(
            transform_fraction(
                "6\x13\x141\x14\x13\x08\x164\x16 or 6\x13\x143\x14\x13\x08\x164\x16"
            ),
            Some("6\u{00BC} or 6\u{00BE}".to_string())
        );
        assert_eq!(
            transform_fraction("\x13\x141\x14\x13\x08\x162\x16"),
            Some("\u{00BD}".to_string())
        );
        assert_eq!(
            transform_fraction("\x13\x143\x14\x13\x08\x162\x16"),
            Some("\u{FFFD}".to_string())
        );
        assert_eq!(transform_fraction("abcd"), None);
        assert_eq!(transform_fraction(""), None);
    }

    #[test]
    fn test_process() {
        assert_eq!(
            process("-40\x14o\x14C is -40\x14o\x14F"),
            Some("-40°C is -40°F".to_string())
        );
        assert_eq!(
            process("6\x13\x141\x14\x13\x08\x162\x16 has \x13\x141\x14\x13\x08\x162\x16!"),
            Some("6\u{00BD} has \u{00BD}!".to_string())
        );
        assert_eq!(
            process("6\x13\x141\x14\x13\x08\x164\x16 or 6\x13\x143\x14\x13\x08\x164\x16"),
            Some("6\u{00BC} or 6\u{00BE}".to_string())
        );
        assert_eq!(process("abcd"), None);
        assert_eq!(process(""), None);
    }
}
