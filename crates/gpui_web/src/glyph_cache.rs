use anyhow::Result;
use gpui::{FontId, GlyphId, SharedString};
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct CanvasGlyph {
    pub(crate) font_id: FontId,
    pub(crate) text: SharedString,
    pub(crate) color: bool,
}

#[derive(Default)]
pub(crate) struct GlyphCache {
    glyphs: Vec<CanvasGlyph>,
    ids_by_font: HashMap<(FontId, bool), HashMap<SharedString, GlyphId>>,
}

impl GlyphCache {
    pub(crate) fn id_for(&self, font_id: FontId, text: &str, color: bool) -> Option<GlyphId> {
        self.ids_by_font.get(&(font_id, color))?.get(text).copied()
    }

    pub(crate) fn get_or_insert(
        &mut self,
        font_id: FontId,
        text: &str,
        color: bool,
    ) -> Result<GlyphId> {
        if let Some(glyph_id) = self.id_for(font_id, text, color) {
            return Ok(glyph_id);
        }
        let glyph_id = GlyphId(u32::try_from(self.glyphs.len())?);
        let text = SharedString::from(text.to_owned());
        self.glyphs.push(CanvasGlyph {
            font_id,
            text: text.clone(),
            color,
        });
        self.ids_by_font
            .entry((font_id, color))
            .or_default()
            .insert(text, glyph_id);
        Ok(glyph_id)
    }

    pub(crate) fn glyph(&self, font_id: FontId, glyph_id: GlyphId) -> Option<&CanvasGlyph> {
        self.glyphs
            .get(glyph_id.0 as usize)
            .filter(|glyph| glyph.font_id == font_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context as _;

    #[test]
    fn warm_lookup_borrows_text_and_preserves_storage() -> Result<()> {
        let mut cache = GlyphCache::default();
        let font_id = FontId(1);
        let glyph_id = cache.get_or_insert(font_id, "👩‍💻", true)?;
        let glyphs_pointer = cache.glyphs.as_ptr();
        let glyphs_capacity = cache.glyphs.capacity();
        let text_pointer = cache
            .glyph(font_id, glyph_id)
            .context("missing glyph")?
            .text
            .as_ptr();
        let surrounding_text = String::from("👩‍💻 suffix");
        let borrowed_text = surrounding_text
            .strip_suffix(" suffix")
            .context("missing suffix")?;

        for _ in 0..100 {
            assert_eq!(cache.id_for(font_id, borrowed_text, true), Some(glyph_id));
            assert_eq!(cache.get_or_insert(font_id, borrowed_text, true)?, glyph_id);
        }

        assert_eq!(cache.glyphs.len(), 1);
        assert_eq!(cache.glyphs.as_ptr(), glyphs_pointer);
        assert_eq!(cache.glyphs.capacity(), glyphs_capacity);
        assert_eq!(
            cache
                .glyph(font_id, glyph_id)
                .context("missing glyph")?
                .text
                .as_ptr(),
            text_pointer
        );
        Ok(())
    }

    #[test]
    fn font_and_presentation_have_distinct_ids() -> Result<()> {
        let mut cache = GlyphCache::default();
        let first_font = FontId(1);
        let second_font = FontId(2);
        let text = cache.get_or_insert(first_font, "❤", false)?;
        let color = cache.get_or_insert(first_font, "❤", true)?;
        let other_font = cache.get_or_insert(second_font, "❤", false)?;

        assert_ne!(text, color);
        assert_ne!(text, other_font);
        assert_ne!(color, other_font);
        assert_eq!(cache.id_for(first_font, "❤", false), Some(text));
        assert_eq!(cache.id_for(first_font, "❤", true), Some(color));
        assert_eq!(cache.id_for(second_font, "❤", false), Some(other_font));
        assert!(cache.glyph(second_font, text).is_none());
        assert!(cache.glyph(first_font, GlyphId(u32::MAX)).is_none());
        assert_eq!(
            cache.glyph(first_font, text).map(|glyph| glyph.color),
            Some(false)
        );
        assert_eq!(
            cache.glyph(first_font, color).map(|glyph| glyph.color),
            Some(true)
        );
        Ok(())
    }

    #[test]
    fn insertion_rechecks_a_previous_miss() -> Result<()> {
        let mut cache = GlyphCache::default();
        let font_id = FontId(1);
        assert_eq!(cache.id_for(font_id, "中", false), None);

        let inserted = cache.get_or_insert(font_id, "中", false)?;
        assert_eq!(cache.get_or_insert(font_id, "中", false)?, inserted);
        assert_eq!(cache.glyphs.len(), 1);
        Ok(())
    }
}
