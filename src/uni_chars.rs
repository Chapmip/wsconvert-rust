//! Module containing significant Unicode characters and modifiers

// Note: The module ws_mappings separately contains definitions of Unicode
// literals for mapping of subscripted and superscripted characters, which
// are inappropriate and too numerous to define individually here.

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

// Unicode strings for substitution (actually all single characters)

pub const NB_SPACE: &str = "\u{00A0}"; // Non-breaking space
pub const HYPHEN: &str = "\u{2010}"; // Hyphen (as opposed to dash)
pub const BLOCK: &str = "\u{2588}"; // Block character