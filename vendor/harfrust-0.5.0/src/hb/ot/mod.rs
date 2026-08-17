use super::buffer::GlyphPropsFlags;
use super::ot_layout::TableIndex;
use super::{common::TagExt, set_digest::hb_set_digest_t};
use crate::hb::hb_tag_t;
use crate::hb::ot_layout_gsubgpos::{BinaryCache, MappingCache};
use crate::hb::tables::TableRanges;
use alloc::vec::Vec;
use lookup::{LookupCache, LookupInfo};
use read_fonts::tables::layout::{ClassRangeRecord, RangeRecord};
use read_fonts::types::GlyphId16;
use read_fonts::{
    tables::{
        gdef::Gdef,
        gpos::{AnchorTable, DeviceOrVariationIndex, Gpos},
        gsub::{ClassDef, FeatureList, FeatureVariations, Gsub, ScriptList},
        layout::{Feature, LangSys, Script},
        varc::{Condition, CoverageTable},
        variations::{DeltaSetIndex, ItemVariationStore},
    },
    types::{BigEndian, F2Dot14, GlyphId, Offset32},
    FontData, FontRef, ReadError, ResolveOffset, TableProvider,
};

pub mod contextual;
pub mod gpos;
pub mod gsub;
pub mod lookup;

pub struct OtCache {
    pub gsub: LookupCache,
    pub gpos: LookupCache,
    pub gdef_glyph_props_cache: MappingCache,
    pub gdef_mark_set_digests: Vec<hb_set_digest_t>,
}

impl OtCache {
    pub fn new(font: &FontRef) -> Self {
        let gsub = font
            .gsub()
            .map(|t| LookupCache::new(&t))
            .unwrap_or_default();
        let gpos = font
            .gpos()
            .map(|t| LookupCache::new(&t))
            .unwrap_or_default();
        let mut gdef_mark_set_digests = Vec::new();
        if let Ok(gdef) = font.gdef() {
            if let Some(Ok(mark_sets)) = gdef.mark_glyph_sets_def() {
                gdef_mark_set_digests.extend(mark_sets.coverages().iter().map(|set| {
                    set.ok()
                        .map(|coverage| hb_set_digest_t::from_coverage(&coverage))
                        .unwrap_or_default()
                }));
            }
        }
        Self {
            gsub,
            gpos,
            gdef_glyph_props_cache: MappingCache::new(),
            gdef_mark_set_digests,
        }
    }
}

#[derive(Clone)]
pub struct GsubTable<'a> {
    pub table: Gsub<'a>,
    pub lookups: &'a LookupCache,
}

impl crate::hb::ot_layout::LayoutTable for GsubTable<'_> {
    const INDEX: TableIndex = TableIndex::GSUB;
    const IN_PLACE: bool = false;

    fn get_lookup(&self, index: u16) -> Option<&LookupInfo> {
        self.lookups.get(&self.table, index)
    }
}

#[derive(Clone)]
pub struct GposTable<'a> {
    pub table: Gpos<'a>,
    pub lookups: &'a LookupCache,
}

impl crate::hb::ot_layout::LayoutTable for GposTable<'_> {
    const INDEX: TableIndex = TableIndex::GPOS;
    const IN_PLACE: bool = true;

    fn get_lookup(&self, index: u16) -> Option<&LookupInfo> {
        self.lookups.get(&self.table, index)
    }
}

#[derive(Clone, Default)]
pub struct GdefTable<'a> {
    table: Option<Gdef<'a>>,
    classes: Option<ClassDef<'a>>,
    mark_classes: Option<ClassDef<'a>>,
    mark_sets: Option<(FontData<'a>, &'a [BigEndian<Offset32>])>,
}

impl<'a> GdefTable<'a> {
    fn new(font: &FontRef<'a>, table_ranges: &TableRanges) -> Self {
        if let Some(gdef) = table_ranges.gdef.resolve_table::<Gdef>(font) {
            let classes = gdef.glyph_class_def().transpose().ok().flatten();
            let mark_classes = gdef.mark_attach_class_def().transpose().ok().flatten();
            let mark_sets = gdef
                .mark_glyph_sets_def()
                .transpose()
                .ok()
                .flatten()
                .map(|sets| (sets.offset_data(), sets.coverage_offsets()));
            Self {
                table: Some(gdef),
                classes,
                mark_classes,
                mark_sets,
            }
        } else {
            Self::default()
        }
    }
}

#[derive(Clone)]
pub struct OtTables<'a> {
    pub gsub: Option<GsubTable<'a>>,
    pub gpos: Option<GposTable<'a>>,
    pub gdef: GdefTable<'a>,
    pub gdef_glyph_props_cache: &'a MappingCache,
    pub gdef_mark_set_digests: &'a [hb_set_digest_t],
    pub coords: &'a [F2Dot14],
    pub var_store: Option<ItemVariationStore<'a>>,
    pub feature_variations: [Option<u32>; 2],
}

impl<'a> OtTables<'a> {
    pub fn new(
        font: &FontRef<'a>,
        cache: &'a OtCache,
        table_offsets: &TableRanges,
        coords: &'a [F2Dot14],
        feature_variations: [Option<u32>; 2],
    ) -> Self {
        let gsub = table_offsets
            .gsub
            .resolve_table(font)
            .map(|table| GsubTable {
                table,
                lookups: &cache.gsub,
            });
        let gpos = table_offsets
            .gpos
            .resolve_table(font)
            .map(|table| GposTable {
                table,
                lookups: &cache.gpos,
            });
        let coords = if coords.iter().any(|coord| *coord != F2Dot14::ZERO) {
            coords
        } else {
            &[]
        };
        let gdef = GdefTable::new(font, table_offsets);
        let var_store = if !coords.is_empty() {
            gdef.table
                .as_ref()
                .and_then(|gdef| gdef.item_var_store().transpose().ok().flatten())
        } else {
            None
        };
        Self {
            gsub,
            gpos,
            gdef,
            gdef_glyph_props_cache: &cache.gdef_glyph_props_cache,
            gdef_mark_set_digests: &cache.gdef_mark_set_digests,
            var_store,
            coords,
            feature_variations,
        }
    }

    pub fn has_glyph_classes(&self) -> bool {
        self.gdef.classes.is_some()
    }

    pub fn glyph_class(&self, glyph_id: u32) -> u16 {
        self.gdef
            .classes
            .as_ref()
            .map_or(0, |class_def| class_def.get(glyph_id))
    }

    pub fn glyph_mark_attachment_class(&self, glyph_id: u32) -> u16 {
        self.gdef
            .mark_classes
            .as_ref()
            .map_or(0, |class_def| class_def.get(glyph_id))
    }

    pub(crate) fn glyph_props(&self, glyph: GlyphId) -> u16 {
        let glyph = glyph.to_u32();

        if let Some(props) = self.gdef_glyph_props_cache.get(glyph) {
            return props as u16;
        }

        let props = match self.glyph_class(glyph) {
            1 => GlyphPropsFlags::BASE_GLYPH.bits(),
            2 => GlyphPropsFlags::LIGATURE.bits(),
            3 => {
                let class = self.glyph_mark_attachment_class(glyph);
                (class << 8) | GlyphPropsFlags::MARK.bits()
            }
            _ => 0,
        };

        self.gdef_glyph_props_cache.set(glyph, props as u32);

        props
    }

    #[inline(never)]
    pub fn is_mark_glyph_gdef(&self, glyph_id: u32, set_index: u16) -> bool {
        self.gdef
            .mark_sets
            .as_ref()
            .and_then(|(data, offsets)| Some((data, offsets.get(set_index as usize)?.get())))
            .and_then(|(data, offset)| offset.resolve::<CoverageTable>(*data).ok())
            .is_some_and(|coverage| coverage.get(glyph_id).is_some())
    }

    #[inline(always)]
    pub fn is_mark_glyph(&self, glyph_id: u32, set_index: u16) -> bool {
        if self
            .gdef_mark_set_digests
            .get(set_index as usize)
            .is_some_and(|digest| digest.may_have(glyph_id))
        {
            self.is_mark_glyph_gdef(glyph_id, set_index)
        } else {
            false
        }
    }

    pub fn table_data(&self, table_index: TableIndex) -> Option<&'a [u8]> {
        if table_index == TableIndex::GSUB {
            self.gsub.as_ref().map(|t| t.table.offset_data().as_bytes())
        } else {
            self.gpos.as_ref().map(|t| t.table.offset_data().as_bytes())
        }
    }

    pub fn table_data_and_lookup(
        &self,
        table_index: TableIndex,
        lookup_index: u16,
    ) -> Option<(&'a [u8], &'a LookupInfo)> {
        if table_index == TableIndex::GSUB {
            let table = self.gsub.as_ref()?;
            Some((
                table.table.offset_data().as_bytes(),
                table.lookups.get(&table.table, lookup_index)?,
            ))
        } else {
            let table = self.gpos.as_ref()?;
            Some((
                table.table.offset_data().as_bytes(),
                table.lookups.get(&table.table, lookup_index)?,
            ))
        }
    }

    pub(super) fn resolve_anchor(&self, anchor: &AnchorTable) -> (i32, i32) {
        let mut x = anchor.x_coordinate() as i32;
        let mut y = anchor.y_coordinate() as i32;
        if let Some(vs) = self.var_store.as_ref() {
            let delta = |val: Option<Result<DeviceOrVariationIndex<'_>, ReadError>>| match val {
                Some(Ok(DeviceOrVariationIndex::VariationIndex(varix))) => vs
                    .compute_delta(
                        DeltaSetIndex {
                            outer: varix.delta_set_outer_index(),
                            inner: varix.delta_set_inner_index(),
                        },
                        self.coords,
                    )
                    .unwrap_or_default(),
                _ => 0,
            };
            x += delta(anchor.x_device());
            y += delta(anchor.y_device());
        }
        (x, y)
    }
}

pub enum LayoutTable<'a> {
    Gsub(Gsub<'a>),
    Gpos(Gpos<'a>),
}

impl<'a> LayoutTable<'a> {
    fn script_list(&self) -> Option<ScriptList<'a>> {
        match self {
            Self::Gsub(gsub) => gsub.script_list().ok(),
            Self::Gpos(gpos) => gpos.script_list().ok(),
        }
    }

    fn feature_list(&self) -> Option<FeatureList<'a>> {
        match self {
            Self::Gsub(gsub) => gsub.feature_list().ok(),
            Self::Gpos(gpos) => gpos.feature_list().ok(),
        }
    }

    fn feature_variations(&self) -> Option<FeatureVariations<'a>> {
        match self {
            Self::Gsub(gsub) => gsub.feature_variations(),
            Self::Gpos(gpos) => gpos.feature_variations(),
        }
        .transpose()
        .ok()
        .flatten()
    }

    fn script(&self, index: u16) -> Option<Script<'a>> {
        self.script_list()?
            .get(index)
            .ok()
            .map(|script| script.element)
    }

    fn langsys_index(&self, script_index: u16, tag: hb_tag_t) -> Option<u16> {
        let script = self.script(script_index)?;
        script.lang_sys_index_for_tag(tag)
    }

    fn langsys(&self, script_index: u16, langsys_index: Option<u16>) -> Option<LangSys<'a>> {
        let script = self.script(script_index)?;
        if let Some(index) = langsys_index {
            let record = script.lang_sys_records().get(index as usize)?;
            record.lang_sys(script.offset_data()).ok()
        } else {
            script.default_lang_sys().transpose().ok().flatten()
        }
    }

    pub(crate) fn feature(&self, index: u16) -> Option<Feature<'a>> {
        self.feature_list()?
            .get(index)
            .ok()
            .map(|feature| feature.element)
    }

    fn feature_tag(&self, index: u16) -> Option<hb_tag_t> {
        self.feature_list()?
            .get(index)
            .ok()
            .map(|feature| feature.tag)
    }

    pub(crate) fn feature_variation_index(&self, coords: &[F2Dot14]) -> Option<u32> {
        let feature_variations = self.feature_variations()?;
        for (index, rec) in feature_variations
            .feature_variation_records()
            .iter()
            .enumerate()
        {
            // If the ConditionSet offset is 0, this is treated as the
            // universal condition: all contexts are matched.
            if rec.condition_set_offset().is_null() {
                return Some(index as u32);
            }
            let Some(Ok(condition_set)) = rec.condition_set(feature_variations.offset_data())
            else {
                continue;
            };
            // Otherwise, all conditions must be satisfied.
            if condition_set
                .conditions()
                .iter()
                // .. except we ignore errors
                .filter_map(Result::ok)
                .all(|cond| match cond {
                    Condition::Format1AxisRange(format1) => {
                        let coord = coords
                            .get(format1.axis_index() as usize)
                            .copied()
                            .unwrap_or_default();
                        coord >= format1.filter_range_min_value()
                            && coord <= format1.filter_range_max_value()
                    }
                    _ => false,
                })
            {
                return Some(index as u32);
            }
        }
        None
    }

    pub(crate) fn feature_substitution(
        &self,
        variation_index: u32,
        feature_index: u16,
    ) -> Option<Feature<'a>> {
        let feature_variations = self.feature_variations()?;
        let record = feature_variations
            .feature_variation_records()
            .get(variation_index as usize)?;
        let subst_table = record
            .feature_table_substitution(feature_variations.offset_data())?
            .ok()?;
        let subst_records = subst_table.substitutions();
        match subst_records.binary_search_by_key(&feature_index, |subst| subst.feature_index()) {
            Ok(ix) => Some(
                subst_records
                    .get(ix)?
                    .alternate_feature(subst_table.offset_data())
                    .ok()?,
            ),
            _ => None,
        }
    }

    pub(crate) fn feature_index(&self, tag: hb_tag_t) -> Option<u16> {
        let list = self.feature_list()?;
        for (index, feature) in list.feature_records().iter().enumerate() {
            if feature.feature_tag() == tag {
                return Some(index as u16);
            }
        }
        None
    }

    pub(crate) fn lookup_count(&self) -> u16 {
        match self {
            Self::Gsub(gsub) => gsub
                .lookup_list()
                .map(|list| list.lookup_count())
                .unwrap_or_default(),
            Self::Gpos(gpos) => gpos
                .lookup_list()
                .map(|list| list.lookup_count())
                .unwrap_or_default(),
        }
    }

    // hb_ot_layout_table_select_script
    /// Returns true + index and tag of the first found script tag in the given GSUB or GPOS table
    /// or false + index and tag if falling back to a default script.
    pub(crate) fn select_script(&self, script_tags: &[hb_tag_t]) -> Option<(bool, u16, hb_tag_t)> {
        let selected = self.script_list()?.select(script_tags)?;
        Some((!selected.is_fallback, selected.index, selected.tag))
    }

    // hb_ot_layout_script_select_language
    /// Returns the index of the first found language tag in the given GSUB or GPOS table,
    /// underneath the specified script index.
    pub(crate) fn select_script_language(
        &self,
        script_index: u16,
        lang_tags: &[hb_tag_t],
    ) -> Option<u16> {
        for &tag in lang_tags {
            if let Some(index) = self.langsys_index(script_index, tag) {
                return Some(index);
            }
        }

        // try finding 'dflt'
        if let Some(index) = self.langsys_index(script_index, hb_tag_t::default_language()) {
            return Some(index);
        }

        None
    }

    // hb_ot_layout_language_get_required_feature
    /// Returns the index and tag of a required feature in the given GSUB or GPOS table,
    /// underneath the specified script and language.
    pub(crate) fn get_required_language_feature(
        &self,
        script_index: u16,
        lang_index: Option<u16>,
    ) -> Option<(u16, hb_tag_t)> {
        let sys = self.langsys(script_index, lang_index)?;
        let idx = sys.required_feature_index();
        if idx == 0xFFFF {
            return None;
        }
        let tag = self.feature_tag(idx)?;
        Some((idx, tag))
    }

    // hb_ot_layout_language_find_feature
    /// Returns the index of a given feature tag in the given GSUB or GPOS table,
    /// underneath the specified script and language.
    pub(crate) fn find_language_feature(
        &self,
        script_index: u16,
        lang_index: Option<u16>,
        feature_tag: hb_tag_t,
    ) -> Option<u16> {
        self.langsys(script_index, lang_index)?
            .feature_index_for_tag(&self.feature_list()?, feature_tag)
    }
}

fn coverage_index(coverage: Result<CoverageTable, ReadError>, gid: GlyphId) -> Option<u16> {
    coverage.ok().and_then(|coverage| coverage.get(gid))
}

fn coverage_index_cached(
    coverage: impl Fn(GlyphId) -> Option<u16>,
    gid: GlyphId,
    cache: &MappingCache,
) -> Option<u16> {
    if let Some(index) = cache.get(gid.into()) {
        if index == MappingCache::MAX_VALUE {
            None
        } else {
            Some(index as u16)
        }
    } else {
        let index = coverage(gid);
        if let Some(index) = index {
            if (index as u32) < MappingCache::MAX_VALUE {
                cache.set(gid.into(), index as u32);
            }
            Some(index)
        } else {
            cache.set(gid.into(), MappingCache::MAX_VALUE);
            None
        }
    }
}

fn coverage_binary_cached(
    coverage: impl Fn(GlyphId) -> Option<u16>,
    gid: GlyphId,
    cache: &BinaryCache,
) -> Option<bool> {
    if let Some(index) = cache.get(gid.into()) {
        if index == BinaryCache::MAX_VALUE {
            None
        } else {
            Some(true)
        }
    } else {
        let index = coverage(gid);
        if index.is_some() {
            cache.set(gid.into(), 0);
            Some(true)
        } else {
            cache.set(gid.into(), BinaryCache::MAX_VALUE);
            None
        }
    }
}

fn covered(coverage: Result<CoverageTable, ReadError>, gid: GlyphId) -> bool {
    coverage_index(coverage, gid).is_some()
}

fn glyph_class(class_def: Result<ClassDef, ReadError>, gid: GlyphId) -> u16 {
    class_def
        .map(|class_def| class_def.get(gid))
        .unwrap_or_default()
}

fn glyph_class_cached(
    class_def: impl Fn(GlyphId) -> u16,
    gid: GlyphId,
    cache: &MappingCache,
) -> u16 {
    if let Some(index) = cache.get(gid.into()) {
        index as u16
    } else {
        let index = class_def(gid);
        cache.set(gid.into(), index as u32);
        index
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct CoverageInfo {
    pub offset: u16,
    pub format: u16,
    pub count: u16,
}

impl CoverageInfo {
    pub fn new(parent_data: &FontData, offset: u16) -> Option<Self> {
        if offset == 0 {
            return None;
        }
        let format = parent_data.read_at::<u16>(offset as usize).ok()?;
        if format != 1 && format != 2 {
            return None;
        }
        let count = parent_data.read_at::<u16>(offset as usize + 2).ok()?;
        Some(Self {
            offset,
            format,
            count,
        })
    }

    pub fn index(&self, parent_data: &FontData, gid: GlyphId) -> Option<u16> {
        if self.offset == 0 {
            return None;
        }
        let gid = gid.to_u32();
        let data_offset = self.offset as usize + 4;
        let len = self.count as usize;
        if self.format == 1 {
            let glyphs = parent_data
                .read_array::<BigEndian<GlyphId16>>(data_offset..data_offset + len * 2)
                .ok()?;
            glyphs
                .binary_search_by_key(&gid, |g| g.get().to_u32())
                .ok()
                .map(|idx| idx as _)
        } else {
            use core::cmp::Ordering;
            let records = parent_data
                .read_array::<RangeRecord>(
                    data_offset..data_offset + len * size_of::<RangeRecord>(),
                )
                .ok()?;
            records
                .binary_search_by(|rec| {
                    if rec.end_glyph_id().to_u32() < gid {
                        Ordering::Less
                    } else if rec.start_glyph_id().to_u32() > gid {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                })
                .ok()
                .map(|idx| {
                    let rec = &records[idx];
                    (rec.start_coverage_index() as u32 + gid - rec.start_glyph_id().to_u32()) as u16
                })
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct ClassDefInfo {
    pub offset: u16,
    pub format: u16,
    // For format 1 only
    pub start_glyph_id: u16,
    pub count: u16,
}

impl ClassDefInfo {
    pub fn new(parent_data: &FontData, offset: u16) -> Option<Self> {
        if offset == 0 {
            return None;
        }
        let format = parent_data.read_at::<u16>(offset as usize).ok()?;
        if format != 1 && format != 2 {
            return None;
        }
        let (start_glyph_id, count) = if format == 1 {
            let start_glyph_id = parent_data.read_at::<u16>(offset as usize + 2).ok()?;
            let count = parent_data.read_at::<u16>(offset as usize + 4).ok()?;
            (start_glyph_id, count)
        } else if format == 2 {
            let count = parent_data.read_at::<u16>(offset as usize + 2).ok()?;
            (0, count)
        } else {
            return None;
        };
        Some(Self {
            offset,
            format,
            start_glyph_id,
            count,
        })
    }

    pub fn class(&self, parent_data: &FontData, gid: GlyphId) -> u16 {
        let offset = self.offset as usize;
        if offset == 0 {
            return 0;
        }
        let gid = gid.to_u32();
        if self.format == 1 {
            let Some(idx) = gid.checked_sub(self.start_glyph_id as u32) else {
                return 0;
            };
            if idx >= self.count as u32 {
                return 0;
            }
            parent_data
                .read_at::<u16>(offset + 6 + idx as usize * 2)
                .unwrap_or(0)
        } else {
            use core::cmp::Ordering;
            let start = offset + 4;
            let end = start + self.count as usize * size_of::<ClassRangeRecord>();
            let Ok(records) = parent_data.read_array::<ClassRangeRecord>(start..end) else {
                return 0;
            };
            records
                .binary_search_by(|rec| {
                    if rec.end_glyph_id().to_u32() < gid {
                        Ordering::Less
                    } else if rec.start_glyph_id().to_u32() > gid {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                })
                .ok()
                .map_or(0, |idx| records[idx].class())
        }
    }
}
