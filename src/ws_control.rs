//! Module to process stand-alone WordStar control characters

use crate::ws_chars;
use std::char;

// PRIVATE HELPER FUNCTIONS

// Unicode strings for substitution (actually all single characters)

const UNI_NB_SPACE: &str = "\u{00A0}"; // Non-breaking space
const UNI_HYPHEN: &str = "\u{2010}"; // Hyphen (as opposed to dash)
const UNI_REPLACEMENT: &str = "\u{FFFD}"; // Invalid marker

// EXTERNAL PUBLIC FUNCTION

pub fn process_control(input: &str, escape: bool) -> String {
    let mut output = String::with_capacity(input.len() * 2);
    for c in input.chars() {
        if c.is_ascii_control() {
            let substitute = match c {
                ws_chars::PHANTOM_SPACE => UNI_REPLACEMENT, // Placeholder
                ws_chars::PHANTOM_RUBOUT => UNI_REPLACEMENT, // Placeholder
                ws_chars::FORM_FEED => "\n---\n",
                ws_chars::NON_BREAKING_SPACE => UNI_NB_SPACE,
                ws_chars::INACTIVE_SOFT_HYPHEN => UNI_HYPHEN,
                ws_chars::ACTIVE_SOFT_HYPHEN => UNI_HYPHEN,
                ws_chars::DELETE => UNI_REPLACEMENT, // Placeholder
                _ => "",
            };
            if !substitute.is_empty() {
                output.push_str(substitute);
            } else if escape {
                output.push('^');
                output.push(match c as u32 {
                    u @ 0..=0x1F => char::from_u32(u + '@' as u32).unwrap_or('*'),
                    0x7F => '#',
                    _ => '?',
                });
            } else {
                output.push(c);
            }
        } else {
            output.push(c);
        }
    }
    output
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_control() {
        assert_eq!(process_control("\x14abc\x16", true), "^Tabc^V".to_string());
        assert_eq!(
            process_control("abc\x0Cdef", true),
            "abc\n---\ndef".to_string()
        );
        assert_eq!(process_control("abcd", true), "abcd".to_string());
        assert_eq!(process_control("", true), "".to_string());
    }
}
