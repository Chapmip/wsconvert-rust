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
static UNI_ONE_QUARTER: &str = "\u{00BC}"; // 1/4 symbol
const UNI_HALF: &str = "\u{00BD}"; // 1/2 symbol
static UNI_THREE_QUARTERS: &str = "\u{00BE}"; // 3/4 symbol
const UNI_REPLACEMENT: &str = "\u{FFFD}"; // Invalid marker

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

fn transform_half(before: &str) -> Option<String> {
    lazy_static! {
        static ref REGEX_HALF: Regex = {
            let mut re = String::with_capacity(7);  // Can't calculate statically
            re.push(ws_chars::SUPERSCRIPT);
            re.push('1');
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

fn get_quarters(caps: &regex::Captures) -> &'static str {
    match &caps["n"] {
        "1" => UNI_ONE_QUARTER,
        "3" => UNI_THREE_QUARTERS,
        _ => UNI_REPLACEMENT,
    }
}

fn transform_quarter(before: &str) -> Option<String> {
    lazy_static! {
        static ref REGEX_QUARTER: Regex = {
            let mut re = String::with_capacity(17);  // Can't calculate statically
            re.push(ws_chars::SUPERSCRIPT);
            re.push_str("(?P<n>[13])");
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

fn transform_subscript(_before: &str) -> Option<String> {
    // DUMMY STUB FOR NOW
    None
}

fn transform_superscript(_before: &str) -> Option<String> {
    // DUMMY STUB FOR NOW
    None
}

/// Returns `Some(replacement)` if the given text slice contains any of the
/// special sequences and therefore needs to be replaced, otherwise `None`
///
/// # Arguments
///
/// * `s` - Slice of text to be processed
///
/// # Examples
/// ```
/// assert_eq!(process_special("6\x141\x14\x08\x162\x16"), Some("6\u{00BD}".to_string()));
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

/*
fn main() {
    println!("{}", '\u{FFFD}'.len_utf8());

    let before = "-40\x14o\x14C is -40\x14o\x14F";
    let after = transform_degrees(before);
    if let Some(s) = after {
        println!("Transformed to {}", s);
    }
    assert_eq!(
        transform_degrees(before),
        Some("-40°C is -40°F".to_string())
    );

    let before = "6\x141\x14\x08\x162\x16 has \x141\x14\x08\x162\x16 in it!";
    let after = transform_half(before);
    if let Some(s) = after {
        println!("Transformed to {}", s);
    }
    assert_eq!(
        transform_half(before),
        Some("6\u{00BD} has \u{00BD} in it!".to_string())
    );

    let before = "6\x141\x14\x08\x164\x16 or 6\x143\x14\x08\x164\x16";
    let after = transform_quarter(before);
    if let Some(s) = after {
        println!("Transformed to {}", s);
    }
    assert_eq!(
        transform_quarter(before),
        Some("6\u{00BC} or 6\u{00BE}".to_string())
    );

    let before = "6\x141\x14\x08\x164\x16 or 6\x143\x14\x08\x164\x16";
    let after = process_special(before);
    if let Some(s) = after {
        println!("Transformed to {}", s);
    }
    assert_eq!(
        process_special(before),
        Some("6\u{00BC} or 6\u{00BE}".to_string())
    );
}
*/
