use crate::ws_chars;

// Unicode modifiers (added after relevant printable character)

const COMB_OVERLINE: char = '\u{0305}'; // Combining overline
const COMB_UNDERLINE: char = '\u{0332}'; // Combining underline

// PRIVATE HELPER FUNCTIONS

fn len_in_chars(s: &str) -> usize {
    s.chars().count()
}

// Note: Always returns true if s is empty, as there are no non-matching chars
fn contains_only_char(s: &str, only: char) -> bool {
    s.chars().all(|ch| ch == only)
}

// Note: Always returns true if s is empty, as there are no non-matching chars
fn contains_only_print(s: &str) -> bool {
    s.chars().all(|ch| !char::is_ascii_control(&ch))
}

fn split_first_three(s: &str, ch: char) -> Option<(&str, &str, &str)> {
    let mut iter = s.splitn(3, ch);
    let left = iter.next()?;
    let within = iter.next()?;
    let rest = iter.next()?;
    Some((left, within, rest))
}

fn split_last_two(s: &str, len: usize) -> Option<(&str, &str)> {
    if len > 0 {
        let (i, _) = s.char_indices().rev().nth(len - 1)?;
        Some((&s[..i], &s[i..]))
    } else {
        Some((s, s))
    }
}

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

pub fn process_underlines(s: &str) -> Option<String> {
    let mut changed = false;
    let mut result = String::new();
    let mut rest = s;
    while let Some((left, text, right)) = split_first_three(rest, ws_chars::UNDERLINE) {
        result.push_str(left);
        let (ctrl_left, text, ctrl_right) = split_wrapped_text(text, char::is_ascii_control);
        result.push_str(ctrl_left);
        let (spc_left, text, spc_right) = split_wrapped_text(text, char::is_ascii_whitespace);
        result.push_str(spc_left);
        let combined = add_combiner(text, COMB_UNDERLINE);
        result.push_str(&combined);
        result.push_str(spc_right);
        result.push_str(ctrl_right);
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
            split_wrapped_text("abc", char::is_ascii_control),
            ("", "abc", "")
        );
        assert_eq!(
            split_wrapped_text("\x13\x02\x13\x02", char::is_ascii_control),
            ("", "", "\x13\x02\x13\x02")
        );
        assert_eq!(split_wrapped_text("", char::is_ascii_control), ("", "", ""));
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
    fn test_process_underline() {
        assert_eq!(
            process_underlines("Go \x13under\x13 and \x13go again\x13."),
            Some(
                "Go u\u{332}n\u{332}d\u{332}e\u{332}r\u{332} and g\u{332}\
                o\u{332} \u{332}a\u{332}g\u{332}a\u{332}i\u{332}n\u{332}."
                    .to_string()
            )
        );
        assert_eq!(
            process_underlines("\x13under\x13"),
            Some("u\u{332}n\u{332}d\u{332}e\u{332}r\u{332}".to_string())
        );
        assert_eq!(
            process_underlines("\x13\x02c\x13\x02"),
            Some("\x02c\u{332}\x02".to_string())
        );
        assert_eq!(
            process_underlines("\x13\x02\x13\x02"),
            Some("\x02\x02".to_string())
        );
        assert_eq!(process_underlines("abcd"), None);
        assert_eq!(process_underlines(""), None);
        assert_eq!(
            process_underlines("\x13\x02  I. INTRO & AIMS\x13\x02"),
            Some(
                "\x02  I\u{332}.\u{332} \u{332}I\u{332}N\u{332}T\u{332}R\u{332}O\u{332} \
                \u{332}&\u{332} \u{332}A\u{332}I\u{332}M\u{332}S\u{332}\x02"
                    .to_string()
            )
        );
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
}
