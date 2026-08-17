use core::ops::Range;

use read_fonts::{
    tables::{
        ankr::Ankr,
        cmap::{Cmap, CmapSubtable, PlatformId},
        feat::Feat,
        gdef::Gdef,
        glyf::Glyf,
        gpos::Gpos,
        gsub::Gsub,
        gvar::Gvar,
        hmtx::Hmtx,
        hvar::Hvar,
        kern::Kern,
        kerx::Kerx,
        loca::Loca,
        morx::Morx,
        mvar::Mvar,
        trak::Trak,
        vmtx::Vmtx,
        vorg::Vorg,
        vvar::Vvar,
    },
    types::Tag,
    FontData, FontRead, FontRef, TableProvider, TopLevelTable,
};

// https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#windows-platform-platform-id--3
const WINDOWS_SYMBOL_ENCODING: u16 = 0;
const WINDOWS_UNICODE_BMP_ENCODING: u16 = 1;
const WINDOWS_UNICODE_FULL_ENCODING: u16 = 10;

// https://docs.microsoft.com/en-us/typography/opentype/spec/name#platform-specific-encoding-and-language-ids-unicode-platform-platform-id--0
const UNICODE_1_0_ENCODING: u16 = 0;
const UNICODE_1_1_ENCODING: u16 = 1;
const UNICODE_ISO_ENCODING: u16 = 2;
const UNICODE_2_0_BMP_ENCODING: u16 = 3;
const UNICODE_2_0_FULL_ENCODING: u16 = 4;

//const UNICODE_VARIATION_ENCODING: u16 = 5;
const UNICODE_FULL_ENCODING: u16 = 6;

#[derive(Clone)]
pub struct TableRanges {
    pub num_glyphs: u32,
    pub units_per_em: u16,
    pub loca_long: bool,
    pub num_v_metrics: u16,
    pub num_h_metrics: u16,
    pub ascent: i16,
    pub descent: i16,
    pub loca: TableRange,
    pub glyf: TableRange,
    pub gvar: TableRange,
    pub hmtx: TableRange,
    pub hvar: TableRange,
    pub vmtx: TableRange,
    pub vvar: TableRange,
    pub vorg: TableRange,
    pub mvar: TableRange,
    pub cmap: TableRange,
    pub cmap_subtable: Option<SelectedCmapSubtable>,
    pub cmap_vs_subtable: Option<u16>,
    pub gdef: TableRange,
    pub gsub: TableRange,
    pub gpos: TableRange,
    pub morx: TableRange,
    pub kerx: TableRange,
    pub ankr: TableRange,
    pub kern: TableRange,
    pub feat: TableRange,
    pub trak: TableRange,
}

#[derive(Copy, Clone)]
pub struct SelectedCmapSubtable {
    pub index: u16,
    pub is_mac_roman: bool,
    pub is_symbol: bool,
}

impl TableRanges {
    pub fn new(font: &FontRef) -> Self {
        let num_glyphs = font
            .maxp()
            .map(|maxp| maxp.num_glyphs() as u32)
            .unwrap_or_default();
        let (units_per_em, loca_long) = font
            .head()
            .map(|head| (head.units_per_em(), head.index_to_loc_format() == 1))
            .unwrap_or((1000, false));
        let os2 = font.os2().ok();
        let hhea = font.hhea().ok();
        let (ascent, descent) = if let Some(os2) = &os2 {
            (os2.s_typo_ascender(), os2.s_typo_descender())
        } else if let Some(hhea) = &hhea {
            (hhea.ascender().to_i16(), hhea.descender().to_i16())
        } else {
            (0, 0) // TODO
        };
        let num_h_metrics = hhea
            .map(|hhea| hhea.number_of_h_metrics())
            .unwrap_or_default();
        let num_v_metrics = font
            .vhea()
            .map(|vhea| vhea.number_of_long_ver_metrics())
            .unwrap_or_default();
        let offset = |tag| TableRange::new(font, tag).unwrap_or_default();
        let loca = offset(Loca::TAG);
        let glyf = offset(Glyf::TAG);
        let gvar = offset(Gvar::TAG);
        let hmtx = offset(Hmtx::TAG);
        let hvar = offset(Hvar::TAG);
        let vmtx = offset(Vmtx::TAG);
        let vvar = offset(Vvar::TAG);
        let vorg = offset(Vorg::TAG);
        let mvar = offset(Mvar::TAG);
        let cmap = offset(Cmap::TAG);
        let cmap_table: Option<Cmap> = cmap.resolve_table(font);
        let cmap_subtable = cmap_table
            .as_ref()
            .and_then(|cmap| find_best_cmap_subtable(cmap))
            .map(|(index, platform, encoding, _)| SelectedCmapSubtable {
                index,
                is_mac_roman: platform == PlatformId::Macintosh,
                is_symbol: platform == PlatformId::Windows && encoding == WINDOWS_SYMBOL_ENCODING,
            });
        let cmap_vs_subtable = cmap_table.and_then(|cmap| {
            let data = cmap.offset_data();
            cmap.encoding_records()
                .iter()
                .enumerate()
                .filter_map(|(index, record)| Some((index, record.subtable(data).ok()?)))
                .find_map(|(index, subtable)| match subtable {
                    CmapSubtable::Format14(_) => Some(index as u16),
                    _ => None,
                })
        });
        let gdef = offset(Gdef::TAG);
        let gsub = offset(Gsub::TAG);
        let gpos = offset(Gpos::TAG);
        let morx = offset(Morx::TAG);
        let kerx = offset(Kerx::TAG);
        let ankr = offset(Ankr::TAG);
        let kern = offset(Kern::TAG);
        let feat = offset(Feat::TAG);
        let trak = offset(Trak::TAG);
        Self {
            num_glyphs,
            units_per_em,
            loca_long,
            num_h_metrics,
            num_v_metrics,
            ascent,
            descent,
            loca,
            glyf,
            gvar,
            hmtx,
            hvar,
            vmtx,
            vvar,
            vorg,
            mvar,
            cmap,
            cmap_subtable,
            cmap_vs_subtable,
            gdef,
            gsub,
            gpos,
            morx,
            kerx,
            ankr,
            kern,
            feat,
            trak,
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct TableRange(u32, u32);

impl TableRange {
    fn new(font: &FontRef, tag: Tag) -> Option<Self> {
        let records = font.table_directory().table_records();
        records
            .binary_search_by_key(&tag, |rec| rec.tag())
            .ok()
            .and_then(|ix| records.get(ix))
            .map(|rec| Self(rec.offset(), rec.length()))
    }

    pub fn resolve(self) -> Option<Range<usize>> {
        let start = self.0 as usize;
        (start != 0).then_some(start..start.wrapping_add(self.1 as usize))
    }

    pub fn resolve_data<'a>(self, font: &FontRef<'a>) -> Option<FontData<'a>> {
        font.data().slice(self.resolve()?)
    }

    pub fn resolve_table<'a, T: FontRead<'a>>(self, font: &FontRef<'a>) -> Option<T> {
        T::read(self.resolve_data(font)?).ok()
    }
}

fn find_best_cmap_subtable<'a>(
    cmap: &Cmap<'a>,
) -> Option<(u16, PlatformId, u16, CmapSubtable<'a>)> {
    // Symbol subtable.
    // Prefer symbol if available.
    // https://github.com/harfbuzz/harfbuzz/issues/1918
    find_cmap_subtable(cmap, PlatformId::Windows, WINDOWS_SYMBOL_ENCODING)
        // 32-bit subtables:
        .or_else(|| find_cmap_subtable(cmap, PlatformId::Windows, WINDOWS_UNICODE_FULL_ENCODING))
        .or_else(|| find_cmap_subtable(cmap, PlatformId::Unicode, UNICODE_FULL_ENCODING))
        .or_else(|| find_cmap_subtable(cmap, PlatformId::Unicode, UNICODE_2_0_FULL_ENCODING))
        // 16-bit subtables:
        .or_else(|| find_cmap_subtable(cmap, PlatformId::Windows, WINDOWS_UNICODE_BMP_ENCODING))
        .or_else(|| find_cmap_subtable(cmap, PlatformId::Unicode, UNICODE_2_0_BMP_ENCODING))
        .or_else(|| find_cmap_subtable(cmap, PlatformId::Unicode, UNICODE_ISO_ENCODING))
        .or_else(|| find_cmap_subtable(cmap, PlatformId::Unicode, UNICODE_1_1_ENCODING))
        .or_else(|| find_cmap_subtable(cmap, PlatformId::Unicode, UNICODE_1_0_ENCODING))
        // MacRoman subtable:
        .or_else(|| find_cmap_subtable(cmap, PlatformId::Macintosh, 0))
}

fn find_cmap_subtable<'a>(
    cmap: &Cmap<'a>,
    platform_id: PlatformId,
    encoding_id: u16,
) -> Option<(u16, PlatformId, u16, CmapSubtable<'a>)> {
    let offset_data = cmap.offset_data();
    for (index, record) in cmap.encoding_records().iter().enumerate() {
        if record.platform_id() != platform_id || record.encoding_id() != encoding_id {
            continue;
        }
        if let Ok(subtable) = record.subtable(offset_data) {
            match subtable {
                CmapSubtable::Format0(_)
                | CmapSubtable::Format4(_)
                | CmapSubtable::Format6(_)
                | CmapSubtable::Format10(_)
                | CmapSubtable::Format12(_)
                | CmapSubtable::Format13(_) => {
                    return Some((index as u16, platform_id, encoding_id, subtable))
                }
                _ => {}
            }
        }
    }
    None
}
