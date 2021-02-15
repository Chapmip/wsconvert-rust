//! Module to maintain sets of counters for ASCII control characters

use std::collections::BTreeMap;
use std::fmt;

/// Holds a name tag for a set of counters and a binary tree of counts for
/// ASCII control characters (as defined by `char::is_ascii_control()`)
#[derive(Debug)]
pub struct ControlCount {
    tag: String,
    counts: BTreeMap<char, i32>,
}

/// Display trait implementation for ControlCount, starting with the name tag,
/// then listing each active counter as a hex ASCII key and a decimal value
impl fmt::Display for ControlCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self.tag)?;
        let mut previous = false;
        for (key, value) in &self.counts {
            if previous {
                write!(f, ", ")?;
            }
            write!(f, "[{:02X}]={}", *key as u32, *value)?;
            previous = true;
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
    /// use control_count;
    ///
    /// let counts = ControlCount::new("name".to_string());
    /// ```
    pub fn new(tag: String) -> ControlCount {
        ControlCount {
            tag,
            counts: BTreeMap::new(),
        }
    }

    /// Attempts to add a +/- delta (i32) to the count for the given character
    ///
    /// If the given character is not an ASCII control character then no action
    /// is taken
    ///
    /// # Arguments
    ///
    /// * `ch` - A character (char) specifying the count to be changed
    /// * `delta` - A positive/negative/zero delta to the current count value
    ///
    /// # Examples
    /// ```
    /// use control_count;
    ///
    /// let counts = ControlCount::new("name".to_string());
    /// counts.add('\x05', 1);
    /// ```
    pub fn add(&mut self, ch: char, delta: i32) {
        if ch.is_ascii_control() {
            let counter = self.counts.entry(ch).or_insert(0);
            *counter += delta;
        }
    }

    /// Attempts to increment (by one) the count for the given character
    ///
    /// If the given character is not an ASCII control character then no action
    /// is taken
    ///
    /// # Arguments
    ///
    /// * `ch` - A character (char) specifying the count to be incremented
    ///
    /// # Examples
    /// ```
    /// use control_count;
    ///
    /// let counts = ControlCount::new("name".to_string());
    /// counts.up('\x06');
    /// ```
    pub fn up(&mut self, ch: char) {
        self.add(ch, 1);
    }

    /// Attempts to decrement (by one) the count for the given character
    ///
    /// If the given character is not an ASCII control character then no action
    /// is taken
    ///
    /// # Arguments
    ///
    /// * `ch` - A character (char) specifying the count to be decremented
    ///
    /// # Examples
    /// ```
    /// use control_count;
    ///
    /// let counts = ControlCount::new("name".to_string());
    /// counts.down('\x07');
    /// ```
    pub fn down(&mut self, ch: char) {
        self.add(ch, -1);
    }

    /// Attempts to return the current count for the given character
    ///
    /// Returns `Some(u32)` if the given character is a valid ASCII control
    /// character or `None` if not
    ///
    /// # Arguments
    ///
    /// * `ch` - A character (char) specifying the count to be returned
    ///
    /// # Examples
    /// ```
    /// use control_count;
    ///
    /// let counts = ControlCount::new("name".to_string());
    /// assert_eq!(counts.get('\x08'), Some(0));
    /// ```
    pub fn get(&self, ch: char) -> Option<i32> {
        self.counts.get(&ch).copied()
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
    fn test_up_down() {
        let mut counts = ControlCount::new("Counts".to_string());
        counts.up('\x04');
        counts.up('\x04');
        counts.down('\x04');
        counts.up('\x04');
        assert_eq!(counts.get('\x04'), Some(2));
    }

    #[test]
    fn test_display() {
        let mut counts = ControlCount::new("Counts".to_string());
        counts.up('\x03');
        counts.up('\x08');
        counts.up('\x19');
        counts.up('\x7F');
        counts.up('\x7F');
        counts.up('\x05');
        counts.up('\x07');
        assert_eq!(
            format!("{}", counts),
            "Counts: [03]=1, [05]=1, [07]=1, [08]=1, [19]=1, [7F]=2"
        );
    }
}
