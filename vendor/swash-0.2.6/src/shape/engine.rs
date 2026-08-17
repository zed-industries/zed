use super::{aat, at};

use super::buffer::*;
use super::internal::{self, at::Gdef, raw_tag, Bytes, RawFont, RawTag};
use crate::font::FontRef;
use crate::text::{Language, Script};

use alloc::vec::Vec;
use core::ops::Range;

/// Shaping engine that handles the various methods available in
/// an OpenType font.
pub struct Engine<'a> {
    pub data: Bytes<'a>,
    pub gdef: Gdef<'a>,
    pub gsub: at::StageOffsets,
    pub gpos: at::StageOffsets,
    pub morx: u32,
    pub kerx: u32,
    pub ankr: u32,
    pub kern: u32,
    pub storage: at::Storage,
    pub coords: &'a [i16],
    pub script: Script,
    pub tags: [RawTag; 4],
    pub sub_mode: SubMode,
    pub pos_mode: PosMode,
    pub use_ot: bool,
    pub mode: EngineMode,
}

impl<'a> Engine<'a> {
    /// Creates a new shaping engine from precreated metadata.
    pub fn new(
        metadata: &EngineMetadata,
        font_data: &'a [u8],
        coords: &'a [i16],
        script: Script,
        lang: Option<Language>,
    ) -> Self {
        let data = Bytes::new(font_data);
        let gdef = Gdef::from_offset(font_data, metadata.gdef).unwrap_or_else(Gdef::empty);
        let script_tag = script.to_opentype();
        let lang_tag = lang.and_then(|l| l.to_opentype());
        let (gsub, stags) = if metadata.sub_mode == SubMode::Gsub {
            at::StageOffsets::new(&data, metadata.gsub, script_tag, lang_tag).unwrap_or_default()
        } else {
            (at::StageOffsets::default(), [0, 0])
        };
        let (gpos, ptags) = if metadata.pos_mode == PosMode::Gpos {
            at::StageOffsets::new(&data, metadata.gpos, script_tag, lang_tag).unwrap_or_default()
        } else {
            (at::StageOffsets::default(), [0, 0])
        };
        let tags = [stags[0], stags[1], ptags[0], ptags[1]];
        let use_ot = gsub.lang != 0 || gpos.lang != 0;
        let mode = if gsub.lang != 0 && script.is_complex() {
            if script == Script::Myanmar {
                EngineMode::Myanmar
            } else {
                EngineMode::Complex
            }
        } else {
            EngineMode::Simple
        };
        let mut sub_mode = metadata.sub_mode;
        let mut pos_mode = metadata.pos_mode;
        if sub_mode == SubMode::Gsub && gsub.lang == 0 {
            sub_mode = SubMode::None;
        }
        if pos_mode == PosMode::Gpos && gpos.lang == 0 {
            pos_mode = PosMode::None;
        }
        Self {
            data,
            gdef,
            gsub,
            gpos,
            morx: metadata.morx,
            kerx: metadata.kerx,
            ankr: metadata.ankr,
            kern: metadata.kern,
            storage: at::Storage::default(),
            coords,
            script,
            tags,
            sub_mode,
            pos_mode,
            use_ot,
            mode,
        }
    }
}

/// OpenType shaping.
impl<'a> Engine<'a> {
    /// Returns the script and language tags that have been selected for
    /// the GSUB and GPOS tables.
    pub fn tags(&self) -> &[RawTag; 4] {
        &self.tags
    }

    /// Builds a feature store for the current engine configuration.
    pub fn collect_features(
        &self,
        builder: &mut at::FeatureStoreBuilder,
        store: &mut at::FeatureStore,
    ) {
        builder.build(
            store,
            self.data.data(),
            self.coords,
            &self.gdef,
            &self.gsub,
            &self.gpos,
        );
        store.groups = store.groups(self.script);
    }

    /// Returns true if feature variations are supported.
    pub fn has_feature_vars(&self) -> bool {
        self.gsub.var != 0 || self.gpos.var != 0
    }

    /// Sets glyph and mark classes for the specified range of the buffer.
    pub fn set_classes(&self, buffer: &mut Buffer, range: Option<Range<usize>>) {
        if !self.gdef.ok() {
            return;
        }
        let slice = if let Some(range) = range {
            &mut buffer.glyphs[range]
        } else {
            &mut buffer.glyphs[..]
        };
        let gdef = &self.gdef;
        if gdef.has_mark_classes() {
            for g in slice.iter_mut() {
                g.class = gdef.class(g.id) as u8;
                g.mark_type = gdef.mark_class(g.id) as u8;
            }
        } else {
            for g in slice.iter_mut() {
                g.class = gdef.class(g.id) as u8;
            }
        }
    }

    /// Applies the GSUB features to the specified range of the buffer.
    pub fn gsub(
        &mut self,
        store: &at::FeatureStore,
        feature_mask: impl Into<at::FeatureMask>,
        buffer: &mut Buffer,
        buffer_range: Option<Range<usize>>,
    ) -> bool {
        at::apply(
            0,
            &self.data,
            self.gsub.base,
            self.coords,
            &self.gdef,
            &mut self.storage,
            store,
            feature_mask.into(),
            buffer,
            buffer_range,
        ) == Some(true)
    }

    /// Applies the GPOS features to the specified range of the buffer.
    pub fn gpos(
        &mut self,
        store: &at::FeatureStore,
        feature_mask: impl Into<at::FeatureMask>,
        buffer: &mut Buffer,
        buffer_range: Option<Range<usize>>,
    ) -> bool {
        at::apply(
            1,
            &self.data,
            self.gpos.base,
            self.coords,
            &self.gdef,
            &mut self.storage,
            store,
            feature_mask.into(),
            buffer,
            buffer_range,
        ) == Some(true)
    }
}

/// Apple shaping.
impl<'a> Engine<'a> {
    /// Converts a feature list into a sorted collection of AAT selectors.
    pub fn collect_selectors(&self, features: &[(RawTag, u16)], selectors: &mut Vec<(u16, u16)>) {
        use internal::aat::morx::feature_from_tag;
        selectors.clear();
        for (tag, value) in features {
            if let Some((selector, [on, off])) = feature_from_tag(*tag) {
                let setting = if *value == 0 { off } else { on };
                selectors.push((selector, setting));
            }
        }
        selectors.sort_unstable();
    }

    /// Applies the extended metamorphosis table.
    pub fn morx(&self, buffer: &mut Buffer, selectors: &[(u16, u16)]) {
        if self.morx != 0 {
            aat::apply_morx(self.data.data(), self.morx, buffer, selectors);
            buffer.ensure_order(false);
        }
    }

    /// Applies the extended kerning table.
    pub fn kerx(&self, buffer: &mut Buffer, disable_kern: bool) {
        if self.kerx != 0 {
            aat::apply_kerx(self.data.data(), self.kerx, self.ankr, buffer, disable_kern);
            buffer.ensure_order(false);
        }
    }

    /// Applies the kerning table.
    pub fn kern(&self, buffer: &mut Buffer) {
        if self.kern != 0 {
            aat::apply_kern(self.data.data(), self.kern, buffer);
        }
    }
}

/// The overall mode of the engine based on a combination of the
/// supported tables and the selected script.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EngineMode {
    Simple,
    Myanmar,
    Complex,
}

/// The substitution mode supported by the engine.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SubMode {
    None,
    Gsub,
    Morx,
}

/// The positioning mode supported by the engine.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PosMode {
    None,
    Gpos,
    Kerx,
    Kern,
}

/// Metadata for creating a shaping engine.
#[derive(Copy, Clone)]
pub struct EngineMetadata {
    pub gdef: u32,
    pub gsub: u32,
    pub gpos: u32,
    pub morx: u32,
    pub kerx: u32,
    pub ankr: u32,
    pub kern: u32,
    pub sub_mode: SubMode,
    pub pos_mode: PosMode,
}

impl EngineMetadata {
    pub fn from_font(font: &FontRef) -> Self {
        let mut this = Self {
            gdef: font.table_offset(raw_tag(b"GDEF")),
            gsub: font.table_offset(raw_tag(b"GSUB")),
            gpos: font.table_offset(raw_tag(b"GPOS")),
            morx: font.table_offset(raw_tag(b"morx")),
            // ltag: font.table_offset(raw_tag(b"ltag")),
            kerx: font.table_offset(raw_tag(b"kerx")),
            ankr: font.table_offset(raw_tag(b"ankr")),
            kern: font.table_offset(raw_tag(b"kern")),
            sub_mode: SubMode::None,
            pos_mode: PosMode::None,
        };
        if this.gsub != 0 {
            this.sub_mode = SubMode::Gsub;
        } else if this.morx != 0 {
            this.sub_mode = SubMode::Morx;
        }
        if this.gpos != 0 {
            this.pos_mode = PosMode::Gpos;
        } else if this.kerx != 0 {
            this.pos_mode = PosMode::Kerx;
        } else if this.kern != 0 {
            this.pos_mode = PosMode::Kern;
        }
        this
    }
}
