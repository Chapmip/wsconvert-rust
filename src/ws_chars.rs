//! Module containing WordStar control characters as constants

// #[allow(dead_code)]

// Used in ws_emphasis module
pub const BOLD: char = '\x02';
pub const DOUBLE: char = '\x04';
pub const OVERPRINT: char = '\x08';
pub const UNDERLINE: char = '\x13';
pub const SUPERSCRIPT: char = '\x14';
pub const STRIKETHRU: char = '\x18';
pub const ITALIC: char = '\x19';
pub const UNDERSCORE: char = '_';

// Wrappers to be aligned (i.e. leading and trailing spaces moved outside wrapper)
pub const WRAPPERS: [char; 5] = [BOLD, DOUBLE, UNDERLINE, STRIKETHRU, ITALIC];

// Others (currently unimplemented)
// pub const SUBSCRIPT: char = '\x16';
// ... etc.
