//! Module to maintain sets of counters for ASCII control characters

use std::collections::BTreeMap;
use std::fmt;

/// Holds a name tag for a set of counters, a 'used' marker and a binary tree of
/// counts for ASCII control characters (as defined by `char::is_ascii_control()`)
#[derive(Debug)]
pub struct ControlCount {
    tag: String,
    used: bool,
    counts: BTreeMap<char, i32>,
}

/// Display trait implementation for ControlCount, starting with the name tag,
/// then listing each active counter as a hex ASCII key and a decimal value
impl fmt::Display for ControlCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self.tag)?;
        if !self.used {
            write!(f, "Skipped")?;
        } else {
            let mut previous = false;
            for (key, value) in &self.counts {
                if previous {
                    write!(f, ", ")?;
                }
                write!(f, "[{:02X}]={}", *key as u32, *value)?;
                previous = true;
            }
            if previous {
                write!(f, " => {} chars, {} types", self.total(), self.bins())?;
            } else {
                write!(f, "None")?;
            }
        }
        Ok(())
    }
}

impl ControlCount {
    /// Returns a set of counters with the given name tag
    ///
    /// # Arguments
    ///
    /// * `name` - String containing name tag (moved into `ControlCount` struct)
    ///
    /// # Examples
    /// ```
    /// let mut counts = ControlCount::new("name".to_string());
    /// ```
    pub fn new(tag: String) -> ControlCount {
        ControlCount {
            tag,
            used: false,
            counts: BTreeMap::new(),
        }
    }

    /// Attempts to increment (by one) the count for the given character
    ///
    /// If the given character is not an ASCII control character then no action
    /// is taken
    ///
    /// # Arguments
    ///
    /// * `ch` - Character (char) specifying the count to be incremented
    ///
    /// # Examples
    /// ```
    /// let mut counts = ControlCount::new("name".to_string());
    /// counts.up('\x06');
    /// ```
    pub fn up(&mut self, ch: char) {
        if ch.is_ascii_control() {
            let counter = self.counts.entry(ch).or_insert(0);
            *counter += 1;
        }
        self.used = true;
    }

    /// Attempts to return the current count for the given character
    ///
    /// Returns `Some(u32)` if a count has been established for the given
    /// character or `None` if not
    ///
    /// # Arguments
    ///
    /// * `ch` - Character (char) specifying the count to be returned
    ///
    /// # Examples
    /// ```
    /// let mut counts = ControlCount::new("name".to_string());
    /// counts.up('\x07');
    /// assert_eq!(counts.get('\x07'), Some(1));
    /// assert_eq!(counts.get('\x08'), None);
    /// ```
    #[allow(dead_code)]
    pub fn get(&self, ch: char) -> Option<i32> {
        self.counts.get(&ch).copied()
    }

    /// Scans text slice and increments counts for each ASCII control
    /// character found in it
    ///
    /// # Arguments
    ///
    /// * `s` - Slice of text to be scanned
    ///
    /// # Examples
    /// ```
    /// let mut counts = ControlCount::new("name".to_string());
    /// counts.scan("ABC\x14DEF");
    /// assert_eq!(counts.get('\x14'), Some(1));
    /// ```
    pub fn scan(&mut self, s: &str) {
        for ch in s.chars() {
            self.up(ch);
        }
        self.used = true;
    }

    /// Returns number of different bins that have been created
    ///
    /// # Examples
    /// ```
    /// let mut counts = ControlCount::new("name".to_string());
    /// counts.scan("A\x14BC\x14DE\x15F");
    /// assert_eq!(counts.bins(), 2);
    /// ```
    pub fn bins(&self) -> usize {
        self.counts.iter().count()
    }

    /// Returns total count from all bins
    ///
    /// # Examples
    /// ```
    /// let mut counts = ControlCount::new("name".to_string());
    /// counts.scan("A\x14BC\x14DE\x15F");
    /// assert_eq!(counts.total(), 3);
    /// ```
    pub fn total(&self) -> i32 {
        self.counts.values().sum()
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut counts = ControlCount::new("Counts".to_string());
        counts.up('\x03');
        assert_eq!(counts.get('\x03'), Some(1));
        assert_eq!(counts.get('\x02'), None);
        assert_eq!(counts.get('\x20'), None);
    }

    #[test]
    fn test_multi() {
        let mut counts = ControlCount::new("Counts".to_string());
        counts.up('\x04');
        counts.up('\x04');
        counts.up('\x04');
        assert_eq!(counts.get('\x04'), Some(3));
    }

    #[test]
    fn test_display() {
        let mut counts = ControlCount::new("Counts".to_string());
        counts.up('\x03');
        counts.up('\x08');
        counts.up('\x19');
        counts.up('\x7F');
        counts.up('\x7F');
        counts.up('\x07');
        assert_eq!(
            format!("{}", counts),
            "Counts: [03]=1, [07]=1, [08]=1, [19]=1, [7F]=2 => 6 chars, 5 types"
        );
    }

    #[test]
    fn test_scan() {
        let mut counts = ControlCount::new("name".to_string());
        counts.scan("a'\x07bc\x14de'\x07f");
        assert_eq!(counts.get('\x07'), Some(2));
        assert_eq!(counts.get('\x14'), Some(1));
    }

    #[test]
    fn test_bins() {
        let mut counts = ControlCount::new("name".to_string());
        counts.scan("A\x14BC\x14DE\x15F");
        assert_eq!(counts.bins(), 2);
    }

    #[test]
    fn test_total() {
        let mut counts = ControlCount::new("name".to_string());
        counts.scan("A\x14BC\x14DE\x15F");
        assert_eq!(counts.total(), 3);
    }
}
