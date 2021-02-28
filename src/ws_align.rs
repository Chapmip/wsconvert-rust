//! Module to re-align spaces outside pairs of WordStar "wrapper" control characters

use crate::ws_chars;

// Wrappers to be aligned (i.e. leading and trailing spaces moved outside wrapper)
const WRAPPERS_TO_ALIGN: [char; 8] = [
    ws_chars::OVERLINE,
    ws_chars::BOLD,
    ws_chars::DOUBLE,
    ws_chars::UNDERLINE,
    ws_chars::SUPERSCRIPT,
    ws_chars::SUBSCRIPT,
    ws_chars::STRIKETHROUGH,
    ws_chars::ITALIC,
];

// PRIVATE HELPER FUNCTIONS

/// Alignment states within `align_reverse()` processing state machine
#[derive(PartialEq)]
enum AlignState {
    Outside,
    Preamble,
    Text,
}

/// Returns `Some((reversed, changed))` if the given text slice contains an even
/// number (including zero) of the given 'wrapper' characters, with the contained
/// tuple holding the reversed (and possibly changed) text and a flag indicating
/// whether any alignment changes have been made, otherwise `None`
///
/// The text slice is scanned in reverse (i.e. from right to left) for any wrapper
/// characters indicating the start and end of a wrapped sequence.  Within this
/// sequence, any whitespace characters are moved ahead of any other control
/// characters, up to the first non-whitespace and non-control character, or the
/// end wrapper character for the sequence.  If the end of the slice is reached
/// unexpectedly before the end wrapper character is found, then the text slice
/// cannot be unambiguously re-aligned, so `None` is returned.  Otherwise, a tuple
/// is returned containing the reversed string and a flag to indicate whether any
/// whitespace has been moved.
///
/// Note that this procedure only works for whitespace found to the left of a
/// starting wrapper character and it returns a reversed string, so the procedure
/// needs to be repeated in order to ensure that whitespace at both ends of the
/// given wrapper sequence is re-aligned.
///
/// Note also that whitespace characters may still appear within text between each
/// pair of wrapper characters -- just not at either end.
///
/// # Arguments
///
/// * `s` - Slice of text to be scanned
///
/// # Examples
/// ```
/// assert_eq!(align_reverse("a* bc *d", '*'), Some(("d *cb *a".to_string(), true)));
/// ```
fn align_reverse(s: &str, wrapper: char) -> Option<(String, bool)> {
    let mut changed = false;
    let mut state = AlignState::Outside;
    let mut result = String::with_capacity(s.len());
    let mut wrappers = String::with_capacity(s.len());

    for c in s.chars().rev() {
        match state {
            AlignState::Outside => {
                if c == wrapper {
                    wrappers.push(c);
                    state = AlignState::Preamble;
                } else {
                    result.push(c);
                }
            }
            AlignState::Preamble => {
                if c == wrapper {
                    result.push_str(&wrappers);
                    wrappers.clear();
                    result.push(c);
                    state = AlignState::Outside;
                } else if char::is_ascii_control(&c) {
                    wrappers.push(c);
                } else if char::is_ascii_whitespace(&c) {
                    changed = true;
                    result.push(c);
                } else {
                    result.push_str(&wrappers);
                    wrappers.clear();
                    result.push(c);
                    state = AlignState::Text;
                }
            }
            AlignState::Text => {
                result.push(c);
                if c == wrapper {
                    state = AlignState::Outside;
                }
            }
        }
    }
    if state == AlignState::Outside {
        Some((result, changed))
    } else {
        None // Odd number of wrappers, so unable to resolve
    }
}

/// Returns `Some(replacement)` if the given text slice contains whitespace characters
/// that have been re-aligned outside one or more pairs of the given wrapper character,
/// otherwise `None`
///
/// The text slice is scanned first from right to left and then from left to right for
/// pairs of the each of the given wrapper character.  If a non-even number of wrapper
/// characters is found then `None` is returned to indicate that the text slice cannot
/// be unambiguously re-aligned and so should be left alone.  If the text between any
/// pair of the given wrapper characters contains whitespace characters at either end,
/// then the text is re-written with the whitespace characters moved outside each pair
/// of wrapper characters and this replacement string is returned.  Otherwise, `None`
/// is returned to indicate that no changes are required for this wrapper character.
///
/// Note also that whitespace characters may still appear within text between each
/// pair of wrapper characters -- just not at either end.
////
/// # Arguments
///
/// * `s` - Slice of text to be scanned
///
/// # Examples
/// ```
/// assert_eq!(align_bothways("a* bc *d", '*'), Some("a *bc* d".to_string()));
/// ```
fn align_bothways(s: &str, wrapper: char) -> Option<String> {
    let (result, changed_rev) = align_reverse(s, wrapper)?;
    let (result, changed_fwd) = align_reverse(&result, wrapper)?;
    (changed_fwd || changed_rev).then(|| result)
}

// EXTERNAL PUBLIC FUNCTIONS

/// Returns `Some(replacement)` if the given text slice contains whitespace characters
/// that have been re-aligned outside any pairs of wrapper characters, otherwise `None`
///
/// This function calls `align_bothways()` for each of the wrapper characters defined
/// in `WRAPPERS_TO_ALIGN`, potentially updating the result further at each successive
/// iteration.  If any changes are made at all, then `Some(replacement)` is returned,
/// otherwise `None`.
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
    let mut result: Option<String> = None;
    let mut line = s;
    for wrapper in &WRAPPERS_TO_ALIGN {
        result = align_bothways(line, *wrapper).or(result);
        line = result.as_deref().unwrap_or(s);
    }
    result
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_reverse() {
        assert_eq!(
            align_reverse("a*  bc  *d", '*'), // Two wrappers
            Some(("d  *cb  *a".to_string(), true))
        );
        assert_eq!(
            align_reverse("a*  bc  d", '*'), // One wrapper
            None
        );
        assert_eq!(
            align_reverse("a  bc  d", '*'), // No wrappers
            Some(("d  cb  a".to_string(), false))
        );
        assert_eq!(
            align_reverse("a * bc * *d", '*'), // Three wrappers
            None
        );
        assert_eq!(
            align_reverse("*a * bc * *d", '*'), // Four wrappers
            Some(("d ** cb  *a*".to_string(), true))
        );
        assert_eq!(
            align_reverse("abcd", '*'),
            Some(("dcba".to_string(), false))
        );
        assert_eq!(align_reverse("", '*'), Some(("".to_string(), false)));
    }

    #[test]
    fn test_align_bothways() {
        assert_eq!(
            align_bothways("a*  bc  *d", '*'), // Two wrappers
            Some("a  *bc*  d".to_string())
        );
        assert_eq!(align_bothways("a*  bc  d", '*'), None); // One wrapper
        assert_eq!(align_bothways("a  *bc  d", '*'), None); // One wrapper
        assert_eq!(align_bothways("a  bc  d", '*'), None); // No wrappers
        assert_eq!(
            align_bothways("a* bc d* e", '*'), // Two wrappers
            Some("a *bc d* e".to_string())
        );
        assert_eq!(
            align_bothways("a *bc d *e", '*'), // Two wrappers
            Some("a *bc d* e".to_string())
        );
        assert_eq!(
            align_bothways("a * bc * *d", '*'), // Three wrappers
            None
        );
        assert_eq!(
            align_bothways("*a * bc * *d", '*'), // Four wrappers
            Some("*a*  bc ** d".to_string())
        );
        assert_eq!(align_bothways("abcd", '*'), None);
        assert_eq!(align_bothways("", '*'), None);
    }

    #[test]
    fn test_process() {
        assert_eq!(
            process("\x13  abc  \x13"),
            Some("  \x13abc\x13  ".to_string())
        );
        assert_eq!(
            process(" \x18 abc \x18 "),
            Some("  \x18abc\x18  ".to_string())
        );
        assert_eq!(
            process("\x18\x13  abc  \x13\x18"),
            Some("  \x18\x13abc\x13\x18  ".to_string())
        );
        assert_eq!(
            process(" \x18  \x13 abc \x01 def \x13 \x01\x18"),
            Some("    \x18\x13abc  \x01def\x13\x01\x18  ".to_string())
        );
        assert_eq!(process("abcd"), None);
        assert_eq!(process(""), None);
    }
}
