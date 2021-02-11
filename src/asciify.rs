//! Module to convert 8-bit input bytes into 7-bit ASCII characters

const ASCII_EOF: u8 = 0x1A;         // ASCII End of File (EOF) marker
const ASCII_MASK: u8 = 0x7F;        // ASCII is 7-bit code -> bit mask


/// Returns a byte (u8) slice containing 7-bit ASCII characters
/// formed by zero-ing the top bit of each byte
///
/// If the returned byte slice is shorter than the input slice then this
/// means that an End of File (EOF) character was encountered, causing the
/// EOF and subsequent characters to be excluded from the return slice
///
/// # Arguments
///
/// * `buf` - A mutable byte (u8) slice of 8-bit input characters
///
/// # Examples
/// ```
/// use asciify::convert;
/// let mut buf = [ 0x41, 0xC2, 0x43, 0x1A, 0x45, 0xC6 ];
/// assert_eq!(convert(&mut buf[..5]), [ 0x41, 0x42, 0x43 ]);
/// ```
pub fn convert(buf: &mut [u8]) -> &[u8] {
    let mut count = 0;
    for byte in &mut buf[..] {
        if *byte == ASCII_EOF {
            break;
        }
        *byte &= ASCII_MASK;
        count += 1;
    }
    &buf[0..count]
}


// Unit tests

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_simple() {
        let mut buf = [ 0x7E, 0x7F, 0x80, 0x81, 0x82 ];
        assert_eq!(convert(&mut buf), [ 0x7E, 0x7F, 0x00, 0x01, 0x02 ]);
    }
    
    #[test]
    fn check_slice() {
        let mut buf = [ 0x41, 0xC2, 0x43, 0xC4, 0x45 ];
        assert_eq!(convert(&mut buf[..2]), [ 0x41, 0x42 ]);
    }

    #[test]
    fn check_eof_first() {
        let mut buf = [ 0x1A, 0xC2, 0x43, 0xC4, 0x45 ];
        assert_eq!(convert(&mut buf), []);
    }

    #[test]
    fn check_eof_middle() {
        let mut buf = [ 0x41, 0xC2, 0x43, 0x1A, 0x45 ];
        assert_eq!(convert(&mut buf), [ 0x41, 0x42, 0x43 ]);
    }

    #[test]
    fn check_eof_last() {
        let mut buf = [ 0x41, 0xC2, 0x43, 0xC4, 0x1A ];
        assert_eq!(convert(&mut buf), [ 0x41, 0x42, 0x43, 0x44 ]);
    }

    #[test]
    fn check_empty() {
        let mut buf = [];
        println!("{:?}", buf);
        assert_eq!(convert(&mut buf), []);
    }
}