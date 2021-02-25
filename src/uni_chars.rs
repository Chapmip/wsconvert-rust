//! Module containing significant Unicode characters and modifiers

// Note: The module ws_mappings separately contains definitions of Unicode
// literals for mapping of subscripted and superscripted characters, which
// are inappropriate and too numerous to define individually here.

/// Starts of Unicode ranges for emphasised alphanumeric characters
pub const BOLD_UPPER_A: char = '\u{1D400}';
pub const BOLD_LOWER_A: char = '\u{1D41A}';
pub const BOLD_ZERO: char = '\u{1D7CE}';
pub const ITALIC_UPPER_A: char = '\u{1D434}';
pub const ITALIC_LOWER_A: char = '\u{1D44E}';
pub const ITALIC_LOWER_H: char = '\u{210E}';
pub const BOLD_ITALIC_UPPER_A: char = '\u{1D468}';
pub const BOLD_ITALIC_LOWER_A: char = '\u{1D482}';

// Unicode strings for substitution (actually all single characters)
// (used in ws_special module)

pub const DEGREE: &str = "\u{00B0}"; // Degree symbol
pub const ONE_QUARTER: &str = "\u{00BC}"; // 1/4 symbol
pub const HALF: &str = "\u{00BD}"; // 1/2 symbol
pub const THREE_QUARTERS: &str = "\u{00BE}"; // 3/4 symbol
pub const REPLACEMENT: &str = "\u{FFFD}"; // Invalid marker

// Unicode modifiers (added after relevant printable character)
// (used in ws_emphasis and ws_special modules)  <- %% CHECK %%

pub const COMB_OVERLINE: char = '\u{0305}'; // Combining overline
pub const COMB_UNDERLINE: char = '\u{0332}'; // Combining underline
pub const COMB_STRIKETHROUGH: char = '\u{0336}'; // Combining strikethrough

// Unicode strings for substitution (actually all single characters)

pub const NB_SPACE: &str = "\u{00A0}"; // Non-breaking space
pub const HYPHEN: &str = "\u{2010}"; // Hyphen (as opposed to dash)
pub const BLOCK: &str = "\u{2588}"; // Block character
