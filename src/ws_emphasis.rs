//! Module to process WordStar "emphasis" wrappers (e.g. bold, underline)

use crate::ws_chars;

// Unicode modifiers (added after relevant printable character)

const COMB_OVERLINE: char = '\u{0305}';     // Combining overline
const COMB_UNDERLINE: char = '\u{0332}';    // Combining underline

// Wrapper conversions to Markdown format (for other than underline and overline)

const CONVERSIONS: [(char, &str); 4] = [
    (ws_chars::BOLD, "**"),
    (ws_chars::ITALIC, "*"),
    (ws_chars::STRIKETHRU, "~~"),
    (ws_chars::DOUBLE, "**"),
];

// PRIVATE HELPER FUNCTIONS

/// Returns length of string slice in characters (not bytes) by iterating though it
///
/// # Arguments
///
/// * `s` - String slice (or String) to be measured as UTF-8 characters
///
/// # Examples
/// ```
/// assert_eq!(ws_emphasis::len_in_chars("a£¬d"), 4);
/// ```
fn len_in_chars(s: &str) -> usize {
    s.chars().count()
}

/// Returns `true` if string slice contains only the given character, otherwise `false`
///
/// Note: Always returns `true` if string slice is empty, as there are no non-matching chars.
///
/// # Arguments
///
/// * `s` - String slice (or String) to be scanned
/// * `ch` - Character (char) to be matched
///
/// # Examples
/// ```
/// assert_eq!(ws_emphasis::contains_only_char("aaaa", 'a'), true);
/// ```
fn contains_only_char(s: &str, only: char) -> bool {
    s.chars().all(|ch| ch == only)
}

/// Returns `true` if string slice contains only printable characters, otherwise `false`
///
/// Note: Always returns `true` if string slice is empty, as there are no non-matching chars.
///
/// # Arguments
///
/// * `s` - String slice (or String) to be scanned
///
/// # Examples
/// ```
/// assert_eq!(ws_emphasis::contains_only_print("abc 123"), true);
/// ```
fn contains_only_print(s: &str) -> bool {
    s.chars().all(|ch| !char::is_ascii_control(&ch))
}

/// Returns `Some(tuple)` if string slice contains a pair of "wrapper" characters,
/// otherwise `None`
///
/// The string slice is scanned from left to right.  The returned tuple (if any) is
/// a set of three string slices (left, within, right) corresponding to the text before,
/// between and after the wrapper characters.  The pair of wrapper characters is not
/// included in any of the returned string slices, but additional wrapper characters
/// (not part of the matched pair) may still appear in the right string slice if present.
///
/// # Arguments
///
/// * `s` - String slice (or String) to be scanned
/// * `ch` - "Wrapper" character (char) to be matched
///
/// # Examples
/// ```
/// assert_eq!(ws_emphasis::split_first_three("ab/cd/ef", '/'), Some(("ab", "cd", "ef")));
/// ```
fn split_first_three(s: &str, ch: char) -> Option<(&str, &str, &str)> {
    let mut iter = s.splitn(3, ch);
    let left = iter.next()?;
    let within = iter.next()?;
    let rest = iter.next()?;
    Some((left, within, rest))
}

/// Returns `Some(tuple)` if string slice can be split into a pair of string slices with
/// the right-hand slice having the specified length in bytes, otherwise `None`
///
/// The string slice is scanned from right to left.  The returned tuple (if any) is a
/// set of two string slices (left, right) corresponding to the text before and after
/// the split point in bytes (as measured from the right-hand end).
///
/// # Arguments
///
/// * `s` - String slice (or String) to be scanned
/// * `len` - Number of bytes to return in right-hand string slice, if possible
///
/// # Examples
/// ```
/// split_last_two("abcdefgh", 3), Some(("abcde", "fgh")));
/// ```
fn split_last_two(s: &str, len: usize) -> Option<(&str, &str)> {
    if len > 0 {
        let (i, _) = s.char_indices().rev().nth(len - 1)?;
        Some((&s[..i], &s[i..]))
    } else {
        Some((s, s))
    }
}

/// Returns `Some(tuple)` if string slice can be split into three string slices with the
/// right-hand two slices both having the specified length in bytes, otherwise `None`
///
/// The string slice is scanned from right to left.  The returned tuple (if any) is a
/// set of three string slices (left, middle, right) corresponding to the text before
/// and after the two split points in bytes (as measured from the right-hand end).
///
/// # Arguments
///
/// * `s` - String slice (or String) to be scanned
/// * `len` - Number of bytes to return in right-hand string slice, if possible
///
/// # Examples
/// ```
/// split_last_two("abcdefgh", 3), Some(("ab", "cde", "fgh")));
/// ```
fn split_last_three(s: &str, len: usize) -> Option<(&str, &str, &str)> {
    let (left, right) = split_last_two(s, len)?;
    let (left, middle) = split_last_two(left, len)?;
    Some((left, middle, right))
}

fn split_wrapped_text(s: &str, func: fn(&char) -> bool) -> (&str, &str, &str) {
    let mut left = 0;
    let mut right = 0;
    let is_control = |&(_, c): &(usize, char)| func(&c);

    let mut iter = s.char_indices().skip_while(is_control);
    if let Some((i, _)) = iter.next() {
        left = i;
        let mut iter = s.char_indices().rev().skip_while(is_control);
        if let Some((i, c)) = iter.next() {
            right = i + c.len_utf8();
        }
    }
    (&s[..left], &s[left..right], &s[right..])
}

fn fix_wrapper(s: &str, marker: char) -> Option<String> {
    let mut changed = false;
    let mut result = String::new();
    let mut rest = s;
    while let Some((left, text, right)) = split_first_three(rest, marker) {
        result.push_str(left);
        let (spc_left, text, spc_right) = split_wrapped_text(text, char::is_ascii_whitespace);
        result.push_str(spc_left);
        result.push(marker);
        result.push_str(text);
        result.push(marker);
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

pub fn fix_all_wrappers(s: &str) -> Option<String> {
    let mut changed = false;
    let mut result = String::new();
    let mut line = s;
    for wrapper in &ws_chars::WRAPPERS {
        if let Some(fixed) = fix_wrapper(line, *wrapper) {
            result = fixed;
            line = &result;
            changed = true;
        }
    }
    changed.then(|| result)
}

fn replace_wrapper(s: &str, wrapper: char, replacement: &str) -> Option<String> {
    let mut changed = false;
    let mut result = String::new();
    let mut rest = s;
    while let Some((left, text, right)) = split_first_three(rest, wrapper) {
        result.push_str(left);
        result.push_str(replacement);
        result.push_str(text);
        result.push_str(replacement);
        rest = right;
        changed = true;
    }
    if changed {
        result.push_str(rest);
        Some(result)
    } else {
        None
    }
}

fn add_combiner(s: &str, combiner: char) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        result.push(ch);
        if !char::is_ascii_control(&ch) {
            result.push(combiner);
        }
    }
    result
}

// EXTERNAL PUBLIC FUNCTIONS

pub fn align_wrappers(s: &str) -> Option<String> {
    let mut changed = false;
    let mut result = String::new();
    let mut line = s;
    while let Some(fixed) = fix_all_wrappers(line) {
        result = fixed;
        line = &result;
        changed = true;
    }
    changed.then(|| result)
}

pub fn process_underlines(s: &str) -> Option<String> {
    let mut changed = false;
    let mut result = String::new();
    let mut rest = s;
    while let Some((left, text, right)) = split_first_three(rest, ws_chars::UNDERLINE) {
        result.push_str(left);
        let combined = add_combiner(text, COMB_UNDERLINE);
        result.push_str(&combined);
        rest = right;
        changed = true;
    }
    if changed {
        result.push_str(rest);
        Some(result)
    } else {
        None
    }
}

pub fn process_overlines(s: &str) -> Option<String> {
    let mut changed = false;
    let mut result = String::new();
    let mut rest = s;
    while let Some((left, bars, right)) = split_first_three(rest, ws_chars::SUPERSCRIPT) {
        if contains_only_char(bars, ws_chars::UNDERSCORE) {
            let len = len_in_chars(bars);
            if let Some((prefix, text, over)) = split_last_three(left, len) {
                if contains_only_char(over, ws_chars::OVERPRINT) && contains_only_print(text) {
                    result.push_str(prefix);
                    result.push_str(&add_combiner(text, COMB_OVERLINE));
                    rest = right;
                    changed = true;
                    continue;
                }
            }
        }
        // Not exact match: restore and store original string up to 'right'
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

pub fn process_others(s: &str) -> Option<String> {
    let mut changed = false;
    let mut result = String::new();
    let mut line = s;
    for (wrapper, replacement) in &CONVERSIONS {
        if let Some(replaced) = replace_wrapper(line, *wrapper, replacement) {
            result = replaced;
            line = &result;
            changed = true;
        }
    }
    changed.then(|| result)
}

pub fn process_emphasis(s: &str) -> Option<String> {
    let mut changed = false;
    let mut result = String::new();
    let mut line = s;

    if let Some(replacement) = align_wrappers(line) {
        result = replacement;
        line = &result;
        changed = true;
    }
    if let Some(replacement) = process_underlines(line) {
        result = replacement;
        line = &result;
        changed = true;
    }
    if let Some(replacement) = process_overlines(line) {
        result = replacement;
        line = &result;
        changed = true;
    }
    if let Some(replacement) = process_others(line) {
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
    fn test_len_in_chars() {
        assert_eq!(len_in_chars("ab£d¬f"), 6);
        assert_eq!(len_in_chars(""), 0);
    }

    #[test]
    fn test_contains_only_char() {
        assert_eq!(contains_only_char("aaaa", 'a'), true);
        assert_eq!(contains_only_char("aaba", 'a'), false);
        assert_eq!(contains_only_char("", 'a'), true);
    }

    #[test]
    fn test_contains_only_print() {
        assert_eq!(contains_only_print("normal text"), true);
        assert_eq!(contains_only_print("bro\x08ken text"), false);
        assert_eq!(contains_only_print(""), true);
    }

    #[test]
    fn test_split_first_three() {
        assert_eq!(
            split_first_three("ab¬/cd/e£f/g", '/'),
            Some(("ab¬", "cd", "e£f/g"))
        );
        assert_eq!(split_first_three("//ab¬/cd", '/'), Some(("", "", "ab¬/cd")));
        assert_eq!(split_first_three("ab¬/cd", '/'), None);
        assert_eq!(split_first_three("abcd", '/'), None);
        assert_eq!(split_first_three("", '/'), None);
    }

    #[test]
    fn test_split_last_two() {
        assert_eq!(
            split_last_two("ab¬/cd/e£f/g", 4),
            Some(("ab¬/cd/e", "£f/g"))
        );
        assert_eq!(split_last_two("//ab¬/cd", 4), Some(("//ab", "¬/cd")));
        assert_eq!(split_last_two("ab¬/cd", 8), None);
        assert_eq!(split_last_two("", 1), None);
    }

    #[test]
    fn test_split_last_three() {
        assert_eq!(
            split_last_three("ab¬/cd/e£f/g", 4),
            Some(("ab¬/", "cd/e", "£f/g"))
        );
        assert_eq!(split_last_three("//ab¬/cd", 4), Some(("", "//ab", "¬/cd")));
        assert_eq!(split_last_three("ab¬/cd", 4), None);
        assert_eq!(split_last_three("", 1), None);
    }

    #[test]
    fn test_split_wrapped_text() {
        assert_eq!(
            split_wrapped_text("\x13\x13¬efef\x13wf£wfwbc¬\x13", char::is_ascii_control),
            ("\x13\x13", "¬efef\x13wf£wfwbc¬", "\x13",)
        );
        assert_eq!(
            split_wrapped_text("\x13\x02\x13\x02", char::is_ascii_control),
            ("", "", "\x13\x02\x13\x02")
        );
        assert_eq!(
            split_wrapped_text("abc", char::is_ascii_control),
            ("", "abc", "")
        );
        assert_eq!(split_wrapped_text("", char::is_ascii_control), ("", "", ""));
    }

    #[test]
    fn test_fix_wrapper() {
        assert_eq!(
            fix_wrapper("a*  bc  *d", '*'),
            Some("a  *bc*  d".to_string())
        );
        assert_eq!(fix_wrapper("a*  bc  d", '*'), None);
        assert_eq!(fix_wrapper("a  bc  d", '*'), None);
        assert_eq!(
            fix_wrapper("*a * bc * *d", '*'),
            Some("*a*  bc ** d".to_string())
        );
        assert_eq!(
            fix_wrapper("*a * bc * *d", '*'),
            Some("*a*  bc ** d".to_string())
        );
        assert_eq!(fix_wrapper("abcd", '*'), None);
        assert_eq!(fix_wrapper("", '*'), None);
    }

    #[test]
    fn test_replace_wrapper() {
        assert_eq!(
            replace_wrapper(".abc.hd .d.  ..", '.', "**"),
            Some("**abc**hd **d**  ****".to_string())
        );
        assert_eq!(
            replace_wrapper("ab..cd", '.', "**"),
            Some("ab****cd".to_string())
        );
        assert_eq!(replace_wrapper("ab..cd", '.', ""), Some("abcd".to_string()));
        assert_eq!(replace_wrapper("ab.cd", '.', "**"), None);
        assert_eq!(replace_wrapper("abcd", '.', "**"), None);
        assert_eq!(replace_wrapper("", '.', "**"), None);
    }

    #[test]
    fn test_add_combiner() {
        assert_eq!(add_combiner("abcd", '*'), "a*b*c*d*".to_string());
        assert_eq!(
            add_combiner("a b\x08c\x13d", '*'),
            "a* *b*\x08c*\x13d*".to_string()
        );
        assert_eq!(add_combiner("\x08\x13", '*'), "\x08\x13".to_string());
        assert_eq!(add_combiner("", '*'), "".to_string());
    }

    #[test]
    fn test_align_wrappers() {
        assert_eq!(
            align_wrappers("\x02\x13  abc  \x13\x02"),
            Some("  \x02\x13abc\x13\x02  ".to_string())
        );
        assert_eq!(
            align_wrappers(" \x02  \x13 abc \x19 def \x13 \x19\x02"),
            Some("    \x02\x13abc  \x19def\x13\x19\x02  ".to_string())
        );
        assert_eq!(align_wrappers("abcd"), None);
        assert_eq!(align_wrappers(""), None);
    }

    #[test]
    fn test_process_underline() {
        assert_eq!(
            process_underlines("\x13under\x13"),
            Some("u\u{332}n\u{332}d\u{332}e\u{332}r\u{332}".to_string())
        );
        assert_eq!(
            process_underlines("Go \x13under\x13 and \x13go again\x13."),
            Some(
                "Go u\u{332}n\u{332}d\u{332}e\u{332}r\u{332} and g\u{332}\
                o\u{332} \u{332}a\u{332}g\u{332}a\u{332}i\u{332}n\u{332}."
                    .to_string()
            )
        );
        assert_eq!(
            process_underlines("\x13\x02c\x13\x02"),
            Some("\x02c\u{332}\x02".to_string())
        );
        assert_eq!(
            process_underlines("\x13\x02\x13\x02"),
            Some("\x02\x02".to_string())
        );
        let text = "\x13\x02  I. INTRO & AIMS\x13\x02";
        assert_eq!(
            process_underlines(&align_wrappers(text).unwrap_or(text.to_string())),
            Some(
                "  \x02I\u{332}.\u{332} \u{332}I\u{332}N\u{332}T\u{332}R\u{332}O\u{332} \
                \u{332}&\u{332} \u{332}A\u{332}I\u{332}M\u{332}S\u{332}\x02"
                    .to_string()
            )
        );
        let text = " \x02  \x13 abc \x19 def \x13 \x19\x02";
        assert_eq!(
            process_underlines(&align_wrappers(text).unwrap_or(text.to_string())),
            Some(
                "    \x02a\u{332}b\u{332}c\u{332} \u{332} \u{332}\x19\
                d\u{332}e\u{332}f\u{332}\x19\x02  "
                    .to_string()
            )
        );
        assert_eq!(process_underlines("abcd"), None);
        assert_eq!(process_underlines(""), None);
    }

    #[test]
    fn test_process_overline() {
        assert_eq!(
            process_overlines("See DAC\x08\x08\x08\x14___\x14, RFD\x08\x08\x08\x14___\x14 and DAV"),
            Some("See D\u{305}A\u{305}C\u{305}, R\u{305}F\u{305}D\u{305} and DAV".to_string())
        );
        assert_eq!(
            process_overlines("See DAC\x08?\x08\x14___\x14, RFD\x08\x08\x08\x14___\x14 and DAV"),
            Some("See DAC\x08?\x08\x14___\x14, R\u{305}F\u{305}D\u{305} and DAV".to_string())
        );
        assert_eq!(process_overlines("abcd"), None);
        assert_eq!(process_overlines(""), None);
    }

    #[test]
    fn test_process_others() {
        assert_eq!(
            process_others("The \x02bold\x02 and \x19italic\x19"),
            Some("The **bold** and *italic*".to_string())
        );
        assert_eq!(
            process_others("\x18strikethru\x18 & \x04double\x04!"),
            Some("~~strikethru~~ & **double**!".to_string())
        );
        assert_eq!(
            process_others("\x02Bold\x02 and \x02bold\x02 and \x02bold\x02"),
            Some("**Bold** and **bold** and **bold**".to_string())
        );
        assert_eq!(
            process_others("\x02Bold\x02 and \x02broken"),
            Some("**Bold** and \x02broken".to_string())
        );
        assert_eq!(process_others("abcd"), None);
        assert_eq!(process_others(""), None);
    }

    #[test]
    fn test_process_emphasis() {
        assert_eq!(
            process_emphasis("\x13 \x02Bold\x02 title  \x13"),
            Some(
                " **B\u{332}o\u{332}l\u{332}d\u{332}** \u{332}t\u{332}i\u{332}t\u{332}\
            l\u{332}e\u{332}  "
                    .to_string()
            )
        );
        assert_eq!(
            process_emphasis("\x13 \x02Bold\x02 title  \x13"),
            Some(
                " **B\u{332}o\u{332}l\u{332}d\u{332}** \u{332}t\u{332}i\u{332}t\u{332}\
            l\u{332}e\u{332}  "
                    .to_string()
            )
        );
        assert_eq!(
            process_emphasis(" \x02DAC\x08\x08\x08\x14___\x14 and\x02 \x18strike\x18"),
            Some(" **D\u{305}A\u{305}C\u{305} and** ~~strike~~".to_string())
        );
        assert_eq!(process_emphasis("abcd"), None);
        assert_eq!(process_emphasis(""), None);
    }
}
