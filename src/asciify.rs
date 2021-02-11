const ASCII_EOF: u8 = 0x1A;         // ASCII End of File (EOF) marker
const ASCII_MASK: u8 = 0x7F;        // ASCII is 7-bit code -> bit mask

pub fn convert(buf: &mut [u8]) -> &[u8] {
    let mut count = 0;
    for b in &mut buf[..] {
        if *b == ASCII_EOF {
            break;
        }
        *b &= ASCII_MASK;
        count += 1;
    }
    &buf[0..count]
}