//! OpenType advanced typography tables.

use super::{raw_tag, Bytes, RawTag};

pub const GDEF: RawTag = raw_tag(b"GDEF");
pub const GSUB: RawTag = raw_tag(b"GSUB");
pub const GPOS: RawTag = raw_tag(b"GPOS");

pub const DFLT: RawTag = raw_tag(b"DFLT");

/// Glyph definition table.
#[derive(Copy, Clone)]
pub struct Gdef<'a> {
    data: Bytes<'a>,
    classes: u16,
    mark_classes: u16,
    mark_sets: u16,
    var_store: u32,
}

impl<'a> Gdef<'a> {
    pub fn new(data: &'a [u8]) -> Option<Self> {
        let b = Bytes::new(data);
        let major = b.read::<u16>(0)?;
        let minor = b.read::<u16>(2)?;
        let classes = b.read::<u16>(4)?;
        let mark_classes = b.read::<u16>(10)?;
        let mark_sets = if major > 1 || minor >= 2 {
            b.read_or_default::<u16>(12)
        } else {
            0
        };
        let var_store = if major > 1 || minor >= 3 {
            b.read_or_default::<u32>(14)
        } else {
            0
        };
        Some(Self {
            data: b,
            classes,
            mark_classes,
            mark_sets,
            var_store,
        })
    }

    pub fn from_offset(data: &'a [u8], offset: u32) -> Option<Self> {
        if offset == 0 {
            return None;
        }
        Self::new(data.get(offset as usize..)?)
    }

    pub fn empty() -> Self {
        Self {
            data: Bytes::new(&[]),
            classes: 0,
            mark_classes: 0,
            mark_sets: 0,
            var_store: 0,
        }
    }

    pub fn ok(&self) -> bool {
        self.data.len() != 0
    }

    /// Returns true if glyph classes are available.
    pub fn has_classes(&self) -> bool {
        self.classes != 0
    }

    /// Returns the class for the specified glyph id.
    pub fn class(&self, glyph_id: u16) -> u16 {
        classdef(&self.data, self.classes as u32, glyph_id)
    }

    /// Returns true if mark glyph classes are available.
    pub fn has_mark_classes(&self) -> bool {
        self.mark_classes != 0
    }

    /// Returns the mark class for the specified glyph id.
    pub fn mark_class(&self, glyph_id: u16) -> u16 {
        classdef(&self.data, self.mark_classes as u32, glyph_id)
    }

    pub fn mark_set_coverage(&self, set_offset: u32, glyph_id: u16) -> Option<u16> {
        if set_offset == 0 {
            return None;
        }
        // Coverage is validated by mark_set_offset() below.
        unsafe { fast_coverage(&self.data, set_offset, glyph_id) }
    }

    pub fn mark_set_offset(&self, set_index: u16) -> Option<u32> {
        if self.mark_sets == 0 {
            return None;
        }
        let set = set_index as usize;
        let b = &self.data;
        let sets_base = self.mark_sets as usize;
        let len = b.read::<u16>(sets_base + 2)? as usize;
        if set >= len {
            return None;
        }
        let offset = b.read::<u32>(sets_base + 4 + set * 4)?;
        let set_offset = sets_base as u32 + offset;
        (offset != 0 && validate_coverage(b, set_offset)).then_some(set_offset)
    }

    pub fn has_var_store(&self) -> bool {
        self.var_store != 0
    }

    pub fn delta(&self, outer: u16, inner: u16, coords: &[i16]) -> f32 {
        if self.var_store != 0 {
            super::var::item_delta(self.data.data(), self.var_store, outer, inner, coords)
                .map(|d| d.to_f32())
                .unwrap_or(0.)
        } else {
            0.
        }
    }
}

/// Feature lookup kind.
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum LookupKind {
    SingleSub,
    MultiSub,
    AltSub,
    LigSub,
    SingleAdj,
    PairAdj,
    Cursive,
    MarkToBase,
    MarkToLig,
    MarkToMark,
    Context,
    ChainContext,
    RevChainContext,
}

/// Data associated with a feature lookup.
#[derive(Copy, Clone)]
pub struct LookupData {
    pub index: u16,
    pub stage: u8,
    pub kind: LookupKind,
    pub feature: u16,
    pub mask: u8,
    pub ignored: u8,
    pub is_ext: bool,
    pub offset: u32,
    pub coverage: u32,
    pub count: u16,
    pub subtables: (u16, u16),
    pub mark_set: u32,
    pub mark_check: u8,
    pub mark_class: u8,
}

impl LookupData {
    pub fn subtable_data(&self, b: &Bytes, index: u16) -> Option<SubtableData> {
        let base = self.offset as usize;
        let subtable_base = base + 6;
        let mut offset = base + b.read::<u16>(subtable_base + index as usize * 2)? as usize;
        if self.is_ext {
            offset = offset + b.read::<u32>(offset + 4)? as usize;
        }
        let fmt = b.read::<u16>(offset)?;
        subtable_data(b, offset as u32, self.kind, fmt)
    }
}

/// Lookup subtable kind, flattened to include the associated format.
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum SubtableKind {
    SingleSub1,
    SingleSub2,
    MultiSub1,
    AltSub1,
    LigSub1,
    SingleAdj1,
    SingleAdj2,
    PairAdj1,
    PairAdj2,
    Cursive1,
    MarkToBase1,
    MarkToLig1,
    MarkToMark1,
    Context1,
    Context2,
    Context3,
    ChainContext1,
    ChainContext2,
    ChainContext3,
    RevChainContext1,
}

/// Data associated with a lookup subtable.
#[derive(Copy, Clone)]
pub struct SubtableData {
    pub offset: u32,
    pub kind: SubtableKind,
    pub coverage: u16,
}

impl SubtableData {
    pub fn coverage(&self, b: &Bytes, glyph_id: u16) -> Option<u16> {
        unsafe { fast_coverage(b, self.offset + self.coverage as u32, glyph_id) }
    }
}

/// Feature substitutions for variable fonts.
#[derive(Copy, Clone)]
pub struct FeatureSubsts(u32);

impl FeatureSubsts {
    pub fn new(b: &Bytes, offset: u32, coords: &[i16]) -> Option<Self> {
        if offset == 0 || coords.is_empty() {
            return None;
        }
        let base = offset as usize;
        let count = b.read::<u32>(base + 4)? as usize;
        for i in 0..count {
            let rec = base + 8 + i * 8;
            let condset_table = base + b.read::<u32>(rec)? as usize;
            let condset_count = b.read::<u16>(condset_table)? as usize;
            let mut matched = 0;
            for j in 0..condset_count {
                let cond_table = condset_table + b.read::<u32>(condset_table + 2 + j * 4)? as usize;
                let format = b.read::<u16>(cond_table)?;
                if format != 1 {
                    break;
                }
                let axis = b.read::<u16>(cond_table + 2)? as usize;
                if axis >= coords.len() {
                    break;
                }
                let coord = coords[axis];
                let min = b.read::<i16>(cond_table + 4)?;
                if coord < min {
                    break;
                }
                let max = b.read::<i16>(cond_table + 6)?;
                if coord > max {
                    break;
                }
                matched += 1;
            }
            if matched == condset_count {
                return Some(Self(offset + b.read::<u32>(rec + 4)?));
            }
        }
        None
    }

    pub fn apply(self, b: &Bytes, index: u16) -> Option<usize> {
        let mut base = self.0 as usize;
        let count = b.read::<u16>(base + 4)? as usize;
        base += 6;
        let mut l = 0;
        let mut h = count;
        while l < h {
            use core::cmp::Ordering::*;
            let i = (l + h) / 2;
            let rec = base + i * 6;
            let idx = b.read::<u16>(rec)?;
            match index.cmp(&idx) {
                Less => h = i,
                Greater => l = i + 1,
                Equal => return Some((self.0 + b.read::<u32>(rec + 2)?) as usize),
            }
        }
        None
    }
}

pub fn script_count(b: &Bytes, gsubgpos_offset: u32) -> u16 {
    if gsubgpos_offset == 0 {
        return 0;
    }
    let base = gsubgpos_offset as usize;
    let offset = b.read_or_default::<u16>(base + 4) as usize;
    if offset == 0 {
        return 0;
    }
    b.read_or_default::<u16>(base + offset)
}

pub fn script_at(b: &Bytes, gsubgpos_offset: u32, index: u16) -> Option<(RawTag, u32)> {
    if gsubgpos_offset == 0 {
        return None;
    }
    let base = gsubgpos_offset as usize;
    let sbase = base + b.read::<u16>(base + 4)? as usize;
    let rec = sbase + 2 + index as usize * 6;
    let tag = b.read::<u32>(rec)?;
    let offset = sbase as u32 + b.read::<u16>(rec + 4)? as u32;
    Some((tag, offset))
}

pub fn script_by_tag(b: &Bytes, gsubgpos_offset: u32, script: RawTag) -> Option<u32> {
    if gsubgpos_offset == 0 {
        return None;
    }
    let base = gsubgpos_offset as usize;
    let sbase = base + b.read::<u16>(base + 4)? as usize;
    let mut l = 0;
    let mut h = b.read::<u16>(sbase)? as usize;
    while l < h {
        use core::cmp::Ordering::*;
        let i = l + (h - l) / 2;
        let rec = sbase + 2 + i * 6;
        let t = b.read::<u32>(rec)?;
        match script.cmp(&t) {
            Less => h = i,
            Greater => l = i + 1,
            Equal => return Some(sbase as u32 + b.read::<u16>(rec + 4)? as u32),
        }
    }
    None
}

pub fn script_language_count(b: &Bytes, script_offset: u32) -> u16 {
    if script_offset == 0 {
        return 0;
    }
    b.read::<u16>(script_offset as usize + 2)
        .map(|n| n + 1)
        .unwrap_or(0)
}

pub fn script_default_language(b: &Bytes, script_offset: u32) -> Option<u32> {
    if script_offset == 0 {
        return None;
    }
    let offset = b.read::<u16>(script_offset as usize)? as u32;
    if offset == 0 {
        None
    } else {
        Some(script_offset + offset)
    }
}

pub fn script_language_at(b: &Bytes, script_offset: u32, index: u16) -> Option<(RawTag, u32)> {
    if script_offset == 0 {
        return None;
    }
    let index = if index == 0 {
        return Some((DFLT, script_default_language(b, script_offset)?));
    } else {
        index - 1
    };
    let rec = script_offset as usize + 4 + index as usize * 6;
    let tag = b.read::<u32>(rec)?;
    let offset = b.read::<u16>(rec + 4)? as u32;
    if offset == 0 {
        return None;
    }
    Some((tag, script_offset + offset))
}

pub fn script_language_by_tag(
    b: &Bytes,
    script_offset: u32,
    language: Option<RawTag>,
) -> Option<(u32, bool)> {
    if script_offset == 0 {
        return None;
    }
    let base = script_offset as usize;
    if let Some(lang) = language {
        let mut l = 0;
        let mut h = b.read::<u16>(base + 2)? as usize;
        while l < h {
            use core::cmp::Ordering::*;
            let i = (l + h) / 2;
            let rec = base + 4 + i * 6;
            let t = b.read::<u32>(rec)?;
            match lang.cmp(&t) {
                Less => h = i,
                Greater => l = i + 1,
                Equal => {
                    let lang_offset = b.read::<u16>(rec + 4)? as usize;
                    if lang_offset == 0 {
                        return None;
                    }
                    return Some((script_offset + lang_offset as u32, false));
                }
            }
        }
    }
    let default = b.read::<u16>(base)? as usize;
    if default == 0 {
        return None;
    }
    Some(((base + default) as u32, true))
}

pub fn language_or_default_by_tags(
    b: &Bytes,
    gsubgpos_offset: u32,
    script: RawTag,
    lang: Option<RawTag>,
) -> Option<(u32, [RawTag; 2])> {
    if let Some(script_offset) = script_by_tag(b, gsubgpos_offset, script) {
        let (lang_offset, is_default) = script_language_by_tag(b, script_offset, lang)?;
        Some((
            lang_offset,
            [
                script,
                if is_default {
                    DFLT
                } else {
                    lang.unwrap_or(DFLT)
                },
            ],
        ))
    } else {
        let (lang_offset, is_default) = language_by_tags(b, gsubgpos_offset, DFLT, lang)?;
        Some((
            lang_offset,
            [
                DFLT,
                if is_default {
                    DFLT
                } else {
                    lang.unwrap_or(DFLT)
                },
            ],
        ))
    }
}

pub fn language_by_tags(
    b: &Bytes,
    gsubgpos_offset: u32,
    script: RawTag,
    language: Option<RawTag>,
) -> Option<(u32, bool)> {
    script_language_by_tag(b, script_by_tag(b, gsubgpos_offset, script)?, language)
}

pub fn language_feature_count(b: &Bytes, language_offset: u32) -> u16 {
    if language_offset == 0 {
        return 0;
    }
    b.read_or_default(language_offset as usize + 4)
}

pub fn language_feature_at(b: &Bytes, language_offset: u32, index: u16) -> Option<u16> {
    b.read(language_offset as usize + 6 + index as usize * 2)
}

pub fn language_features<'a>(
    b: Bytes<'a>,
    gsubgpos_offset: u32,
    language_offset: u32,
) -> impl Iterator<Item = (RawTag, u32)> + 'a + Clone {
    let mut count = language_feature_count(&b, language_offset);
    if gsubgpos_offset == 0 {
        count = 0;
    }
    let base = gsubgpos_offset as usize;
    let fbase = b.read_or_default::<u16>(base + 6) as usize;
    if fbase == 0 {
        count = 0;
    }
    let fbase = base + fbase;
    (0..count).filter_map(move |i| {
        let index = language_feature_at(&b, language_offset, i)?;
        let rec = fbase + 2 + index as usize * 6;
        let tag = b.read::<u32>(rec)?;
        let offset = b.read::<u16>(rec + 4)?;
        if offset == 0 {
            return None;
        }
        Some((tag, fbase as u32 + offset as u32))
    })
}

pub fn feature_count(b: &Bytes, gsubgpos_offset: u32) -> u16 {
    if gsubgpos_offset == 0 {
        return 0;
    }
    let base = gsubgpos_offset as usize;
    let fbase = b.read_or_default::<u16>(base + 6) as usize;
    if fbase == 0 {
        return 0;
    }
    b.read_or_default::<u16>(base + fbase)
}

pub fn feature_at(b: &Bytes, gsubgpos_offset: u32, index: u16) -> Option<(RawTag, u32)> {
    if gsubgpos_offset == 0 {
        return None;
    }
    let base = gsubgpos_offset as usize;
    let fbase = b.read::<u16>(base + 6)? as usize;
    if fbase == 0 {
        return None;
    }
    let fbase = base + fbase;
    let rec = fbase + 2 + index as usize * 6;
    let tag = b.read::<u32>(rec)?;
    let offset = b.read::<u16>(rec + 4)?;
    if offset == 0 {
        return None;
    }
    Some((tag, fbase as u32 + offset as u32))
}

pub fn feature_var_offset(b: &Bytes, gsubgpos_offset: u32) -> u32 {
    if gsubgpos_offset == 0 {
        return 0;
    }
    let base = gsubgpos_offset as usize;
    let major = b.read_or_default::<u16>(base);
    if major > 1 || (major == 1 && b.read_or_default::<u16>(base + 2) >= 1) {
        let offset = b.read_or_default::<u32>(base + 10);
        if offset != 0 {
            gsubgpos_offset + offset
        } else {
            0
        }
    } else {
        0
    }
}

pub fn lookup_data(
    b: &Bytes,
    stage: u8,
    list_base: u32,
    index: u16,
    mask: u8,
    gdef: Option<&Gdef>,
) -> Option<LookupData> {
    if list_base == 0 {
        return None;
    }
    let base = list_base as usize;
    let rec = base + 2 + index as usize * 2;
    let offset = b.read::<u16>(rec)?;
    let base = base + offset as usize;
    let mut kind = b.read::<u16>(base)? as u8;
    let flag = b.read::<u16>(base + 2)?;
    let f = flag as u8;
    let count = b.read::<u16>(base + 4)?;
    let mark_class = (flag >> 8) as u8;
    let ignore_marks = f & (1 << 3) != 0;
    let mut mark_check = 0;
    let mut mark_set = 0;
    if !ignore_marks {
        if let Some(gdef) = gdef {
            mark_check = (mark_class != 0 && gdef.has_mark_classes()) as u8;
            mark_set = if gdef.ok() && flag & 0x10 != 0 {
                let idx = b.read::<u16>(base + 6 + count as usize * 2)?;
                mark_check = 1;
                gdef.mark_set_offset(idx).unwrap_or(0)
            } else {
                0
            };
        }
    }
    let is_sub = stage == 0;
    let subtables = base + 6;
    let is_ext = (is_sub && kind == 7) || (!is_sub && kind == 9);
    if is_ext && count > 0 {
        let s = base + b.read::<u16>(subtables)? as usize;
        kind = b.read::<u16>(s + 2)? as u8;
    }
    use LookupKind::*;
    let kind = if stage == 0 {
        match kind {
            1 => SingleSub,
            2 => MultiSub,
            3 => AltSub,
            4 => LigSub,
            5 => Context,
            6 => ChainContext,
            8 => RevChainContext,
            _ => return None,
        }
    } else {
        match kind {
            1 => SingleAdj,
            2 => PairAdj,
            3 => Cursive,
            4 => MarkToBase,
            5 => MarkToLig,
            6 => MarkToMark,
            7 => Context,
            8 => ChainContext,
            _ => return None,
        }
    };
    let ignored = (f & 0b1110) | 1 << 5;
    Some(LookupData {
        index,
        stage,
        kind,
        feature: 0,
        mask,
        ignored,
        is_ext,
        offset: base as u32,
        count,
        coverage: !0,
        subtables: (0, 0),
        mark_class,
        mark_set,
        mark_check,
    })
}

pub fn subtable_data(b: &Bytes, offset: u32, kind: LookupKind, fmt: u16) -> Option<SubtableData> {
    let base = offset as usize;
    fn cov(b: &Bytes, base: usize, offset: usize) -> Option<u16> {
        let c = b.read::<u16>(base + offset)?;
        validate_coverage(b, base as u32 + c as u32).then_some(c)
    }
    use LookupKind::*;
    match kind {
        SingleSub => {
            let kind = match fmt {
                1 => SubtableKind::SingleSub1,
                2 => SubtableKind::SingleSub2,
                _ => return None,
            };
            let coverage = cov(b, base, 2)?;
            Some(SubtableData {
                offset,
                kind,
                coverage,
            })
        }
        MultiSub => {
            let kind = match fmt {
                1 => SubtableKind::MultiSub1,
                _ => return None,
            };
            let coverage = cov(b, base, 2)?;
            Some(SubtableData {
                offset,
                kind,
                coverage,
            })
        }
        AltSub => {
            let kind = match fmt {
                1 => SubtableKind::AltSub1,
                _ => return None,
            };
            let coverage = cov(b, base, 2)?;
            Some(SubtableData {
                offset,
                kind,
                coverage,
            })
        }
        LigSub => {
            let kind = match fmt {
                1 => SubtableKind::LigSub1,
                _ => return None,
            };
            let coverage = cov(b, base, 2)?;
            Some(SubtableData {
                offset,
                kind,
                coverage,
            })
        }
        SingleAdj => {
            let kind = match fmt {
                1 => SubtableKind::SingleAdj1,
                2 => SubtableKind::SingleAdj2,
                _ => return None,
            };
            let coverage = cov(b, base, 2)?;
            Some(SubtableData {
                offset,
                kind,
                coverage,
            })
        }
        PairAdj => {
            let kind = match fmt {
                1 => SubtableKind::PairAdj1,
                2 => SubtableKind::PairAdj2,
                _ => return None,
            };
            let coverage = cov(b, base, 2)?;
            Some(SubtableData {
                offset,
                kind,
                coverage,
            })
        }
        Cursive => {
            let kind = match fmt {
                1 => SubtableKind::Cursive1,
                _ => return None,
            };
            let coverage = cov(b, base, 2)?;
            Some(SubtableData {
                offset,
                kind,
                coverage,
            })
        }
        MarkToBase => {
            let kind = match fmt {
                1 => SubtableKind::MarkToBase1,
                _ => return None,
            };
            let coverage = cov(b, base, 2)?;
            Some(SubtableData {
                offset,
                kind,
                coverage,
            })
        }
        MarkToLig => {
            let kind = match fmt {
                1 => SubtableKind::MarkToLig1,
                _ => return None,
            };
            let coverage = cov(b, base, 2)?;
            Some(SubtableData {
                offset,
                kind,
                coverage,
            })
        }
        MarkToMark => {
            let kind = match fmt {
                1 => SubtableKind::MarkToMark1,
                _ => return None,
            };
            let coverage = cov(b, base, 2)?;
            Some(SubtableData {
                offset,
                kind,
                coverage,
            })
        }
        Context => match fmt {
            1 | 2 => {
                let kind = if fmt == 1 {
                    SubtableKind::Context1
                } else {
                    SubtableKind::Context2
                };
                let coverage = cov(b, base, 2)?;
                Some(SubtableData {
                    offset,
                    kind,
                    coverage,
                })
            }
            3 => {
                let coverage = cov(b, base, 6)?;
                Some(SubtableData {
                    kind: SubtableKind::Context3,
                    offset,
                    coverage,
                })
            }
            _ => None,
        },
        ChainContext => match fmt {
            1 | 2 => {
                let kind = if fmt == 1 {
                    SubtableKind::ChainContext1
                } else {
                    SubtableKind::ChainContext2
                };
                let coverage = cov(b, base, 2)?;
                Some(SubtableData {
                    offset,
                    kind,
                    coverage,
                })
            }
            3 => {
                let backtrack_len = b.read::<u16>(base + 2)? as usize * 2;
                let input_len = b.read::<u16>(base + backtrack_len + 4)?;
                if input_len == 0 {
                    return None;
                }
                let coverage = cov(b, base, backtrack_len + 6)?;
                Some(SubtableData {
                    kind: SubtableKind::ChainContext3,
                    offset,
                    coverage,
                })
            }
            _ => None,
        },
        RevChainContext => {
            let kind = match fmt {
                1 => SubtableKind::RevChainContext1,
                _ => return None,
            };
            let coverage = cov(b, base, 2)?;
            Some(SubtableData {
                offset,
                kind,
                coverage,
            })
        }
    }
}

fn validate_coverage(b: &Bytes, coverage_offset: u32) -> bool {
    if coverage_offset == 0 {
        return false;
    }
    let base = coverage_offset as usize;
    let arr = base + 4;
    match (b.read::<u16>(base), b.read::<u16>(base + 2)) {
        // Empty subtable coverage is useless, so mark empty coverage subtables as invalid.
        (Some(_), Some(0)) => false,
        (Some(1), Some(len)) => b.check_range(arr, len as usize * 2),
        (Some(2), Some(len)) => b.check_range(arr, len as usize * 6),
        _ => false,
    }
}

pub unsafe fn fast_coverage(b: &Bytes, coverage_offset: u32, glyph_id: u16) -> Option<u16> {
    let base = coverage_offset as usize;
    let fmt = b.read_unchecked::<u16>(base);
    let len = b.read_unchecked::<u16>(base + 2) as usize;
    let arr = base + 4;
    if fmt == 1 {
        let mut l = 0;
        let mut h = len;
        while l < h {
            use core::cmp::Ordering::*;
            let i = (l + h) / 2;
            let g = b.read_unchecked::<u16>(arr + i * 2);
            match glyph_id.cmp(&g) {
                Less => h = i,
                Greater => l = i + 1,
                Equal => return Some(i as u16),
            }
        }
    } else if fmt == 2 {
        let mut l = 0;
        let mut h = len;
        while l < h {
            let i = (l + h) / 2;
            let rec = arr + i * 6;
            let start = b.read_unchecked::<u16>(rec);
            if glyph_id < start {
                h = i;
            } else if glyph_id > b.read_unchecked::<u16>(rec + 2) {
                l = i + 1;
            } else {
                let base = b.read_unchecked::<u16>(rec + 4);
                return Some(base + glyph_id - start);
            }
        }
    }
    None
}

pub fn coverage(b: &Bytes, coverage_offset: u32, glyph_id: u16) -> Option<u16> {
    if coverage_offset == 0 {
        return None;
    }
    let base = coverage_offset as usize;
    let fmt = b.read::<u16>(base)?;
    let len = b.read::<u16>(base + 2)? as usize;
    let arr = base + 4;
    if fmt == 1 {
        if !b.check_range(arr, len * 2) {
            return None;
        }
        let mut l = 0;
        let mut h = len;
        while l < h {
            use core::cmp::Ordering::*;
            let i = (l + h) / 2;
            let g = unsafe { b.read_unchecked::<u16>(arr + i * 2) };
            match glyph_id.cmp(&g) {
                Less => h = i,
                Greater => l = i + 1,
                Equal => return Some(i as u16),
            }
        }
    } else if fmt == 2 {
        if !b.check_range(arr, len * 6) {
            return None;
        }
        let mut l = 0;
        let mut h = len;
        while l < h {
            let i = (l + h) / 2;
            let rec = arr + i * 6;
            let start = unsafe { b.read_unchecked::<u16>(rec) };
            if glyph_id < start {
                h = i;
            } else if glyph_id > unsafe { b.read_unchecked::<u16>(rec + 2) } {
                l = i + 1;
            } else {
                let base = unsafe { b.read_unchecked::<u16>(rec + 4) };
                return Some(base + (glyph_id - start));
            }
        }
    }
    None
}

pub fn classdef(b: &Bytes, classdef_offset: u32, glyph_id: u16) -> u16 {
    if classdef_offset == 0 {
        return 0;
    }
    let base = classdef_offset as usize;
    let fmt = b.read_or_default::<u16>(base);
    if fmt == 1 {
        let start = b.read_or_default::<u16>(base + 2);
        let len = b.read_or_default::<u16>(base + 4);
        let end = start + len - 1;
        let arr = base + 6;
        if glyph_id >= start && glyph_id <= end {
            return b.read_or_default::<u16>(arr + (glyph_id - start) as usize * 2);
        }
        return 0;
    } else if fmt == 2 {
        let len = b.read_or_default::<u16>(base + 2) as usize;
        let arr = base + 4;
        if !b.check_range(arr, len * 6) {
            return 0;
        }
        let mut l = 0;
        let mut h = len;
        while l < h {
            let i = (l + h) / 2;
            let rec = arr + i * 6;
            let start = unsafe { b.read_unchecked::<u16>(rec) };
            if glyph_id < start {
                h = i;
            } else if glyph_id > unsafe { b.read_unchecked::<u16>(rec + 2) } {
                l = i + 1;
            } else {
                return unsafe { b.read_unchecked::<u16>(rec + 4) };
            }
        }
    }
    0
}
