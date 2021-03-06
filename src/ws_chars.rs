//! Module containing WordStar control characters

// Used in ws_align, ws_wrappers, ws_emphasis and ws_special modules
pub const OVERLINE: char = '\x01';
pub const BOLD: char = '\x02';
pub const DOUBLE: char = '\x04';
pub const OVERPRINT: char = '\x08';
pub const UNDERLINE: char = '\x13';
pub const SUPERSCRIPT: char = '\x14';
pub const SUBSCRIPT: char = '\x16';
pub const STRIKETHROUGH: char = '\x18';
pub const ITALIC: char = '\x19';
pub const UNDERSCORE: char = '_';

// Used in ws_control module
pub const PHANTOM_SPACE: char = '\x06'; // Daisywheel printer spare slot!
pub const PHANTOM_RUBOUT: char = '\x07'; // Daisywheel printer spare slot!
pub const FORM_FEED: char = '\x0C';
pub const NON_BREAKING_SPACE: char = '\x0F';
pub const INACTIVE_SOFT_HYPHEN: char = '\x1E';
pub const ACTIVE_SOFT_HYPHEN: char = '\x1F';
pub const DELETE: char = '\x7F';
