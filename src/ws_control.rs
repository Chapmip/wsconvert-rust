//! Module to process standalone WordStar control characters

use crate::uni_chars;
use crate::ws_chars;
use std::char;

// PRIVATE HELPER FUNCTIONS

/// Returns `Some(replacement)` if the given character is a standalone WordStar
/// control character that can be mapped to replacement text, otherwise `None`
///
/// Two of the mappings here are for "phantom" characters on daisywheel printers
/// in which additional "hidden" characters on the printhead could be invoked.
/// In the absence of modern equivalents, these characters are mapped to Unicode
/// "block" characters.
///
/// # Arguments
///
/// * `c` - Character to be mapped to a replacement (if possible)
///
/// # Examples
/// ```
/// assert_eq!(get_mapping('\x0F'), Some("\u{00A0}"));
/// ```
fn get_mapping(c: char) -> Option<&'static str> {
    match c {
        ws_chars::PHANTOM_SPACE => Some(uni_chars::BLOCK),
        ws_chars::PHANTOM_RUBOUT => Some(uni_chars::BLOCK),
        ws_chars::FORM_FEED => Some("\n---\n"),
        ws_chars::NON_BREAKING_SPACE => Some(uni_chars::NB_SPACE),
        ws_chars::INACTIVE_SOFT_HYPHEN => Some(uni_chars::HYPHEN),
        ws_chars::ACTIVE_SOFT_HYPHEN => Some(uni_chars::HYPHEN),
        ws_chars::DELETE => Some(""), //Just remove it
        _ => None,
    }
}

/// Returns `Some(replacement)` if the given character is an ASCII control
/// character that can be mapped to an "escaped" sequence ('^' + substitute
/// printable character), otherwise `None`
///
/// Control characters in the range 00 hex to 1F hex are transformed to
/// their ASCII counterparts in the range 40 hex ('@') to 5F hex ('_').  A
/// "delete" character (7F hex) is additionally mapped to a "^#" sequence.
///
/// # Arguments
///
/// * `c` - Character to be mapped to a replacement (if possible)
///
/// # Examples
/// ```
/// assert_eq!(get_escaped('\x03'), Some("^C".to_string()));
/// ```
fn get_escaped(c: char) -> Option<String> {
    let printable = match c as u32 {
        u @ 0..=0x1F => char::from_u32(u + '@' as u32),
        0x7F => Some('#'),
        _ => None,
    }?;
    let mut escaped = String::with_capacity(2);
    escaped.push('^');
    escaped.push(printable);
    Some(escaped)
}

// EXTERNAL PUBLIC FUNCTION

/// Returns `Some(replacement)` if the given text slice contains control characters
/// that have been converted to an alternative representation, otherwise `None`
///
/// Two stages of conversion are attempted: the first maps recognised standalone
/// WordStar control characters to suitable alternatives; the second (only carried
/// out if `escape` is `true`), maps remaining ASCII control characters to their
/// "escaped" form (a sequence of '^' plus a corresponding printable character).
///
/// # Arguments
///
/// * `s` - Slice of text to be processed
/// * `escape` - Flag to convert unrecognised ASCII control characters to '^' format
///
/// # Examples
/// ```
/// assert_eq!(process_control("a\x0Fb", true), Some("a\u{00A0}b".to_string()));
/// ```
pub fn process_control(s: &str, escape: bool) -> Option<String> {
    let mut changed = false;
    let mut result = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        if c.is_ascii_control() {
            if let Some(substitute) = get_mapping(c) {
                result.push_str(substitute);
                changed = true;
            } else if escape {
                if let Some(substitute) = get_escaped(c) {
                    result.push_str(&substitute);
                    changed = true;
                } else {
                    result.push(c); // No escape sequence (shouldn't happen!)
                }
            } else {
                result.push(c); // Not escaping unmatched chars
            }
        } else {
            result.push(c); // Not a control character
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
        assert_eq!(get_mapping('\x0C'), Some("\n---\n"));
        assert_eq!(get_mapping('\x0F'), Some(uni_chars::NB_SPACE));
        assert_eq!(get_mapping('\x1E'), Some(uni_chars::HYPHEN));
        assert_eq!(get_mapping('\x7F'), Some(""));
        assert_eq!(get_mapping('a'), None);
    }

    #[test]
    fn test_get_escaped() {
        assert_eq!(get_escaped('\x00'), Some("^@".to_string()));
        assert_eq!(get_escaped('\x03'), Some("^C".to_string()));
        assert_eq!(get_escaped('\x13'), Some("^S".to_string()));
        assert_eq!(get_escaped('\x1F'), Some("^_".to_string()));
        assert_eq!(get_escaped('\x7F'), Some("^#".to_string()));
        assert_eq!(get_escaped('a'), None);
    }

    #[test]
    fn test_process_control() {
        assert_eq!(
            process_control("ab\x0Fcd\x1Eef\x1Fgh", true),
            Some("ab\u{00A0}cd\u{2010}ef\u{2010}gh".to_string())
        );
        assert_eq!(
            process_control("\x14ab\x06cd\x1Eef\x01", true),
            Some("^Tab\u{2588}cd\u{2010}ef^A".to_string())
        );
        assert_eq!(
            process_control("\x14ab\x06cd\x1Eef\x01", false),
            Some("\x14ab\u{2588}cd\u{2010}ef\x01".to_string())
        );
        assert_eq!(process_control("\x14abcde\x01", false), None);
        assert_eq!(
            process_control("abc\x0Cdef", true),
            Some("abc\n---\ndef".to_string())
        );
        assert_eq!(process_control("abcd", true), None);
        assert_eq!(process_control("", true), None);
    }
}
