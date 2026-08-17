//! Styles, scripts and glyph style mapping.

use super::metrics::BlueZones;
use super::shape::{ShaperCoverageKind, VisitedLookupSet};
use alloc::vec::Vec;
use raw::types::{GlyphId, Tag};

/// Defines the script and style associated with a single glyph.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub(crate) struct GlyphStyle(pub(super) u16);

impl GlyphStyle {
    // The following flags roughly correspond to those defined in FreeType
    // here: https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afglobal.h#L76
    // but with different values because we intend to store "meta style"
    // information differently.
    const STYLE_INDEX_MASK: u16 = 0xFF;
    const UNASSIGNED: u16 = Self::STYLE_INDEX_MASK;
    // A non-base character, perhaps more commonly referred to as a "mark"
    const NON_BASE: u16 = 0x100;
    const DIGIT: u16 = 0x200;
    // Used as intermediate state to mark when a glyph appears as GSUB output
    // for a given script
    const FROM_GSUB_OUTPUT: u16 = 0x8000;

    pub const fn is_unassigned(self) -> bool {
        self.0 & Self::STYLE_INDEX_MASK == Self::UNASSIGNED
    }

    pub const fn is_non_base(self) -> bool {
        self.0 & Self::NON_BASE != 0
    }

    pub const fn is_digit(self) -> bool {
        self.0 & Self::DIGIT != 0
    }

    pub fn style_class(self) -> Option<&'static StyleClass> {
        StyleClass::from_index(self.style_index()?)
    }

    pub fn style_index(self) -> Option<u16> {
        let ix = self.0 & Self::STYLE_INDEX_MASK;
        if ix != Self::UNASSIGNED {
            Some(ix)
        } else {
            None
        }
    }

    fn maybe_assign(&mut self, other: Self) {
        // FreeType walks the style array in order so earlier styles
        // have precedence. Since we walk the cmap and binary search
        // on the full range mapping, our styles are mapped in a
        // different order. This check allows us to replace a currently
        // mapped style if the new style index is lower which matches
        // FreeType's behavior.
        //
        // Note that we keep the extra bits because FreeType allows
        // setting the NON_BASE bit to an already mapped style.
        if other.0 & Self::STYLE_INDEX_MASK <= self.0 & Self::STYLE_INDEX_MASK {
            self.0 = (self.0 & !Self::STYLE_INDEX_MASK) | other.0;
        }
    }

    pub(super) fn set_from_gsub_output(&mut self) {
        self.0 |= Self::FROM_GSUB_OUTPUT
    }

    pub(super) fn clear_from_gsub(&mut self) {
        self.0 &= !Self::FROM_GSUB_OUTPUT;
    }

    /// Assign a style if we've been marked as GSUB output _and_ the
    /// we don't currently have an assigned style.
    ///
    /// This also clears the GSUB output bit.
    ///
    /// Returns `true` if this style was applied.
    pub(super) fn maybe_assign_gsub_output_style(&mut self, style: &StyleClass) -> bool {
        let style_ix = style.index as u16;
        if self.0 & Self::FROM_GSUB_OUTPUT != 0 && self.is_unassigned() {
            self.clear_from_gsub();
            self.0 = (self.0 & !Self::STYLE_INDEX_MASK) | style_ix;
            true
        } else {
            false
        }
    }
}

impl Default for GlyphStyle {
    fn default() -> Self {
        Self(Self::UNASSIGNED)
    }
}

/// Sentinel for unused styles in [`GlyphStyleMap::metrics_map`].
const UNMAPPED_STYLE: u8 = 0xFF;

/// Maps glyph identifiers to glyph styles.
///
/// Also keeps track of the styles that are actually used so we can allocate
/// an appropriately sized metrics array.
#[derive(Debug)]
pub(crate) struct GlyphStyleMap {
    /// List of styles, indexed by glyph id.
    styles: Vec<GlyphStyle>,
    /// Maps an actual style class index into a compacted index for the
    /// metrics table.
    ///
    /// Uses `0xFF` to signify unused styles.
    metrics_map: [u8; MAX_STYLES],
    /// Number of metrics styles in use.
    metrics_count: u8,
}

impl GlyphStyleMap {
    /// Computes a new glyph style map for the given glyph count and character
    /// map.
    ///
    /// Roughly based on <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afglobal.c#L126>
    pub fn new(glyph_count: u32, shaper: &Shaper) -> Self {
        let lookup_count = shaper.lookup_count() as usize;
        if lookup_count > 0 {
            // If we're processing lookups, allocate some temporary memory to
            // store the visited set
            let lookup_set_byte_size = lookup_count.div_ceil(8);
            super::super::memory::with_temporary_memory(lookup_set_byte_size, |bytes| {
                Self::new_inner(glyph_count, shaper, VisitedLookupSet::new(bytes))
            })
        } else {
            Self::new_inner(glyph_count, shaper, VisitedLookupSet::new(&mut []))
        }
    }

    fn new_inner(glyph_count: u32, shaper: &Shaper, mut visited_set: VisitedLookupSet) -> Self {
        let mut map = Self {
            styles: vec![GlyphStyle::default(); glyph_count as usize],
            metrics_map: [UNMAPPED_STYLE; MAX_STYLES],
            metrics_count: 0,
        };
        // Step 1: compute styles for glyphs covered by OpenType features
        // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afglobal.c#L233>
        for style in super::style::STYLE_CLASSES {
            if style.feature.is_some()
                && shaper.compute_coverage(
                    style,
                    ShaperCoverageKind::Script,
                    &mut map.styles,
                    &mut visited_set,
                )
            {
                map.use_style(style.index);
            }
        }
        // Step 2: compute styles for glyphs contained in the cmap
        // cmap entries are sorted so we keep track of the most recent range to
        // avoid a binary search per character
        let mut last_range: Option<(usize, StyleRange)> = None;
        for (ch, gid) in shaper.charmap().mappings() {
            let Some(style) = map.styles.get_mut(gid.to_u32() as usize) else {
                continue;
            };
            // Charmaps enumerate in order so we're likely to encounter at least
            // a few codepoints in the same range.
            if let Some(last) = last_range {
                if last.1.contains(ch) {
                    style.maybe_assign(last.1.style);
                    continue;
                }
            }
            let ix = match STYLE_RANGES.binary_search_by(|x| x.first.cmp(&ch)) {
                Ok(i) => i,
                Err(i) => i.saturating_sub(1),
            };
            let Some(range) = STYLE_RANGES.get(ix).copied() else {
                continue;
            };
            if range.contains(ch) {
                style.maybe_assign(range.style);
                if let Some(style_ix) = range.style.style_index() {
                    map.use_style(style_ix as usize);
                }
                last_range = Some((ix, range));
            }
        }
        // Step 3a: compute script based coverage
        // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afglobal.c#L239>
        for style in super::style::STYLE_CLASSES {
            if style.feature.is_none()
                && shaper.compute_coverage(
                    style,
                    ShaperCoverageKind::Script,
                    &mut map.styles,
                    &mut visited_set,
                )
            {
                map.use_style(style.index);
            }
        }
        // Step 3b: compute coverage for "default" script which is always set
        // to Latin in FreeType
        // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afglobal.c#L248>
        let default_style = &STYLE_CLASSES[StyleClass::LATN];
        if shaper.compute_coverage(
            default_style,
            ShaperCoverageKind::Default,
            &mut map.styles,
            &mut visited_set,
        ) {
            map.use_style(default_style.index);
        }
        // Step 4: Assign a default to all remaining glyphs
        // For some reason, FreeType uses Hani as a default fallback style so
        // let's do the same.
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afglobal.h#L69>
        let mut need_hani = false;
        for style in map.styles.iter_mut() {
            if style.is_unassigned() {
                style.0 &= !GlyphStyle::STYLE_INDEX_MASK;
                style.0 |= StyleClass::HANI as u16;
                need_hani = true;
            }
        }
        if need_hani {
            map.use_style(StyleClass::HANI);
        }
        // Step 5: Mark ASCII digits
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afglobal.c#L251>
        for digit_char in '0'..='9' {
            if let Some(style) = shaper
                .charmap()
                .map(digit_char)
                .and_then(|gid| map.styles.get_mut(gid.to_u32() as usize))
            {
                style.0 |= GlyphStyle::DIGIT;
            }
        }
        map
    }

    pub fn style(&self, glyph_id: GlyphId) -> Option<GlyphStyle> {
        self.styles.get(glyph_id.to_u32() as usize).copied()
    }

    /// Returns a compacted metrics index for the given glyph style.
    pub fn metrics_index(&self, style: GlyphStyle) -> Option<usize> {
        let ix = style.style_index()? as usize;
        let metrics_ix = *self.metrics_map.get(ix)? as usize;
        if metrics_ix != UNMAPPED_STYLE as usize {
            Some(metrics_ix)
        } else {
            None
        }
    }

    /// Returns the required size of the compacted metrics array.
    pub fn metrics_count(&self) -> usize {
        self.metrics_count as usize
    }

    /// Returns an ordered iterator yielding each style class referenced by
    /// this map.
    pub fn metrics_styles(&self) -> impl Iterator<Item = &'static StyleClass> + '_ {
        // Need to build a reverse map so that these are properly ordered
        let mut reverse_map = [UNMAPPED_STYLE; MAX_STYLES];
        for (ix, &entry) in self.metrics_map.iter().enumerate() {
            if entry != UNMAPPED_STYLE {
                reverse_map[entry as usize] = ix as u8;
            }
        }
        reverse_map
            .into_iter()
            .enumerate()
            .filter_map(move |(mapped, style_ix)| {
                if mapped == UNMAPPED_STYLE as usize {
                    None
                } else {
                    STYLE_CLASSES.get(style_ix as usize)
                }
            })
    }

    fn use_style(&mut self, style_ix: usize) {
        let mapped = &mut self.metrics_map[style_ix];
        if *mapped == UNMAPPED_STYLE {
            // This the first time we've seen this style so record
            // it in the metrics map
            *mapped = self.metrics_count;
            self.metrics_count += 1;
        }
    }
}

impl Default for GlyphStyleMap {
    fn default() -> Self {
        Self {
            styles: Default::default(),
            metrics_map: [UNMAPPED_STYLE; MAX_STYLES],
            metrics_count: 0,
        }
    }
}

/// Determines which algorithms the autohinter will use while generating
/// metrics and processing a glyph outline.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub(crate) enum ScriptGroup {
    /// All scripts that are not CJK or Indic.
    ///
    /// FreeType calls this Latin.
    #[default]
    Default,
    Cjk,
    Indic,
}

/// Defines the basic properties for each script supported by the
/// autohinter.
#[derive(Clone, Debug)]
pub(crate) struct ScriptClass {
    #[allow(unused)]
    pub name: &'static str,
    /// Group that defines how glyphs belonging to this script are hinted.
    pub group: ScriptGroup,
    /// Unicode tag for the script.
    #[allow(unused)]
    pub tag: Tag,
    /// True if outline edges are processed top to bottom.
    pub hint_top_to_bottom: bool,
    /// Characters used to define standard width and height of stems.
    pub std_chars: &'static str,
    /// "Blue" characters used to define alignment zones.
    pub blues: &'static [(&'static str, BlueZones)],
}

/// Defines the basic properties for each style supported by the
/// autohinter.
///
/// There's mostly a 1:1 correspondence between styles and scripts except
/// in the cases where style coverage is determined by OpenType feature
/// coverage.
#[derive(Clone, Debug)]
pub(crate) struct StyleClass {
    #[allow(unused)]
    pub name: &'static str,
    /// Index of self in the STYLE_CLASSES array.
    pub index: usize,
    /// Associated Unicode script.
    pub script: &'static ScriptClass,
    /// OpenType feature tag for styles that derive coverage from layout
    /// tables.
    #[allow(unused)]
    pub feature: Option<Tag>,
}

impl StyleClass {
    pub(crate) fn from_index(index: u16) -> Option<&'static StyleClass> {
        STYLE_CLASSES.get(index as usize)
    }
}

/// Associates a basic glyph style with a range of codepoints.
#[derive(Copy, Clone, Debug)]
pub(super) struct StyleRange {
    pub first: u32,
    pub last: u32,
    pub style: GlyphStyle,
}

impl StyleRange {
    pub fn contains(&self, ch: u32) -> bool {
        (self.first..=self.last).contains(&ch)
    }
}

// The following are helpers for generated code.
const fn base_range(first: u32, last: u32, style_index: u16) -> StyleRange {
    StyleRange {
        first,
        last,
        style: GlyphStyle(style_index),
    }
}

const fn non_base_range(first: u32, last: u32, style_index: u16) -> StyleRange {
    StyleRange {
        first,
        last,
        style: GlyphStyle(style_index | GlyphStyle::NON_BASE),
    }
}

const MAX_STYLES: usize = STYLE_CLASSES.len();

use super::shape::Shaper;

include!("../../../generated/generated_autohint_styles.rs");

#[cfg(test)]
mod tests {
    use super::{super::shape::ShaperMode, *};
    use crate::{raw::TableProvider, FontRef, MetadataProvider};

    /// Ensure that style mapping accurately applies the DIGIT bit to
    /// ASCII digit glyphs.
    #[test]
    fn capture_digit_styles() {
        let font = FontRef::new(font_test_data::AHEM).unwrap();
        let shaper = Shaper::new(&font, ShaperMode::Nominal);
        let num_glyphs = font.maxp().unwrap().num_glyphs() as u32;
        let style_map = GlyphStyleMap::new(num_glyphs, &shaper);
        let charmap = font.charmap();
        let mut digit_count = 0;
        for (ch, gid) in charmap.mappings() {
            let style = style_map.style(gid).unwrap();
            let is_char_digit = char::from_u32(ch).unwrap().is_ascii_digit();
            assert_eq!(style.is_digit(), is_char_digit);
            digit_count += is_char_digit as u32;
        }
        // This font has all 10 ASCII digits
        assert_eq!(digit_count, 10);
    }

    #[test]
    fn glyph_styles() {
        // generated by printf debugging in FreeType
        // (gid, Option<(script_name, is_non_base_char)>)
        // where "is_non_base_char" more common means "is_mark"
        let expected = &[
            (0, Some(("CJKV ideographs", false))),
            (1, Some(("Latin", true))),
            (2, Some(("Armenian", true))),
            (3, Some(("Hebrew", true))),
            (4, Some(("Arabic", false))),
            (5, Some(("Arabic", false))),
            (6, Some(("Arabic", true))),
            (7, Some(("Devanagari", true))),
            (8, Some(("Devanagari", false))),
            (9, Some(("Bengali", true))),
            (10, Some(("Bengali", false))),
            (11, Some(("Gurmukhi", true))),
            (12, Some(("Gurmukhi", false))),
            (13, Some(("Gujarati", true))),
            (14, Some(("Gujarati", true))),
            (15, Some(("Oriya", true))),
            (16, Some(("Oriya", false))),
            (17, Some(("Tamil", true))),
            (18, Some(("Tamil", false))),
            (19, Some(("Telugu", true))),
            (20, Some(("Telugu", false))),
            (21, Some(("Kannada", true))),
            (22, Some(("Kannada", false))),
            (23, Some(("Malayalam", true))),
            (24, Some(("Malayalam", false))),
            (25, Some(("Sinhala", true))),
            (26, Some(("Sinhala", false))),
            (27, Some(("Thai", true))),
            (28, Some(("Thai", false))),
            (29, Some(("Lao", true))),
            (30, Some(("Lao", false))),
            (31, Some(("Tibetan", true))),
            (32, Some(("Tibetan", false))),
            (33, Some(("Myanmar", true))),
            (34, Some(("Ethiopic", true))),
            (35, Some(("Buhid", true))),
            (36, Some(("Buhid", false))),
            (37, Some(("Khmer", true))),
            (38, Some(("Khmer", false))),
            (39, Some(("Mongolian", true))),
            (40, Some(("Canadian Syllabics", false))),
            (41, Some(("Limbu", true))),
            (42, Some(("Limbu", false))),
            (43, Some(("Khmer Symbols", false))),
            (44, Some(("Sundanese", true))),
            (45, Some(("Ol Chiki", false))),
            (46, Some(("Georgian (Mkhedruli)", false))),
            (47, Some(("Sundanese", false))),
            (48, Some(("Latin Superscript Fallback", false))),
            (49, Some(("Latin", true))),
            (50, Some(("Greek", true))),
            (51, Some(("Greek", false))),
            (52, Some(("Latin Subscript Fallback", false))),
            (53, Some(("Coptic", true))),
            (54, Some(("Coptic", false))),
            (55, Some(("Georgian (Khutsuri)", false))),
            (56, Some(("Tifinagh", false))),
            (57, Some(("Ethiopic", false))),
            (58, Some(("Cyrillic", true))),
            (59, Some(("CJKV ideographs", true))),
            (60, Some(("CJKV ideographs", false))),
            (61, Some(("Lisu", false))),
            (62, Some(("Vai", false))),
            (63, Some(("Cyrillic", true))),
            (64, Some(("Bamum", true))),
            (65, Some(("Syloti Nagri", true))),
            (66, Some(("Syloti Nagri", false))),
            (67, Some(("Saurashtra", true))),
            (68, Some(("Saurashtra", false))),
            (69, Some(("Kayah Li", true))),
            (70, Some(("Kayah Li", false))),
            (71, Some(("Myanmar", false))),
            (72, Some(("Tai Viet", true))),
            (73, Some(("Tai Viet", false))),
            (74, Some(("Cherokee", false))),
            (75, Some(("Armenian", false))),
            (76, Some(("Hebrew", false))),
            (77, Some(("Arabic", false))),
            (78, Some(("Carian", false))),
            (79, Some(("Gothic", false))),
            (80, Some(("Deseret", false))),
            (81, Some(("Shavian", false))),
            (82, Some(("Osmanya", false))),
            (83, Some(("Osage", false))),
            (84, Some(("Cypriot", false))),
            (85, Some(("Avestan", true))),
            (86, Some(("Avestan", true))),
            (87, Some(("Old Turkic", false))),
            (88, Some(("Hanifi Rohingya", false))),
            (89, Some(("Chakma", true))),
            (90, Some(("Chakma", false))),
            (91, Some(("Mongolian", false))),
            (92, Some(("CJKV ideographs", false))),
            (93, Some(("Medefaidrin", false))),
            (94, Some(("Glagolitic", true))),
            (95, Some(("Glagolitic", true))),
            (96, Some(("Adlam", true))),
            (97, Some(("Adlam", false))),
        ];
        check_styles(font_test_data::AUTOHINT_CMAP, ShaperMode::Nominal, expected);
    }

    #[test]
    fn shaped_glyph_styles() {
        // generated by printf debugging in FreeType
        // (gid, Option<(script_name, is_non_base_char)>)
        // where "is_non_base_char" more common means "is_mark"
        let expected = &[
            (0, Some(("CJKV ideographs", false))),
            (1, Some(("Latin", false))),
            (2, Some(("Latin", false))),
            (3, Some(("Latin", false))),
            (4, Some(("Latin", false))),
            // Note: ligatures starting with 'f' are assigned the Cyrillic
            // script which matches FreeType
            (5, Some(("Cyrillic", false))),
            (6, Some(("Cyrillic", false))),
            (7, Some(("Cyrillic", false))),
            // Capture the Latin c2sc feature
            (8, Some(("Latin small capitals from capitals", false))),
        ];
        check_styles(
            font_test_data::NOTOSERIF_AUTOHINT_SHAPING,
            ShaperMode::BestEffort,
            expected,
        );
    }

    fn check_styles(font_data: &[u8], mode: ShaperMode, expected: &[(u32, Option<(&str, bool)>)]) {
        let font = FontRef::new(font_data).unwrap();
        let shaper = Shaper::new(&font, mode);
        let num_glyphs = font.maxp().unwrap().num_glyphs() as u32;
        let style_map = GlyphStyleMap::new(num_glyphs, &shaper);
        let results = style_map
            .styles
            .iter()
            .enumerate()
            .map(|(gid, style)| {
                (
                    gid as u32,
                    style
                        .style_class()
                        .map(|style_class| (style_class.name, style.is_non_base())),
                )
            })
            .collect::<Vec<_>>();
        for (i, result) in results.iter().enumerate() {
            assert_eq!(result, &expected[i]);
        }
        // Ensure each style has a remapped metrics index
        for style in &style_map.styles {
            style_map.metrics_index(*style).unwrap();
        }
    }
}
