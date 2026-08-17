use super::super::JoiningType;
use super::UserData;
use crate::GlyphId;

/// Character output from the cluster parser.
#[derive(Copy, Clone, Debug)]
pub struct Char {
    /// The character.
    pub ch: char,
    /// Offset of the character in code units.
    pub offset: u32,
    /// Shaping class of the character.
    pub shape_class: ShapeClass,
    /// Joining type of the character.
    pub joining_type: JoiningType,
    /// True if the character is ignorable.
    pub ignorable: bool,
    /// True if the character should be considered when mapping glyphs.
    pub contributes_to_shaping: bool,
    /// Nominal glyph identifier.
    pub glyph_id: GlyphId,
    /// Arbitrary user data.
    pub data: UserData,
}

impl Default for Char {
    fn default() -> Self {
        Self {
            ch: '\0',
            shape_class: ShapeClass::Base,
            joining_type: JoiningType::U,
            ignorable: false,
            contributes_to_shaping: true,
            glyph_id: 0,
            data: 0,
            offset: 0,
        }
    }
}

/// Shaping class of a character.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ShapeClass {
    /// Reph form.
    Reph,
    /// Pre-base form.
    Pref,
    /// Myanmar three character prefix.
    Kinzi,
    /// Base character.
    Base,
    /// Mark character.
    Mark,
    /// Halant modifier.
    Halant,
    /// Medial consonant Ra.
    MedialRa,
    /// Pre-base vowel modifier.
    VMPre,
    /// Pre-base dependent vowel.
    VPre,
    /// Below base dependent vowel.
    VBlw,
    /// Anusvara class.
    Anusvara,
    /// Zero width joiner.
    Zwj,
    /// Zero width non-joiner.
    Zwnj,
    /// Control character.
    Control,
    /// Variation selector.
    Vs,
    /// Other character.
    Other,
}

impl Default for ShapeClass {
    fn default() -> Self {
        Self::Base
    }
}
