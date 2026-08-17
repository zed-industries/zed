//! The [CFF2](https://learn.microsoft.com/en-us/typography/opentype/spec/cff2) table

include!("../../generated/generated_cff2.rs");

use super::postscript::Index2;

/// The [Compact Font Format (CFF) version 2](https://learn.microsoft.com/en-us/typography/opentype/spec/cff2) table
#[derive(Clone)]
pub struct Cff2<'a> {
    header: Cff2Header<'a>,
    global_subrs: Index2<'a>,
}

impl<'a> Cff2<'a> {
    pub fn offset_data(&self) -> FontData<'a> {
        self.header.offset_data()
    }

    pub fn header(&self) -> &Cff2Header<'a> {
        &self.header
    }

    /// Returns the raw data containing the top dict.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#7-top-dict-data>
    pub fn top_dict_data(&self) -> &'a [u8] {
        self.header.top_dict_data()
    }

    /// Returns the global subroutine index.
    ///
    /// This contains sub-programs that are referenced by one or more
    /// charstrings in the font set.
    ///
    /// See "Local/Global Subrs INDEXes" at <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf#page=25>
    pub fn global_subrs(&self) -> Index2<'a> {
        self.global_subrs.clone()
    }
}

impl TopLevelTable for Cff2<'_> {
    const TAG: Tag = Tag::new(b"CFF2");
}

impl<'a> FontRead<'a> for Cff2<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let header = Cff2Header::read(data)?;
        let global_subrs = Index2::read(FontData::new(header.trailing_data()))?;
        Ok(Self {
            header,
            global_subrs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FontData, FontRead, FontRef, TableProvider};

    #[test]
    fn read_example_cff2_table() {
        let cff2 = Cff2::read(FontData::new(font_test_data::cff2::EXAMPLE)).unwrap();
        assert_eq!(cff2.header().major_version(), 2);
        assert_eq!(cff2.header().minor_version(), 0);
        assert_eq!(cff2.header().header_size(), 5);
        assert_eq!(cff2.top_dict_data().len(), 7);
        assert_eq!(cff2.global_subrs().count(), 0);
    }

    #[test]
    fn read_cantarell() {
        let font = FontRef::new(font_test_data::CANTARELL_VF_TRIMMED).unwrap();
        let cff2 = font.cff2().unwrap();
        assert_eq!(cff2.header().major_version(), 2);
        assert_eq!(cff2.header().minor_version(), 0);
        assert_eq!(cff2.header().header_size(), 5);
        assert_eq!(cff2.top_dict_data().len(), 7);
        assert_eq!(cff2.global_subrs().count(), 0);
    }
}
