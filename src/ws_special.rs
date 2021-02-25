//! Module to process special WordStar sequences (e.g. simple fractions)

// Written as an exercise in Rust regular expression parsing, as an alternative
// approach to the direct string processing used in the ws_emphasis module.

// Need to build regular expression raw strings dynamically as Rust does not
// yet support compile-time concatenation of constants, as opposed to literal
// expressions (so `concat!` is not an option).  This activity is constrained
// to occur only once (first time round) by using the `lazy_static!` macro.

use crate::uni_chars;
use crate::ws_chars;
use crate::ws_mappings;
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
            re.push(uni_chars::COMB_UNDERLINE);
            re.push(ws_chars::SUPERSCRIPT);
            re.push(ws_chars::OVERPRINT);
            re.push(ws_chars::SUBSCRIPT);
            re.push('2');
            re.push(ws_chars::SUBSCRIPT);
            Regex::new(&re).unwrap()
        };
    }
    if let Cow::Owned(after) = REGEX_HALF.replace_all(before, uni_chars::HALF) {
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
        "1" => uni_chars::ONE_QUARTER,
        "3" => uni_chars::THREE_QUARTERS,
        _ => uni_chars::REPLACEMENT,
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
            re.push_str(r"(?P<n>[13])");
            re.push(uni_chars::COMB_UNDERLINE);
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

/// Returns `Some(replacement)` if the given text slice contains one or more sequences
/// of subscripted characters that have been converted to the closest possible Unicode
/// symbols (or left alone if there is no close match), otherwise `None`
///
/// A subscripted sequence is indicated by a pair of `ws_chars::SUBSCRIPT` wrapper
/// characters with zero or more other characters between them.
///
/// # Arguments
///
/// * `s` - Slice of text to be processed
///
/// # Examples
/// ```
/// let before = "ab\x1620\x16cd";
/// assert_eq!(transform_subscript(before), Some("ab\u{2082}\u{2080}cd".to_string()));
/// ```
fn transform_subscript(before: &str) -> Option<String> {
    lazy_static! {
        static ref REGEX_SUBSCRIPT: Regex = {
            let mut re = String::with_capacity(9);  // Can't calculate statically
            re.push(ws_chars::SUBSCRIPT);
            re.push_str(r"([^");
            re.push(ws_chars::SUBSCRIPT);
            re.push_str(r"]*)");
            re.push(ws_chars::SUBSCRIPT);
            Regex::new(&re).unwrap()
        };
    }
    if let Cow::Owned(after) = REGEX_SUBSCRIPT.replace_all(before, |caps: &regex::Captures| {
        caps[1]
            .chars()
            .map(|c| ws_mappings::get_subscript(c).unwrap_or(c))
            .collect::<String>()
    }) {
        Some(after)
    } else {
        None
    }
}

/// Returns `Some(replacement)` if the given text slice contains one or more sequences
/// of superscripted characters that have been converted to the closest possible Unicode
/// symbols (or left alone if there is no close match), otherwise `None`
///
/// A superscripted sequence is indicated by a pair of `ws_chars::SUPERSCRIPT` wrapper
/// characters with zero or more other characters between them.
///
/// # Arguments
///
/// * `s` - Slice of text to be processed
///
/// # Examples
/// ```
/// let before = "ab\x1620\x16cd";
/// assert_eq!(transform_superscript(before), Some("ab\u{00B2}\u{2070}cd".to_string()));
/// ```
fn transform_superscript(before: &str) -> Option<String> {
    lazy_static! {
        static ref REGEX_SUPERSCRIPT: Regex = {
            let mut re = String::with_capacity(9);  // Can't calculate statically
            re.push(ws_chars::SUPERSCRIPT);
            re.push_str(r"([^");
            re.push(ws_chars::SUPERSCRIPT);
            re.push_str(r"]*)");
            re.push(ws_chars::SUPERSCRIPT);
            Regex::new(&re).unwrap()
        };
    }
    if let Cow::Owned(after) = REGEX_SUPERSCRIPT.replace_all(before, |caps: &regex::Captures| {
        caps[1]
            .chars()
            .map(|c| ws_mappings::get_superscript(c).unwrap_or(c))
            .collect::<String>()
    }) {
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
    fn test_transform_subscript() {
        assert_eq!(
            transform_subscript("ab\x1620\x16cd"),
            Some("ab\u{2082}\u{2080}cd".to_string())
        );
        assert_eq!(
            transform_subscript("ab\x1620\x16"),
            Some("ab\u{2082}\u{2080}".to_string())
        );
        assert_eq!(transform_subscript("ab\x1620cd"), None);
        assert_eq!(transform_subscript("abcd"), None);
        assert_eq!(transform_subscript(""), None);
    }

    #[test]
    fn test_transform_superscript() {
        assert_eq!(
            transform_superscript("ab\x1420\x14cd"),
            Some("ab\u{00B2}\u{2070}cd".to_string())
        );
        assert_eq!(
            transform_superscript("ab\x1420\x14"),
            Some("ab\u{00B2}\u{2070}".to_string())
        );
        assert_eq!(transform_superscript("ab\x1420cd"), None);
        assert_eq!(transform_superscript("abcd"), None);
        assert_eq!(transform_superscript(""), None);
    }

    #[test]
    fn test_process() {
        assert_eq!(
            process("-40\x14o\x14C is -40\x14o\x14F"),
            Some("-40°C is -40°F".to_string())
        );
        assert_eq!(
            process("6\x141\u{0332}\x14\x08\x162\x16 has \x141\u{0332}\x14\x08\x162\x16!"),
            Some("6\u{00BD} has \u{00BD}!".to_string())
        );
        assert_eq!(
            process("6\x141\u{0332}\x14\x08\x164\x16 or 6\x143\u{0332}\x14\x08\x164\x16"),
            Some("6\u{00BC} or 6\u{00BE}".to_string())
        );
        assert_eq!(
            process("ab\x1620\x16cd"),
            Some("ab\u{2082}\u{2080}cd".to_string())
        );
        assert_eq!(
            process("ab\x1420\x14cd"),
            Some("ab\u{00B2}\u{2070}cd".to_string())
        );
        assert_eq!(process("abcd"), None);
        assert_eq!(process(""), None);
    }
}
