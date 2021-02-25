//! Module to process WordStar "wrapper" characters and modify text accordingly

// Note: utilises new "bool then" feature in Rust 1.50 to simplify Option return
//     (condition).then(|| ())
//  -> if (condition) { Some( () ) } else { None }

use crate::uni_chars;
use crate::ws_chars;
use crate::ws_mappings;

// "WRAPPERS" OBJECT

// Holds states of WordStar wrapper characters that toggle functions on and off
#[derive(Default, Debug)]
pub struct Wrappers {
    overline: bool,
    bold: bool,
    double: bool,
    underline: bool,
    subscript: bool,
    superscript: bool,
    strikethrough: bool,
    italic: bool,
}

impl Wrappers {
    /// Creates a new `Wrapper` object with all fields set to `false` (default)
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns `true` if the given character is a "wrapper" control character
    /// that changes the state of this `Wrappers` object, otherwise `false`
    ///
    /// # Arguments
    ///
    /// * `c` - Character to be examined
    ///
    fn check_toggle(&mut self, c: char) -> bool {
        match c {
            ws_chars::OVERLINE => self.overline = !self.overline,
            ws_chars::BOLD => self.bold = !self.bold,
            ws_chars::DOUBLE => self.double = !self.double,
            ws_chars::UNDERLINE => self.underline = !self.underline,
            ws_chars::SUBSCRIPT => self.subscript = !self.subscript,
            ws_chars::SUPERSCRIPT => self.superscript = !self.superscript,
            ws_chars::STRIKETHROUGH => self.strikethrough = !self.strikethrough,
            ws_chars::ITALIC => self.italic = !self.italic,
            _ => return false,
        };
        true
    }

    /// Returns `Some(mapped)` if the given character can be mapped to a new
    /// Unicode character that incorporates the current state of this `Wrappers`
    /// object, otherwise `None`
    ///
    /// # Arguments
    ///
    /// * `c` - Character to be mapped (if possible)
    ///
    fn get_mapped(&self, c: char) -> Option<char> {
        if self.superscript {
            ws_mappings::get_superscript(c)
        } else if self.subscript {
            ws_mappings::get_subscript(c)
        } else if self.bold ^ self.double {
            if self.italic {
                ws_mappings::get_bold_italic(c)
            } else {
                ws_mappings::get_bold(c)
            }
        } else if self.italic {
            ws_mappings::get_italic(c)
        } else {
            None
        }
    }

    /// Returns `Some(replacement)` if the given text slice can be modified to
    /// incorporate the updated state of this `Wrappers` object, otherwise `None`
    ///
    /// # Arguments
    ///
    /// * `s` - Slice of text to be processed
    ///
    /// # Examples
    /// ```
    /// let mut w = Wrappers::new();
    /// assert_eq!(w.process("\x02C\x02"), Some("\u{1D402}".to_string()));
    /// ```
    pub fn process(&mut self, s: &str) -> Option<String> {
        let mut changed = false;
        let mut result = String::with_capacity(s.len() * 7); // Worst case
        for c in s.chars() {
            if c.is_ascii_control() {
                if self.check_toggle(c) {
                    changed = true; // Eat wrapper control character
                } else {
                    result.push(c); // Retain other control character
                }
                continue; // Finished with control characters
            }
            if !self.underline && !self.overline && !self.strikethrough {
                if let Some(mapped) = self.get_mapped(c) {
                    result.push(mapped);
                    changed = true;
                } else {
                    result.push(c);
                }
                continue; // Finished with mapped or no-line original character
            }
            result.push(c);
            if self.underline {
                result.push(uni_chars::COMB_UNDERLINE);
                changed = true;
            }
            if self.overline {
                result.push(uni_chars::COMB_OVERLINE);
                changed = true;
            }
            if self.strikethrough {
                result.push(uni_chars::COMB_STRIKETHROUGH);
                changed = true;
            }
        }
        changed.then(|| result)
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emphasis() {
        let mut w = Wrappers::new();
        // bold, double, italic
        assert_eq!(w.process("\x02C\x02"), Some("\u{1D402}".to_string()));
        assert_eq!(w.process("\x19C\x19"), Some("\u{1D436}".to_string()));
        assert_eq!(w.process("\x04C\x04"), Some("\u{1D402}".to_string()));
    }

    #[test]
    fn test_sub_super() {
        let mut w = Wrappers::new();
        // sub/superscript
        assert_eq!(w.process("r\x16s\x16t"), Some("r\u{209B}t".to_string()));
        assert_eq!(w.process("r\x14s\x14t"), Some("r\u{02E2}t".to_string()));
    }

    #[test]
    fn test_lines() {
        let mut w = Wrappers::new();
        // lines
        assert_eq!(
            w.process("\x13a b\x13"),
            Some("a\u{0332} \u{0332}b\u{0332}".to_string())
        );
        assert_eq!(
            w.process("\x01a b\x01"),
            Some("a\u{0305} \u{0305}b\u{0305}".to_string())
        );
        assert_eq!(
            w.process("\x18a b\x18"),
            Some("a\u{0336} \u{0336}b\u{0336}".to_string())
        );
    }

    #[test]
    fn test_comb_emphasis() {
        let mut w = Wrappers::new();
        // combinations of bold, double, italic
        assert_eq!(
            w.process("\x02\x19C\x19\x02"),
            Some("\u{1D46A}".to_string())
        );
        assert_eq!(w.process("a\x02b\x02c"), Some("a\u{1D41B}c".to_string()));
        assert_eq!(
            w.process("\x02a\x04b\x02c\x04"),
            Some("\u{1D41A}b\u{1D41C}".to_string())
        );
    }

    #[test]
    fn test_comb_lines() {
        let mut w = Wrappers::new();
        // combinations of lines
        assert_eq!(
            w.process("\x13a\x18b\x13\x18"),
            Some("a\u{0332}b\u{0332}\u{0336}".to_string())
        );
        assert_eq!(
            w.process("\x01\x18\x13T\x13\x18\x01"),
            Some("T\u{0332}\u{0305}\u{0336}".to_string())
        );
        assert_eq!(
            w.process("\x18a \x13b\x18\x13"),
            Some("a\u{0336} \u{0336}b\u{0332}\u{0336}".to_string())
        );
    }

    #[test]
    fn test_competing() {
        let mut w = Wrappers::new();
        // competing cases
        assert_eq!(
            w.process("\x13\x16a\x16\x13"),
            Some("a\u{0332}".to_string())
        );
        assert_eq!(
            w.process("\x16a\x13b\x13c\x16"),
            Some("\u{2090}b\u{0332}c".to_string())
        );
        assert_eq!(w.process("\x14\x02T\x02\x14"), Some("\u{1D40}".to_string()));
        assert_eq!(
            w.process("\x02x\x142\x14\x02"),
            Some("\u{1D431}\u{00B2}".to_string())
        );
    }

    #[test]
    fn test_null() {
        let mut w = Wrappers::new();
        // null cases
        assert_eq!(w.process("abc"), None);
        assert_eq!(w.process(""), None);
    }
}
