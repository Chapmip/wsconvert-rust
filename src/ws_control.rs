//! Module to process standalone WordStar control characters

use crate::ws_chars;
use std::char;

// Unicode strings for substitution (actually all single characters)

const UNI_NB_SPACE: &str = "\u{00A0}"; // Non-breaking space
const UNI_HYPHEN: &str = "\u{2010}"; // Hyphen (as opposed to dash)
const UNI_REPLACEMENT: &str = "\u{FFFD}"; // Invalid marker

// PRIVATE HELPER FUNCTIONS

fn get_mapping(c: char) -> Option<&'static str> {
    match c {
        ws_chars::PHANTOM_SPACE => Some(UNI_REPLACEMENT), // Placeholder
        ws_chars::PHANTOM_RUBOUT => Some(UNI_REPLACEMENT), // Placeholder
        ws_chars::FORM_FEED => Some("\n---\n"),
        ws_chars::NON_BREAKING_SPACE => Some(UNI_NB_SPACE),
        ws_chars::INACTIVE_SOFT_HYPHEN => Some(UNI_HYPHEN),
        ws_chars::ACTIVE_SOFT_HYPHEN => Some(UNI_HYPHEN),
        ws_chars::DELETE => Some(UNI_REPLACEMENT), // Placeholder
        _ => None,
    }
}

fn get_escaped(c: char) -> String {
    let mut escaped = String::with_capacity(2);
    if c.is_ascii_control() {
        escaped.push('^');
        escaped.push(match c as u32 {
            u @ 0..=0x1F => char::from_u32(u + '@' as u32).unwrap_or('*'),
            0x7F => '#',
            _ => '?',
        });
    } else {
        escaped.push(c);
    }
    escaped
}

// EXTERNAL PUBLIC FUNCTION

pub fn process_control(input: &str, escape: bool) -> Option<String> {
    let mut changed = false;
    let mut result = String::with_capacity(input.len() * 2);
    for c in input.chars() {
        if c.is_ascii_control() {
            if let Some(substitute) = get_mapping(c) {
                result.push_str(substitute);
                changed = true;
            } else if escape {
                let substitute = get_escaped(c);
                result.push_str(&substitute);
                changed = true;
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }
    changed.then(|| result)
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mapping() {
        // %% TO BE ADDED %%
    }

    #[test]
    fn test_get_escaped() {
        assert_eq!(get_escaped('\x00'), "^@".to_string());
        assert_eq!(get_escaped('\x03'), "^C".to_string());
        assert_eq!(get_escaped('\x13'), "^S".to_string());
        assert_eq!(get_escaped('\x1F'), "^_".to_string());
        assert_eq!(get_escaped('\x7F'), "^#".to_string());
        assert_eq!(get_escaped('a'), "a".to_string());
    }

    #[test]
    fn test_process_control() {
        // %% MORE TO BE ADDED %%
        assert_eq!(
            process_control("\x14abc\x16", true),
            Some("^Tabc^V".to_string())
        );
        assert_eq!(
            process_control("abc\x0Cdef", true),
            Some("abc\n---\ndef".to_string())
        );
        assert_eq!(process_control("abcd", true), None);
        assert_eq!(process_control("", true), None);
    }
}
