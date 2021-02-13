//! Module to maintain sets of counters on ASCII control characters

use std::fmt;

const ASC_NUL: char = '\x00';           // First control char in block
const ASC_US: char = '\x1F';            // Last control char in block
const ASC_DEL: char = '\x7F';           // Extra control char on its own

const U32_NUL: u32 = ASC_NUL as u32;
const U32_US: u32 = ASC_US as u32;
const U32_DEL: u32 = ASC_DEL as u32;

const BLK_SIZE: usize = (U32_US - U32_NUL + 1) as usize;
const ARRAY_SIZE: usize = BLK_SIZE + 1;


/// Holds a name tag for a set of counters and an array of values that map to
/// ASCII control characters as defined by `char::is_ascii_control()`
#[derive(Debug)]
pub struct ControlCount {
    tag: String,
    arr: [i32; ARRAY_SIZE],
}

/// Display trait implementation for ControlCount, starting with the name tag,
/// then listing each non-zero count as a hex ASCII value and a decimal count
impl fmt::Display for ControlCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self.tag)?;
        let mut previous = false;
        for i in 0..=ARRAY_SIZE-1 {
            if self.arr[i] != 0 {
                if previous {
                    write!(f, ", ")?;
                }
                let mut u = (i as u32) + U32_NUL;
                if i == ARRAY_SIZE-1 {
                    u = U32_DEL;
                }
                write!(f, "[{:02X}]={}", u, self.arr[i])?;
                previous = true;
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
    ///
    /// ```
    /// use control_count;
    ///
    /// let counts = ControlCount::new("name".to_string());
    /// ```
    pub fn new(tag: String) -> ControlCount {
        ControlCount { tag, arr: [0; ARRAY_SIZE] }
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
            match ch as u32 {
                u @ U32_NUL..=U32_US => self.arr[(u - U32_NUL) as usize] += delta,
                U32_DEL => self.arr[BLK_SIZE] += delta,
                u @ _ => {
                    eprintln!("ERROR: is_ascii_control() didn't filter [{:02X}]", u);
                    panic!();
                }
            }
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
    pub fn up(&mut self, ch:char) {
        &self.add(ch, 1);
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
        &self.add(ch, -1);
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
        if ch.is_ascii_control() {
            match ch as u32 {
                u @ U32_NUL..=U32_US => Some(self.arr[(u - U32_NUL) as usize]),
                U32_DEL => Some(self.arr[BLK_SIZE]),
                u @ _ => {
                    eprintln!("ERROR: is_ascii_control() didn't filter [{:02X}]", u);
                    panic!();
                }
            }
        } else {
            None
        }
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
        assert_eq!(counts.get('\x02'), Some(0));
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
    fn test_multi() {
        let mut counts = ControlCount::new("Counts".to_string());
        counts.up('\x03');
        counts.up('\x08');
        counts.up('\x19');
        counts.up(ASC_DEL);
        counts.up('\x7F');
        counts.up('\x05');
        counts.up('\x07');
        assert_eq!(counts.arr, [0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 2]);
    }
}