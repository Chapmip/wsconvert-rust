//! Module to process WordStar dot commands

use crate::uni_chars;
use crate::ws_chars;

// PRIVATE HELPER FUNCTIONS

/// Returns `Some(tuple)` if text slice contains a dot followed by a two character
/// command (an alphabetic then an alphanumeric character), otherwise `None`
///
/// The text slice is scanned from left to right.  The returned tuple (if any) is a
/// pair of text slices (command, optional text) corresponding to the two character
/// command plus any following text (`Some(text)` if so or `None` if not).
///
/// # Arguments
///
/// * `s` - Slice of text to be scanned
///
/// # Examples
/// ```
/// assert_eq!(check_dot_cmd(".cw 8"), Some(("cw", Some(" 8"))));
/// ```
// Note: utilises new "bool then" feature in Rust 1.50 to simplify use of '?' operator
//     (condition).then(|| ())
//  -> if (condition) { Some( () ) } else { None }
//
fn check_dot_cmd(s: &str) -> Option<(&str, Option<&str>)> {
    let go_on = || ();
    let is_dot = |&c: &char| c == '.';

    let mut iter = s.char_indices();
    let (_, c) = iter.next()?;
    is_dot(&c).then(go_on)?;
    let (i, c) = iter.next()?;
    char::is_ascii_alphabetic(&c).then(go_on)?;
    let (_, c) = iter.next()?;
    char::is_ascii_alphanumeric(&c).then(go_on)?;
    match iter.next() {
        Some((j, _)) => Some((&s[i..j], Some(&s[j..]))),
        None => Some((&s[i..], None)),
    }
}

/// Returns new String formed from given text slice with control characters removed
///
/// # Arguments
///
/// * `s` - Slice of text to be processed
///
/// # Examples
/// ```
/// assert_eq!(strip_control_chars("\x13ab\x08c"), "abc");
/// ```
fn strip_control_chars(s: &str) -> String {
    s.chars()
        .filter(|c| !char::is_ascii_control(c))
        .collect::<String>()
}

/// Returns `Some(replacement)` if the optional text is present, with a replacement
/// string as the given wrapper text slice as a prefix and suffix to the optional
/// text with control characters removed, otherwise `None`
///
/// # Arguments
///
/// * `wrapper` - Slice of text containing prefix/suffix to the optional text content
/// * `opt_text` - Must contain `Some(text)` in order to make a new header
///
/// # Examples
/// ```
/// assert_eq!(make_header("#", Some("hello")), Some("#hello#".to_string()));
/// ```
fn make_header(wrapper: &str, opt_text: Option<&str>) -> Option<String> {
    let text = opt_text?;
    let mut result = String::new();
    result.push_str(wrapper);
    let conv_text = strip_control_chars(text);
    result.push_str(conv_text.trim());
    result.push_str(wrapper);
    Some(result)
}

// EXTERNAL PUBLIC FUNCTION

/// Returns `Some(replacement)` wrapping text to be substituted if a valid dot command
/// is detected, otherwise `None`
///
/// The replacement text may be "", indicating that the line containing the dot command
/// needs to be eliminated entirely, rather than just replaced with a blank line.
///
/// # Arguments
///
/// * `s` - Slice of text to be processed
///
/// # Examples
/// ```
/// assert_eq!(process(".he abc"), Some("\x13abc\x13".to_string()));
/// ```
pub fn process(s: &str) -> Option<String> {
    let (cmd, opt_text) = check_dot_cmd(s)?;
    let lower_cmd = cmd.to_ascii_lowercase();
    match &lower_cmd[..] {
        "he" | "fo" => make_header(&ws_chars::UNDERLINE.to_string(), opt_text),
        "h1" | "h2" | "h3" | "h4" | "h5" | "f1" | "f2" | "f3" | "f4" | "f5" => {
            make_header(&ws_chars::UNDERLINE.to_string(), opt_text)
        }
        "pa" | "xl" => Some(uni_chars::HORIZONTAL_BAR.to_string().repeat(39)),
        _ => Some("".to_string()),
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_dot_cmds() {
        assert_eq!(check_dot_cmd(".cw 8"), Some(("cw", Some(" 8"))));
        assert_eq!(check_dot_cmd(".op"), Some(("op", None)));
        assert_eq!(check_dot_cmd(".h4"), Some(("h4", None)));
        assert_eq!(check_dot_cmd(".4h"), None);
        assert_eq!(check_dot_cmd(".h!"), None);
        assert_eq!(check_dot_cmd("abc"), None);
        assert_eq!(check_dot_cmd(""), None);
    }

    #[test]
    fn test_strip_control_chars() {
        let text = "\x08  jdj  \x06df  kf\x08\x08\x08  ";
        assert_eq!(strip_control_chars(text), "  jdj  df  kf  ");
        assert_eq!(strip_control_chars("abc"), "abc");
        assert_eq!(strip_control_chars("\x08\x13"), "");
        assert_eq!(strip_control_chars(""), "");
    }

    #[test]
    fn test_make_header() {
        assert_eq!(make_header("#", Some("hello")), Some("#hello#".to_string()));
        assert_eq!(
            make_header("#", Some("he\x03llo")),
            Some("#hello#".to_string())
        );
        assert_eq!(make_header("#", None), None);
    }

    #[test]
    fn test_process() {
        let text = ".He \x03 the \x04 words *¬£   \x05  ";
        assert_eq!(process(text), Some("\x13the  words *¬£\x13".to_string()));
        assert_eq!(
            process(".f3 \x13\x14TEST\x13\x14"),
            Some("\x13TEST\x13".to_string())
        );
        assert_eq!(process(".op"), Some("".to_string()));
        assert_eq!(process("abc"), None);
        assert_eq!(process(""), None);
    }
}
