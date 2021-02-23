//! Module containing WordStar control characters and associated Unicode modifiers

#![allow(dead_code)]

// Used in ws_emphasis and ws_special modules
pub const BOLD: char = '\x02';
pub const DOUBLE: char = '\x04';
pub const OVERPRINT: char = '\x08';
pub const UNDERLINE: char = '\x13';
pub const SUPERSCRIPT: char = '\x14';
pub const STRIKETHROUGH: char = '\x18';
pub const ITALIC: char = '\x19';
pub const UNDERSCORE: char = '_';

// Used in ws_special module
pub const SUBSCRIPT: char = '\x16';

// Wrappers to be aligned (i.e. leading and trailing spaces moved outside wrapper)
pub const WRAPPERS: [char; 5] = [BOLD, DOUBLE, UNDERLINE, STRIKETHROUGH, ITALIC];

// Others (currently unimplemented)
pub const PHANTOM_SPACE: char = '\x06'; // %% maps to ??? %%
pub const PHANTOM_RUBOUT: char = '\x07'; // %% maps to ??? %%
pub const NON_BREAKING_SPACE: char = '\x0F'; // maps to '\u{00A0}'
pub const INACTIVE_SOFT_HYPHEN: char = '\x1E'; // maps to '\u{2010}'
pub const ACTIVE_SOFT_HYPHEN: char = '\x1F'; // maps to '\u{2010}'
pub const DELETE: char = '\x7F'; // %% maps to ??? %%

// Unicode modifiers (added after relevant printable character)

pub const COMB_OVERLINE: char = '\u{0305}'; // Combining overline
pub const COMB_UNDERLINE: char = '\u{0332}'; // Combining underline
