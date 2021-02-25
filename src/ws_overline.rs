//! Module to process WordStar "overline" sequences (adding a bar above text)

// Note: utilises new "bool then" feature in Rust 1.50 to simplify Option return
//     (condition).then(|| ())
//  -> if (condition) { Some( () ) } else { None }

use crate::ws_chars;
use crate::ws_string;

// EXTERNAL PUBLIC FUNCTIONS

/// Returns `Some(replacement)` if the given text slice contains one or more
/// overlined sections to be converted, otherwise `None`
///
/// Overlining is marked by a special sequence: a number of `ws_chars::OVERPRINT`
/// characters followed by a `ws_chars::SUPERSCRIPT` wrapper character, the same
/// number of `ws_chars::UNDERSCORE` characters as the overprint characters and
/// then another `ws_chars::SUPERSCRIPT` wrapper character.  The same number of
/// non control characters must be found before this special sequence.  This text
/// is converted by wrapping it in a pair of `ws_chars::OVERLINE` characters.
/// The rest of the special sequence is discarded from the replacement String.
///
/// If the above special sequence is not matched precisely, then no replacement
/// will be made for it.
///
/// # Arguments
///
/// * `s` - Slice of text to be processed
///
/// # Examples
/// ```
/// assert_eq!(process("Q\x08\x14_\x14"), Some("\x01Q\x01".to_string()));
/// ```
pub fn process(s: &str) -> Option<String> {
    let mut changed = false;
    let mut result = String::with_capacity(s.len());
    let mut rest = s;
    while let Some((left, bars, right)) = ws_string::split_first_three(rest, ws_chars::SUPERSCRIPT)
    {
        if ws_string::contains_only_char(bars, ws_chars::UNDERSCORE) {
            let len = ws_string::len_in_chars(bars);
            if let Some((prefix, text, over)) = ws_string::split_last_three(left, len) {
                if ws_string::contains_only_char(over, ws_chars::OVERPRINT)
                    && ws_string::contains_only_print(text)
                {
                    result.push_str(prefix);
                    result.push(ws_chars::OVERLINE);
                    result.push_str(text);
                    result.push(ws_chars::OVERLINE);
                    rest = right;
                    changed = true;
                    continue;
                }
            }
        }
        // Not an exact match: restore and store original text up to 'right'
        result.push_str(left);
        result.push(ws_chars::SUPERSCRIPT);
        result.push_str(bars);
        result.push(ws_chars::SUPERSCRIPT);
        rest = right;
    }
    if changed {
        result.push_str(rest);
        Some(result)
    } else {
        None
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(
            process("See DAC\x08\x08\x08\x14___\x14, RFD\x08\x08\x08\x14___\x14 and DAV"),
            Some("See \x01DAC\x01, \x01RFD\x01 and DAV".to_string())
        );
        assert_eq!(
            process("See DAC\x08?\x08\x14___\x14, RFD\x08\x08\x08\x14___\x14 and DAV"),
            Some("See DAC\x08?\x08\x14___\x14, \x01RFD\x01 and DAV".to_string())
        );
        assert_eq!(process("abcd"), None);
        assert_eq!(process(""), None);
    }
}
