use super::layout::DELETED_GLYPH;
use super::map::RangeFlags;
use crate::hb::buffer::{hb_buffer_t, HB_BUFFER_SCRATCH_FLAG_SHAPER0};
use crate::hb::face::hb_font_t;
use crate::hb::hb_mask_t;
use crate::hb::ot_layout_gsubgpos::MappingCache;
use crate::hb::ot_shape_plan::hb_ot_shape_plan_t;
use crate::U32Set;
use read_fonts::tables::aat::*;
use read_fonts::types::{FixedSize, GlyphId};

pub const HB_BUFFER_SCRATCH_FLAG_AAT_HAS_DELETED: u32 = HB_BUFFER_SCRATCH_FLAG_SHAPER0;

pub(crate) const START_OF_TEXT: u16 = 0;

pub(crate) type ClassCache = MappingCache;

pub(crate) fn get_class<T: bytemuck::AnyBitPattern + FixedSize>(
    machine: &ExtendedStateTable<'_, T>,
    glyph_id: GlyphId,
    cache: &ClassCache,
) -> u16 {
    if let Some(klass) = cache.get(glyph_id.to_u32()) {
        return klass as u16;
    }
    let klass = machine
        .class(glyph_id)
        .unwrap_or(class::OUT_OF_BOUNDS as u16);
    cache.set(glyph_id.to_u32(), klass as u32);
    klass
}

/// HB: hb_aat_apply_context_t
///
/// See <https://github.com/harfbuzz/harfbuzz/blob/2c22a65f0cb99544c36580b9703a43b5dc97a9e1/src/hb-aat-layout-common.hh#L108>
#[doc(alias = "hb_aat_apply_context_t")]
pub struct AatApplyContext<'a> {
    pub plan: &'a hb_ot_shape_plan_t,
    pub face: &'a hb_font_t<'a>,
    pub buffer: &'a mut hb_buffer_t,
    pub has_glyph_classes: bool,
    pub range_flags: Option<&'a [RangeFlags]>,
    pub subtable_flags: hb_mask_t,
    pub(crate) buffer_is_reversed: bool,
    // Caches
    using_buffer_glyph_set: bool,
    pub(crate) first_set: Option<&'a U32Set>,
    pub(crate) second_set: Option<&'a U32Set>,
    pub(crate) machine_class_cache: Option<&'a ClassCache>,
    pub(crate) start_end_safe_to_break: u64,
}

impl<'a> AatApplyContext<'a> {
    pub fn new(
        plan: &'a hb_ot_shape_plan_t,
        face: &'a hb_font_t<'a>,
        buffer: &'a mut hb_buffer_t,
    ) -> Self {
        Self {
            plan,
            face,
            buffer,
            has_glyph_classes: face.ot_tables.has_glyph_classes(),
            range_flags: None,
            subtable_flags: 0,
            buffer_is_reversed: false,
            using_buffer_glyph_set: false,
            first_set: None,
            second_set: None,
            machine_class_cache: None,
            start_end_safe_to_break: 0,
        }
    }

    pub(crate) fn reverse_buffer(&mut self) {
        self.buffer.reverse();
        self.buffer_is_reversed = !self.buffer_is_reversed;
    }

    pub(crate) fn setup_buffer_glyph_set(&mut self) {
        self.using_buffer_glyph_set = self.buffer.len >= 4;

        if self.using_buffer_glyph_set {
            self.buffer.update_glyph_set();
        }
    }

    pub(crate) fn buffer_intersects_machine(&self) -> bool {
        if let Some(first_set) = &self.first_set {
            if self.using_buffer_glyph_set {
                return self.buffer.glyph_set.intersects_set(first_set);
            }
            for info in &self.buffer.info {
                if first_set.contains(info.glyph_id) {
                    return true;
                }
            }
            false
        } else {
            true
        }
    }

    pub fn output_glyph(&mut self, glyph: u32) {
        if self.using_buffer_glyph_set {
            self.buffer.glyph_set.insert(glyph);
        }
        if glyph == DELETED_GLYPH {
            self.buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_AAT_HAS_DELETED;
            self.buffer.cur_mut(0).set_aat_deleted();
        } else {
            if self.has_glyph_classes {
                self.buffer
                    .cur_mut(0)
                    .set_glyph_props(self.face.ot_tables.glyph_props(glyph.into()));
            }
        }
        self.buffer.output_glyph(glyph);
    }

    pub fn replace_glyph(&mut self, glyph: u32) {
        if glyph == DELETED_GLYPH {
            self.buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_AAT_HAS_DELETED;
            self.buffer.cur_mut(0).set_aat_deleted();
        }

        if self.using_buffer_glyph_set {
            self.buffer.glyph_set.insert(glyph);
        }
        if self.has_glyph_classes {
            self.buffer
                .cur_mut(0)
                .set_glyph_props(self.face.ot_tables.glyph_props(glyph.into()));
        }
        self.buffer.replace_glyph(glyph);
    }

    pub fn delete_glyph(&mut self) {
        self.buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_AAT_HAS_DELETED;
        self.buffer.cur_mut(0).set_aat_deleted();
        self.buffer.replace_glyph(DELETED_GLYPH);
    }

    pub fn replace_glyph_inplace(&mut self, i: usize, glyph: u32) {
        self.buffer.info[i].glyph_id = glyph;
        if self.using_buffer_glyph_set {
            self.buffer.glyph_set.insert(glyph);
        }
        if self.has_glyph_classes {
            self.buffer.info[i].set_glyph_props(self.face.ot_tables.glyph_props(glyph.into()));
        }
    }
}

pub trait TypedCollectGlyphs<T: LookupValue> {
    /// Add all indices into `set`.
    fn collect_glyphs(&self, set: &mut U32Set, num_glyphs: u32) {
        self.collect_glyphs_filtered::<_>(set, num_glyphs, |_| true);
    }

    /// For each valid index, read the value of type `T`.
    /// If `filter(&value)` returns true, insert the index into `set`.
    fn collect_glyphs_filtered<F>(&self, _set: &mut U32Set, _num_glyphs: u32, _filter: F)
    where
        F: Fn(T) -> bool;
}

impl<T> TypedCollectGlyphs<T> for TypedLookup<'_, T>
where
    T: LookupValue,
{
    fn collect_glyphs(&self, set: &mut U32Set, num_glyphs: u32) {
        self.lookup.collect_glyphs::<T>(set, num_glyphs);
    }
    fn collect_glyphs_filtered<F>(&self, set: &mut U32Set, num_glyphs: u32, filter: F)
    where
        F: Fn(T) -> bool,
    {
        self.lookup
            .collect_glyphs_filtered::<T, F>(set, num_glyphs, filter);
    }
}

pub trait CollectGlyphs {
    /// Add all indices into `set`.
    fn collect_glyphs<T>(&self, set: &mut U32Set, num_glyphs: u32)
    where
        T: LookupValue,
    {
        self.collect_glyphs_filtered::<T, _>(set, num_glyphs, |_| true);
    }

    /// For each valid index, read the value of type `T`.
    /// If `filter(&value)` returns true, insert the index into `set`.
    fn collect_glyphs_filtered<T, F>(&self, _set: &mut U32Set, _num_glyphs: u32, _filter: F)
    where
        T: LookupValue,
        F: Fn(T) -> bool;
}

impl CollectGlyphs for Lookup<'_> {
    fn collect_glyphs<T>(&self, set: &mut U32Set, num_glyphs: u32)
    where
        T: LookupValue,
    {
        match self {
            Lookup::Format0(lookup) => lookup.collect_glyphs::<T>(set, num_glyphs),
            Lookup::Format2(lookup) => lookup.collect_glyphs::<T>(set, num_glyphs),
            Lookup::Format4(lookup) => lookup.collect_glyphs::<T>(set, num_glyphs),
            Lookup::Format6(lookup) => lookup.collect_glyphs::<T>(set, num_glyphs),
            Lookup::Format8(lookup) => lookup.collect_glyphs::<T>(set, num_glyphs),
            Lookup::Format10(lookup) => lookup.collect_glyphs::<T>(set, num_glyphs),
        }
    }
    fn collect_glyphs_filtered<T, F>(&self, set: &mut U32Set, num_glyphs: u32, filter: F)
    where
        T: LookupValue,
        F: Fn(T) -> bool,
    {
        match self {
            Lookup::Format0(lookup) => {
                lookup.collect_glyphs_filtered::<T, F>(set, num_glyphs, filter);
            }
            Lookup::Format2(lookup) => {
                lookup.collect_glyphs_filtered::<T, F>(set, num_glyphs, filter);
            }
            Lookup::Format4(lookup) => {
                lookup.collect_glyphs_filtered::<T, F>(set, num_glyphs, filter);
            }
            Lookup::Format6(lookup) => {
                lookup.collect_glyphs_filtered::<T, F>(set, num_glyphs, filter);
            }
            Lookup::Format8(lookup) => {
                lookup.collect_glyphs_filtered::<T, F>(set, num_glyphs, filter);
            }
            Lookup::Format10(lookup) => {
                lookup.collect_glyphs_filtered::<T, F>(set, num_glyphs, filter);
            }
        }
    }
}

impl CollectGlyphs for Lookup0<'_> {
    fn collect_glyphs<T>(&self, set: &mut U32Set, num_glyphs: u32)
    where
        T: LookupValue,
    {
        set.insert_range(0..=num_glyphs.saturating_sub(1));
    }
    fn collect_glyphs_filtered<T, F>(&self, set: &mut U32Set, num_glyphs: u32, filter: F)
    where
        T: LookupValue,
        F: Fn(T) -> bool,
    {
        if let Ok(values) = self.values::<T>() {
            for (i, value) in values.iter().take(num_glyphs as usize).enumerate() {
                if filter(value.get()) {
                    set.insert(i as u32);
                }
            }
        }
    }
}
impl CollectGlyphs for Lookup2<'_> {
    fn collect_glyphs_filtered<T, F>(&self, set: &mut U32Set, _num_glyphs: u32, filter: F)
    where
        T: LookupValue,
        F: Fn(T) -> bool,
    {
        if let Ok(segments) = self.segments::<T>() {
            for segment in segments {
                let value = segment.value;
                if filter(value.get()) {
                    if segment.first_glyph.get() as u32 == DELETED_GLYPH {
                        continue;
                    }
                    set.insert_range(
                        segment.first_glyph.get() as u32..=segment.last_glyph.get() as u32,
                    );
                }
            }
        }
    }
}
impl CollectGlyphs for Lookup4<'_> {
    fn collect_glyphs<T>(&self, set: &mut U32Set, _num_glyphs: u32)
    where
        T: LookupValue,
    {
        for segment in self.segments() {
            if segment.first_glyph.get() as u32 == DELETED_GLYPH {
                continue;
            }
            set.insert_range(segment.first_glyph.get() as u32..=segment.last_glyph.get() as u32);
        }
    }
    fn collect_glyphs_filtered<T, F>(&self, set: &mut U32Set, _num_glyphs: u32, filter: F)
    where
        T: LookupValue,
        F: Fn(T) -> bool,
    {
        for (segment_idx, segment) in self.segments().iter().enumerate() {
            if segment.first_glyph.get() as u32 == DELETED_GLYPH {
                continue;
            }
            let segment_values = self.segment_values(segment_idx);
            if let Ok(segment_values) = segment_values {
                for (i, value) in segment_values.iter().enumerate() {
                    if filter(value.get()) {
                        set.insert(segment.first_glyph.get() as u32 + i as u32);
                    }
                }
            }
        }
    }
}
impl CollectGlyphs for Lookup6<'_> {
    fn collect_glyphs_filtered<T, F>(&self, set: &mut U32Set, _num_glyphs: u32, filter: F)
    where
        T: LookupValue,
        F: Fn(T) -> bool,
    {
        let entries = self.entries();
        if let Ok(entries) = entries {
            for entry in entries {
                let value = entry.value;
                if filter(value.get()) {
                    if entry.glyph.get() as u32 == DELETED_GLYPH {
                        continue;
                    }
                    set.insert(entry.glyph.get() as u32);
                }
            }
        }
    }
}
impl CollectGlyphs for Lookup8<'_> {
    fn collect_glyphs<T>(&self, set: &mut U32Set, _num_glyphs: u32)
    where
        T: LookupValue,
    {
        let n_values = self.value_array().len();
        let first_glyph = self.first_glyph();
        if first_glyph as u32 == DELETED_GLYPH {
            return;
        }
        set.insert_range(
            first_glyph as u32..=first_glyph as u32 + n_values.saturating_sub(1) as u32,
        );
    }
    fn collect_glyphs_filtered<T, F>(&self, set: &mut U32Set, _num_glyphs: u32, filter: F)
    where
        T: LookupValue,
        F: Fn(T) -> bool,
    {
        let values = self.value_array();
        let first_glyph = self.first_glyph();
        if first_glyph as u32 == DELETED_GLYPH {
            return;
        }
        for (i, value) in values.iter().enumerate() {
            if filter(T::from_u16(value.get())) {
                set.insert(first_glyph as u32 + i as u32);
            }
        }
    }
}
impl CollectGlyphs for Lookup10<'_> {
    fn collect_glyphs<T>(&self, set: &mut U32Set, _num_glyphs: u32)
    where
        T: LookupValue,
    {
        let n_values = self.glyph_count();
        let first_glyph = self.first_glyph();
        if first_glyph as u32 == DELETED_GLYPH {
            return;
        }
        set.insert_range(
            first_glyph as u32..=first_glyph as u32 + n_values.saturating_sub(1) as u32,
        );
    }
    fn collect_glyphs_filtered<T, F>(&self, set: &mut U32Set, _num_glyphs: u32, filter: F)
    where
        T: LookupValue,
        F: Fn(T) -> bool,
    {
        let first_glyph = self.first_glyph();
        if first_glyph as u32 == DELETED_GLYPH {
            return;
        }
        for i in 0..self.glyph_count() {
            let idx = first_glyph as u32 + i as u32;
            // TODO: Speed up by accessing the value array directly
            let value = self.value::<T>(idx as u16);
            if let Ok(value) = value {
                if filter(value) {
                    set.insert(idx);
                }
            }
        }
    }
}
