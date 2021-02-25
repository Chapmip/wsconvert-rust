//! Module to map ASCII characters to Unicode characters in a given context

// EXTERNAL PUBLIC FUNCTIONS

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
    let mapped = match c.to_ascii_lowercase() {
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
    let mapped = match c.to_ascii_lowercase() {
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
        _ => return None,
    };
    Some(mapped)
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_subscript() {
        assert_eq!(get_subscript('m'), Some('\u{2098}'));
        assert_eq!(get_subscript('&'), None);
    }

    #[test]
    fn test_get_superscripted() {
        assert_eq!(get_superscript('m'), Some('\u{1D50}'));
        assert_eq!(get_superscript('&'), None);
    }
}
