//! Module to convert 8-bit input bytes into 7-bit ASCII characters

const EOF_BYTE: u8 = 0x1A;          // End of File (EOF) marker
const ASCII_MASK: u8 = 0x7F;        // ASCII is 7-bit code -> bit mask


/// Converts to 7-bit ASCII format a chunk of data in a byte (u8) slice,
/// modifying it in place and returning a potentially smaller slice
///
/// Returns a byte (u8) slice up to an End of File marker (if present)
/// containing 7-bit ASCII characters formed by zero-ing the top bit
/// of each original 8-bit byte
///
/// If the returned byte slice is smaller than the input slice then this
/// means that an End of File (EOF) marker was encountered, causing the
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
/// assert_eq!(convert_chunk(&mut buf[..5]), [ 0x41, 0x42, 0x43 ]);
/// ```
pub fn convert_chunk(buf: &mut [u8]) -> &[u8] {
    let mut count = 0;
    for byte in &mut buf[..] {
        if *byte == EOF_BYTE {
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
        assert_eq!(convert_chunk(&mut buf), [ 0x7E, 0x7F, 0x00, 0x01, 0x02 ]);
    }
    
    #[test]
    fn check_slice() {
        let mut buf = [ 0x41, 0xC2, 0x43, 0xC4, 0x45 ];
        assert_eq!(convert_chunk(&mut buf[..2]), [ 0x41, 0x42 ]);
    }

    #[test]
    fn check_eof_first() {
        let mut buf = [ EOF_BYTE, 0xC2, 0x43, 0xC4, 0x45 ];
        assert_eq!(convert_chunk(&mut buf), []);
    }

    #[test]
    fn check_eof_middle() {
        let mut buf = [ 0x41, 0xC2, 0x43, EOF_BYTE, 0x45 ];
        assert_eq!(convert_chunk(&mut buf), [ 0x41, 0x42, 0x43 ]);
    }

    #[test]
    fn check_eof_last() {
        let mut buf = [ 0x41, 0xC2, 0x43, 0xC4, EOF_BYTE ];
        assert_eq!(convert_chunk(&mut buf), [ 0x41, 0x42, 0x43, 0x44 ]);
    }

    #[test]
    fn check_empty() {
        let mut buf = [];
        println!("{:?}", buf);
        assert_eq!(convert_chunk(&mut buf), []);
    }
}