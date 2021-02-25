//! Module to re-align spaces outside pairs of certain WordStar "wrappers"

// Note: utilises new "bool then" feature in Rust 1.50 to simplify Option return
//     (condition).then(|| ())
//  -> if (condition) { Some( () ) } else { None }

use crate::ws_chars;
use crate::ws_string;

// Wrappers to be aligned (i.e. leading and trailing spaces moved outside wrapper)
pub const WRAPPERS_TO_ALIGN: [char; 5] = [
    ws_chars::BOLD,   // %% TO REMOVE SOON %%
    ws_chars::DOUBLE, // %% TO REMOVE SOON %%
    //  ws_chars::OVERLINE,     // %% TO ADD SOON %%
    ws_chars::UNDERLINE,
    ws_chars::STRIKETHROUGH,
    ws_chars::ITALIC, // %% TO REMOVE SOON %%
];

// PRIVATE HELPER FUNCTIONS

/// Returns `Some(replacement)` if the given text slice has whitespace characters
/// immediately inside a pair of the given "wrapper" characters, otherwise `None`
///
/// The text slice is scanned from left to right for a pair of wrapper characters.
/// If a pair is found and the text between them contains whitespace characters at
/// either end, then the text is re-written with the whitespace characters moved
/// outside the pair of wrapper characters, and this new String is returned.
///
/// Note that whitespace characters may still appear within the text between the
/// pair of wrapper characters -- just not at either end.
///
/// # Arguments
///
/// * `s` - Slice of text to be scanned
///
/// # Examples
/// ```
/// assert_eq!(align_wrapper("a* bc *d", '*'), Some("a *bc* d".to_string()));
/// ```
fn align_wrapper(s: &str, wrapper: char) -> Option<String> {
    let mut changed = false;
    let mut result = String::with_capacity(s.len());
    let mut rest = s;
    while let Some((left, text, right)) = ws_string::split_first_three(rest, wrapper) {
        result.push_str(left);
        let (spc_left, text, spc_right) = ws_string::split_space_at_ends(text);
        result.push_str(spc_left);
        result.push(wrapper);
        result.push_str(text);
        result.push(wrapper);
        result.push_str(spc_right);
        rest = right;
        if !spc_left.is_empty() || !spc_right.is_empty() {
            changed = true;
        }
    }
    if changed {
        result.push_str(rest);
        Some(result)
    } else {
        None
    }
}

/// Returns `Some(replacement)` if the given text slice contains whitespace characters
/// that have been re-aligned outside a pair of wrapper characters, otherwise `None`
///
/// The text slice is scanned from left to right for a pair of each of the defined
/// set of wrapper characters (in `WRAPPERS_TO_ALIGN`).  If a pair is found and the
/// text between them contains whitespace characters at either end, then the text
/// is re-written with the whitespace characters moved outside the pair of wrapper
/// characters, and this new String is returned.
///
/// Note that whitespace characters may still appear within the text between pairs
/// of wrapper characters -- just not at either end.
///
/// # Arguments
///
/// * `s` - Slice of text to be scanned
///
/// # Examples
/// ```
/// assert_eq!(align_all_wrappers("\x13 abc \x13"), Some(" \x13abc\x13 ".to_string()));
/// ```
fn align_all_wrappers(s: &str) -> Option<String> {
    let mut changed = false;
    let mut result = String::new(); // Always gets replaced if needed
    let mut line = s;
    for wrapper in &WRAPPERS_TO_ALIGN {
        if let Some(fixed) = align_wrapper(line, *wrapper) {
            result = fixed;
            line = &result;
            changed = true;
        }
    }
    changed.then(|| result)
}

// EXTERNAL PUBLIC FUNCTIONS

/// Returns `Some(replacement)` if the given text slice contains whitespace characters
/// that have been re-aligned outside a pair of wrapper characters, after scanning
/// repeatedly until no further re-alignments are possible, otherwise `None`
///
/// This function calls `align_all_wrappers()` repeatedly until no further changes
/// can be made, to handle the possibility that whitespace needs to be moved outside
/// multiple layers of "wrapper" characters in stages.
///
/// # Arguments
///
/// * `s` - Slice of text to be processed
///
/// # Examples
/// ```
/// assert_eq!(process("\x18\x13 a \x13\x18"), Some(" \x18\x13a\x13\x18 ".to_string()));
/// ```
pub fn process(s: &str) -> Option<String> {
    let mut changed = false;
    let mut result = String::new(); // Always gets replaced if needed
    let mut line = s;
    while let Some(fixed) = align_all_wrappers(line) {
        result = fixed;
        line = &result;
        changed = true;
    }
    changed.then(|| result)
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_wrapper() {
        assert_eq!(
            align_wrapper("a*  bc  *d", '*'),
            Some("a  *bc*  d".to_string())
        );
        assert_eq!(align_wrapper("a*  bc  d", '*'), None);
        assert_eq!(align_wrapper("a  bc  d", '*'), None);
        assert_eq!(
            align_wrapper("*a * bc * *d", '*'),
            Some("*a*  bc ** d".to_string())
        );
        assert_eq!(
            align_wrapper("*a * bc * *d", '*'),
            Some("*a*  bc ** d".to_string())
        );
        assert_eq!(align_wrapper("abcd", '*'), None);
        assert_eq!(align_wrapper("", '*'), None);
    }

    #[test]
    fn test_align_all_wrappers() {
        assert_eq!(
            align_all_wrappers("\x13  abc  \x13"),
            Some("  \x13abc\x13  ".to_string())
        );
        assert_eq!(
            align_all_wrappers(" \x18 abc \x18 "),
            Some("  \x18abc\x18  ".to_string())
        );
        assert_eq!(align_all_wrappers("abcd"), None);
        assert_eq!(align_all_wrappers(""), None);
    }

    #[test]
    fn test_process() {
        assert_eq!(
            process("\x18\x13  abc  \x13\x18"),
            Some("  \x18\x13abc\x13\x18  ".to_string())
        );
        assert_eq!(
            process(" \x18  \x13 abc \x19 def \x13 \x19\x18"),
            Some("    \x18\x13abc  \x19def\x13\x19\x18  ".to_string())
        );
        assert_eq!(process("abcd"), None);
        assert_eq!(process(""), None);
    }
}
