use crate::common::{DebugAddrBase, DebugAddrIndex, DebugAddrOffset, Encoding, SectionId};
use crate::read::{Error, Reader, ReaderOffset, Result, Section};

/// The raw contents of the `.debug_addr` section.
#[derive(Debug, Default, Clone, Copy)]
pub struct DebugAddr<R> {
    section: R,
}

impl<R: Reader> DebugAddr<R> {
    /// Returns the address at the given `base` and `index`.
    ///
    /// A set of addresses in the `.debug_addr` section consists of a header
    /// followed by a series of addresses.
    ///
    /// The `base` must be the `DW_AT_addr_base` value from the compilation unit DIE.
    /// This is an offset that points to the first address following the header.
    ///
    /// The `index` is the value of a `DW_FORM_addrx` attribute.
    ///
    /// The `address_size` must be the size of the address for the compilation unit.
    /// This value must also match the header. However, note that we do not parse the
    /// header to validate this, since locating the header is unreliable, and the GNU
    /// extensions do not emit it.
    pub fn get_address(
        &self,
        address_size: u8,
        base: DebugAddrBase<R::Offset>,
        index: DebugAddrIndex<R::Offset>,
    ) -> Result<u64> {
        let input = &mut self.section.clone();
        input.skip(base.0)?;
        input.skip(R::Offset::from_u64(
            index.0.into_u64() * u64::from(address_size),
        )?)?;
        input.read_address(address_size)
    }

    /// Iterate the sets of entries in the `.debug_addr` section.
    ///
    /// Each set of entries belongs to a single unit.
    pub fn headers(&self) -> AddrHeaderIter<R> {
        AddrHeaderIter {
            input: self.section.clone(),
            offset: DebugAddrOffset(R::Offset::from_u8(0)),
        }
    }
}

impl<T> DebugAddr<T> {
    /// Create a `DebugAddr` section that references the data in `self`.
    ///
    /// This is useful when `R` implements `Reader` but `T` does not.
    ///
    /// Used by `DwarfSections::borrow`.
    pub fn borrow<'a, F, R>(&'a self, mut borrow: F) -> DebugAddr<R>
    where
        F: FnMut(&'a T) -> R,
    {
        borrow(&self.section).into()
    }
}

impl<R> Section<R> for DebugAddr<R> {
    fn id() -> SectionId {
        SectionId::DebugAddr
    }

    fn reader(&self) -> &R {
        &self.section
    }
}

impl<R> From<R> for DebugAddr<R> {
    fn from(section: R) -> Self {
        DebugAddr { section }
    }
}

/// An iterator over the headers of a `.debug_addr` section.
#[derive(Clone, Debug)]
pub struct AddrHeaderIter<R: Reader> {
    input: R,
    offset: DebugAddrOffset<R::Offset>,
}

impl<R: Reader> AddrHeaderIter<R> {
    /// Advance the iterator to the next header.
    pub fn next(&mut self) -> Result<Option<AddrHeader<R>>> {
        if self.input.is_empty() {
            return Ok(None);
        }

        let len = self.input.len();
        match AddrHeader::parse(&mut self.input, self.offset) {
            Ok(header) => {
                self.offset.0 += len - self.input.len();
                Ok(Some(header))
            }
            Err(e) => {
                self.input.empty();
                Err(e)
            }
        }
    }
}

#[cfg(feature = "fallible-iterator")]
impl<R: Reader> fallible_iterator::FallibleIterator for AddrHeaderIter<R> {
    type Item = AddrHeader<R>;
    type Error = Error;

    fn next(&mut self) -> ::core::result::Result<Option<Self::Item>, Self::Error> {
        AddrHeaderIter::next(self)
    }
}

/// A header for a set of entries in the `.debug_addr` section.
///
/// These entries all belong to a single unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddrHeader<R, Offset = <R as Reader>::Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset,
{
    offset: DebugAddrOffset<Offset>,
    encoding: Encoding,
    length: Offset,
    entries: R,
}

impl<R, Offset> AddrHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset,
{
    fn parse(input: &mut R, offset: DebugAddrOffset<Offset>) -> Result<Self> {
        let (length, format) = input.read_initial_length()?;
        let mut rest = input.split(length)?;

        // Check the version. The DWARF 5 spec says that this is always 5.
        let version = rest.read_u16()?;
        if version != 5 {
            return Err(Error::UnknownVersion(u64::from(version)));
        }

        let address_size = rest.read_address_size()?;
        let segment_size = rest.read_u8()?;
        if segment_size != 0 {
            return Err(Error::UnsupportedSegmentSize);
        }

        // unit_length + version + address_size + segment_size
        let header_length = format.initial_length_size() + 2 + 1 + 1;

        // The first tuple following the header in each set begins at an offset that is
        // a multiple of the size of a single tuple (that is, the size of a segment,
        // which must be zero, and an address).
        let tuple_length = address_size;
        if tuple_length == 0 {
            return Err(Error::UnsupportedAddressSize(address_size));
        }
        let padding = if header_length % tuple_length == 0 {
            0
        } else {
            tuple_length - header_length % tuple_length
        };
        rest.skip(R::Offset::from_u8(padding))?;

        let encoding = Encoding {
            format,
            version,
            address_size,
        };
        Ok(AddrHeader {
            offset,
            encoding,
            length,
            entries: rest,
        })
    }

    /// Return the offset of this header within the `.debug_addr` section.
    #[inline]
    pub fn offset(&self) -> DebugAddrOffset<Offset> {
        self.offset
    }

    /// Return the length of this set of entries, including the header.
    #[inline]
    pub fn length(&self) -> Offset {
        self.length
    }

    /// Return the encoding parameters for this set of entries.
    #[inline]
    pub fn encoding(&self) -> Encoding {
        self.encoding
    }

    /// Return the address entries in this set.
    #[inline]
    pub fn entries(&self) -> AddrEntryIter<R> {
        AddrEntryIter {
            input: self.entries.clone(),
            encoding: self.encoding,
        }
    }
}

/// An iterator over the addresses from a `.debug_addr` section.
///
/// Can be [used with
/// `FallibleIterator`](./index.html#using-with-fallibleiterator).
#[derive(Debug, Clone)]
pub struct AddrEntryIter<R: Reader> {
    input: R,
    encoding: Encoding,
}

impl<R: Reader> AddrEntryIter<R> {
    /// Advance the iterator and return the next address.
    ///
    /// Returns the newly parsed address as `Ok(Some(addr))`. Returns `Ok(None)`
    /// when iteration is complete and all addresses have already been parsed and
    /// yielded. If an error occurs while parsing the next address, then this error
    /// is returned as `Err(e)`, and all subsequent calls return `Ok(None)`.
    pub fn next(&mut self) -> Result<Option<u64>> {
        if self.input.is_empty() {
            return Ok(None);
        }

        match self.input.read_address(self.encoding.address_size) {
            Ok(entry) => Ok(Some(entry)),
            Err(e) => {
                self.input.empty();
                Err(e)
            }
        }
    }
}

#[cfg(feature = "fallible-iterator")]
impl<R: Reader> fallible_iterator::FallibleIterator for AddrEntryIter<R> {
    type Item = u64;
    type Error = Error;

    fn next(&mut self) -> ::core::result::Result<Option<Self::Item>, Self::Error> {
        AddrEntryIter::next(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read::EndianSlice;
    use crate::test_util::GimliSectionMethods;
    use crate::{Format, LittleEndian};
    use test_assembler::{Endian, Label, LabelMaker, Section};

    #[test]
    fn test_get_address() {
        for format in [Format::Dwarf32, Format::Dwarf64] {
            for address_size in [4, 8] {
                let zero = Label::new();
                let length = Label::new();
                let start = Label::new();
                let first = Label::new();
                let end = Label::new();
                let mut section = Section::with_endian(Endian::Little)
                    .mark(&zero)
                    .initial_length(format, &length, &start)
                    .D16(5)
                    .D8(address_size)
                    .D8(0)
                    .mark(&first);
                for i in 0..20 {
                    section = section.word(address_size, 1000 + i);
                }
                section = section.mark(&end);
                length.set_const((&end - &start) as u64);

                let section = section.get_contents().unwrap();
                let debug_addr = DebugAddr::from(EndianSlice::new(&section, LittleEndian));
                let base = DebugAddrBase((&first - &zero) as usize);

                assert_eq!(
                    debug_addr.get_address(address_size, base, DebugAddrIndex(0)),
                    Ok(1000)
                );
                assert_eq!(
                    debug_addr.get_address(address_size, base, DebugAddrIndex(19)),
                    Ok(1019)
                );
            }
        }
    }

    #[test]
    fn test_iterator() {
        let length = Label::new();
        let start = Label::new();
        let end = Label::new();
        // First CU.
        let mut section = Section::with_endian(Endian::Little)
            .initial_length(Format::Dwarf32, &length, &start)
            .D16(5) // Version
            .D8(4) // Address size
            .D8(0) // Segment size
            .word(4, 0x12345678)
            .word(4, 0xdeadbeef)
            .mark(&end);
        length.set_const((&end - &start) as u64);
        // Second CU.
        let length = Label::new();
        let start = Label::new();
        let end = Label::new();
        section = section
            .initial_length(Format::Dwarf64, &length, &start)
            .D16(5) // Version
            .D8(8) // Address size
            .D8(0) // Segment size
            .word(8, 0x123456789abcdef0)
            .word(8, 0xdeadbeefdeadbeef)
            .mark(&end);
        length.set_const((&end - &start) as u64);
        let section = section.get_contents().unwrap();
        let debug_addr = DebugAddr::from(EndianSlice::new(&section, LittleEndian));
        let mut iter = debug_addr.headers();
        let first_header = iter.next().unwrap().unwrap();
        let first_encoding = first_header.encoding();
        assert_eq!(first_encoding.address_size, 4);
        assert_eq!(first_encoding.format, Format::Dwarf32);
        assert_eq!(first_encoding.version, 5);
        assert_eq!(first_header.length(), 12);
        let mut first_entries = first_header.entries();
        assert_eq!(first_entries.next(), Ok(Some(0x12345678)));
        assert_eq!(first_entries.next(), Ok(Some(0xdeadbeef)));
        assert_eq!(first_entries.next(), Ok(None));
        let second_header = iter.next().unwrap().unwrap();
        let second_encoding = second_header.encoding();
        assert_eq!(second_encoding.address_size, 8);
        assert_eq!(second_encoding.format, Format::Dwarf64);
        assert_eq!(second_encoding.version, 5);
        assert_eq!(second_header.length(), 20);
        let mut second_entries = second_header.entries();
        assert_eq!(second_entries.next(), Ok(Some(0x123456789abcdef0)));
        assert_eq!(second_entries.next(), Ok(Some(0xdeadbeefdeadbeef)));
        assert_eq!(second_entries.next(), Ok(None));
        assert_eq!(iter.next(), Ok(None));
    }
}
