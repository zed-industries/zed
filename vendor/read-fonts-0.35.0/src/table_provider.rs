//! a trait for things that can serve font tables

use types::{BigEndian, Tag};

use crate::{tables, FontData, FontRead, ReadError};

/// A table that has an associated tag.
///
/// This is true of top-level tables, but not their various subtables.
pub trait TopLevelTable {
    /// The table's tag.
    const TAG: Tag;
}

/// An interface for accessing tables from a font (or font-like object)
pub trait TableProvider<'a> {
    fn data_for_tag(&self, tag: Tag) -> Option<FontData<'a>>;

    fn expect_data_for_tag(&self, tag: Tag) -> Result<FontData<'a>, ReadError> {
        self.data_for_tag(tag).ok_or(ReadError::TableIsMissing(tag))
    }

    fn expect_table<T: TopLevelTable + FontRead<'a>>(&self) -> Result<T, ReadError> {
        self.expect_data_for_tag(T::TAG).and_then(FontRead::read)
    }

    fn head(&self) -> Result<tables::head::Head<'a>, ReadError> {
        self.expect_table()
    }

    fn name(&self) -> Result<tables::name::Name<'a>, ReadError> {
        self.expect_table()
    }

    fn hhea(&self) -> Result<tables::hhea::Hhea<'a>, ReadError> {
        self.expect_table()
    }

    fn vhea(&self) -> Result<tables::vhea::Vhea<'a>, ReadError> {
        self.expect_table()
    }

    fn hmtx(&self) -> Result<tables::hmtx::Hmtx<'a>, ReadError> {
        //FIXME: should we make the user pass these in?
        let number_of_h_metrics = self.hhea().map(|hhea| hhea.number_of_h_metrics())?;
        let data = self.expect_data_for_tag(tables::hmtx::Hmtx::TAG)?;
        tables::hmtx::Hmtx::read(data, number_of_h_metrics)
    }

    fn hdmx(&self) -> Result<tables::hdmx::Hdmx<'a>, ReadError> {
        let num_glyphs = self.maxp().map(|maxp| maxp.num_glyphs())?;
        let data = self.expect_data_for_tag(tables::hdmx::Hdmx::TAG)?;
        tables::hdmx::Hdmx::read(data, num_glyphs)
    }

    fn vmtx(&self) -> Result<tables::vmtx::Vmtx<'a>, ReadError> {
        //FIXME: should we make the user pass these in?
        let number_of_v_metrics = self.vhea().map(|vhea| vhea.number_of_long_ver_metrics())?;
        let data = self.expect_data_for_tag(tables::vmtx::Vmtx::TAG)?;
        tables::vmtx::Vmtx::read(data, number_of_v_metrics)
    }

    fn vorg(&self) -> Result<tables::vorg::Vorg<'a>, ReadError> {
        self.expect_table()
    }

    fn fvar(&self) -> Result<tables::fvar::Fvar<'a>, ReadError> {
        self.expect_table()
    }

    fn avar(&self) -> Result<tables::avar::Avar<'a>, ReadError> {
        self.expect_table()
    }

    fn hvar(&self) -> Result<tables::hvar::Hvar<'a>, ReadError> {
        self.expect_table()
    }

    fn vvar(&self) -> Result<tables::vvar::Vvar<'a>, ReadError> {
        self.expect_table()
    }

    fn mvar(&self) -> Result<tables::mvar::Mvar<'a>, ReadError> {
        self.expect_table()
    }

    fn maxp(&self) -> Result<tables::maxp::Maxp<'a>, ReadError> {
        self.expect_table()
    }

    fn os2(&self) -> Result<tables::os2::Os2<'a>, ReadError> {
        self.expect_table()
    }

    fn post(&self) -> Result<tables::post::Post<'a>, ReadError> {
        self.expect_table()
    }

    fn gasp(&self) -> Result<tables::gasp::Gasp<'a>, ReadError> {
        self.expect_table()
    }

    /// is_long can be optionally provided, if known, otherwise we look it up in head.
    fn loca(&self, is_long: impl Into<Option<bool>>) -> Result<tables::loca::Loca<'a>, ReadError> {
        let is_long = match is_long.into() {
            Some(val) => val,
            None => self.head()?.index_to_loc_format() == 1,
        };
        let data = self.expect_data_for_tag(tables::loca::Loca::TAG)?;
        tables::loca::Loca::read(data, is_long)
    }

    fn glyf(&self) -> Result<tables::glyf::Glyf<'a>, ReadError> {
        self.expect_table()
    }

    fn gvar(&self) -> Result<tables::gvar::Gvar<'a>, ReadError> {
        self.expect_table()
    }

    /// Returns the array of entries for the control value table which is used
    /// for TrueType hinting.
    fn cvt(&self) -> Result<&'a [BigEndian<i16>], ReadError> {
        let table_data = self.expect_data_for_tag(Tag::new(b"cvt "))?;
        table_data.read_array(0..table_data.len())
    }

    fn cvar(&self) -> Result<tables::cvar::Cvar<'a>, ReadError> {
        self.expect_table()
    }

    fn cff(&self) -> Result<tables::cff::Cff<'a>, ReadError> {
        self.expect_table()
    }

    fn cff2(&self) -> Result<tables::cff2::Cff2<'a>, ReadError> {
        self.expect_table()
    }

    fn cmap(&self) -> Result<tables::cmap::Cmap<'a>, ReadError> {
        self.expect_table()
    }

    fn gdef(&self) -> Result<tables::gdef::Gdef<'a>, ReadError> {
        self.expect_table()
    }

    fn gpos(&self) -> Result<tables::gpos::Gpos<'a>, ReadError> {
        self.expect_table()
    }

    fn gsub(&self) -> Result<tables::gsub::Gsub<'a>, ReadError> {
        self.expect_table()
    }

    fn feat(&self) -> Result<tables::feat::Feat<'a>, ReadError> {
        self.expect_table()
    }

    fn ltag(&self) -> Result<tables::ltag::Ltag<'a>, ReadError> {
        self.expect_table()
    }

    fn ankr(&self) -> Result<tables::ankr::Ankr<'a>, ReadError> {
        self.expect_table()
    }

    fn trak(&self) -> Result<tables::trak::Trak<'a>, ReadError> {
        self.expect_table()
    }

    fn morx(&self) -> Result<tables::morx::Morx<'a>, ReadError> {
        self.expect_table()
    }

    fn kerx(&self) -> Result<tables::kerx::Kerx<'a>, ReadError> {
        self.expect_table()
    }

    fn kern(&self) -> Result<tables::kern::Kern<'a>, ReadError> {
        self.expect_table()
    }

    fn colr(&self) -> Result<tables::colr::Colr<'a>, ReadError> {
        self.expect_table()
    }

    fn cpal(&self) -> Result<tables::cpal::Cpal<'a>, ReadError> {
        self.expect_table()
    }

    fn cblc(&self) -> Result<tables::cblc::Cblc<'a>, ReadError> {
        self.expect_table()
    }

    fn cbdt(&self) -> Result<tables::cbdt::Cbdt<'a>, ReadError> {
        self.expect_table()
    }

    fn eblc(&self) -> Result<tables::eblc::Eblc<'a>, ReadError> {
        self.expect_table()
    }

    fn ebdt(&self) -> Result<tables::ebdt::Ebdt<'a>, ReadError> {
        self.expect_table()
    }

    fn sbix(&self) -> Result<tables::sbix::Sbix<'a>, ReadError> {
        // should we make the user pass this in?
        let num_glyphs = self.maxp().map(|maxp| maxp.num_glyphs())?;
        let data = self.expect_data_for_tag(tables::sbix::Sbix::TAG)?;
        tables::sbix::Sbix::read(data, num_glyphs)
    }

    fn stat(&self) -> Result<tables::stat::Stat<'a>, ReadError> {
        self.expect_table()
    }

    fn svg(&self) -> Result<tables::svg::Svg<'a>, ReadError> {
        self.expect_table()
    }

    fn varc(&self) -> Result<tables::varc::Varc<'a>, ReadError> {
        self.expect_table()
    }

    #[cfg(feature = "ift")]
    fn ift(&self) -> Result<tables::ift::Ift<'a>, ReadError> {
        self.expect_data_for_tag(tables::ift::IFT_TAG)
            .and_then(FontRead::read)
    }

    #[cfg(feature = "ift")]
    fn iftx(&self) -> Result<tables::ift::Ift<'a>, ReadError> {
        self.expect_data_for_tag(tables::ift::IFTX_TAG)
            .and_then(FontRead::read)
    }

    fn meta(&self) -> Result<tables::meta::Meta<'a>, ReadError> {
        self.expect_table()
    }

    fn base(&self) -> Result<tables::base::Base<'a>, ReadError> {
        self.expect_table()
    }

    fn dsig(&self) -> Result<tables::dsig::Dsig<'a>, ReadError> {
        self.expect_table()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// https://github.com/googlefonts/fontations/issues/105
    #[test]
    fn bug_105() {
        // serve some dummy versions of the tables used to compute hmtx. The only
        // fields that matter are maxp::num_glyphs and hhea::number_of_h_metrics,
        // everything else is zero'd out
        struct DummyProvider;
        impl TableProvider<'static> for DummyProvider {
            fn data_for_tag(&self, tag: Tag) -> Option<FontData<'static>> {
                if tag == Tag::new(b"maxp") {
                    Some(FontData::new(&[
                        0, 0, 0x50, 0, // version 0.5
                        0, 3, // num_glyphs = 3
                    ]))
                } else if tag == Tag::new(b"hhea") {
                    Some(FontData::new(&[
                        0, 1, 0, 0, // version 1.0
                        0, 0, 0, 0, // ascender/descender
                        0, 0, 0, 0, // line gap/advance width
                        0, 0, 0, 0, // min left/right side bearing
                        0, 0, 0, 0, // x_max, caret_slope_rise
                        0, 0, 0, 0, // caret_slope_run, caret_offset
                        0, 0, 0, 0, // reserved1/2
                        0, 0, 0, 0, // reserved 3/4
                        0, 0, 0, 1, // metric format, number_of_h_metrics
                    ]))
                } else if tag == Tag::new(b"hmtx") {
                    Some(FontData::new(&[
                        0, 4, 0, 6, // LongHorMetric: 4, 6
                        0, 30, 0, 111, // two lsb entries
                    ]))
                } else {
                    None
                }
            }
        }

        let number_of_h_metrics = DummyProvider.hhea().unwrap().number_of_h_metrics();
        let num_glyphs = DummyProvider.maxp().unwrap().num_glyphs();
        let hmtx = DummyProvider.hmtx().unwrap();

        assert_eq!(number_of_h_metrics, 1);
        assert_eq!(num_glyphs, 3);
        assert_eq!(hmtx.h_metrics().len(), 1);
        assert_eq!(hmtx.left_side_bearings().len(), 2);
    }
}
