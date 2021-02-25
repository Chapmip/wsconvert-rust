//! Module to map ASCII characters to Unicode characters with added attributes

use crate::uni_chars;
use std::char;

// EXTERNAL PUBLIC FUNCTIONS

/// Returns `Some(replacement)` if the given character can be mapped to a Unicode
/// bold version, or `None` if no conversion is available
///
/// # Arguments
///
/// * `c` - Character to be transformed into its bold equivalent (if any)
///
/// # Examples
/// ```
/// assert_eq!(get_bold('m'), Some('\u{1D426}'));
/// ```
pub fn get_bold(c: char) -> Option<char> {
    match c {
        'A'..='Z' => char::from_u32(c as u32 - 'A' as u32 + uni_chars::BOLD_UPPER_A as u32),
        'a'..='z' => char::from_u32(c as u32 - 'a' as u32 + uni_chars::BOLD_LOWER_A as u32),
        '0'..='9' => char::from_u32(c as u32 - '0' as u32 + uni_chars::BOLD_ZERO as u32),
        _ => None,
    }
}

/// Returns `Some(replacement)` if the given character can be mapped to a Unicode
/// italic version, or `None` if no conversion is available
///
/// # Arguments
///
/// * `c` - Character to be transformed into its italic equivalent (if any)
///
/// # Examples
/// ```
/// assert_eq!(get_italic('m'), Some('\u{1D45A}'));
/// ```
pub fn get_italic(c: char) -> Option<char> {
    match c {
        'A'..='Z' => char::from_u32(c as u32 - 'A' as u32 + uni_chars::ITALIC_UPPER_A as u32),
        'h' => Some(uni_chars::ITALIC_LOWER_H),
        'a'..='z' => char::from_u32(c as u32 - 'a' as u32 + uni_chars::ITALIC_LOWER_A as u32),
        _ => None,
    }
}

/// Returns `Some(replacement)` if the given character can be mapped to a Unicode
/// bold italic version, or `None` if no conversion is available
///
/// # Arguments
///
/// * `c` - Character to be transformed into its bold italic equivalent (if any)
///
/// # Examples
/// ```
/// assert_eq!(get_bold_italic('m'), Some('\u{1D48E}'));
/// ```
pub fn get_bold_italic(c: char) -> Option<char> {
    match c {
        'A'..='Z' => char::from_u32(c as u32 - 'A' as u32 + uni_chars::BOLD_ITALIC_UPPER_A as u32),
        'a'..='z' => char::from_u32(c as u32 - 'a' as u32 + uni_chars::BOLD_ITALIC_LOWER_A as u32),
        '0'..='9' => char::from_u32(c as u32 - '0' as u32 + uni_chars::BOLD_ZERO as u32),
        _ => None,
    }
}

/// Returns `Some(replacement)` if the given character can be mapped to a Unicode
/// subscripted version, or `None` if no conversion is available
///
/// # Arguments
///
/// * `c` - Character to be transformed into its subscripted equivalent (if any)
///
/// # Examples
/// ```
/// assert_eq!(get_subscript('m'), Some('\u{2098}'));
/// ```
pub fn get_subscript(c: char) -> Option<char> {
    let mapped = match c {
        '0' => '\u{2080}',
        '1' => '\u{2081}',
        '2' => '\u{2082}',
        '3' => '\u{2083}',
        '4' => '\u{2084}',
        '5' => '\u{2085}',
        '6' => '\u{2086}',
        '7' => '\u{2087}',
        '8' => '\u{2088}',
        '9' => '\u{2089}',
        '+' => '\u{208A}',
        '-' => '\u{208B}',
        '=' => '\u{208C}',
        '(' => '\u{208D}',
        ')' => '\u{208E}',
        'a' => '\u{2090}',
        'e' => '\u{2091}',
        'h' => '\u{2096}',
        'i' => '\u{1D62}',
        'j' => '\u{2C7C}',
        'k' => '\u{2095}',
        'l' => '\u{2097}',
        'm' => '\u{2098}',
        'n' => '\u{2099}',
        'o' => '\u{2092}',
        'p' => '\u{209A}',
        'r' => '\u{1D63}',
        's' => '\u{209B}',
        't' => '\u{209C}',
        'u' => '\u{1D64}',
        'v' => '\u{1D65}',
        'x' => '\u{2093}',
        _ => return None,
    };
    Some(mapped)
}

/// Returns `Some(replacement)` if the given character can be mapped to a Unicode
/// superscripted version, or `None` if no conversion is available
///
/// # Arguments
///
/// * `c` - Character to be transformed into its superscripted equivalent (if any)
///
/// # Examples
/// ```
/// assert_eq!(get_superscript('m'), Some('\u{1D50}'));
/// ```
pub fn get_superscript(c: char) -> Option<char> {
    let mapped = match c {
        '0' => '\u{2070}',
        '1' => '\u{00B9}',
        '2' => '\u{00B2}',
        '3' => '\u{00B3}',
        '4' => '\u{2074}',
        '5' => '\u{2075}',
        '6' => '\u{2076}',
        '7' => '\u{2077}',
        '8' => '\u{2078}',
        '9' => '\u{2079}',
        '+' => '\u{207A}',
        '-' => '\u{207B}',
        '=' => '\u{207C}',
        '(' => '\u{207D}',
        ')' => '\u{207E}',
        'a' => '\u{1D43}',
        'b' => '\u{1D47}',
        'c' => '\u{1D9C}',
        'd' => '\u{1D48}',
        'e' => '\u{1D49}',
        'f' => '\u{1DA0}',
        'g' => '\u{1D4D}',
        'h' => '\u{02B0}',
        'i' => '\u{2071}',
        'j' => '\u{02B2}',
        'k' => '\u{1D4F}',
        'l' => '\u{02E1}',
        'm' => '\u{1D50}',
        'n' => '\u{207F}',
        'o' => '\u{1D52}',
        'p' => '\u{1D56}',
        'r' => '\u{02B3}',
        's' => '\u{02E2}',
        't' => '\u{1D57}',
        'u' => '\u{1D58}',
        'v' => '\u{1D5B}',
        'w' => '\u{02B7}',
        'x' => '\u{02E3}',
        'y' => '\u{02B8}',
        'z' => '\u{1DBB}',
        'A' => '\u{1D2C}',
        'B' => '\u{1D2E}',
        'D' => '\u{1D30}',
        'E' => '\u{1D31}',
        'G' => '\u{1D33}',
        'H' => '\u{1D34}',
        'I' => '\u{1D35}',
        'J' => '\u{1D36}',
        'K' => '\u{1D37}',
        'L' => '\u{1D38}',
        'M' => '\u{1D39}',
        'N' => '\u{1D3A}',
        'O' => '\u{1D3C}',
        'P' => '\u{1D3E}',
        'R' => '\u{1D3F}',
        'T' => '\u{1D40}',
        'U' => '\u{1D41}',
        'V' => '\u{2C7D}',
        'W' => '\u{1D42}',
        _ => return None,
    };
    Some(mapped)
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bold() {
        assert_eq!(get_bold('m'), Some('\u{1D426}'));
        assert_eq!(get_bold('H'), Some('\u{1D407}'));
        assert_eq!(get_bold('&'), None);
    }

    #[test]
    fn test_get_italic() {
        assert_eq!(get_italic('m'), Some('\u{1D45A}'));
        assert_eq!(get_italic('H'), Some('\u{1D43B}'));
        assert_eq!(get_italic('&'), None);
    }

    #[test]
    fn test_get_bold_italic() {
        assert_eq!(get_bold_italic('m'), Some('\u{1D48E}'));
        assert_eq!(get_bold_italic('H'), Some('\u{1D46F}'));
        assert_eq!(get_bold_italic('&'), None);
    }

    #[test]
    fn test_get_subscript() {
        assert_eq!(get_subscript('m'), Some('\u{2098}'));
        assert_eq!(get_subscript('7'), Some('\u{2087}'));
        assert_eq!(get_subscript('&'), None);
    }

    #[test]
    fn test_get_superscript() {
        assert_eq!(get_superscript('m'), Some('\u{1D50}'));
        assert_eq!(get_superscript('7'), Some('\u{2077}'));
        assert_eq!(get_superscript('&'), None);
    }
}
