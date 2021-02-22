//! Module to process special WordStar sequences (e.g. simple fractions)

// Written as an exercise in Rust regular expression parsing, as an alternative
// approach to the direct string processing used in the ws_emphasis module.

// Need to build regular expression raw strings dynamically as Rust does not
// yet support compile-time concatenation of constants, as opposed to literal
// expressions (so `concat!` is not an option).  This activity is constrained
// to occur only once (first time round) by using the `lazy_static!` macro.

use crate::ws_chars;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

// Unicode strings for substitution (actually all single characters)

const UNI_DEGREE: &str = "\u{00B0}"; // Degree symbol
const UNI_ONE_QUARTER: &str = "\u{00BC}"; // 1/4 symbol
const UNI_HALF: &str = "\u{00BD}"; // 1/2 symbol
const UNI_THREE_QUARTERS: &str = "\u{00BE}"; // 3/4 symbol
const UNI_REPLACEMENT: &str = "\u{FFFD}"; // Invalid marker

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
    if let Cow::Owned(after) = REGEX_DEGREE.replace_all(before, UNI_DEGREE) {
        Some(after)
    } else {
        None
    }
}

/// Returns `Some(replacement)` if the given text slice contains one or more special
/// sequences that have been converted to 1/2 (half) symbols, otherwise `None`
///
/// A half symbol is indicated a pair of `ws_chars::SUPERSCRIPT` wrapper characters
/// surrounding an underlined '1' followed by a `ws_chars::OVERPRINT` characters and
/// then a pair of `ws_chars::SUBSCRIPT` wrapper characters surrounding a '2'.  This
/// sequence is converted to the corresponding Unicode "half" symbol.
///
/// Note: Each special sequence can only be detected correctly if the input text has
/// previously been processed using `ws_emphasis::process_underlines()` (possibly
/// called via `ws_emphasis::process_emphasis()`) to render the underline below the
/// numerator of the fraction using the Unicode underline combiner character.
///
/// # Arguments
///
/// * `s` - Slice of text to be processed
///
/// # Examples
/// ```
/// let before = "\x141\u{0332}\x14\x08\x162\x16";
/// assert_eq!(transform_half(before), Some("\u{00BD}".to_string()));
/// ```
fn transform_half(before: &str) -> Option<String> {
    lazy_static! {
        static ref REGEX_HALF: Regex = {
            let mut re = String::with_capacity(9);  // Can't calculate statically
            re.push(ws_chars::SUPERSCRIPT);
            re.push('1');
            re.push(ws_chars::COMB_UNDERLINE);
            re.push(ws_chars::SUPERSCRIPT);
            re.push(ws_chars::OVERPRINT);
            re.push(ws_chars::SUBSCRIPT);
            re.push('2');
            re.push(ws_chars::SUBSCRIPT);
            Regex::new(&re).unwrap()
        };
    }
    if let Cow::Owned(after) = REGEX_HALF.replace_all(before, UNI_HALF) {
        Some(after)
    } else {
        None
    }
}

/// Returns text slice containing Unicode "quarters" character corresponding to the
/// "1" or "3" numerator passed in the captured "n" parameter.  Returns the Unicode
/// `U+FFFD REPLACEMENT CHARACTER` if the "n" parameter is not "1" or "3".
///
/// # Arguments
///
/// * `caps` - Reference to group of captured strings for a regular expression match
///
fn get_quarters(caps: &regex::Captures) -> &'static str {
    match &caps["n"] {
        "1" => UNI_ONE_QUARTER,
        "3" => UNI_THREE_QUARTERS,
        _ => UNI_REPLACEMENT,
    }
}

/// Returns `Some(replacement)` if the given text slice contains one or more special
/// sequences that have been converted to 1/4 (one quarter) or 3/4 (three quarters)
/// symbols, otherwise `None`
///
/// A one or three quarter symbol is indicated a pair of `ws_chars::SUPERSCRIPT`
/// wrapper characters surrounding an underlined '1' or '3' (as appropriate)followed
/// by a `ws_chars::OVERPRINT` characters and then a pair of `ws_chars::SUBSCRIPT`
/// wrapper characters surrounding a '4'.  This sequence is converted to the
/// corresponding Unicode "one quarter" or "three quarters" symbol.
///
/// Note: Each special sequence can only be detected correctly if the input text has
/// previously been processed using `ws_emphasis::process_underlines()` (possibly
/// called via `ws_emphasis::process_emphasis()`) to render the underline below the
/// numerator of the fraction using the Unicode underline combiner character.
///
/// # Arguments
///
/// * `s` - Slice of text to be processed
///
/// # Examples
/// ```
/// let before = "\x143\u{0332}\x14\x08\x164\x16";
/// assert_eq!(transform_quarter(before), Some("\u{00BE}".to_string()));
/// ```
fn transform_quarter(before: &str) -> Option<String> {
    lazy_static! {
        static ref REGEX_QUARTER: Regex = {
            let mut re = String::with_capacity(19);  // Can't calculate statically
            re.push(ws_chars::SUPERSCRIPT);
            re.push_str("(?P<n>[13])");
            re.push(ws_chars::COMB_UNDERLINE);
            re.push(ws_chars::SUPERSCRIPT);
            re.push(ws_chars::OVERPRINT);
            re.push(ws_chars::SUBSCRIPT);
            re.push('4');
            re.push(ws_chars::SUBSCRIPT);
            Regex::new(&re).unwrap()
        };
    }
    if let Cow::Owned(after) =
        REGEX_QUARTER.replace_all(before, |caps: &regex::Captures| get_quarters(caps))
    {
        Some(after)
    } else {
        None
    }
}

/// Returns the nearest equivalent Unicode subscripted version (if any) of the given
/// character, or just the original given character if no conversion is available
///
/// # Arguments
///
/// * `c` - Character to be transformed into its subscripted equivalent (if any)
///
/// # Examples
/// ```
/// assert_eq!(get_subscripted('m'), '\u{2098}');
/// ```
fn get_subscripted(c: char) -> char {
    match c.to_ascii_lowercase() {
        '0' => '\u{2080}',
        '1' => '\u{2081}',
        '2' => '\u{2082}',
        '3' => '\u{2083}',
        '4' => '\u{2084}',
        '5' => '\u{2085}',
        '6' => '\u{2086}',
        '7' => '\u{2087}',
        '8' => '\u{2088}',
        '9' => '\u{2089}',
        '+' => '\u{208A}',
        '-' => '\u{208B}',
        '=' => '\u{208C}',
        '(' => '\u{208D}',
        ')' => '\u{208E}',
        'a' => '\u{2090}',
        'e' => '\u{2091}',
        'h' => '\u{2096}',
        'i' => '\u{1D62}',
        'j' => '\u{2C7C}',
        'k' => '\u{2095}',
        'l' => '\u{2097}',
        'm' => '\u{2098}',
        'n' => '\u{2099}',
        'o' => '\u{2092}',
        'p' => '\u{209A}',
        'r' => '\u{1D63}',
        's' => '\u{209B}',
        't' => '\u{209C}',
        'u' => '\u{1D64}',
        'v' => '\u{1D65}',
        'x' => '\u{2093}',
        c => c,
    }
}

fn transform_subscript(_before: &str) -> Option<String> {
    // DUMMY STUB FOR NOW
    dbg!(get_subscripted('a'));
    None
}

/// Returns the nearest equivalent Unicode superscripted version (if any) of the given
/// character, or just the original given character if no conversion is available
///
/// # Arguments
///
/// * `c` - Character to be transformed into its superscripted equivalent (if any)
///
/// # Examples
/// ```
/// assert_eq!(get_superscripted('m'), '\u{1D50}');
/// ```
fn get_superscripted(c: char) -> char {
    match c.to_ascii_lowercase() {
        '0' => '\u{2070}',
        '1' => '\u{00B9}',
        '2' => '\u{00B2}',
        '3' => '\u{00B3}',
        '4' => '\u{2074}',
        '5' => '\u{2075}',
        '6' => '\u{2076}',
        '7' => '\u{2077}',
        '8' => '\u{2078}',
        '9' => '\u{2079}',
        '+' => '\u{207A}',
        '-' => '\u{207B}',
        '=' => '\u{207C}',
        '(' => '\u{207D}',
        ')' => '\u{207E}',
        'a' => '\u{1D43}',
        'b' => '\u{1D47}',
        'c' => '\u{1D9C}',
        'd' => '\u{1D48}',
        'e' => '\u{1D49}',
        'f' => '\u{1DA0}',
        'g' => '\u{1D4D}',
        'h' => '\u{02B0}',
        'i' => '\u{2071}',
        'j' => '\u{02B2}',
        'k' => '\u{1D4F}',
        'l' => '\u{02E1}',
        'm' => '\u{1D50}',
        'n' => '\u{207F}',
        'o' => '\u{1D52}',
        'p' => '\u{1D56}',
        'r' => '\u{02B3}',
        's' => '\u{02E2}',
        't' => '\u{1D57}',
        'u' => '\u{1D58}',
        'v' => '\u{1D5B}',
        'w' => '\u{02B7}',
        'x' => '\u{02E3}',
        'y' => '\u{02B8}',
        'z' => '\u{1DBB}',
        c => c,
    }
}

fn transform_superscript(_before: &str) -> Option<String> {
    // DUMMY STUB FOR NOW
    dbg!(get_superscripted('a'));
    None
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
/// assert_eq!(process_special(before), Some("6\u{00BD}".to_string()));
/// ```
pub fn process_special(s: &str) -> Option<String> {
    let mut changed = false;
    let mut result = String::new(); // Always gets replaced if needed
    let mut line = s;

    if let Some(replacement) = transform_degrees(line) {
        result = replacement;
        line = &result;
        changed = true;
    }
    if let Some(replacement) = transform_half(line) {
        result = replacement;
        line = &result;
        changed = true;
    }
    if let Some(replacement) = transform_quarter(line) {
        result = replacement;
        line = &result;
        changed = true;
    }
    if let Some(replacement) = transform_subscript(line) {
        result = replacement;
        line = &result;
        changed = true;
    }
    if let Some(replacement) = transform_superscript(line) {
        result = replacement;
        changed = true;
    }
    changed.then(|| result)
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
    fn test_transform_half() {
        assert_eq!(
            transform_half("6\x141\u{0332}\x14\x08\x162\x16 has \x141\u{0332}\x14\x08\x162\x16!"),
            Some("6\u{00BD} has \u{00BD}!".to_string())
        );
        assert_eq!(transform_half("abcd"), None);
        assert_eq!(transform_half(""), None);
    }

    #[test]
    fn test_transform_quarter() {
        assert_eq!(
            transform_quarter("6\x141\u{0332}\x14\x08\x164\x16 or 6\x143\u{0332}\x14\x08\x164\x16"),
            Some("6\u{00BC} or 6\u{00BE}".to_string())
        );
        assert_eq!(transform_quarter("abcd"), None);
        assert_eq!(transform_quarter(""), None);
    }

    #[test]
    fn test_get_subscripted() {
        assert_eq!(get_subscripted('m'), '\u{2098}');
        assert_eq!(get_subscripted('&'), '&');
    }

    #[test]
    fn test_transform_subscript() {
        // DUMMY STUB FOR NOW
        assert_eq!(transform_subscript("abcd"), None);
        assert_eq!(transform_subscript(""), None);
    }

    #[test]
    fn test_get_superscripted() {
        assert_eq!(get_superscripted('m'), '\u{1D50}');
        assert_eq!(get_superscripted('&'), '&');
    }

    #[test]
    fn test_transform_superscript() {
        // DUMMY STUB FOR NOW
        assert_eq!(transform_superscript("abcd"), None);
        assert_eq!(transform_superscript(""), None);
    }

    #[test]
    fn test_process_special() {
        assert_eq!(
            process_special("-40\x14o\x14C is -40\x14o\x14F"),
            Some("-40°C is -40°F".to_string())
        );
        assert_eq!(
            process_special("6\x141\u{0332}\x14\x08\x162\x16 has \x141\u{0332}\x14\x08\x162\x16!"),
            Some("6\u{00BD} has \u{00BD}!".to_string())
        );
        assert_eq!(
            process_special("6\x141\u{0332}\x14\x08\x164\x16 or 6\x143\u{0332}\x14\x08\x164\x16"),
            Some("6\u{00BC} or 6\u{00BE}".to_string())
        );
        assert_eq!(process_special("abcd"), None);
        assert_eq!(process_special(""), None);
    }
}
