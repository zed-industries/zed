//! Reading OpenType tables
//!
//! This crate provides memory safe zero-allocation parsing of font files.
//! It is unopinionated, and attempts to provide raw access to the underlying
//! font data as it is described in the [OpenType specification][spec].
//!
//! This crate is intended for use by other parts of a font stack, such as a
//! shaping engine or a glyph rasterizer.
//!
//! In addition to raw data access, this crate may also provide reference
//! implementations of algorithms for interpreting that data, where such an
//! implementation is required for the data to be useful. For instance, we
//! provide functions for [mapping codepoints to glyph identifiers][cmap-impl]
//! using the `cmap` table, or for [decoding entries in the `name` table][NameString].
//!
//! For higher level/more ergonomic access to font data, you may want to look
//! into using [`skrifa`] instead.
//!
//! ## Structure & codegen
//!
//! The root [`tables`] module contains a submodule for each supported
//! [table][table-directory], and that submodule contains items for each table,
//! record, flagset or enum described in the relevant portion of the spec.
//!
//! The majority of the code in the tables module is auto-generated. For more
//! information on our use of codegen, see the [codegen tour].
//!
//! # Related projects
//!
//! - [`write-fonts`] is a companion crate for creating/modifying font files
//! - [`skrifa`] provides access to glyph outlines and metadata (in the same vein
//!   as [freetype])
//!
//! # Example
//!
//! ```no_run
//! # let path_to_my_font_file = std::path::Path::new("");
//! use read_fonts::{FontRef, TableProvider};
//! let font_bytes = std::fs::read(path_to_my_font_file).unwrap();
//! // Single fonts only. for font collections (.ttc) use FontRef::from_index
//! let font = FontRef::new(&font_bytes).expect("failed to read font data");
//! let head = font.head().expect("missing 'head' table");
//! let maxp = font.maxp().expect("missing 'maxp' table");
//!
//! println!("font version {} containing {} glyphs", head.font_revision(), maxp.num_glyphs());
//! ```
//!
//!
//! [spec]: https://learn.microsoft.com/en-us/typography/opentype/spec/
//! [codegen-tour]: https://github.com/googlefonts/fontations/blob/main/docs/codegen-tour.md
//! [cmap-impl]: tables::cmap::Cmap::map_codepoint
//! [`write-fonts`]: https://docs.rs/write-fonts/
//! [`skrifa`]: https://docs.rs/skrifa/
//! [freetype]: http://freetype.org
//! [codegen tour]: https://github.com/googlefonts/fontations/blob/main/docs/codegen-tour.md
//! [NameString]: tables::name::NameString
//! [table-directory]: https://learn.microsoft.com/en-us/typography/opentype/spec/otff#table-directory

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
#[macro_use]
extern crate core as std;

pub mod array;
#[cfg(feature = "std")]
pub mod collections;
mod font_data;
mod offset;
mod offset_array;
mod read;
mod table_provider;
mod table_ref;
pub mod tables;
#[cfg(feature = "experimental_traverse")]
pub mod traversal;

#[cfg(any(test, feature = "codegen_test"))]
pub mod codegen_test;

pub use font_data::FontData;
pub use offset::{Offset, ResolveNullableOffset, ResolveOffset};
pub use offset_array::{ArrayOfNullableOffsets, ArrayOfOffsets};
pub use read::{ComputeSize, FontRead, FontReadWithArgs, ReadArgs, ReadError, VarSize};
pub use table_provider::{TableProvider, TopLevelTable};
pub use table_ref::{MinByteRange, TableRef};

/// Public re-export of the font-types crate.
pub extern crate font_types as types;

/// All the types that may be referenced in auto-generated code.
#[doc(hidden)]
pub(crate) mod codegen_prelude {
    pub use crate::array::{ComputedArray, VarLenArray};
    pub use crate::font_data::{Cursor, FontData};
    pub use crate::offset::{Offset, ResolveNullableOffset, ResolveOffset};
    pub use crate::offset_array::{ArrayOfNullableOffsets, ArrayOfOffsets};
    //pub(crate) use crate::read::sealed;
    pub use crate::read::{
        ComputeSize, FontRead, FontReadWithArgs, Format, ReadArgs, ReadError, VarSize,
    };
    pub use crate::table_provider::TopLevelTable;
    pub use crate::table_ref::{MinByteRange, TableRef};
    pub use std::ops::Range;

    pub use types::*;

    #[cfg(feature = "experimental_traverse")]
    pub use crate::traversal::{self, Field, FieldType, RecordResolver, SomeRecord, SomeTable};

    // used in generated traversal code to get type names of offset fields, which
    // may include generics
    #[cfg(feature = "experimental_traverse")]
    pub(crate) fn better_type_name<T>() -> &'static str {
        let raw_name = std::any::type_name::<T>();
        let last = raw_name.rsplit("::").next().unwrap_or(raw_name);
        // this happens if we end up getting a type name like TableRef<'a, module::SomeMarker>
        last.trim_end_matches("Marker>")
    }

    /// named transforms used in 'count', e.g
    pub(crate) mod transforms {
        pub fn subtract<T: TryInto<usize>, U: TryInto<usize>>(lhs: T, rhs: U) -> usize {
            lhs.try_into()
                .unwrap_or_default()
                .saturating_sub(rhs.try_into().unwrap_or_default())
        }

        pub fn add<T: TryInto<usize>, U: TryInto<usize>>(lhs: T, rhs: U) -> usize {
            lhs.try_into()
                .unwrap_or_default()
                .saturating_add(rhs.try_into().unwrap_or_default())
        }

        #[allow(dead_code)]
        pub fn bitmap_len<T: TryInto<usize>>(count: T) -> usize {
            count.try_into().unwrap_or_default().div_ceil(8)
        }

        #[cfg(feature = "ift")]
        pub fn max_value_bitmap_len<T: TryInto<usize>>(count: T) -> usize {
            let count: usize = count.try_into().unwrap_or_default() + 1usize;
            count.div_ceil(8)
        }

        pub fn add_multiply<T: TryInto<usize>, U: TryInto<usize>, V: TryInto<usize>>(
            a: T,
            b: U,
            c: V,
        ) -> usize {
            a.try_into()
                .unwrap_or_default()
                .saturating_add(b.try_into().unwrap_or_default())
                .saturating_mul(c.try_into().unwrap_or_default())
        }

        #[cfg(feature = "ift")]
        pub fn multiply_add<T: TryInto<usize>, U: TryInto<usize>, V: TryInto<usize>>(
            a: T,
            b: U,
            c: V,
        ) -> usize {
            a.try_into()
                .unwrap_or_default()
                .saturating_mul(b.try_into().unwrap_or_default())
                .saturating_add(c.try_into().unwrap_or_default())
        }

        pub fn half<T: TryInto<usize>>(val: T) -> usize {
            val.try_into().unwrap_or_default() / 2
        }

        pub fn subtract_add_two<T: TryInto<usize>, U: TryInto<usize>>(lhs: T, rhs: U) -> usize {
            lhs.try_into()
                .unwrap_or_default()
                .saturating_sub(rhs.try_into().unwrap_or_default())
                .saturating_add(2)
        }
    }
}

include!("../generated/font.rs");

#[derive(Clone)]
/// Reference to the content of a font or font collection file.
pub enum FileRef<'a> {
    /// A single font.
    Font(FontRef<'a>),
    /// A collection of fonts.
    Collection(CollectionRef<'a>),
}

impl<'a> FileRef<'a> {
    /// Creates a new reference to a file representing a font or font collection.
    pub fn new(data: &'a [u8]) -> Result<Self, ReadError> {
        Ok(if let Ok(collection) = CollectionRef::new(data) {
            Self::Collection(collection)
        } else {
            Self::Font(FontRef::new(data)?)
        })
    }

    /// Returns an iterator over the fonts contained in the file.
    pub fn fonts(&self) -> impl Iterator<Item = Result<FontRef<'a>, ReadError>> + 'a + Clone {
        let (iter_one, iter_two) = match self {
            Self::Font(font) => (Some(Ok(font.clone())), None),
            Self::Collection(collection) => (None, Some(collection.iter())),
        };
        iter_two.into_iter().flatten().chain(iter_one)
    }
}

/// Reference to the content of a font collection file.
#[derive(Clone)]
pub struct CollectionRef<'a> {
    data: FontData<'a>,
    header: TTCHeader<'a>,
}

impl<'a> CollectionRef<'a> {
    /// Creates a new reference to a font collection.
    pub fn new(data: &'a [u8]) -> Result<Self, ReadError> {
        let data = FontData::new(data);
        let header = TTCHeader::read(data)?;
        if header.ttc_tag() != TTC_HEADER_TAG {
            Err(ReadError::InvalidTtc(header.ttc_tag()))
        } else {
            Ok(Self { data, header })
        }
    }

    /// Returns the number of fonts in the collection.
    pub fn len(&self) -> u32 {
        self.header.num_fonts()
    }

    /// Returns true if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the font in the collection at the specified index.
    pub fn get(&self, index: u32) -> Result<FontRef<'a>, ReadError> {
        let offset = self
            .header
            .table_directory_offsets()
            .get(index as usize)
            .ok_or(ReadError::InvalidCollectionIndex(index))?
            .get() as usize;
        let table_dir_data = self.data.slice(offset..).ok_or(ReadError::OutOfBounds)?;
        FontRef::with_table_directory(
            self.data,
            TableDirectory::read(table_dir_data)?,
            Some(index),
        )
    }

    /// Returns an iterator over the fonts in the collection.
    pub fn iter(&self) -> impl Iterator<Item = Result<FontRef<'a>, ReadError>> + 'a + Clone {
        let copy = self.clone();
        (0..self.len()).map(move |ix| copy.get(ix))
    }
}

impl TableDirectory<'_> {
    fn is_sorted(&self) -> bool {
        let mut last_tag = Tag::new(&[0u8; 4]);

        for tag in self.table_records().iter().map(|rec| rec.tag()) {
            if tag <= last_tag {
                return false;
            }

            last_tag = tag;
        }

        true
    }
}

/// Reference to an in-memory font.
///
/// This is a simple implementation of the [`TableProvider`] trait backed
/// by a borrowed slice containing font data.
#[derive(Clone)]
pub struct FontRef<'a> {
    data: FontData<'a>,
    pub table_directory: TableDirectory<'a>,
    /// The index of this font in a TrueType collection
    ttc_index: u32,
    /// Whether this font is a member of a TrueType collection.
    ///
    /// We use a bool rather than an Option to avoid bloating the struct
    /// size.
    in_ttc: bool,
    // Whether the table directory is sorted and thus we can use binary search for
    // finding table records. In principle, fonts are required to have a sorted
    // table directory, but certain fonts don't seem to follow that requirement.
    table_directory_sorted: bool,
}

impl<'a> FontRef<'a> {
    /// Creates a new reference to an in-memory font backed by the given data.
    ///
    /// The data must be a single font (not a font collection) and must begin with a
    /// [table directory] to be considered valid.
    ///
    /// To load a font from a font collection, use [`FontRef::from_index`] instead.
    ///
    /// [table directory]: https://github.com/googlefonts/fontations/pull/549
    pub fn new(data: &'a [u8]) -> Result<Self, ReadError> {
        let data = FontData::new(data);
        Self::with_table_directory(data, TableDirectory::read(data)?, None)
    }

    /// Creates a new reference to an in-memory font at the specified index
    /// backed by the given data.
    ///
    /// The data slice must begin with either a
    /// [table directory](https://learn.microsoft.com/en-us/typography/opentype/spec/otff#table-directory)
    /// or a [ttc header](https://learn.microsoft.com/en-us/typography/opentype/spec/otff#ttc-header)
    /// to be considered valid.
    ///
    /// In other words, this accepts either font collection (ttc) or single
    /// font (ttf/otf) files. If a single font file is provided, the index
    /// parameter must be 0.
    pub fn from_index(data: &'a [u8], index: u32) -> Result<Self, ReadError> {
        let file = FileRef::new(data)?;
        match file {
            FileRef::Font(font) => {
                if index == 0 {
                    Ok(font)
                } else {
                    Err(ReadError::InvalidCollectionIndex(index))
                }
            }
            FileRef::Collection(collection) => collection.get(index),
        }
    }

    /// Returns the underlying font data.
    ///
    /// This is the base from which tables are loaded, meaning that for
    /// TrueType collection files, this will be the entire font file data.
    pub fn data(&self) -> FontData<'a> {
        self.data
    }

    /// If the font is in a TrueType collection (ttc) file, returns the index
    /// of the font in that collection.
    pub fn ttc_index(&self) -> Option<u32> {
        self.in_ttc.then_some(self.ttc_index)
    }

    /// Returns the associated table directory.
    pub fn table_directory(&self) -> &TableDirectory<'a> {
        &self.table_directory
    }

    /// Returns the data for the table with the specified tag, if present.
    pub fn table_data(&self, tag: Tag) -> Option<FontData<'a>> {
        let entry = if self.table_directory_sorted {
            self.table_directory
                .table_records()
                .binary_search_by(|rec| rec.tag.get().cmp(&tag))
                .ok()
        } else {
            self.table_directory
                .table_records()
                .iter()
                .position(|rec| rec.tag.get().eq(&tag))
        };

        entry
            .and_then(|idx| self.table_directory.table_records().get(idx))
            .and_then(|record| {
                let start = Offset32::new(record.offset()).non_null()?;
                let len = record.length() as usize;
                self.data.slice(start..start.checked_add(len)?)
            })
    }

    /// Returns an iterator over all of the available fonts in
    /// the given font data.
    pub fn fonts(
        data: &'a [u8],
    ) -> impl Iterator<Item = Result<FontRef<'a>, ReadError>> + 'a + Clone {
        let count = match FileRef::new(data) {
            Ok(FileRef::Font(_)) => 1,
            Ok(FileRef::Collection(ttc)) => ttc.len(),
            _ => 0,
        };
        (0..count).map(|idx| FontRef::from_index(data, idx))
    }

    fn with_table_directory(
        data: FontData<'a>,
        table_directory: TableDirectory<'a>,
        ttc_index: Option<u32>,
    ) -> Result<Self, ReadError> {
        if [TT_SFNT_VERSION, CFF_SFNT_VERSION, TRUE_SFNT_VERSION]
            .contains(&table_directory.sfnt_version())
        {
            let table_directory_sorted = table_directory.is_sorted();

            Ok(FontRef {
                data,
                table_directory,
                ttc_index: ttc_index.unwrap_or_default(),
                in_ttc: ttc_index.is_some(),
                table_directory_sorted,
            })
        } else {
            Err(ReadError::InvalidSfnt(table_directory.sfnt_version()))
        }
    }
}

impl<'a> TableProvider<'a> for FontRef<'a> {
    fn data_for_tag(&self, tag: Tag) -> Option<FontData<'a>> {
        self.table_data(tag)
    }
}

#[cfg(test)]
mod tests {
    use font_test_data::{be_buffer, bebuffer::BeBuffer, ttc::TTC, AHEM};
    use types::{Tag, TT_SFNT_VERSION};

    use crate::{FileRef, FontRef};

    #[test]
    fn file_ref_non_collection() {
        assert!(matches!(FileRef::new(AHEM), Ok(FileRef::Font(_))));
    }

    #[test]
    fn file_ref_collection() {
        let Ok(FileRef::Collection(collection)) = FileRef::new(TTC) else {
            panic!("Expected a collection");
        };
        assert_eq!(2, collection.len());
        assert!(!collection.is_empty());
    }

    #[test]
    fn font_ref_fonts_iter() {
        assert_eq!(FontRef::fonts(AHEM).count(), 1);
        assert_eq!(FontRef::fonts(TTC).count(), 2);
        assert_eq!(FontRef::fonts(b"NOT_A_FONT").count(), 0);
    }

    #[test]
    fn ttc_index() {
        for (idx, font) in FontRef::fonts(TTC).map(|font| font.unwrap()).enumerate() {
            assert_eq!(font.ttc_index(), Some(idx as u32));
        }
        assert!(FontRef::new(AHEM).unwrap().ttc_index().is_none());
    }

    #[test]
    fn unsorted_table_directory() {
        let cff2_data = font_test_data::cff2::EXAMPLE;
        let post_data = font_test_data::post::SIMPLE;
        let gdef_data = [
            font_test_data::gdef::GDEF_HEADER,
            font_test_data::gdef::GLYPHCLASSDEF_TABLE,
        ]
        .concat();
        let gpos_data = font_test_data::gpos::SINGLEPOSFORMAT1;

        let font_data = be_buffer! {
            TT_SFNT_VERSION,
            4u16,    // num tables
            64u16,   // search range
            2u16,    // entry selector
            0u16,    // range shift

            (Tag::new(b"post")),
            0u32,    // checksum
            76u32,   // offset
            (post_data.len() as u32),

            (Tag::new(b"GPOS")),
            0u32,    // checksum
            108u32,  // offset
            (gpos_data.len() as u32),

            (Tag::new(b"GDEF")),
            0u32,    // checksum
            128u32,  // offset
            (gdef_data.len() as u32),

            (Tag::new(b"CFF2")),
            0u32,    // checksum
            160u32,  // offset
            (cff2_data.len() as u32)
        };

        let mut full_font = font_data.to_vec();

        full_font.extend_from_slice(post_data);
        full_font.extend_from_slice(gpos_data);
        full_font.extend_from_slice(&gdef_data);
        full_font.extend_from_slice(cff2_data);

        let font = FontRef::new(&full_font).unwrap();

        assert!(!font.table_directory_sorted);

        assert!(font.table_data(Tag::new(b"CFF2")).is_some());
        assert!(font.table_data(Tag::new(b"GDEF")).is_some());
        assert!(font.table_data(Tag::new(b"GPOS")).is_some());
        assert!(font.table_data(Tag::new(b"post")).is_some());
    }
}
