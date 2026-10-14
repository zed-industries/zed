// These outlines form continuous strokes, but independent hinting can move
// their horizontal segments onto different pixel rows.
// #65215
const LILEX_UNHINTED_GLYPHS: &[&str] = &[
    "bar_hyphen_start.seq",  // | at the start of |- or |--
    "bar_hyphen_middle.seq", // | between hyphens in -|- or --|--
    "bar_hyphen_end.seq",    // | at the end of -| or --|
    "hyphen_start.seq",      // first - in -|- or --|--
    "hyphen_middle.seq",     // an interior - in |-- or --|--
    "hyphen_end.seq",        // final - in |-, -|-, or --|--
];

pub(crate) struct FontHinting(&'static [&'static str]);

impl FontHinting {
    pub(crate) fn for_font(postscript_name: &str) -> Self {
        if postscript_name.starts_with("Lilex-") {
            Self(LILEX_UNHINTED_GLYPHS)
        } else {
            Self(&[])
        }
    }

    pub(crate) fn should_hint(&self, font: swash::FontRef<'_>, glyph_id: u16) -> bool {
        if self.0.is_empty() {
            return true;
        }
        // Contextual glyphs have no character-map entry, so identify them by
        // the names stored in Lilex's post table after shaping.
        !matches!(font.glyph_name(glyph_id), Some(name) if self.0.contains(&name))
    }
}
