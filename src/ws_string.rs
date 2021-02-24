//! Module containing helper functions for processing lines of WordStar text

// Written as an exercise in Rust string processing, without resorting to the `regex`
// crate for regular expression parsing (for which the code would probably be simpler)

// EXTERNAL PUBLIC FUNCTIONS

/// Returns length of text slice in characters (not bytes) by iterating though it
///
/// # Arguments
///
/// * `s` - Slice of text to be measured as UTF-8 characters
///
/// # Examples
/// ```
/// assert_eq!(ws_emphasis::len_in_chars("a£¬d"), 4);
/// ```
pub fn len_in_chars(s: &str) -> usize {
    s.chars().count()
}

/// Returns `true` if text slice contains only the given character, otherwise `false`
///
/// Note: Always returns `true` if text slice is empty, as there are no non-matching chars.
///
/// # Arguments
///
/// * `s` - Slice of text to be scanned
/// * `ch` - Character (char) to be matched
///
/// # Examples
/// ```
/// assert_eq!(ws_emphasis::contains_only_char("aaaa", 'a'), true);
/// ```
pub fn contains_only_char(s: &str, only: char) -> bool {
    s.chars().all(|ch| ch == only)
}

/// Returns `true` if text slice contains only printable characters, otherwise `false`
///
/// Note: Always returns `true` if text slice is empty, as there are no non-matching chars.
///
/// # Arguments
///
/// * `s` - Slice of text to be scanned
///
/// # Examples
/// ```
/// assert_eq!(ws_emphasis::contains_only_print("abc 123"), true);
/// ```
pub fn contains_only_print(s: &str) -> bool {
    s.chars().all(|ch| !char::is_ascii_control(&ch))
}

/// Returns `Some(tuple)` if text slice contains at least one pair of "wrapper" characters,
/// otherwise `None`
///
/// The text slice is scanned from left to right.  The returned tuple (if any) is
/// a set of three text slices (left, within, right) corresponding to the text before,
/// between and after the found wrapper characters.  The pair of wrapper characters is
/// not included in any of the returned text slices, but additional wrapper characters
/// (not part of the matched pair) may still appear in the right text slice if present.
///
/// # Arguments
///
/// * `s` - Slice of text to be scanned
/// * `ch` - "Wrapper" character (char) to be matched
///
/// # Examples
/// ```
/// assert_eq!(ws_emphasis::split_first_three("ab/cd/ef", '/'), Some(("ab", "cd", "ef")));
/// ```
pub fn split_first_three(s: &str, ch: char) -> Option<(&str, &str, &str)> {
    let mut iter = s.splitn(3, ch);
    let left = iter.next()?;
    let within = iter.next()?;
    let rest = iter.next()?;
    Some((left, within, rest))
}

/// Returns `Some(tuple)` if text slice can be split into a pair of text slices with
/// the right-hand slice having the specified length in bytes, otherwise `None`
///
/// The text slice is scanned from right to left.  The returned tuple (if any) is a
/// set of two text slices (left, right) corresponding to the text before and after
/// the split point in bytes (as measured from the right-hand end).
///
/// # Arguments
///
/// * `s` - Slice of text to be scanned
/// * `len` - Number of bytes to return in right-hand text slice, if possible
///
/// # Examples
/// ```
/// assert_eq!(split_last_two("abcdefgh", 3), Some(("abcde", "fgh")));
/// ```
pub fn split_last_two(s: &str, len: usize) -> Option<(&str, &str)> {
    if len > 0 {
        let (i, _) = s.char_indices().rev().nth(len - 1)?;
        Some((&s[..i], &s[i..]))
    } else {
        Some((s, s))
    }
}

/// Returns `Some(tuple)` if text slice can be split into three text slices with the
/// right-hand two slices both having the specified length in bytes, otherwise `None`
///
/// The text slice is scanned from right to left.  The returned tuple (if any) is a
/// set of three text slices (left, middle, right) corresponding to the text before
/// and after the two split points in bytes (as measured from the right-hand end).
///
/// # Arguments
///
/// * `s` - Slice of text to be scanned
/// * `len` - Number of bytes to return in right-hand text slice, if possible
///
/// # Examples
/// ```
/// assert_eq!(split_last_two("abcdefgh", 3), Some(("ab", "cde", "fgh")));
/// ```
pub fn split_last_three(s: &str, len: usize) -> Option<(&str, &str, &str)> {
    let (left, right) = split_last_two(s, len)?;
    let (left, middle) = split_last_two(left, len)?;
    Some((left, middle, right))
}

/// Returns tuple that splits off whitespace characters (if any) at each end of
/// a text slice from the text contained within.
///
/// The text slice is scanned from both ends.  The returned tuple is a set of three
/// text slices (spc_left, middle, spc_right) corresponding to the whitespace at the
/// left-hand end, the text between any whitespace at the left-hand and right-hand
/// ends, and the whitespace at the right-hand end.  If there is no whitespace at the
/// left-hand end, then spc_left = "".  If there is no whitespace at the right-hand
/// end, then spc_right = "".  If there is no whitespace at either end, then middle
/// contains the whole of the input text.  Note that whitespace characters may still
/// appear in the middle slice -- just not at either end.
///
/// # Arguments
///
/// * `s` - Slice of text to be processed
///
/// # Examples
/// ```
/// assert_eq!(split_space_at_ends(" abc def "), (" ", "abc def", " "));
/// ```
pub fn split_space_at_ends(s: &str) -> (&str, &str, &str) {
    let mut left = 0;
    let mut right = 0;
    let is_space = |&(_, c): &(usize, char)| char::is_ascii_whitespace(&c);

    let mut iter = s.char_indices().skip_while(is_space);
    if let Some((i, _)) = iter.next() {
        left = i;
        let mut iter = s.char_indices().rev().skip_while(is_space);
        if let Some((i, c)) = iter.next() {
            right = i + c.len_utf8();
        }
    }
    (&s[..left], &s[left..right], &s[right..])
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
    fn test_split_space_at_ends() {
        assert_eq!(
            split_space_at_ends("  abc¬ def  gh "),
            ("  ", "abc¬ def  gh", " ",)
        );
        assert_eq!(split_space_at_ends("   "), ("", "", "   "));
        assert_eq!(split_space_at_ends("abc"), ("", "abc", ""));
        assert_eq!(split_space_at_ends(""), ("", "", ""));
    }
}
