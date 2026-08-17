use super::internal::{at::*, *};
use super::{buffer::*, feature::*, Direction};
use crate::text::Script;

use alloc::vec::Vec;
use core::ops::Range;

pub type FeatureBit = u16;

#[derive(Copy, Clone, Default)]
pub struct FeatureMask {
    bits: [u64; 4],
}

impl FeatureMask {
    pub fn is_empty(&self) -> bool {
        self.bits.iter().all(|word| *word == 0)
    }

    pub fn set(&mut self, bit: u16) {
        let word = bit as usize / 64;
        let mask = 1 << (bit as u64 & 63);
        self.bits[word] |= mask;
    }

    pub fn clear(&mut self, bit: u16) {
        let word = bit as usize / 64;
        let mask = 1 << (bit as u64 & 63);
        self.bits[word] &= !mask;
    }

    pub fn test(&self, bit: u16) -> bool {
        let word = bit as usize / 64;
        let mask = 1 << (bit as u64 & 63);
        self.bits[word] & mask != 0
    }
}

impl core::ops::BitOr for FeatureMask {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        let mut result = FeatureMask::default();
        for ((r, a), b) in result.bits.iter_mut().zip(&self.bits).zip(&other.bits) {
            *r = *a | *b;
        }
        result
    }
}

impl core::ops::BitOrAssign for FeatureMask {
    fn bitor_assign(&mut self, other: Self) {
        for (a, b) in self.bits.iter_mut().zip(&other.bits) {
            *a |= *b;
        }
    }
}

/// Masks or bits for specific feature groups.
#[derive(Copy, Clone, Default)]
pub struct FeatureGroups {
    pub default: FeatureMask,
    pub reph: Option<FeatureBit>,
    pub pref: Option<FeatureBit>,
    pub stage1: FeatureMask,
    pub stage2: FeatureMask,
    pub basic: FeatureMask,
    pub position: FeatureMask,
    pub vert: FeatureMask,
    pub rtl: FeatureMask,
}

impl From<Option<FeatureBit>> for FeatureMask {
    fn from(bit: Option<u16>) -> Self {
        bit.map(|bit| {
            let mut mask = FeatureMask::default();
            mask.set(bit);
            mask
        })
        .unwrap_or_default()
    }
}

/// Offsets for a particular layout stage.
#[derive(Copy, Clone, Default)]
pub struct StageOffsets {
    pub base: u32,
    pub lang: u32,
    pub var: u32,
}

impl StageOffsets {
    pub fn new(
        b: &Bytes,
        base: u32,
        script: RawTag,
        lang: Option<RawTag>,
    ) -> Option<(Self, [RawTag; 2])> {
        let (lang, tags) = language_or_default_by_tags(b, base, script, lang)?;
        let var = feature_var_offset(b, base);
        Some((Self { base, lang, var }, tags))
    }
}

/// Maximum number of features that are allowed per stage.
const MAX_CACHED_FEATURES: usize = 256;

const MAX_NESTED_LOOKUPS: usize = 4;
const MAX_SEQUENCE: usize = 32;

/// Cache of features, lookups and subtables for a particular
/// script/language pair.
#[derive(Clone, Default)]
pub struct FeatureStore {
    pub sub_features: Vec<(RawTag, FeatureBit)>,
    pub pos_features: Vec<(RawTag, FeatureBit)>,
    pub lookups: Vec<LookupData>,
    pub subtables: Vec<SubtableData>,
    pub coverage: Vec<u16>,
    pub pos_start: usize,
    pub sub_count: usize,
    pub truncated: bool,
    pub groups: FeatureGroups,
}

impl FeatureStore {
    pub fn clear(&mut self) {
        self.sub_features.clear();
        self.pos_features.clear();
        self.lookups.clear();
        self.subtables.clear();
        self.coverage.clear();
        self.pos_start = 0;
        self.sub_count = 0;
        self.truncated = false;
        self.groups = FeatureGroups::default();
    }

    pub fn sub_bit(&self, feature: RawTag) -> Option<FeatureBit> {
        match self.sub_features.binary_search_by(|x| x.0.cmp(&feature)) {
            Ok(index) => Some(self.sub_features[index].1),
            _ => None,
        }
    }

    pub fn pos_bit(&self, feature: RawTag) -> Option<FeatureBit> {
        match self.pos_features.binary_search_by(|x| x.0.cmp(&feature)) {
            Ok(index) => Some(self.pos_features[index].1),
            _ => None,
        }
    }

    /// Returns new `basic` and `position` masks based on the
    /// specified custom features.
    pub fn custom_masks(
        &self,
        features: &[(RawTag, u16)],
        sub_args: &mut Vec<u16>,
        pos_args: &mut Vec<u16>,
        dir: Direction,
    ) -> (FeatureMask, FeatureMask) {
        let sub_count = self.sub_features.len();
        sub_args.clear();
        sub_args.resize(sub_count, 0);
        let pos_count = self.pos_features.len();
        pos_args.clear();
        pos_args.resize(pos_count, 0);
        let mut sub = self.groups.basic;
        if dir == Direction::RightToLeft {
            sub |= self.groups.rtl;
        }
        let sub = Self::custom_masks_for_stage(
            &self.sub_features,
            features,
            self.groups.basic,
            sub_args.as_mut_slice(),
        );
        let pos = Self::custom_masks_for_stage(
            &self.pos_features,
            features,
            self.groups.position,
            pos_args.as_mut_slice(),
        );
        (sub, pos)
    }

    fn custom_masks_for_stage(
        stage_features: &[(RawTag, FeatureBit)],
        requested_features: &[(RawTag, u16)],
        mut mask: FeatureMask,
        args: &mut [u16],
    ) -> FeatureMask {
        for req_feature in requested_features {
            if let Ok(index) = stage_features.binary_search_by(|x| x.0.cmp(&req_feature.0)) {
                let stage_feature = stage_features[index];
                let bit_ix = stage_feature.1;
                let arg = req_feature.1;
                args[bit_ix as usize] = arg;
                if arg != 0 {
                    mask.set(bit_ix);
                } else {
                    mask.clear(bit_ix);
                }
            }
        }
        mask
    }

    pub fn groups(&self, script: Script) -> FeatureGroups {
        let mut g = FeatureGroups::default();
        feature_masks(self, Some(&mut g.vert), Some(&mut g.position), &[VRT2]);
        feature_masks(self, Some(&mut g.rtl), Some(&mut g.position), &[RTLM]);
        if g.vert.is_empty() {
            feature_masks(self, Some(&mut g.vert), Some(&mut g.position), &[VERT]);
        }
        if script.is_complex() {
            match script {
                Script::Myanmar => {
                    feature_masks(
                        self,
                        Some(&mut g.default),
                        Some(&mut g.position),
                        &[CALT, CCMP, LOCL, RVRN],
                    );
                    g.reph = self.sub_bit(RPHF);
                    g.pref = self.sub_bit(PREF);
                    feature_masks(
                        self,
                        Some(&mut g.stage1),
                        Some(&mut g.position),
                        &[BLWF, PSTF],
                    );
                    feature_masks(
                        self,
                        Some(&mut g.stage2),
                        Some(&mut g.position),
                        &[PRES, ABVS, BLWS, PSTS],
                    );
                    feature_masks(
                        self,
                        Some(&mut g.basic),
                        Some(&mut g.position),
                        &[DIST, KERN, MARK, MKMK],
                    );
                }
                _ => {
                    feature_masks(
                        self,
                        Some(&mut g.default),
                        Some(&mut g.position),
                        &[AKHN, CALT, CCMP, LOCL, NUKT, RVRN],
                    );
                    g.reph = self.sub_bit(RPHF);
                    g.pref = self.sub_bit(PREF);
                    feature_masks(
                        self,
                        Some(&mut g.stage1),
                        Some(&mut g.position),
                        &[ABVF, BLWF, CJCT, HALF, PSTF, RKRF, VATU],
                    );
                    if script.is_joined() {
                        feature_masks(
                            self,
                            Some(&mut g.stage2),
                            Some(&mut g.position),
                            &[FIN2, FIN3, FINA, INIT, ISOL, MED2, MEDI],
                        );
                    }
                    feature_masks(
                        self,
                        Some(&mut g.basic),
                        Some(&mut g.position),
                        &[ABVS, BLWS, CALT, CLIG, HALN, LIGA, PRES, PSTS, RCLT, RLIG],
                    );
                    feature_masks(
                        self,
                        Some(&mut g.basic),
                        Some(&mut g.position),
                        &[ABVM, BLWM, CURS, DIST, KERN, MARK, MKMK],
                    );
                }
            }
        } else {
            match script {
                Script::Hangul => {
                    feature_masks(
                        self,
                        Some(&mut g.basic),
                        Some(&mut g.position),
                        &[CCMP, LJMO, RVRN, TJMO, VJMO],
                    );
                }
                _ => {
                    if script.is_joined() {
                        feature_masks(
                            self,
                            Some(&mut g.basic),
                            Some(&mut g.position),
                            &[
                                CALT, CCMP, CLIG, FIN2, FIN3, FINA, INIT, ISOL, LIGA, LOCL, MED2,
                                MEDI, MSET, RLIG, RVRN,
                            ],
                        );
                    } else {
                        feature_masks(
                            self,
                            Some(&mut g.basic),
                            Some(&mut g.position),
                            &[CALT, CCMP, CLIG, LIGA, LOCL, RVRN],
                        );
                    }
                    feature_masks(
                        self,
                        Some(&mut g.basic),
                        Some(&mut g.position),
                        &[CURS, DIST, KERN, MARK, MKMK],
                    );
                }
            }
        }
        g
    }

    fn test(&self, key: u32, glyph_id: u16) -> bool {
        if key == !0 {
            return true;
        }
        let cache = &self.coverage;
        let base = key as usize;
        let first = cache[base];
        if glyph_id >= first && glyph_id <= cache[base + 1] {
            let bit = glyph_id - first;
            let idx = base + 2 + bit as usize / 16;
            cache[idx] & (1 << (bit & 15)) != 0
        } else {
            false
        }
    }

    // pub fn memory_usage(&self) -> usize {
    //     use crate::util::mem::vec_usage as v;
    //     v(&self.features).1 + v(&self.lookups).1 + v(&self.subtables).1 + v(&self.coverage).1
    // }
}

fn feature_masks(
    store: &FeatureStore,
    sub_mask: Option<&mut FeatureMask>,
    pos_mask: Option<&mut FeatureMask>,
    features: &[RawTag],
) {
    if let Some(sub_mask) = sub_mask {
        for feature in features {
            if let Some(bit) = store.sub_bit(*feature) {
                sub_mask.set(bit);
            }
        }
    }
    if let Some(pos_mask) = pos_mask {
        for feature in features {
            if let Some(bit) = store.pos_bit(*feature) {
                pos_mask.set(bit);
            }
        }
    }
}

/// Builder for a feature cache.
#[derive(Default)]
pub struct FeatureStoreBuilder {
    indices: Vec<(u16, FeatureBit, u8)>,
    coverage: CoverageBuilder,
    next_bit: FeatureBit,
}

impl FeatureStoreBuilder {
    pub fn build(
        &mut self,
        cache: &mut FeatureStore,
        data: &[u8],
        coords: &[i16],
        gdef: &Gdef,
        gsub: &StageOffsets,
        gpos: &StageOffsets,
    ) {
        let b = Bytes::new(data);
        cache.clear();
        if gsub.base != 0 {
            self.build_stage(cache, &b, coords, gdef, gsub, 0);
            cache.sub_features.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        }
        cache.sub_count = cache.sub_features.len();
        cache.pos_start = cache.lookups.len();
        if gpos.base != 0 {
            self.build_stage(cache, &b, coords, gdef, gpos, 1);
            cache.pos_features.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        }
    }

    fn build_stage(
        &mut self,
        cache: &mut FeatureStore,
        b: &Bytes,
        coords: &[i16],
        gdef: &Gdef,
        offsets: &StageOffsets,
        stage: u8,
    ) -> Option<()> {
        self.next_bit = 0;
        self.indices.clear();
        let gdef = if gdef.ok() { Some(gdef) } else { None };
        let base = offsets.base;
        let lbase = offsets.lang as usize;
        let list_base = b.read_u16(base as usize + 8)? as u32 + base;
        let vars = FeatureSubsts::new(b, offsets.var, coords);
        let fbase = b.read_u16(base as usize + 6)? as usize + base as usize;
        let count = b.read_u16(lbase + 4)? as usize;
        let actual_count = count.min(MAX_CACHED_FEATURES);
        if actual_count < count {
            cache.truncated = true;
        }
        let features = if stage == 0 {
            &mut cache.sub_features
        } else {
            &mut cache.pos_features
        };
        for i in 0..actual_count {
            let findex = b.read_u16(lbase + 6 + i * 2)? as usize;
            let rec = fbase + 2 + findex * 6;
            let ftag = b.read_u32(rec)?;
            let fbit = self.next_bit;
            self.next_bit += 1;
            let mask = if stage == 0 {
                match ftag {
                    // joining masks
                    ISOL => ISOL_MASK,
                    INIT => INIT_MASK,
                    MEDI => MEDI_MASK,
                    FINA => FINA_MASK,
                    MED2 => MED2_MASK,
                    FIN2 => FIN2_MASK,
                    FIN3 => FIN3_MASK,
                    // jamo masks
                    LJMO => LJMO_MASK,
                    VJMO => VJMO_MASK,
                    TJMO => TJMO_MASK,
                    _ => 0,
                }
            } else {
                0
            };
            features.push((ftag, fbit));
            let foffset = if let Some(v) = vars {
                if let Some(offset) = v.apply(b, findex as u16) {
                    offset
                } else {
                    fbase + b.read::<u16>(rec + 4)? as usize
                }
            } else {
                fbase + b.read::<u16>(rec + 4)? as usize
            };
            let lcount = b.read_u16(foffset + 2)? as usize;
            for i in 0..lcount {
                let lookup_index = b.read_u16(foffset + 4 + i * 2)?;
                self.indices.push((lookup_index, fbit, mask));
            }
        }
        self.indices.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        //self.indices.dedup_by(|a, b| a.0 == b.0);
        let mut last_index = None;
        for (index, feature, mask) in &self.indices {
            if last_index == Some(*index) {
                let mut lookup = *cache.lookups.last().unwrap();
                lookup.feature = *feature;
                cache.lookups.push(lookup);
                continue;
            }
            if let Some(ref mut lookup) = lookup_data(b, stage, list_base, *index, *mask, gdef) {
                let start = cache.subtables.len();
                self.coverage.begin();
                if Self::collect_subtables(b, cache, &mut self.coverage, lookup) == Some(true) {
                    lookup.coverage = self.coverage.finish(&mut cache.coverage);
                    lookup.feature = *feature;
                    cache.lookups.push(*lookup);
                    last_index = Some(*index);
                } else {
                    cache.subtables.truncate(start);
                }
            }
        }
        Some(())
    }

    fn collect_subtables(
        b: &Bytes,
        cache: &mut FeatureStore,
        coverage: &mut CoverageBuilder,
        lookup: &mut LookupData,
    ) -> Option<bool> {
        let start = cache.subtables.len();
        if start >= u16::MAX as usize {
            return None;
        }
        lookup.subtables.0 = start as u16;
        let base = lookup.offset as usize;
        let subtable_base = base + 6;
        let count = lookup.count as usize;
        let ext = lookup.is_ext;
        let kind = lookup.kind;
        for i in 0..count {
            let mut subtable = base + b.read::<u16>(subtable_base + i * 2)? as usize;
            if ext {
                subtable = subtable + b.read::<u32>(subtable + 4)? as usize;
            }
            let fmt = b.read::<u16>(subtable)?;
            if let Some(ref s) = subtable_data(b, subtable as u32, kind, fmt) {
                coverage.add_coverage(b, s.offset as usize + s.coverage as usize)?;
                cache.subtables.push(*s);
            }
        }
        let end = cache.subtables.len();
        if end >= u16::MAX as usize {
            return None;
        }
        lookup.subtables.1 = end as u16;
        Some(lookup.subtables.1 > lookup.subtables.0)
    }
}

#[derive(Default)]
struct CoverageBuilder {
    coverage: BitSet,
    min: u16,
    max: u16,
}

impl CoverageBuilder {
    fn begin(&mut self) {
        self.coverage.clear();
        self.min = u16::MAX;
        self.max = 0;
    }

    fn add_coverage(&mut self, b: &Bytes, base: usize) -> Option<()> {
        let fmt = b.read::<u16>(base)?;
        let len = b.read::<u16>(base + 2)? as usize;
        let arr = base + 4;
        if fmt == 1 {
            for g in b.read_array::<u16>(arr, len)?.iter() {
                self.add(g);
            }
        } else if fmt == 2 {
            for i in 0..len {
                let rec = arr + i * 6;
                let first = b.read::<u16>(rec)?;
                let last = b.read::<u16>(rec + 2)?;
                for g in first..=last {
                    self.add(g);
                }
            }
        } else {
            return None;
        }
        Some(())
    }

    fn finish(&self, coverage: &mut Vec<u16>) -> u32 {
        let key = coverage.len() as u32;
        coverage.push(self.min);
        coverage.push(self.max);
        let bit_base = coverage.len();
        let range_len = (self.max - self.min) as usize + 1;
        coverage.resize(coverage.len() + (range_len + 15) / 16, 0);
        for g in &self.coverage.list {
            let bit = g - self.min;
            let idx = bit_base + bit as usize / 16;
            coverage[idx] |= 1 << (bit & 15);
        }
        key
    }

    #[inline]
    fn add(&mut self, glyph_id: u16) {
        if self.coverage.insert(glyph_id) {
            self.min = glyph_id.min(self.min);
            self.max = glyph_id.max(self.max);
        }
    }
}

#[derive(Default)]
pub struct BitSet {
    list: Vec<u16>,
    bits: Vec<u64>,
}

impl BitSet {
    pub fn clear(&mut self) {
        self.list.clear();
        for b in &mut self.bits {
            *b = 0;
        }
    }

    pub fn insert(&mut self, value: u16) -> bool {
        let value = value as usize;
        let index = value / 64;
        let shift = value & 63;
        let bit = 1u64 << shift;
        if index >= self.bits.len() {
            self.bits.resize(index + 8, 0);
            self.bits[index] |= bit;
            self.list.push(value as u16);
            true
        } else {
            let word_ptr = &mut self.bits[index];
            if *word_ptr & bit != 0 {
                false
            } else {
                *word_ptr |= bit;
                self.list.push(value as u16);
                true
            }
        }
    }
}

pub fn apply(
    stage: u8,
    data: &Bytes,
    gsubgpos: u32,
    coords: &[i16],
    gdef: &Gdef,
    storage: &mut Storage,
    cache: &FeatureStore,
    feature_mask: FeatureMask,
    buffer: &mut Buffer,
    buffer_range: Option<Range<usize>>,
) -> Option<bool> {
    if gsubgpos == 0 || feature_mask.is_empty() {
        return Some(false);
    }
    let buffer_range = if let Some(range) = buffer_range {
        range
    } else {
        0..buffer.len()
    };
    let mut acx = ApplyContext::new(
        stage,
        data,
        gsubgpos,
        gdef,
        coords,
        cache,
        storage,
        buffer,
        buffer_range.clone(),
    );
    let lookups = if stage == 0 {
        &cache.lookups[..cache.pos_start]
    } else {
        &cache.lookups[cache.pos_start..]
    };
    let mut applied = false;
    for lookup in lookups {
        if !feature_mask.test(lookup.feature) {
            continue;
        }
        let table_range = lookup.subtables.0 as usize..lookup.subtables.1 as usize;
        let tables = cache.subtables.get(table_range)?;
        if let Some(true) = acx.apply(lookup, tables, buffer_range.start, None, 0) {
            applied = true;
        }
    }
    Some(applied)
}

#[derive(Copy, Clone, Default)]
struct LookupState {
    skip_state: SkipState,
    cur: usize,
    end: usize,
}

struct ApplyContext<'a, 'b, 'c> {
    stage: u8,
    data: &'a Bytes<'a>,
    gsubgpos: u32,
    defs: &'a Gdef<'a>,
    coords: &'a [i16],
    enable_var: bool,
    cache: &'a FeatureStore,
    storage: &'b mut Storage,
    top: u8,
    arg: u16,
    start: usize,
    end: usize,
    s: LookupState,
    buf: &'c mut Buffer,
}

impl<'a, 'b, 'c> ApplyContext<'a, 'b, 'c> {
    pub fn new(
        stage: u8,
        data: &'a Bytes<'a>,
        gsubgpos: u32,
        defs: &'a Gdef<'a>,
        coords: &'a [i16],
        cache: &'a FeatureStore,
        storage: &'b mut Storage,
        buffer: &'c mut Buffer,
        range: Range<usize>,
    ) -> Self {
        Self {
            stage,
            data,
            gsubgpos,
            defs,
            coords,
            enable_var: defs.has_var_store() && !coords.is_empty(),
            cache,
            storage,
            top: 0,
            arg: 0,
            start: range.start,
            end: range.end,
            s: LookupState::default(),
            buf: buffer,
        }
    }

    fn apply_skip_state(&mut self) {
        if self.s.skip_state == self.buf.skip_state {
            return;
        }
        self.buf.skip_state = self.s.skip_state;
        self.update_glyphs_skip(None);
    }

    fn update_glyphs_skip(&mut self, range: Option<Range<usize>>) {
        let range = range.unwrap_or(0..self.buf.glyphs.len());
        let ss = &self.s.skip_state;
        let mask = ss.mask;
        if ss.mark_check != 0 {
            if ss.mark_set != 0 {
                for g in self.buf.glyphs[range].iter_mut() {
                    g.skip = (ss.flags & (1 << g.class) != 0) || (g.mask & mask != mask);
                    if !g.skip && g.class == 3 {
                        g.skip = self.defs.mark_set_coverage(ss.mark_set, g.id).is_none();
                    }
                }
            } else {
                for g in self.buf.glyphs[range].iter_mut() {
                    g.skip = (ss.flags & (1 << g.class) != 0) || (g.mask & mask != mask);
                    if !g.skip && g.class == 3 {
                        g.skip = g.mark_type != ss.mark_class;
                    }
                }
            }
        } else if mask != 0 {
            for g in self.buf.glyphs[range].iter_mut() {
                g.skip = (ss.flags & (1 << g.class) != 0) || (g.mask & mask != mask);
            }
        } else {
            for g in self.buf.glyphs[range].iter_mut() {
                g.skip = ss.flags & (1 << g.class) != 0;
            }
        }
    }

    fn update_glyphs(&mut self, start: usize, end: usize) {
        if self.defs.has_mark_classes() {
            for g in &mut self.buf.glyphs[start..end] {
                let class = self.defs.class(g.id) as u8;
                g.class = class;
                g.mark_type = if class == 3 {
                    self.defs.mark_class(g.id) as u8
                } else {
                    0
                };
            }
        } else {
            for g in &mut self.buf.glyphs[start..end] {
                g.class = self.defs.class(g.id) as u8;
            }
        }
        self.update_glyphs_skip(Some(start..end));
    }

    fn update_glyph(&mut self, index: usize) {
        let ss = &self.s.skip_state;
        let mask = ss.mask;
        let g = &mut self.buf.glyphs[index];
        let class = self.defs.class(g.id) as u8;
        g.class = class;
        g.skip = (ss.flags & (1 << class) != 0) || (g.mask & mask != mask);
        if class == 3 {
            g.mark_type = self.defs.mark_class(g.id) as u8;
            if ss.mark_check != 0 && !g.skip {
                if ss.mark_set != 0 {
                    g.skip = self.defs.mark_set_coverage(ss.mark_set, g.id).is_none();
                } else {
                    g.skip = g.mark_type != ss.mark_class;
                }
            }
        } else {
            g.mark_type = 0;
        }
    }

    #[inline(always)]
    fn ignored(&self, index: usize) -> bool {
        self.buf.glyphs[index].skip
    }

    fn next(&self, index: usize) -> Option<usize> {
        ((index + 1)..self.s.end).find(|&i| !self.ignored(i))
    }

    fn previous(&self, index: usize) -> Option<usize> {
        if index > self.start {
            for i in (self.start..=(index - 1)).rev() {
                if !self.ignored(i) {
                    return Some(i);
                }
            }
        }
        None
    }

    fn previous_base(&self, index: usize) -> Option<usize> {
        if index > self.start {
            for i in (self.start..=(index - 1)).rev() {
                if !self.ignored(i) {
                    let class = self.buf.glyphs[i].class;
                    if class != 3 {
                        return Some(i);
                    }
                }
            }
        }
        None
    }

    fn move_first(&mut self) -> bool {
        while self.s.cur < self.s.end {
            if !self.buf.glyphs[self.s.cur].skip {
                break;
            }
            self.s.cur += 1;
        }
        self.s.cur < self.s.end
    }

    fn move_last(&mut self) -> bool {
        if self.s.end == 0 {
            return false;
        }
        self.s.cur = self.s.end - 1;
        loop {
            if !self.ignored(self.s.cur) {
                break;
            }
            if self.s.cur == 0 {
                return false;
            }
            self.s.cur -= 1;
        }
        true
    }

    fn move_next(&mut self) -> bool {
        self.s.cur += 1;
        while self.s.cur < self.s.end {
            if !self.buf.glyphs[self.s.cur].skip {
                break;
            }
            self.s.cur += 1;
        }
        self.s.cur < self.s.end
    }

    fn _move_previous(&mut self) -> bool {
        if self.s.cur == self.start {
            return false;
        }
        for i in (self.start..=(self.s.cur - 1)).rev() {
            if !self.ignored(i) {
                self.s.cur = i;
                return true;
            }
        }
        false
    }

    fn move_to(&mut self, index: usize) -> bool {
        if !self.move_first() {
            return false;
        }
        for _ in 0..index {
            if !self.move_next() {
                return false;
            }
        }
        true
    }

    fn collect_sequence(&mut self, len: usize) -> bool {
        let mut collected = 0usize;
        let avail = self.s.end - self.s.cur;
        if avail < (len + 1) {
            return false;
        }
        let mut i = self.s.cur + 1;
        for g in &self.buf.glyphs[self.s.cur + 1..self.s.end] {
            if !g.skip {
                self.storage.indices[collected] = i;
                self.storage.ids[collected] = g.id;
                collected += 1;
                if collected == len {
                    return true;
                }
            }
            i += 1;
        }
        false
    }

    fn extend(&mut self, count: usize) {
        self.end += count;
        self.s.end += count;
        self.s.cur += count;
        for i in 0..self.top as usize {
            self.storage.stack[i].end += count;
            self.storage.stack[i].cur += count;
        }
    }

    fn match_backtrack<F>(&self, start: usize, len: usize, pred: F) -> Option<bool>
    where
        F: Fn(usize, u16) -> bool,
    {
        let mut idx = start;
        for i in 0..len {
            idx = self.previous(idx)?;
            if !pred(i, self.buf.glyphs[idx].id) {
                return None;
            }
        }
        Some(true)
    }

    fn match_sequence<F>(&self, start: usize, len: usize, pred: F) -> Option<usize>
    where
        F: Fn(usize, u16) -> bool,
    {
        let mut idx = start;
        for i in 0..len {
            idx = self.next(idx)?;
            if !pred(i, self.buf.glyphs[idx].id) {
                return None;
            }
        }
        Some(idx)
    }
}

impl<'a, 'b, 'c> ApplyContext<'a, 'b, 'c> {
    #[inline(never)]
    pub fn apply(
        &mut self,
        lookup: &LookupData,
        subtables: &[SubtableData],
        cur: usize,
        end: Option<usize>,
        first: usize,
    ) -> Option<bool> {
        let feature_index = lookup.feature as usize;
        self.arg = if lookup.stage == 0 {
            self.buf.sub_args[feature_index]
        } else {
            self.buf.pos_args[feature_index]
        };
        let b = self.data;
        self.s.skip_state = SkipState {
            flags: lookup.ignored,
            mask: lookup.mask,
            mark_check: lookup.mark_check,
            mark_class: lookup.mark_class,
            mark_set: lookup.mark_set,
        };
        self.s.cur = cur;
        self.s.end = end.unwrap_or(self.end);
        self.apply_skip_state();
        let mut applied = false;
        if lookup.kind == LookupKind::RevChainContext {
            if !self.move_last() {
                return Some(false);
            }
            loop {
                let i = self.s.cur;
                let g = self.buf.glyphs.get(i)?;
                if !g.skip {
                    let id = g.id;
                    if self.cache.test(lookup.coverage, id) {
                        for s in subtables {
                            if let Some(index) = s.coverage(b, id) {
                                if self.apply_subtable(b, s, index as usize, i, id) == Some(true) {
                                    applied = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if self.s.cur == 0 {
                    break;
                }
                self.s.cur -= 1;
            }
        } else {
            if !self.move_to(first) {
                return Some(false);
            }
            while self.s.cur < self.s.end {
                let i = self.s.cur;
                let g = self.buf.glyphs.get(i)?;
                if !g.skip {
                    let id = g.id;
                    if self.cache.test(lookup.coverage, id) {
                        for s in subtables {
                            if let Some(index) = s.coverage(b, id) {
                                if self.apply_subtable(b, s, index as usize, i, id) == Some(true) {
                                    applied = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                self.s.cur += 1;
            }
        }
        Some(applied)
    }

    #[inline(never)]
    fn apply_subtable(
        &mut self,
        b: &'a Bytes<'a>,
        subtable: &SubtableData,
        index: usize,
        cur: usize,
        g: u16,
    ) -> Option<bool> {
        use SubtableKind::*;
        let kind = subtable.kind;
        let base = subtable.offset as usize;
        // if TRACE {
        //     for _ in 0..self.top {
        //         print!("    ");
        //     }
        //     println!(
        //         "{:?} offset: {}, cur: {}, gid: {}",
        //         subtable.kind, base, cur, g
        //     );
        // }
        match kind {
            SingleSub1 => {
                let delta = b.read::<i16>(base + 4)? as i32;
                let subst = (g as i32 + delta) as u16;
                self.buf.substitute(cur, subst);
                self.update_glyph(cur);
                return Some(true);
            }
            SingleSub2 => {
                let arr = base + 6;
                let subst = b.read::<u16>(arr + index * 2)?;
                self.buf.substitute(cur, subst);
                self.update_glyph(cur);
                return Some(true);
            }
            MultiSub1 => {
                let seqbase = base + b.read::<u16>(base + 6 + index * 2)? as usize;
                let seqlen = b.read::<u16>(seqbase)? as usize;
                if seqlen > MAX_SEQUENCE {
                    return Some(false);
                }
                let seqarr = seqbase + 2;
                for i in 0..seqlen {
                    let subst = b.read::<u16>(seqarr + i * 2)?;
                    self.storage.ids[i] = subst;
                }
                self.buf
                    .substitute_multiple(cur, &self.storage.ids[0..seqlen]);
                self.update_glyphs(cur, cur + seqlen);
                self.extend(seqlen - 1);
                return Some(true);
            }
            AltSub1 => {
                let offset = b.read::<u16>(base + 6 + index * 2)? as usize;
                if offset == 0 {
                    return Some(false);
                }
                let arg = self.arg as usize;
                let setbase = base + offset;
                let count = b.read::<u16>(setbase)? as usize;
                if arg >= count {
                    return Some(false);
                }
                let subst = b.read::<u16>(setbase + 2 + arg * 2)?;
                self.buf.substitute(cur, subst);
                self.update_glyph(cur);
                return Some(true);
            }
            LigSub1 => {
                let setbase = base + b.read::<u16>(base + 6 + index * 2)? as usize;
                let ligcount = b.read::<u16>(setbase)? as usize;
                let mut seqlen = 0usize;
                for i in 0..ligcount {
                    let ligbase = setbase + b.read::<u16>(setbase + 2 + i * 2)? as usize;
                    let mut compcount = b.read::<u16>(ligbase + 2)? as usize;
                    if compcount == 0 {
                        continue;
                    }
                    compcount -= 1;
                    if compcount >= MAX_SEQUENCE {
                        continue;
                    }
                    let arr = ligbase + 4;
                    if seqlen < compcount {
                        if !self.collect_sequence(compcount) {
                            continue;
                        }
                        seqlen = compcount;
                    }
                    let components = b.read_array::<u16>(arr, compcount)?;
                    let mut matched = true;
                    for (a, b) in components.iter().zip(&self.storage.ids) {
                        if a != *b {
                            matched = false;
                            break;
                        }
                    }
                    if !matched {
                        continue;
                    }
                    let glyph = b.read::<u16>(ligbase)?;
                    self.buf
                        .substitute_ligature(cur, glyph, &self.storage.indices[0..compcount]);
                    self.update_glyph(cur);
                    return Some(true);
                }
            }
            SingleAdj1 => {
                let mut pos = [0f32; 4];
                self.value_record(base, base + 6, b.read::<u16>(base + 4)?, &mut pos)?;
                self.buf.position(cur, pos[0], pos[1], pos[2], pos[3]);
                return Some(true);
            }
            SingleAdj2 => {
                let vf = b.read::<u16>(base + 4)?;
                let len = vf.count_ones() as usize * 2;
                let mut pos = [0f32; 4];
                self.value_record(base, base + 8 + index * len, vf, &mut pos)?;
                self.buf.position(cur, pos[0], pos[1], pos[2], pos[3]);
                return Some(true);
            }
            PairAdj1 => {
                let next = self.next(cur)?;
                let g2 = self.buf.glyphs[next].id;
                let vf1 = b.read::<u16>(base + 4)?;
                let vf2 = b.read::<u16>(base + 6)?;
                let len1 = vf1.count_ones() as usize * 2;
                let step = len1 + vf2.count_ones() as usize * 2 + 2;
                let setbase = base + b.read::<u16>(base + 10 + index * 2)? as usize;
                let count = b.read::<u16>(setbase)? as usize;
                let vbase = setbase + 2;
                let mut l = 0;
                let mut h = count;
                while l < h {
                    use core::cmp::Ordering::*;
                    let i = (l + h) / 2;
                    let v = vbase + i * step;
                    let gv = b.read::<u16>(v)?;
                    match g2.cmp(&gv) {
                        Greater => l = i + 1,
                        Less => h = i,
                        Equal => {
                            if vf1 != 0 {
                                let mut pos = [0f32; 4];
                                self.value_record(setbase, v + 2, vf1, &mut pos)?;
                                self.buf.position(cur, pos[0], pos[1], pos[2], pos[3]);
                            }
                            if vf2 != 0 {
                                let mut pos = [0f32; 4];
                                self.value_record(setbase, v + 2 + len1, vf2, &mut pos)?;
                                self.buf.position(next, pos[0], pos[1], pos[2], pos[3]);
                            }
                            return Some(true);
                        }
                    }
                }
            }
            PairAdj2 => {
                let next = self.next(cur)?;
                let g2 = self.buf.glyphs[next].id;
                let vf1 = b.read::<u16>(base + 4)?;
                let vf2 = b.read::<u16>(base + 6)?;
                let len1 = vf1.count_ones() as usize * 2;
                let step = len1 + vf2.count_ones() as usize * 2;
                let class1 = self.class(base + b.read::<u16>(base + 8)? as usize, g) as usize;
                let class2 = self.class(base + b.read::<u16>(base + 10)? as usize, g2) as usize;
                let class2_count = b.read::<u16>(base + 14)? as usize;
                let v = base + 16 + (class1 * step * class2_count) + (class2 * step);
                if vf1 != 0 {
                    let mut pos = [0f32; 4];
                    self.value_record(base, v, vf1, &mut pos)?;
                    self.buf.position(cur, pos[0], pos[1], pos[2], pos[3]);
                }
                if vf2 != 0 {
                    let mut pos = [0f32; 4];
                    self.value_record(base, v + len1, vf2, &mut pos)?;
                    self.buf.position(next, pos[0], pos[1], pos[2], pos[3]);
                }
                return Some(true);
            }
            Cursive1 => {
                let next = self.next(cur)?;
                if next - cur > 255 {
                    return Some(false);
                }
                let g2 = self.buf.glyphs[next].id;
                let index2 = subtable.coverage(b, g2)? as usize;
                let recbase = base + 6;
                let mut exit_offset = b.read::<u16>(recbase + index * 4 + 2)? as usize;
                let mut entry_offset = b.read::<u16>(recbase + index2 * 4)? as usize;
                if exit_offset == 0 || entry_offset == 0 {
                    return Some(false);
                }
                exit_offset += base;
                entry_offset += base;
                let exit = self.anchor(exit_offset)?;
                let entry = self.anchor(entry_offset)?;
                let dx = entry.0 - exit.0;
                let dy = entry.1 - exit.1;
                self.buf.position_cursive(cur, next, dx, dy);
                return Some(true);
            }
            MarkToBase1 | MarkToMark1 => {
                let prev = if kind == MarkToBase1 {
                    self.previous_base(cur)?
                } else {
                    self.previous(cur)?
                };
                let diff = cur - prev;
                if diff > 255 {
                    return Some(false);
                }
                let g2 = self.buf.glyphs[prev].id;
                let index2 = self.coverage(base + b.read::<u16>(base + 4)? as usize, g2)? as usize;
                let (mark_class, mark_anchor) = {
                    let markbase = base + b.read::<u16>(base + 8)? as usize;
                    let a = self.mark_anchor(markbase, index as u16)?;
                    (a.0 as usize, a.1)
                };
                let base_anchor = {
                    let class_count = b.read::<u16>(base + 6)? as usize;
                    let basebase = base + b.read::<u16>(base + 10)? as usize;
                    let count = b.read::<u16>(basebase)? as usize * class_count;
                    let index = class_count * index2 + mark_class;
                    if index >= count {
                        return Some(false);
                    }
                    let abase = basebase + b.read::<u16>(basebase + 2 + index * 2)? as usize;
                    self.anchor(abase)?
                };
                let dx = base_anchor.0 - mark_anchor.0;
                let dy = base_anchor.1 - mark_anchor.1;
                self.buf.position_mark(cur, prev, dx, dy);
                return Some(true);
            }
            MarkToLig1 => {
                let comp_index = self.buf.glyphs[cur].component as usize;
                if comp_index == 0xFF {
                    return None;
                }
                let prev = self.previous_base(cur)?;
                let diff = cur - prev;
                if diff > 255 {
                    return None;
                }
                let g2 = self.buf.glyphs[prev].id;
                let mark_index = index as u16;
                let base_index = self.coverage(base + b.read::<u16>(base + 4)? as usize, g2)?;
                let class_count = b.read::<u16>(base + 6)? as usize;
                let mark_anchor =
                    self.mark_anchor(base + b.read::<u16>(base + 8)? as usize, mark_index)?;
                let mark_class = mark_anchor.0 as usize;
                let mark_anchor = mark_anchor.1;
                let mut lig_array = b.read::<u16>(base + 10)? as usize;
                if lig_array == 0 {
                    return None;
                }
                lig_array += base;
                let lig_array_len = b.read::<u16>(lig_array)?;
                if base_index >= lig_array_len {
                    return None;
                }
                let mut lig_attach =
                    b.read::<u16>(lig_array + 2 + base_index as usize * 2)? as usize;
                if lig_attach == 0 {
                    return None;
                }

                lig_attach += lig_array;
                let comp_count = b.read::<u16>(lig_attach)? as usize;
                if comp_count == 0 || comp_index >= comp_count {
                    return None;
                }
                let comp_rec = lig_attach + 2 + comp_index * class_count * 2 + mark_class * 2;
                let anchor_offset = b.read::<u16>(comp_rec)? as usize;
                if anchor_offset == 0 {
                    return None;
                }
                let base_anchor = self.anchor(lig_attach + anchor_offset)?;
                let dx = base_anchor.0 - mark_anchor.0;
                let dy = base_anchor.1 - mark_anchor.1;
                self.buf.position_mark(cur, prev, dx, dy);
                return Some(true);
            }
            Context1 => {
                let set_index = index;
                let mut c = b.stream_at(base + 4)?;
                let set_count = c.read::<u16>()? as usize;
                let set_offsets = c.read_array::<u16>(set_count)?;
                let mut offset = set_offsets.get(set_index)? as usize;
                if offset == 0 {
                    return Some(false);
                }
                offset += base;
                let mut c = b.stream_at(offset)?;
                let rule_count = c.read::<u16>()? as usize;
                let rule_offsets = c.read_array::<u16>(rule_count)?;
                for i in 0..rule_count {
                    let rule_offset = offset + rule_offsets.get(i)? as usize;
                    let mut c = b.stream_at(rule_offset)?;
                    let mut input_count = c.read::<u16>()? as usize;
                    let subst_count = c.read::<u16>()? as usize;
                    let mut input_end = cur;
                    if input_count > 1 {
                        input_count -= 1;
                        let seq = c.read_array::<u16>(input_count)?;
                        if let Some(end) = self
                            .match_sequence(cur, input_count, |i, id| id == seq.get(i).unwrap_or(0))
                        {
                            input_end = end;
                        } else {
                            continue;
                        }
                    }
                    // if let Some(true) = self.apply_contextual(c, subst_count, input_end) {
                    //     return Some(true);
                    // }
                    self.apply_contextual(c, subst_count, input_end);
                    return Some(true);
                }
            }
            Context2 => {
                let mut c = b.stream_at(base + 4)?;
                let mut input_classdef = c.read::<u16>()? as usize;
                if input_classdef == 0 {
                    return Some(false);
                }
                input_classdef += base;
                let set_index = self.class(input_classdef, g) as usize;
                let set_count = c.read::<u16>()? as usize;
                let set_offsets = c.read_array::<u16>(set_count)?;
                let mut offset = set_offsets.get(set_index)? as usize;
                if offset == 0 {
                    return Some(false);
                }
                offset += base;
                let mut c = b.stream_at(offset)?;
                let rule_count = c.read::<u16>()? as usize;
                let rule_offsets = c.read_array::<u16>(rule_count)?;
                for i in 0..rule_count {
                    let rule_offset = offset + rule_offsets.get(i)? as usize;
                    let mut c = b.stream_at(rule_offset)?;
                    let mut input_count = c.read::<u16>()? as usize;
                    let subst_count = c.read::<u16>()? as usize;
                    let mut input_end = cur;
                    if input_count > 1 {
                        input_count -= 1;
                        let seq = c.read_array::<u16>(input_count)?;
                        if let Some(end) = self.match_sequence(cur, input_count, |i, id| {
                            self.class(input_classdef, id) == seq.get(i).unwrap_or(0)
                        }) {
                            input_end = end;
                        } else {
                            continue;
                        }
                    }
                    // if let Some(true) = self.apply_contextual(c, subst_count, input_end) {
                    //     return Some(true);
                    // }
                    self.apply_contextual(c, subst_count, input_end);
                    return Some(true);
                }
            }
            Context3 => {
                let mut c = b.stream_at(base + 2)?;
                let mut input_count = c.read::<u16>()? as usize;
                if input_count == 0 {
                    return None;
                }
                input_count -= 1;
                let subst_count = c.read::<u16>()? as usize;
                c.skip(2)?;
                let input = c.read_array::<u16>(input_count)?;
                let input_end = self.match_sequence(cur, input_count, |i, id| {
                    self.coverage(base + input.get(i).unwrap_or(0) as usize, id)
                        .is_some()
                })?;
                self.apply_contextual(c, subst_count, input_end);
                return Some(true);
            }
            ChainContext1 => {
                let set_index = index;
                let mut c = b.stream_at(base + 4)?;
                let set_count = c.read::<u16>()? as usize;
                let set_offsets = c.read_array::<u16>(set_count)?;
                let mut offset = set_offsets.get(set_index)? as usize;
                if offset == 0 {
                    return Some(false);
                }
                offset += base;
                let mut c = b.stream_at(offset)?;
                let rule_count = c.read::<u16>()? as usize;
                let rule_offsets = c.read_array::<u16>(rule_count)?;
                for i in 0..rule_count {
                    let rule_offset = offset + rule_offsets.get(i)? as usize;
                    let mut c = b.stream_at(rule_offset)?;
                    let backtrack_count = c.read::<u16>()? as usize;
                    if backtrack_count != 0 {
                        let seq = c.read_array::<u16>(backtrack_count)?;
                        let pred = |i, id| id == seq.get(i).unwrap_or(0);
                        if self.match_backtrack(cur, backtrack_count, pred).is_none() {
                            continue;
                        }
                    }
                    let mut input_count = c.read::<u16>()? as usize;
                    let mut input_end = cur;
                    if input_count > 1 {
                        input_count -= 1;
                        let seq = c.read_array::<u16>(input_count)?;
                        if let Some(end) = self
                            .match_sequence(cur, input_count, |i, id| id == seq.get(i).unwrap_or(0))
                        {
                            input_end = end;
                        } else {
                            continue;
                        }
                    }
                    let lookahead_count = c.read::<u16>()? as usize;
                    if lookahead_count != 0 {
                        let seq = c.read_array::<u16>(lookahead_count)?;
                        let pred = |i, id| id == seq.get(i).unwrap_or(0);
                        if self
                            .match_sequence(input_end, lookahead_count, pred)
                            .is_none()
                        {
                            continue;
                        }
                    }
                    let count = c.read::<u16>()? as usize;
                    // if let Some(true) = self.apply_contextual(c, count, input_end) {
                    //     return Some(true);
                    // }
                    self.apply_contextual(c, count, input_end);
                    return Some(true);
                }
            }
            ChainContext2 => {
                let mut c = b.stream_at(base + 4)?;
                let backtrack_classdef = base + c.read::<u16>()? as usize;
                let mut input_classdef = c.read::<u16>()? as usize;
                if input_classdef == 0 {
                    return Some(false);
                }
                input_classdef += base;
                let set_index = self.class(input_classdef, g) as usize;
                let lookahead_classdef = base + c.read::<u16>()? as usize;
                let set_count = c.read::<u16>()? as usize;
                let set_offsets = c.read_array::<u16>(set_count)?;
                let mut offset = set_offsets.get(set_index)? as usize;
                if offset == 0 {
                    return Some(false);
                }
                offset += base;
                let mut c = b.stream_at(offset)?;
                let rule_count = c.read::<u16>()? as usize;
                let rule_offsets = c.read_array::<u16>(rule_count)?;
                for i in 0..rule_count {
                    let rule_offset = offset + rule_offsets.get(i)? as usize;
                    let mut c = b.stream_at(rule_offset)?;
                    let backtrack_count = c.read::<u16>()? as usize;
                    if backtrack_count != 0 {
                        let seq = c.read_array::<u16>(backtrack_count)?;
                        let pred =
                            |i, id| self.class(backtrack_classdef, id) == seq.get(i).unwrap_or(0);
                        if self.match_backtrack(cur, backtrack_count, pred).is_none() {
                            continue;
                        }
                    }
                    let mut input_count = c.read::<u16>()? as usize;
                    let mut input_end = cur;
                    if input_count > 1 {
                        input_count -= 1;
                        let seq = c.read_array::<u16>(input_count)?;
                        if let Some(end) = self.match_sequence(cur, input_count, |i, id| {
                            self.class(input_classdef, id) == seq.get(i).unwrap_or(0)
                        }) {
                            input_end = end;
                        } else {
                            continue;
                        }
                    }
                    let lookahead_count = c.read::<u16>()? as usize;
                    if lookahead_count != 0 {
                        let seq = c.read_array::<u16>(lookahead_count)?;
                        let pred =
                            |i, id| self.class(lookahead_classdef, id) == seq.get(i).unwrap_or(0);
                        if self
                            .match_sequence(input_end, lookahead_count, pred)
                            .is_none()
                        {
                            continue;
                        }
                    }
                    let count = c.read::<u16>()? as usize;
                    // if let Some(true) = self.apply_contextual(c, count, input_end) {
                    //     return Some(true);
                    // }
                    self.apply_contextual(c, count, input_end);
                    return Some(true);
                }
            }
            ChainContext3 => {
                let mut c = b.stream_at(base + 2)?;
                let backtrack_count = c.read::<u16>()? as usize;
                if backtrack_count != 0 {
                    if backtrack_count > cur - self.start {
                        return None;
                    }
                    let backtrack = c.read_array::<u16>(backtrack_count)?;
                    self.match_backtrack(cur, backtrack_count, |i, id| {
                        self.coverage(base + backtrack.get_or(i, 0) as usize, id)
                            .is_some()
                    })?;
                }
                let input_count = c.read::<u16>()? as usize - 1;
                c.skip(2);
                let mut input_end = cur;
                if input_count != 0 {
                    let input = c.read_array::<u16>(input_count)?;
                    input_end = self.match_sequence(cur, input_count, |i, id| {
                        self.coverage(base + input.get_or(i, 0) as usize, id)
                            .is_some()
                    })?;
                }
                let lookahead_count = c.read::<u16>()? as usize;
                if lookahead_count != 0 {
                    if lookahead_count > self.s.end - input_end {
                        return None;
                    }
                    let lookahead = c.read_array::<u16>(lookahead_count)?;
                    self.match_sequence(input_end, lookahead_count, |i, id| {
                        self.coverage(base + lookahead.get_or(i, 0) as usize, id)
                            .is_some()
                    })?;
                }
                let count = c.read::<u16>()? as usize;
                self.apply_contextual(c, count, input_end);
                return Some(true);
            }
            RevChainContext1 => {
                let mut c = b.stream_at(base + 4)?;
                let backtrack_count = c.read::<u16>()? as usize;
                if backtrack_count != 0 {
                    if backtrack_count > cur - self.start {
                        return None;
                    }
                    let backtrack = c.read_array::<u16>(backtrack_count)?;
                    self.match_backtrack(cur, backtrack_count, |i, id| {
                        self.coverage(base + backtrack.get_or(i, 0) as usize, id)
                            .is_some()
                    })?;
                }
                let lookahead_count = c.read::<u16>()? as usize;
                if lookahead_count != 0 {
                    if lookahead_count + cur + 1 > self.s.end {
                        return None;
                    }
                    let lookahead = c.read_array::<u16>(lookahead_count)?;
                    self.match_sequence(cur, lookahead_count, |i, id| {
                        self.coverage(base + lookahead.get_or(i, 0) as usize, id)
                            .is_some()
                    })?;
                }
                let count = c.read::<u16>()? as usize;
                let substs = c.read_array::<u16>(count)?;
                let subst = substs.get(index)?;
                self.buf.substitute(cur, subst);
                return Some(true);
            }
        }
        None
    }

    fn apply_nested(
        &mut self,
        index: u16,
        _start: usize,
        cur: usize,
        end: usize,
        first: usize,
    ) -> Option<bool> {
        if self.top as usize == MAX_NESTED_LOOKUPS {
            return None;
        }
        let b = self.data;
        let list_base = self.gsubgpos + b.read::<u16>(self.gsubgpos as usize + 8)? as u32;
        let lookup = lookup_data(self.data, self.stage, list_base, index, 0, Some(self.defs))?;
        self.storage.stack[self.top as usize] = self.s;
        self.top += 1;
        let v = self.apply_uncached(&lookup, cur, end + 1, first);
        self.top -= 1;
        self.s = self.storage.stack[self.top as usize];
        v
    }

    fn apply_uncached(
        &mut self,
        lookup: &LookupData,
        cur: usize,
        end: usize,
        first: usize,
    ) -> Option<bool> {
        let b = self.data;
        let base = lookup.offset as usize;
        //self.s.ignored = lookup.ignored;
        self.s.cur = cur;
        self.s.end = end.min(self.buf.len());
        // self.s.mark_check = lookup.mark_check;
        // self.s.mark_set = lookup.mark_set;
        // self.s.mark_class = lookup.mark_class;
        let mut applied = false;
        let subtables = base + 6;
        let count = lookup.count as usize;
        let ext = lookup.is_ext;
        let kind = lookup.kind;
        let reverse = lookup.kind == LookupKind::RevChainContext;
        if reverse {
            if !self.move_last() {
                return Some(false);
            }
        } else if !self.move_to(first) {
            return Some(false);
        }
        // loop {
        let cur = self.s.cur;
        let g = self.buf.glyphs[cur].id;
        for i in 0..count {
            let mut subtable = base + b.read::<u16>(subtables + i * 2)? as usize;
            if ext {
                subtable = subtable + b.read::<u32>(subtable + 4)? as usize;
            }
            let fmt = b.read::<u16>(subtable)?;
            if let Some(ref s) = subtable_data(b, subtable as u32, kind, fmt) {
                if let Some(index) = s.coverage(b, g) {
                    if let Some(true) = self.apply_subtable(b, s, index as usize, cur, g) {
                        applied = true;
                        break;
                    }
                }
            }
        }
        //     if reverse {
        //         if !self.move_previous() {
        //             break;
        //         }
        //     } else if !self.move_next() {
        //         break;
        //     }
        // }
        Some(applied)
    }

    fn apply_contextual(&mut self, mut c: Stream<'a>, count: usize, end: usize) -> Option<bool> {
        let mut applied = false;
        let start = self.s.cur;
        for _ in 0..count {
            let first = c.read::<u16>()? as usize;
            let lookup = c.read::<u16>()?;
            if let Some(true) = self.apply_nested(lookup, start, start, end, first) {
                applied = true;
            }
        }
        if applied {
            self.s.cur = end;
        }
        Some(applied)
    }

    #[inline(always)]
    fn coverage(&self, coverage_offset: usize, glyph_id: u16) -> Option<u16> {
        coverage(self.data, coverage_offset as u32, glyph_id)
    }

    #[inline(always)]
    fn class(&self, classdef_offset: usize, glyph_id: u16) -> u16 {
        classdef(self.data, classdef_offset as u32, glyph_id)
    }
}

impl<'a, 'b, 'c> ApplyContext<'a, 'b, 'c> {
    fn value_record(
        &self,
        parent_offset: usize,
        mut offset: usize,
        format: u16,
        pos: &mut [f32; 4],
    ) -> Option<()> {
        let b = &self.data;
        if format == 4 {
            pos[2] = b.read_i16(offset)? as f32;
            return Some(());
        }
        if format & 1 != 0 {
            pos[0] = b.read::<i16>(offset)? as f32;
            offset += 2;
        }
        if format & 2 != 0 {
            pos[1] = b.read::<i16>(offset)? as f32;
            offset += 2;
        }
        if format & 4 != 0 {
            pos[2] = b.read::<i16>(offset)? as f32;
            offset += 2;
        }
        if format & 8 != 0 {
            pos[3] = b.read::<i16>(offset)? as f32;
            offset += 2;
        }
        if format & (0x10 | 0x20 | 0x40 | 0x80) == 0 {
            return Some(());
        }
        if self.enable_var {
            if format & 0x10 != 0 {
                pos[0] += self.value_delta(parent_offset, b.read::<u16>(offset)?)?;
                offset += 2;
            }
            if format & 0x20 != 0 {
                pos[1] += self.value_delta(parent_offset, b.read::<u16>(offset)?)?;
                offset += 2;
            }
            if format & 0x40 != 0 {
                pos[2] += self.value_delta(parent_offset, b.read::<u16>(offset)?)?;
                offset += 2;
            }
            if format & 0x80 != 0 {
                pos[3] += self.value_delta(parent_offset, b.read::<u16>(offset)?)?;
            }
        }
        Some(())
    }

    fn value_delta(&self, parent_offset: usize, offset: u16) -> Option<f32> {
        if offset == 0 {
            return Some(0.);
        }
        let b = &self.data;
        let offset = parent_offset + offset as usize;
        let format = b.read::<u16>(offset + 4)?;
        if format != 0x8000 {
            return Some(0.);
        }
        let outer = b.read::<u16>(offset)?;
        let inner = b.read::<u16>(offset + 2)?;
        Some(self.defs.delta(outer, inner, self.coords))
    }

    fn anchor(&self, offset: usize) -> Option<(f32, f32)> {
        let b = &self.data;
        let format = b.read::<u16>(offset)?;
        let mut x = b.read::<i16>(offset + 2)? as f32;
        let mut y = b.read::<i16>(offset + 4)? as f32;
        if format == 3 && self.defs.has_var_store() && !self.coords.is_empty() {
            x += self.value_delta(offset, b.read::<u16>(offset + 6)?)?;
            y += self.value_delta(offset, b.read::<u16>(offset + 8)?)?;
        }
        Some((x, y))
    }

    fn mark_anchor(&self, marks: usize, index: u16) -> Option<(u16, (f32, f32))> {
        let b = &self.data;
        if index >= b.read::<u16>(marks)? {
            return None;
        }
        let rec = marks + 2 + index as usize * 4;
        let class = b.read::<u16>(rec)?;
        let offset = b.read::<u16>(rec + 2)? as usize;
        if offset == 0 {
            return None;
        }
        Some((class, self.anchor(marks + offset)?))
    }
}

#[derive(Default)]
pub struct Storage {
    stack: [LookupState; MAX_NESTED_LOOKUPS],
    ids: [u16; MAX_SEQUENCE],
    indices: [usize; MAX_SEQUENCE],
}
