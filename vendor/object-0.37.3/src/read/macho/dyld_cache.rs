use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{self, Debug};
use core::{mem, slice};

use crate::endian::{Endian, Endianness, U16, U32, U64};
use crate::macho;
use crate::read::{Architecture, Error, File, ReadError, ReadRef, Result};

/// A parsed representation of the dyld shared cache.
#[derive(Debug)]
pub struct DyldCache<'data, E = Endianness, R = &'data [u8]>
where
    E: Endian,
    R: ReadRef<'data>,
{
    endian: E,
    data: R,
    /// The first entry is the main cache file, and the rest are subcaches.
    files: Vec<DyldFile<'data, E, R>>,
    images: &'data [macho::DyldCacheImageInfo<E>],
    arch: Architecture,
}

/// A slice of structs describing each subcache.
///
/// The struct gained an additional field (the file suffix) in dyld-1042.1 (macOS 13 / iOS 16),
/// so this is an enum of the two possible slice types.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum DyldSubCacheSlice<'data, E: Endian> {
    /// V1, used between dyld-940 and dyld-1042.1.
    V1(&'data [macho::DyldSubCacheEntryV1<E>]),
    /// V2, used since dyld-1042.1.
    V2(&'data [macho::DyldSubCacheEntryV2<E>]),
}

// This is the offset of the end of the images_count field.
const MIN_HEADER_SIZE_SUBCACHES_V1: u32 = 0x1c8;

// This is the offset of the end of the cache_sub_type field.
const MIN_HEADER_SIZE_SUBCACHES_V2: u32 = 0x1d0;

impl<'data, E, R> DyldCache<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data>,
{
    /// Return the suffixes of the subcache files given the data of the main cache file.
    ///
    /// Each of these should be appended to the path of the main cache file.
    pub fn subcache_suffixes(data: R) -> Result<Vec<String>> {
        let header = macho::DyldCacheHeader::<E>::parse(data)?;
        let (_arch, endian) = header.parse_magic()?;
        let Some(subcaches_info) = header.subcaches(endian, data)? else {
            return Ok(Vec::new());
        };
        let mut subcache_suffixes: Vec<String> = match subcaches_info {
            DyldSubCacheSlice::V1(subcaches) => {
                // macOS 12: Subcaches have the file suffixes .1, .2, .3 etc.
                (1..subcaches.len() + 1).map(|i| format!(".{i}")).collect()
            }
            DyldSubCacheSlice::V2(subcaches) => {
                // macOS 13+: The subcache file suffix is written down in the header of the main cache.
                subcaches
                    .iter()
                    .map(|s| {
                        // The suffix is a nul-terminated string in a fixed-size byte array.
                        let suffix = s.file_suffix;
                        let len = suffix.iter().position(|&c| c == 0).unwrap_or(suffix.len());
                        String::from_utf8_lossy(&suffix[..len]).to_string()
                    })
                    .collect()
            }
        };
        if header.symbols_subcache_uuid(endian).is_some() {
            subcache_suffixes.push(".symbols".to_string());
        }
        Ok(subcache_suffixes)
    }

    /// Parse the raw dyld shared cache data.
    ///
    /// For shared caches from macOS 12 / iOS 15 and above, the subcache files need to be
    /// supplied as well, in the correct order. Use [`Self::subcache_suffixes`] to obtain
    /// the suffixes for the path of the files.
    pub fn parse(data: R, subcache_data: &[R]) -> Result<Self> {
        let header = macho::DyldCacheHeader::parse(data)?;
        let (arch, endian) = header.parse_magic()?;

        let mut files = Vec::new();
        let mappings = header.mappings(endian, data)?;
        files.push(DyldFile { data, mappings });

        let symbols_subcache_uuid = header.symbols_subcache_uuid(endian);
        let subcaches_info = header.subcaches(endian, data)?;
        let subcaches_count = match subcaches_info {
            Some(DyldSubCacheSlice::V1(subcaches)) => subcaches.len(),
            Some(DyldSubCacheSlice::V2(subcaches)) => subcaches.len(),
            None => 0,
        };
        if subcache_data.len() != subcaches_count + symbols_subcache_uuid.is_some() as usize {
            return Err(Error("Incorrect number of SubCaches"));
        }

        // Split out the .symbols subcache data from the other subcaches.
        let (symbols_subcache_data_and_uuid, subcache_data) =
            if let Some(symbols_uuid) = symbols_subcache_uuid {
                let (sym_data, rest_data) = subcache_data.split_last().unwrap();
                (Some((*sym_data, symbols_uuid)), rest_data)
            } else {
                (None, subcache_data)
            };

        // Read the regular SubCaches, if present.
        if let Some(subcaches_info) = subcaches_info {
            let (v1, v2) = match subcaches_info {
                DyldSubCacheSlice::V1(s) => (s, &[][..]),
                DyldSubCacheSlice::V2(s) => (&[][..], s),
            };
            let uuids = v1.iter().map(|e| &e.uuid).chain(v2.iter().map(|e| &e.uuid));
            for (&data, uuid) in subcache_data.iter().zip(uuids) {
                let header = macho::DyldCacheHeader::<E>::parse(data)?;
                if &header.uuid != uuid {
                    return Err(Error("Unexpected SubCache UUID"));
                }
                let mappings = header.mappings(endian, data)?;
                files.push(DyldFile { data, mappings });
            }
        }

        // Read the .symbols SubCache, if present.
        // Other than the UUID verification, the symbols SubCache is currently unused.
        let _symbols_subcache = match symbols_subcache_data_and_uuid {
            Some((data, uuid)) => {
                let header = macho::DyldCacheHeader::<E>::parse(data)?;
                if header.uuid != uuid {
                    return Err(Error("Unexpected .symbols SubCache UUID"));
                }
                let mappings = header.mappings(endian, data)?;
                Some(DyldFile { data, mappings })
            }
            None => None,
        };

        let images = header.images(endian, data)?;
        Ok(DyldCache {
            endian,
            data,
            files,
            images,
            arch,
        })
    }

    /// Get the architecture type of the file.
    pub fn architecture(&self) -> Architecture {
        self.arch
    }

    /// Get the endianness of the file.
    #[inline]
    pub fn endianness(&self) -> Endianness {
        if self.is_little_endian() {
            Endianness::Little
        } else {
            Endianness::Big
        }
    }

    /// Get the data of the main cache file.
    #[inline]
    pub fn data(&self) -> R {
        self.data
    }

    /// Return true if the file is little endian, false if it is big endian.
    pub fn is_little_endian(&self) -> bool {
        self.endian.is_little_endian()
    }

    /// Iterate over the images in this cache.
    pub fn images<'cache>(&'cache self) -> DyldCacheImageIterator<'data, 'cache, E, R> {
        DyldCacheImageIterator {
            cache: self,
            iter: self.images.iter(),
        }
    }

    /// Return all the mappings in this cache.
    pub fn mappings<'cache>(
        &'cache self,
    ) -> impl Iterator<Item = DyldCacheMapping<'data, E, R>> + 'cache {
        let endian = self.endian;
        self.files
            .iter()
            .flat_map(move |file| file.mappings(endian))
    }

    /// Find the address in a mapping and return the cache or subcache data it was found in,
    /// together with the translated file offset.
    pub fn data_and_offset_for_address(&self, address: u64) -> Option<(R, u64)> {
        for file in &self.files {
            if let Some(file_offset) = file.address_to_file_offset(self.endian, address) {
                return Some((file.data, file_offset));
            }
        }
        None
    }
}

/// The data for one file in the cache.
#[derive(Debug)]
struct DyldFile<'data, E = Endianness, R = &'data [u8]>
where
    E: Endian,
    R: ReadRef<'data>,
{
    data: R,
    mappings: DyldCacheMappingSlice<'data, E>,
}

impl<'data, E, R> DyldFile<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data>,
{
    /// Return an iterator for the mappings.
    fn mappings(&self, endian: E) -> DyldCacheMappingIterator<'data, E, R> {
        let iter = match self.mappings {
            DyldCacheMappingSlice::V1(info) => DyldCacheMappingVersionIterator::V1(info.iter()),
            DyldCacheMappingSlice::V2(info) => DyldCacheMappingVersionIterator::V2(info.iter()),
        };
        DyldCacheMappingIterator {
            endian,
            data: self.data,
            iter,
        }
    }

    /// Find the file offset an address in the mappings.
    fn address_to_file_offset(&self, endian: E, address: u64) -> Option<u64> {
        for mapping in self.mappings(endian) {
            let mapping_address = mapping.address();
            if address >= mapping_address && address < mapping_address.wrapping_add(mapping.size())
            {
                return Some(address - mapping_address + mapping.file_offset());
            }
        }
        None
    }
}

/// An iterator over all the images (dylibs) in the dyld shared cache.
#[derive(Debug)]
pub struct DyldCacheImageIterator<'data, 'cache, E = Endianness, R = &'data [u8]>
where
    E: Endian,
    R: ReadRef<'data>,
{
    cache: &'cache DyldCache<'data, E, R>,
    iter: slice::Iter<'data, macho::DyldCacheImageInfo<E>>,
}

impl<'data, 'cache, E, R> Iterator for DyldCacheImageIterator<'data, 'cache, E, R>
where
    E: Endian,
    R: ReadRef<'data>,
{
    type Item = DyldCacheImage<'data, 'cache, E, R>;

    fn next(&mut self) -> Option<DyldCacheImage<'data, 'cache, E, R>> {
        let image_info = self.iter.next()?;
        Some(DyldCacheImage {
            cache: self.cache,
            image_info,
        })
    }
}

/// One image (dylib) from inside the dyld shared cache.
#[derive(Debug)]
pub struct DyldCacheImage<'data, 'cache, E = Endianness, R = &'data [u8]>
where
    E: Endian,
    R: ReadRef<'data>,
{
    pub(crate) cache: &'cache DyldCache<'data, E, R>,
    image_info: &'data macho::DyldCacheImageInfo<E>,
}

impl<'data, 'cache, E, R> DyldCacheImage<'data, 'cache, E, R>
where
    E: Endian,
    R: ReadRef<'data>,
{
    /// Return the raw data structure for this image.
    pub fn info(&self) -> &'data macho::DyldCacheImageInfo<E> {
        self.image_info
    }

    /// The file system path of this image.
    pub fn path(&self) -> Result<&'data str> {
        let path = self.image_info.path(self.cache.endian, self.cache.data)?;
        // The path should always be ascii, so from_utf8 should always succeed.
        let path = core::str::from_utf8(path).map_err(|_| Error("Path string not valid utf-8"))?;
        Ok(path)
    }

    /// The subcache data which contains the Mach-O header for this image,
    /// together with the file offset at which this image starts.
    pub fn image_data_and_offset(&self) -> Result<(R, u64)> {
        let address = self.image_info.address.get(self.cache.endian);
        self.cache
            .data_and_offset_for_address(address)
            .ok_or(Error("Address not found in any mapping"))
    }

    /// Parse this image into an Object.
    pub fn parse_object(&self) -> Result<File<'data, R>> {
        File::parse_dyld_cache_image(self)
    }
}

/// The array of mappings for a single dyld cache file.
///
/// The mappings gained slide info in dyld-832.7 (macOS 11)
/// so this is an enum of the two possible slice types.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum DyldCacheMappingSlice<'data, E: Endian = Endianness> {
    /// V1, used before dyld-832.7.
    V1(&'data [macho::DyldCacheMappingInfo<E>]),
    /// V2, used since dyld-832.7.
    V2(&'data [macho::DyldCacheMappingAndSlideInfo<E>]),
}

// This is the offset of the end of the mapping_with_slide_count field.
const MIN_HEADER_SIZE_MAPPINGS_V2: u32 = 0x140;

/// An iterator over all the mappings for one subcache in a dyld shared cache.
#[derive(Debug)]
pub struct DyldCacheMappingIterator<'data, E = Endianness, R = &'data [u8]>
where
    E: Endian,
    R: ReadRef<'data>,
{
    endian: E,
    data: R,
    iter: DyldCacheMappingVersionIterator<'data, E>,
}

#[derive(Debug)]
enum DyldCacheMappingVersionIterator<'data, E = Endianness>
where
    E: Endian,
{
    V1(slice::Iter<'data, macho::DyldCacheMappingInfo<E>>),
    V2(slice::Iter<'data, macho::DyldCacheMappingAndSlideInfo<E>>),
}

impl<'data, E, R> Iterator for DyldCacheMappingIterator<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data>,
{
    type Item = DyldCacheMapping<'data, E, R>;

    fn next(&mut self) -> Option<Self::Item> {
        let info = match &mut self.iter {
            DyldCacheMappingVersionIterator::V1(iter) => DyldCacheMappingVersion::V1(iter.next()?),
            DyldCacheMappingVersionIterator::V2(iter) => DyldCacheMappingVersion::V2(iter.next()?),
        };
        Some(DyldCacheMapping {
            endian: self.endian,
            data: self.data,
            info,
        })
    }
}

/// Information about a mapping.
#[derive(Clone, Copy)]
pub struct DyldCacheMapping<'data, E = Endianness, R = &'data [u8]>
where
    E: Endian,
    R: ReadRef<'data>,
{
    endian: E,
    data: R,
    info: DyldCacheMappingVersion<'data, E>,
}

#[derive(Clone, Copy)]
enum DyldCacheMappingVersion<'data, E = Endianness>
where
    E: Endian,
{
    V1(&'data macho::DyldCacheMappingInfo<E>),
    V2(&'data macho::DyldCacheMappingAndSlideInfo<E>),
}

impl<'data, E, R> Debug for DyldCacheMapping<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DyldCacheMapping")
            .field("address", &format_args!("{:#x}", self.address()))
            .field("size", &format_args!("{:#x}", self.size()))
            .field("file_offset", &format_args!("{:#x}", self.file_offset()))
            .field("max_prot", &format_args!("{:#x}", self.max_prot()))
            .field("init_prot", &format_args!("{:#x}", self.init_prot()))
            .finish()
    }
}

impl<'data, E, R> DyldCacheMapping<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data>,
{
    /// The mapping address
    pub fn address(&self) -> u64 {
        match self.info {
            DyldCacheMappingVersion::V1(info) => info.address.get(self.endian),
            DyldCacheMappingVersion::V2(info) => info.address.get(self.endian),
        }
    }

    /// The mapping size
    pub fn size(&self) -> u64 {
        match self.info {
            DyldCacheMappingVersion::V1(info) => info.size.get(self.endian),
            DyldCacheMappingVersion::V2(info) => info.size.get(self.endian),
        }
    }

    /// The mapping file offset
    pub fn file_offset(&self) -> u64 {
        match self.info {
            DyldCacheMappingVersion::V1(info) => info.file_offset.get(self.endian),
            DyldCacheMappingVersion::V2(info) => info.file_offset.get(self.endian),
        }
    }

    /// The mapping maximum protection
    pub fn max_prot(&self) -> u32 {
        match self.info {
            DyldCacheMappingVersion::V1(info) => info.max_prot.get(self.endian),
            DyldCacheMappingVersion::V2(info) => info.max_prot.get(self.endian),
        }
    }

    /// The mapping initial protection
    pub fn init_prot(&self) -> u32 {
        match self.info {
            DyldCacheMappingVersion::V1(info) => info.init_prot.get(self.endian),
            DyldCacheMappingVersion::V2(info) => info.init_prot.get(self.endian),
        }
    }

    /// The mapping data
    pub fn data(&self) -> Result<&'data [u8]> {
        self.data
            .read_bytes_at(self.file_offset(), self.size())
            .read_error("Failed to read bytes for mapping")
    }

    /// Relocations for the mapping
    pub fn relocations(&self) -> Result<DyldCacheRelocationIterator<'data, E, R>> {
        let data = self.data;
        let endian = self.endian;
        let version = match self.info {
            DyldCacheMappingVersion::V1(_) => DyldCacheRelocationIteratorVersion::None,
            DyldCacheMappingVersion::V2(mapping) => match mapping.slide(self.endian, self.data)? {
                DyldCacheSlideInfo::None => DyldCacheRelocationIteratorVersion::None,
                DyldCacheSlideInfo::V2 {
                    slide,
                    page_starts,
                    page_extras,
                } => {
                    let delta_mask = slide.delta_mask.get(endian);
                    let delta_shift = delta_mask.trailing_zeros();
                    DyldCacheRelocationIteratorVersion::V2(DyldCacheRelocationIteratorV2 {
                        data,
                        endian,
                        mapping_file_offset: mapping.file_offset.get(endian),
                        page_size: slide.page_size.get(endian).into(),
                        delta_mask,
                        delta_shift,
                        value_add: slide.value_add.get(endian),
                        page_starts,
                        page_extras,
                        state: RelocationStateV2::Start,
                        start_index: 0,
                        extra_index: 0,
                        page_offset: 0,
                        offset: 0,
                    })
                }
                DyldCacheSlideInfo::V3 { slide, page_starts } => {
                    DyldCacheRelocationIteratorVersion::V3(DyldCacheRelocationIteratorV3 {
                        data,
                        endian,
                        mapping_file_offset: mapping.file_offset.get(endian),
                        page_size: slide.page_size.get(endian).into(),
                        auth_value_add: slide.auth_value_add.get(endian),
                        page_starts,
                        state: RelocationStateV3::Start,
                        start_index: 0,
                        offset: 0,
                    })
                }
                DyldCacheSlideInfo::V5 { slide, page_starts } => {
                    DyldCacheRelocationIteratorVersion::V5(DyldCacheRelocationIteratorV5 {
                        data,
                        endian,
                        mapping_file_offset: mapping.file_offset.get(endian),
                        page_size: slide.page_size.get(endian).into(),
                        value_add: slide.value_add.get(endian),
                        page_starts,
                        state: RelocationStateV5::Start,
                        start_index: 0,
                        offset: 0,
                    })
                }
            },
        };
        Ok(DyldCacheRelocationIterator { version })
    }
}

/// The slide info for a dyld cache mapping, including variable length arrays.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum DyldCacheSlideInfo<'data, E: Endian> {
    None,
    V2 {
        slide: &'data macho::DyldCacheSlideInfo2<E>,
        page_starts: &'data [U16<E>],
        page_extras: &'data [U16<E>],
    },
    V3 {
        slide: &'data macho::DyldCacheSlideInfo3<E>,
        page_starts: &'data [U16<E>],
    },
    V5 {
        slide: &'data macho::DyldCacheSlideInfo5<E>,
        page_starts: &'data [U16<E>],
    },
}

/// An iterator over relocations in a mapping
#[derive(Debug)]
pub struct DyldCacheRelocationIterator<'data, E = Endianness, R = &'data [u8]>
where
    E: Endian,
    R: ReadRef<'data>,
{
    version: DyldCacheRelocationIteratorVersion<'data, E, R>,
}

impl<'data, E, R> Iterator for DyldCacheRelocationIterator<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data>,
{
    type Item = Result<DyldRelocation>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.version {
            DyldCacheRelocationIteratorVersion::None => Ok(None),
            DyldCacheRelocationIteratorVersion::V2(iter) => iter.next(),
            DyldCacheRelocationIteratorVersion::V3(iter) => iter.next(),
            DyldCacheRelocationIteratorVersion::V5(iter) => iter.next(),
        }
        .transpose()
    }
}

#[derive(Debug)]
enum DyldCacheRelocationIteratorVersion<'data, E = Endianness, R = &'data [u8]>
where
    E: Endian,
    R: ReadRef<'data>,
{
    None,
    V2(DyldCacheRelocationIteratorV2<'data, E, R>),
    V3(DyldCacheRelocationIteratorV3<'data, E, R>),
    V5(DyldCacheRelocationIteratorV5<'data, E, R>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RelocationStateV2 {
    Start,
    Extra,
    Page,
    PageExtra,
}

#[derive(Debug)]
struct DyldCacheRelocationIteratorV2<'data, E = Endianness, R = &'data [u8]>
where
    E: Endian,
    R: ReadRef<'data>,
{
    data: R,
    endian: E,
    mapping_file_offset: u64,
    page_size: u64,
    delta_mask: u64,
    delta_shift: u32,
    value_add: u64,
    page_starts: &'data [U16<E>],
    page_extras: &'data [U16<E>],

    state: RelocationStateV2,
    /// The next index within page_starts.
    start_index: usize,
    /// The next index within page_extras.
    extra_index: usize,
    /// The current page offset within the mapping.
    page_offset: u64,
    /// The offset of the next linked list entry within the page.
    offset: u64,
}

impl<'data, E, R> DyldCacheRelocationIteratorV2<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data>,
{
    fn next(&mut self) -> Result<Option<DyldRelocation>> {
        loop {
            match self.state {
                RelocationStateV2::Start => {
                    let Some(page_start) = self.page_starts.get(self.start_index) else {
                        return Ok(None);
                    };
                    self.page_offset = self.start_index as u64 * self.page_size;
                    self.start_index += 1;

                    let page_start = page_start.get(self.endian);
                    if page_start & macho::DYLD_CACHE_SLIDE_PAGE_ATTR_NO_REBASE != 0 {
                        self.state = RelocationStateV2::Start;
                    } else if page_start & macho::DYLD_CACHE_SLIDE_PAGE_ATTR_EXTRA != 0 {
                        self.state = RelocationStateV2::Extra;
                        self.extra_index =
                            usize::from(page_start & !macho::DYLD_CACHE_SLIDE_PAGE_ATTRS);
                    } else {
                        self.state = RelocationStateV2::Page;
                        self.offset =
                            u64::from(page_start & !macho::DYLD_CACHE_SLIDE_PAGE_ATTRS) * 4;
                    }
                }
                RelocationStateV2::Extra => {
                    let Some(page_extra) = self.page_extras.get(self.extra_index) else {
                        return Ok(None);
                    };
                    self.extra_index += 1;

                    let page_extra = page_extra.get(self.endian);
                    self.offset = u64::from(page_extra & !macho::DYLD_CACHE_SLIDE_PAGE_ATTRS) * 4;
                    if page_extra & macho::DYLD_CACHE_SLIDE_PAGE_ATTR_END != 0 {
                        self.state = RelocationStateV2::Page;
                    } else {
                        self.state = RelocationStateV2::PageExtra;
                    }
                }
                RelocationStateV2::Page | RelocationStateV2::PageExtra => {
                    let offset = self.offset;
                    let pointer = self
                        .data
                        .read_at::<U64<E>>(self.mapping_file_offset + self.page_offset + offset)
                        .read_error("Invalid dyld cache slide pointer offset")?
                        .get(self.endian);

                    let next = (pointer & self.delta_mask) >> self.delta_shift;
                    if next == 0 {
                        if self.state == RelocationStateV2::PageExtra {
                            self.state = RelocationStateV2::Extra
                        } else {
                            self.state = RelocationStateV2::Start
                        };
                    } else {
                        self.offset = offset + next * 4;
                    };

                    let value = pointer & !self.delta_mask;
                    if value != 0 {
                        return Ok(Some(DyldRelocation {
                            offset,
                            value: value + self.value_add,
                            auth: None,
                        }));
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RelocationStateV3 {
    Start,
    Page,
}

#[derive(Debug)]
struct DyldCacheRelocationIteratorV3<'data, E = Endianness, R = &'data [u8]>
where
    E: Endian,
    R: ReadRef<'data>,
{
    data: R,
    endian: E,
    mapping_file_offset: u64,
    auth_value_add: u64,
    page_size: u64,
    page_starts: &'data [U16<E>],

    state: RelocationStateV3,
    /// Index of the page within the mapping.
    start_index: usize,
    /// The current offset within the mapping.
    offset: u64,
}

impl<'data, E, R> DyldCacheRelocationIteratorV3<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data>,
{
    fn next(&mut self) -> Result<Option<DyldRelocation>> {
        loop {
            match self.state {
                RelocationStateV3::Start => {
                    let Some(page_start) = self.page_starts.get(self.start_index) else {
                        return Ok(None);
                    };
                    let page_offset = self.start_index as u64 * self.page_size;
                    self.start_index += 1;

                    let page_start = page_start.get(self.endian);
                    if page_start == macho::DYLD_CACHE_SLIDE_V3_PAGE_ATTR_NO_REBASE {
                        self.state = RelocationStateV3::Start;
                    } else {
                        self.state = RelocationStateV3::Page;
                        self.offset = page_offset + u64::from(page_start);
                    }
                }
                RelocationStateV3::Page => {
                    let offset = self.offset;
                    let pointer = self
                        .data
                        .read_at::<U64<E>>(self.mapping_file_offset + offset)
                        .read_error("Invalid dyld cache slide pointer offset")?
                        .get(self.endian);
                    let pointer = macho::DyldCacheSlidePointer3(pointer);

                    let next = pointer.next();
                    if next == 0 {
                        self.state = RelocationStateV3::Start;
                    } else {
                        self.offset = offset + next * 8;
                    }

                    if pointer.is_auth() {
                        let value = pointer.runtime_offset() + self.auth_value_add;
                        let key = match pointer.key() {
                            1 => macho::PtrauthKey::IB,
                            2 => macho::PtrauthKey::DA,
                            3 => macho::PtrauthKey::DB,
                            _ => macho::PtrauthKey::IA,
                        };
                        let auth = Some(DyldRelocationAuth {
                            key,
                            diversity: pointer.diversity(),
                            addr_div: pointer.addr_div(),
                        });
                        return Ok(Some(DyldRelocation {
                            offset,
                            value,
                            auth,
                        }));
                    } else {
                        let value = pointer.target() | pointer.high8() << 56;
                        return Ok(Some(DyldRelocation {
                            offset,
                            value,
                            auth: None,
                        }));
                    };
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RelocationStateV5 {
    Start,
    Page,
}

#[derive(Debug)]
struct DyldCacheRelocationIteratorV5<'data, E = Endianness, R = &'data [u8]>
where
    E: Endian,
    R: ReadRef<'data>,
{
    data: R,
    endian: E,
    mapping_file_offset: u64,
    page_size: u64,
    value_add: u64,
    page_starts: &'data [U16<E>],

    state: RelocationStateV5,
    /// The next index within page_starts.
    start_index: usize,
    /// The current offset within the mapping.
    offset: u64,
}

impl<'data, E, R> DyldCacheRelocationIteratorV5<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data>,
{
    fn next(&mut self) -> Result<Option<DyldRelocation>> {
        loop {
            match self.state {
                RelocationStateV5::Start => {
                    let Some(page_start) = self.page_starts.get(self.start_index) else {
                        return Ok(None);
                    };
                    let page_offset = self.start_index as u64 * self.page_size;
                    self.start_index += 1;

                    let page_start = page_start.get(self.endian);
                    if page_start == macho::DYLD_CACHE_SLIDE_V5_PAGE_ATTR_NO_REBASE {
                        self.state = RelocationStateV5::Start;
                    } else {
                        self.state = RelocationStateV5::Page;
                        self.offset = page_offset + u64::from(page_start);
                    }
                }
                RelocationStateV5::Page => {
                    let offset = self.offset;
                    let pointer = self
                        .data
                        .read_at::<U64<E>>(self.mapping_file_offset + offset)
                        .read_error("Invalid dyld cache slide pointer offset")?
                        .get(self.endian);
                    let pointer = macho::DyldCacheSlidePointer5(pointer);

                    let next = pointer.next();
                    if next == 0 {
                        self.state = RelocationStateV5::Start;
                    } else {
                        self.offset = offset + next * 8;
                    }

                    let mut value = pointer.runtime_offset() + self.value_add;
                    let auth = if pointer.is_auth() {
                        let key = if pointer.key_is_data() {
                            macho::PtrauthKey::DA
                        } else {
                            macho::PtrauthKey::IA
                        };
                        Some(DyldRelocationAuth {
                            key,
                            diversity: pointer.diversity(),
                            addr_div: pointer.addr_div(),
                        })
                    } else {
                        value |= pointer.high8() << 56;
                        None
                    };
                    return Ok(Some(DyldRelocation {
                        offset,
                        value,
                        auth,
                    }));
                }
            }
        }
    }
}

/// A cache mapping relocation.
pub struct DyldRelocation {
    /// The offset of the relocation within the mapping.
    ///
    /// This can be added to either the mapping file offset or the
    /// mapping address.
    pub offset: u64,
    /// The value to be relocated.
    pub value: u64,
    /// The pointer authentication data, if present.
    pub auth: Option<DyldRelocationAuth>,
}

impl Debug for DyldRelocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DyldRelocation")
            .field("offset", &format_args!("{:#x}", self.offset))
            .field("value", &format_args!("{:#x}", self.value))
            .field("auth", &self.auth)
            .finish()
    }
}

/// Pointer authentication data.
///
/// This is used for signing pointers for the arm64e ABI.
pub struct DyldRelocationAuth {
    /// The key used to generate the signed value.
    pub key: macho::PtrauthKey,
    /// The integer diversity value.
    pub diversity: u16,
    /// Whether the address should be blended with the diversity value.
    pub addr_div: bool,
}

impl Debug for DyldRelocationAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ptrauth")
            .field("key", &self.key)
            .field("diversity", &format_args!("{:#x}", self.diversity))
            .field("addr_div", &self.addr_div)
            .finish()
    }
}

impl<E: Endian> macho::DyldCacheHeader<E> {
    /// Read the dyld cache header.
    pub fn parse<'data, R: ReadRef<'data>>(data: R) -> Result<&'data Self> {
        data.read_at::<macho::DyldCacheHeader<E>>(0)
            .read_error("Invalid dyld cache header size or alignment")
    }

    /// Returns (arch, endian) based on the magic string.
    pub fn parse_magic(&self) -> Result<(Architecture, E)> {
        let (arch, is_big_endian) = match &self.magic {
            b"dyld_v1    i386\0" => (Architecture::I386, false),
            b"dyld_v1  x86_64\0" => (Architecture::X86_64, false),
            b"dyld_v1 x86_64h\0" => (Architecture::X86_64, false),
            b"dyld_v1     ppc\0" => (Architecture::PowerPc, true),
            b"dyld_v1   armv6\0" => (Architecture::Arm, false),
            b"dyld_v1   armv7\0" => (Architecture::Arm, false),
            b"dyld_v1  armv7f\0" => (Architecture::Arm, false),
            b"dyld_v1  armv7s\0" => (Architecture::Arm, false),
            b"dyld_v1  armv7k\0" => (Architecture::Arm, false),
            b"dyld_v1   arm64\0" => (Architecture::Aarch64, false),
            b"dyld_v1  arm64e\0" => (Architecture::Aarch64, false),
            _ => return Err(Error("Unrecognized dyld cache magic")),
        };
        let endian =
            E::from_big_endian(is_big_endian).read_error("Unsupported dyld cache endian")?;
        Ok((arch, endian))
    }

    /// Return the mapping information table.
    pub fn mappings<'data, R: ReadRef<'data>>(
        &self,
        endian: E,
        data: R,
    ) -> Result<DyldCacheMappingSlice<'data, E>> {
        let header_size = self.mapping_offset.get(endian);
        if header_size >= MIN_HEADER_SIZE_MAPPINGS_V2 {
            let info = data
                .read_slice_at::<macho::DyldCacheMappingAndSlideInfo<E>>(
                    self.mapping_with_slide_offset.get(endian).into(),
                    self.mapping_with_slide_count.get(endian) as usize,
                )
                .read_error("Invalid dyld cache mapping size or alignment")?;
            Ok(DyldCacheMappingSlice::V2(info))
        } else {
            let info = data
                .read_slice_at::<macho::DyldCacheMappingInfo<E>>(
                    self.mapping_offset.get(endian).into(),
                    self.mapping_count.get(endian) as usize,
                )
                .read_error("Invalid dyld cache mapping size or alignment")?;
            Ok(DyldCacheMappingSlice::V1(info))
        }
    }

    /// Return the information about subcaches, if present.
    ///
    /// Returns `None` for dyld caches produced before dyld-940 (macOS 12).
    pub fn subcaches<'data, R: ReadRef<'data>>(
        &self,
        endian: E,
        data: R,
    ) -> Result<Option<DyldSubCacheSlice<'data, E>>> {
        let header_size = self.mapping_offset.get(endian);
        if header_size >= MIN_HEADER_SIZE_SUBCACHES_V2 {
            let subcaches = data
                .read_slice_at::<macho::DyldSubCacheEntryV2<E>>(
                    self.sub_cache_array_offset.get(endian).into(),
                    self.sub_cache_array_count.get(endian) as usize,
                )
                .read_error("Invalid dyld subcaches size or alignment")?;
            Ok(Some(DyldSubCacheSlice::V2(subcaches)))
        } else if header_size >= MIN_HEADER_SIZE_SUBCACHES_V1 {
            let subcaches = data
                .read_slice_at::<macho::DyldSubCacheEntryV1<E>>(
                    self.sub_cache_array_offset.get(endian).into(),
                    self.sub_cache_array_count.get(endian) as usize,
                )
                .read_error("Invalid dyld subcaches size or alignment")?;
            Ok(Some(DyldSubCacheSlice::V1(subcaches)))
        } else {
            Ok(None)
        }
    }

    /// Return the UUID for the .symbols subcache, if present.
    pub fn symbols_subcache_uuid(&self, endian: E) -> Option<[u8; 16]> {
        if self.mapping_offset.get(endian) >= MIN_HEADER_SIZE_SUBCACHES_V1 {
            let uuid = self.symbol_file_uuid;
            if uuid != [0; 16] {
                return Some(uuid);
            }
        }
        None
    }

    /// Return the image information table.
    pub fn images<'data, R: ReadRef<'data>>(
        &self,
        endian: E,
        data: R,
    ) -> Result<&'data [macho::DyldCacheImageInfo<E>]> {
        if self.mapping_offset.get(endian) >= MIN_HEADER_SIZE_SUBCACHES_V1 {
            data.read_slice_at::<macho::DyldCacheImageInfo<E>>(
                self.images_offset.get(endian).into(),
                self.images_count.get(endian) as usize,
            )
            .read_error("Invalid dyld cache image size or alignment")
        } else {
            data.read_slice_at::<macho::DyldCacheImageInfo<E>>(
                self.images_offset_old.get(endian).into(),
                self.images_count_old.get(endian) as usize,
            )
            .read_error("Invalid dyld cache image size or alignment")
        }
    }
}

impl<E: Endian> macho::DyldCacheImageInfo<E> {
    /// The file system path of this image.
    ///
    /// `data` should be the main cache file, not the subcache containing the image.
    pub fn path<'data, R: ReadRef<'data>>(&self, endian: E, data: R) -> Result<&'data [u8]> {
        let r_start = self.path_file_offset.get(endian).into();
        let r_end = data.len().read_error("Couldn't get data len()")?;
        data.read_bytes_at_until(r_start..r_end, 0)
            .read_error("Couldn't read dyld cache image path")
    }
}

impl<E: Endian> macho::DyldCacheMappingAndSlideInfo<E> {
    /// Return the (optional) array of slide information structs
    pub fn slide<'data, R: ReadRef<'data>>(
        &self,
        endian: E,
        data: R,
    ) -> Result<DyldCacheSlideInfo<'data, E>> {
        // TODO: limit further reads to this size?
        if self.slide_info_file_size.get(endian) == 0 {
            return Ok(DyldCacheSlideInfo::None);
        }

        let slide_info_file_offset = self.slide_info_file_offset.get(endian);
        let version = data
            .read_at::<U32<E>>(slide_info_file_offset)
            .read_error("Invalid slide info file offset size or alignment")?
            .get(endian);
        match version {
            2 => {
                let slide = data
                    .read_at::<macho::DyldCacheSlideInfo2<E>>(slide_info_file_offset)
                    .read_error("Invalid dyld cache slide info offset or alignment")?;
                let page_starts_offset = slide_info_file_offset
                    .checked_add(slide.page_starts_offset.get(endian) as u64)
                    .read_error("Invalid dyld cache page starts offset")?;
                let page_starts = data
                    .read_slice_at::<U16<E>>(
                        page_starts_offset,
                        slide.page_starts_count.get(endian) as usize,
                    )
                    .read_error("Invalid dyld cache page starts size or alignment")?;
                let page_extras_offset = slide_info_file_offset
                    .checked_add(slide.page_extras_offset.get(endian) as u64)
                    .read_error("Invalid dyld cache page extras offset")?;
                let page_extras = data
                    .read_slice_at::<U16<E>>(
                        page_extras_offset,
                        slide.page_extras_count.get(endian) as usize,
                    )
                    .read_error("Invalid dyld cache page extras size or alignment")?;
                Ok(DyldCacheSlideInfo::V2 {
                    slide,
                    page_starts,
                    page_extras,
                })
            }
            3 => {
                let slide = data
                    .read_at::<macho::DyldCacheSlideInfo3<E>>(slide_info_file_offset)
                    .read_error("Invalid dyld cache slide info offset or alignment")?;
                let page_starts_offset = slide_info_file_offset
                    .checked_add(mem::size_of::<macho::DyldCacheSlideInfo3<E>>() as u64)
                    .read_error("Invalid dyld cache page starts offset")?;
                let page_starts = data
                    .read_slice_at::<U16<E>>(
                        page_starts_offset,
                        slide.page_starts_count.get(endian) as usize,
                    )
                    .read_error("Invalid dyld cache page starts size or alignment")?;
                Ok(DyldCacheSlideInfo::V3 { slide, page_starts })
            }
            5 => {
                let slide = data
                    .read_at::<macho::DyldCacheSlideInfo5<E>>(slide_info_file_offset)
                    .read_error("Invalid dyld cache slide info offset or alignment")?;
                let page_starts_offset = slide_info_file_offset
                    .checked_add(mem::size_of::<macho::DyldCacheSlideInfo5<E>>() as u64)
                    .read_error("Invalid dyld cache page starts offset")?;
                let page_starts = data
                    .read_slice_at::<U16<E>>(
                        page_starts_offset,
                        slide.page_starts_count.get(endian) as usize,
                    )
                    .read_error("Invalid dyld cache page starts size or alignment")?;
                Ok(DyldCacheSlideInfo::V5 { slide, page_starts })
            }
            _ => Err(Error("Unsupported dyld cache slide info version")),
        }
    }
}
