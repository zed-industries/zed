
//! Describes all meta data possible in an exr file.
//! Contains functionality to read and write meta data from bytes.
//! Browse the `exr::image` module to get started with the high-level interface.

pub mod attribute;
pub mod header;


use crate::io::*;
use ::smallvec::SmallVec;
use self::attribute::*;
use crate::block::chunk::{TileCoordinates, CompressedBlock};
use crate::error::*;
use std::fs::File;
use std::io::{BufReader};
use crate::math::*;
use std::collections::{HashSet};
use std::convert::TryFrom;
use crate::meta::header::{Header};
use crate::block::{BlockIndex, UncompressedBlock};


// TODO rename MetaData to ImageInfo?

/// Contains the complete meta data of an exr image.
/// Defines how the image is split up in the file,
/// the number and type of images and channels,
/// and various other attributes.
/// The usage of custom attributes is encouraged.
#[derive(Debug, Clone, PartialEq)]
pub struct MetaData {

    /// Some flags summarizing the features that must be supported to decode the file.
    pub requirements: Requirements,

    /// One header to describe each layer in this file.
    // TODO rename to layer descriptions?
    pub headers: Headers,
}


/// List of `Header`s.
pub type Headers = SmallVec<[Header; 3]>;

/// List of `OffsetTable`s.
pub type OffsetTables = SmallVec<[OffsetTable; 3]>;


/// The offset table is an ordered list of indices referencing pixel data in the exr file.
/// For each pixel tile in the image, an index exists, which points to the byte-location
/// of the corresponding pixel data in the file. That index can be used to load specific
/// portions of an image without processing all bytes in a file. For each header,
/// an offset table exists with its indices ordered by `LineOrder::Increasing`.
// If the multipart bit is unset and the chunkCount attribute is not present,
// the number of entries in the chunk table is computed using the
// dataWindow, tileDesc, and compression attribute.
//
// If the multipart bit is set, the header must contain a
// chunkCount attribute, that contains the length of the offset table.
pub type OffsetTable = Vec<u64>;


/// A summary of requirements that must be met to read this exr file.
/// Used to determine whether this file can be read by a given reader.
/// It includes the OpenEXR version number. This library aims to support version `2.0`.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Requirements {

    /// This library supports reading version 1 and 2, and writing version 2.
    // TODO write version 1 for simple images
    pub file_format_version: u8,

    /// If true, this image has tiled blocks and contains only a single layer.
    /// If false and not deep and not multilayer, this image is a single layer image with scan line blocks.
    pub is_single_layer_and_tiled: bool,

    // in c or bad c++ this might have been relevant (omg is he allowed to say that)
    /// Whether this file has strings with a length greater than 31.
    /// Strings can never be longer than 255.
    pub has_long_names: bool,

    /// This image contains at least one layer with deep data.
    pub has_deep_data: bool,

    /// Whether this file contains multiple layers.
    pub has_multiple_layers: bool,
}


/// Locates a rectangular section of pixels in an image.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct TileIndices {

    /// Index of the tile.
    pub location: TileCoordinates,

    /// Pixel size of the tile.
    pub size: Vec2<usize>,
}

/// How the image pixels are split up into separate blocks.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BlockDescription {

    /// The image is divided into scan line blocks.
    /// The number of scan lines in a block depends on the compression method.
    ScanLines,

    /// The image is divided into tile blocks.
    /// Also specifies the size of each tile in the image
    /// and whether this image contains multiple resolution levels.
    Tiles(TileDescription)
}


/*impl TileIndices {
    pub fn cmp(&self, other: &Self) -> Ordering {
        match self.location.level_index.1.cmp(&other.location.level_index.1) {
            Ordering::Equal => {
                match self.location.level_index.0.cmp(&other.location.level_index.0) {
                    Ordering::Equal => {
                        match self.location.tile_index.1.cmp(&other.location.tile_index.1) {
                            Ordering::Equal => {
                                self.location.tile_index.0.cmp(&other.location.tile_index.0)
                            },

                            other => other,
                        }
                    },

                    other => other
                }
            },

            other => other
        }
    }
}*/

impl BlockDescription {

    /// Whether this image is tiled. If false, this image is divided into scan line blocks.
    pub fn has_tiles(&self) -> bool {
        match self {
            BlockDescription::Tiles { .. } => true,
            _ => false
        }
    }
}





/// The first four bytes of each exr file.
/// Used to abort reading non-exr files.
pub mod magic_number {
    use super::*;

    /// The first four bytes of each exr file.
    pub const BYTES: [u8; 4] = [0x76, 0x2f, 0x31, 0x01];

    /// Without validation, write this instance to the byte stream.
    pub fn write(write: &mut impl Write) -> Result<()> {
        u8::write_slice_ne(write, &self::BYTES)
    }

    /// Consumes four bytes from the reader and returns whether the file may be an exr file.
    // TODO check if exr before allocating BufRead
    pub fn is_exr(read: &mut impl Read) -> Result<bool> {
        let mut magic_num = [0; 4];
        u8::read_slice_ne(read, &mut magic_num)?;
        Ok(magic_num == self::BYTES)
    }

    /// Validate this image. If it is an exr file, return `Ok(())`.
    pub fn validate_exr(read: &mut impl Read) -> UnitResult {
        if self::is_exr(read)? {
            Ok(())

        } else {
            Err(Error::invalid("file identifier missing"))
        }
    }
}

/// A `0_u8` at the end of a sequence.
pub mod sequence_end {
    use super::*;

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size() -> usize {
        1
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(write: &mut W) -> UnitResult {
        0_u8.write_le(write)
    }

    /// Peeks the next byte. If it is zero, consumes the byte and returns true.
    pub fn has_come(read: &mut PeekRead<impl Read>) -> Result<bool> {
        Ok(read.skip_if_eq(0)?)
    }
}

fn missing_attribute(name: &str) -> Error {
    Error::invalid(format!("missing or invalid {} attribute", name))
}


/// Compute the number of tiles required to contain all values.
pub fn compute_block_count(full_res: usize, tile_size: usize) -> usize {
    // round up, because if the image is not evenly divisible by the tiles,
    // we add another tile at the end (which is only partially used)
    RoundingMode::Up.divide(full_res, tile_size)
}

/// Compute the start position and size of a block inside a dimension.
#[inline]
pub fn calculate_block_position_and_size(total_size: usize, block_size: usize, block_index: usize) -> Result<(usize, usize)> {
    let block_position = block_size * block_index;

    Ok((
        block_position,
        calculate_block_size(total_size, block_size, block_position)?
    ))
}

/// Calculate the size of a single block. If this is the last block,
/// this only returns the required size, which is always smaller than the default block size.
// TODO use this method everywhere instead of convoluted formulas
#[inline]
pub fn calculate_block_size(total_size: usize, block_size: usize, block_position: usize) -> Result<usize> {
    if block_position >= total_size {
        return Err(Error::invalid("block index"))
    }

    if block_position + block_size <= total_size {
        Ok(block_size)
    }
    else {
        Ok(total_size - block_position)
    }
}


/// Calculate number of mip levels in a given resolution.
// TODO this should be cached? log2 may be very expensive
pub fn compute_level_count(round: RoundingMode, full_res: usize) -> usize {
    usize::try_from(round.log2(u32::try_from(full_res).unwrap())).unwrap() + 1
}

/// Calculate the size of a single mip level by index.
// TODO this should be cached? log2 may be very expensive
pub fn compute_level_size(round: RoundingMode, full_res: usize, level_index: usize) -> usize {
    assert!(level_index < std::mem::size_of::<usize>() * 8, "largest level size exceeds maximum integer value");
    round.divide(full_res,  1 << level_index).max(1)
}

/// Iterates over all rip map level resolutions of a given size, including the indices of each level.
/// The order of iteration conforms to `LineOrder::Increasing`.
// TODO cache these?
// TODO compute these directly instead of summing up an iterator?
pub fn rip_map_levels(round: RoundingMode, max_resolution: Vec2<usize>) -> impl Iterator<Item=(Vec2<usize>, Vec2<usize>)> {
    rip_map_indices(round, max_resolution).map(move |level_indices|{
        // TODO progressively divide instead??
        let width = compute_level_size(round, max_resolution.width(), level_indices.x());
        let height = compute_level_size(round, max_resolution.height(), level_indices.y());
        (level_indices, Vec2(width, height))
    })
}

/// Iterates over all mip map level resolutions of a given size, including the indices of each level.
/// The order of iteration conforms to `LineOrder::Increasing`.
// TODO cache all these level values when computing table offset size??
// TODO compute these directly instead of summing up an iterator?
pub fn mip_map_levels(round: RoundingMode, max_resolution: Vec2<usize>) -> impl Iterator<Item=(usize, Vec2<usize>)> {
    mip_map_indices(round, max_resolution)
        .map(move |level_index|{
            // TODO progressively divide instead??
            let width = compute_level_size(round, max_resolution.width(), level_index);
            let height = compute_level_size(round, max_resolution.height(), level_index);
            (level_index, Vec2(width, height))
        })
}

/// Iterates over all rip map level indices of a given size.
/// The order of iteration conforms to `LineOrder::Increasing`.
pub fn rip_map_indices(round: RoundingMode, max_resolution: Vec2<usize>) -> impl Iterator<Item=Vec2<usize>> {
    let (width, height) = (
        compute_level_count(round, max_resolution.width()),
        compute_level_count(round, max_resolution.height())
    );

    (0..height).flat_map(move |y_level|{
        (0..width).map(move |x_level|{
            Vec2(x_level, y_level)
        })
    })
}

/// Iterates over all mip map level indices of a given size.
/// The order of iteration conforms to `LineOrder::Increasing`.
pub fn mip_map_indices(round: RoundingMode, max_resolution: Vec2<usize>) -> impl Iterator<Item=usize> {
    0..compute_level_count(round, max_resolution.width().max(max_resolution.height()))
}

/// Compute the number of chunks that an image is divided into. May be an expensive operation.
// If not multilayer and chunkCount not present,
// the number of entries in the chunk table is computed
// using the dataWindow and tileDesc attributes and the compression format
pub fn compute_chunk_count(compression: Compression, data_size: Vec2<usize>, blocks: BlockDescription) -> usize {

    if let BlockDescription::Tiles(tiles) = blocks {
        let round = tiles.rounding_mode;
        let Vec2(tile_width, tile_height) = tiles.tile_size;

        // TODO cache all these level values??
        use crate::meta::attribute::LevelMode::*;
        match tiles.level_mode {
            Singular => {
                let tiles_x = compute_block_count(data_size.width(), tile_width);
                let tiles_y = compute_block_count(data_size.height(), tile_height);
                tiles_x * tiles_y
            }

            MipMap => {
                mip_map_levels(round, data_size).map(|(_, Vec2(level_width, level_height))| {
                    compute_block_count(level_width, tile_width) * compute_block_count(level_height, tile_height)
                }).sum()
            },

            RipMap => {
                rip_map_levels(round, data_size).map(|(_, Vec2(level_width, level_height))| {
                    compute_block_count(level_width, tile_width) * compute_block_count(level_height, tile_height)
                }).sum()
            }
        }
    }

    // scan line blocks never have mip maps
    else {
        compute_block_count(data_size.height(), compression.scan_lines_per_block())
    }
}



impl MetaData {

    /// Read the exr meta data from a file.
    /// Use `read_from_unbuffered` instead if you do not have a file.
    /// Does not validate the meta data.
    #[must_use]
    pub fn read_from_file(path: impl AsRef<::std::path::Path>, pedantic: bool) -> Result<Self> {
        Self::read_from_unbuffered(File::open(path)?, pedantic)
    }

    /// Buffer the reader and then read the exr meta data from it.
    /// Use `read_from_buffered` if your reader is an in-memory reader.
    /// Use `read_from_file` if you have a file path.
    /// Does not validate the meta data.
    #[must_use]
    pub fn read_from_unbuffered(unbuffered: impl Read, pedantic: bool) -> Result<Self> {
        Self::read_from_buffered(BufReader::new(unbuffered), pedantic)
    }

    /// Read the exr meta data from a reader.
    /// Use `read_from_file` if you have a file path.
    /// Use `read_from_unbuffered` if this is not an in-memory reader.
    /// Does not validate the meta data.
    #[must_use]
    pub fn read_from_buffered(buffered: impl Read, pedantic: bool) -> Result<Self> {
        let mut read = PeekRead::new(buffered);
        MetaData::read_unvalidated_from_buffered_peekable(&mut read, pedantic)
    }

    /// Does __not validate__ the meta data completely.
    #[must_use]
    pub(crate) fn read_unvalidated_from_buffered_peekable(read: &mut PeekRead<impl Read>, pedantic: bool) -> Result<Self> {
        magic_number::validate_exr(read)?;

        let requirements = Requirements::read(read)?;

        // do this check now in order to fast-fail for newer versions and features than version 2
        requirements.validate()?;

        let headers = Header::read_all(read, &requirements, pedantic)?;

        // TODO check if supporting requirements 2 always implies supporting requirements 1
        Ok(MetaData { requirements, headers })
    }

    /// Validates the meta data.
    #[must_use]
    pub(crate) fn read_validated_from_buffered_peekable(
        read: &mut PeekRead<impl Read>, pedantic: bool
    ) -> Result<Self> {
        let meta_data = Self::read_unvalidated_from_buffered_peekable(read, !pedantic)?;
        MetaData::validate(meta_data.headers.as_slice(), pedantic)?;
        Ok(meta_data)
    }

    /// Validates the meta data and writes it to the stream.
    /// If pedantic, throws errors for files that may produce errors in other exr readers.
    /// Returns the automatically detected minimum requirement flags.
    pub(crate) fn write_validating_to_buffered(write: &mut impl Write, headers: &[Header], pedantic: bool) -> Result<Requirements> {
        // pedantic validation to not allow slightly invalid files
        // that still could be read correctly in theory
        let minimal_requirements = Self::validate(headers, pedantic)?;

        magic_number::write(write)?;
        minimal_requirements.write(write)?;
        Header::write_all(headers, write, minimal_requirements.has_multiple_layers)?;
        Ok(minimal_requirements)
    }

    /// Read one offset table from the reader for each header.
    pub fn read_offset_tables(read: &mut PeekRead<impl Read>, headers: &Headers) -> Result<OffsetTables> {
        headers.iter()
            .map(|header| u64::read_vec_le(read, header.chunk_count, u16::MAX as usize, None, "offset table size"))
            .collect()
    }

    /// Skip the offset tables by advancing the reader by the required byte count.
    // TODO use seek for large (probably all) tables!
    pub fn skip_offset_tables(read: &mut PeekRead<impl Read>, headers: &Headers) -> Result<usize> {
        let chunk_count: usize = headers.iter().map(|header| header.chunk_count).sum();
        crate::io::skip_bytes(read, chunk_count * u64::BYTE_SIZE)?; // TODO this should seek for large tables
        Ok(chunk_count)
    }

    /// This iterator tells you the block indices of all blocks that must be in the image.
    /// The order of the blocks depends on the `LineOrder` attribute
    /// (unspecified line order is treated the same as increasing line order).
    /// The blocks written to the file must be exactly in this order,
    /// except for when the `LineOrder` is unspecified.
    /// The index represents the block index, in increasing line order, within the header.
    pub fn enumerate_ordered_header_block_indices(&self) -> impl '_ + Iterator<Item=(usize, BlockIndex)> {
        crate::block::enumerate_ordered_header_block_indices(&self.headers)
    }

    /// Go through all the block indices in the correct order and call the specified closure for each of these blocks.
    /// That way, the blocks indices are filled with real block data and returned as an iterator.
    /// The closure returns the an `UncompressedBlock` for each block index.
    pub fn collect_ordered_blocks<'s>(&'s self, mut get_block: impl 's + FnMut(BlockIndex) -> UncompressedBlock)
        -> impl 's + Iterator<Item=(usize, UncompressedBlock)>
    {
        self.enumerate_ordered_header_block_indices().map(move |(index_in_header, block_index)|{
            (index_in_header, get_block(block_index))
        })
    }

    /// Go through all the block indices in the correct order and call the specified closure for each of these blocks.
    /// That way, the blocks indices are filled with real block data and returned as an iterator.
    /// The closure returns the byte data for each block index.
    pub fn collect_ordered_block_data<'s>(&'s self, mut get_block_data: impl 's + FnMut(BlockIndex) -> Vec<u8>)
        -> impl 's + Iterator<Item=(usize, UncompressedBlock)>
    {
        self.collect_ordered_blocks(move |block_index|
            UncompressedBlock { index: block_index, data: get_block_data(block_index) }
        )
    }

    /// Validates this meta data. Returns the minimal possible requirements.
    pub fn validate(headers: &[Header], pedantic: bool) -> Result<Requirements> {
        if headers.len() == 0 {
            return Err(Error::invalid("at least one layer is required"));
        }

        let deep = false; // TODO deep data
        let is_multilayer = headers.len() > 1;
        let first_header_has_tiles = headers.iter().next()
            .map_or(false, |header| header.blocks.has_tiles());

        let mut minimal_requirements = Requirements {
            // according to the spec, version 2  should only be necessary if `is_multilayer || deep`.
            // but the current open exr library does not support images with version 1, so always use version 2.
            file_format_version: 2,

            // start as low as possible, later increasing if required
            has_long_names: false,

            is_single_layer_and_tiled: !is_multilayer && first_header_has_tiles,
            has_multiple_layers: is_multilayer,
            has_deep_data: deep,
        };

        for header in headers {
            if header.deep { // TODO deep data (and then remove this check)
                return Err(Error::unsupported("deep data not supported yet"));
            }

            header.validate(is_multilayer, &mut minimal_requirements.has_long_names, pedantic)?;
        }

        // TODO validation fn!
        /*if let Some(max) = max_pixel_bytes {
            let byte_size: usize = headers.iter()
                .map(|header| header.total_pixel_bytes())
                .sum();

            if byte_size > max {
                return Err(Error::invalid("image larger than specified maximum"));
            }
        }*/

        if pedantic { // check for duplicate header names
            let mut header_names = HashSet::with_capacity(headers.len());
            for header in headers {
                if !header_names.insert(&header.own_attributes.layer_name) {
                    return Err(Error::invalid(format!(
                        "duplicate layer name: `{}`",
                        header.own_attributes.layer_name.as_ref().expect("header validation bug")
                    )));
                }
            }
        }

        if pedantic {
            let must_share = headers.iter().flat_map(|header| header.own_attributes.other.iter())
                .any(|(_, value)| value.to_chromaticities().is_ok() || value.to_time_code().is_ok());

            if must_share {
                return Err(Error::invalid("chromaticities and time code attributes must must not exist in own attributes but shared instead"));
            }
        }

        if pedantic && headers.len() > 1 { // check for attributes that should not differ in between headers
            let first_header = headers.first().expect("header count validation bug");
            let first_header_attributes = &first_header.shared_attributes;

            for header in &headers[1..] {
                if &header.shared_attributes != first_header_attributes {
                    return Err(Error::invalid("display window, pixel aspect, chromaticities, and time code attributes must be equal for all headers"))
                }
            }
        }

        debug_assert!(minimal_requirements.validate().is_ok(), "inferred requirements are invalid");
        Ok(minimal_requirements)
    }
}




impl Requirements {

    // this is actually used for control flow, as the number of headers may be 1 in a multilayer file
    /// Is this file declared to contain multiple layers?
    pub fn is_multilayer(&self) -> bool {
        self.has_multiple_layers
    }

    /// Read the value without validating.
    pub fn read<R: Read>(read: &mut R) -> Result<Self> {
        use ::bit_field::BitField;

        let version_and_flags = u32::read_le(read)?;

        // take the 8 least significant bits, they contain the file format version number
        let version = (version_and_flags & 0x000F) as u8;

        // the 24 most significant bits are treated as a set of boolean flags
        let is_single_tile = version_and_flags.get_bit(9);
        let has_long_names = version_and_flags.get_bit(10);
        let has_deep_data = version_and_flags.get_bit(11);
        let has_multiple_layers = version_and_flags.get_bit(12);

        // all remaining bits except 9, 10, 11 and 12 are reserved and should be 0
        // if a file has any of these bits set to 1, it means this file contains
        // a feature that we don't support
        let unknown_flags = version_and_flags >> 13; // all flags excluding the 12 bits we already parsed

        if unknown_flags != 0 { // TODO test if this correctly detects unsupported files
            return Err(Error::unsupported("too new file feature flags"));
        }

        let version = Requirements {
            file_format_version: version,
            is_single_layer_and_tiled: is_single_tile, has_long_names,
            has_deep_data, has_multiple_layers,
        };

        Ok(version)
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(self, write: &mut W) -> UnitResult {
        use ::bit_field::BitField;

        // the 8 least significant bits contain the file format version number
        // and the flags are set to 0
        let mut version_and_flags = self.file_format_version as u32;

        // the 24 most significant bits are treated as a set of boolean flags
        version_and_flags.set_bit(9, self.is_single_layer_and_tiled);
        version_and_flags.set_bit(10, self.has_long_names);
        version_and_flags.set_bit(11, self.has_deep_data);
        version_and_flags.set_bit(12, self.has_multiple_layers);
        // all remaining bits except 9, 10, 11 and 12 are reserved and should be 0

        version_and_flags.write_le(write)?;
        Ok(())
    }

    /// Validate this instance.
    pub fn validate(&self) -> UnitResult {
        if self.file_format_version == 2 {

            match (
                self.is_single_layer_and_tiled, self.has_deep_data, self.has_multiple_layers,
                self.file_format_version
            ) {
                // Single-part scan line. One normal scan line image.
                (false, false, false, 1..=2) => Ok(()),

                // Single-part tile. One normal tiled image.
                (true, false, false, 1..=2) => Ok(()),

                // Multi-part (new in 2.0).
                // Multiple normal images (scan line and/or tiled).
                (false, false, true, 2) => Ok(()),

                // Single-part deep data (new in 2.0).
                // One deep tile or deep scan line part
                (false, true, false, 2) => Ok(()),

                // Multi-part deep data (new in 2.0).
                // Multiple parts (any combination of:
                // tiles, scan lines, deep tiles and/or deep scan lines).
                (false, true, true, 2) => Ok(()),

                _ => Err(Error::invalid("file feature flags"))
            }
        }
        else {
            Err(Error::unsupported("file versions other than 2.0 are not supported"))
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::meta::header::{ImageAttributes, LayerAttributes};

    #[test]
    fn round_trip_requirements() {
        let requirements = Requirements {
            file_format_version: 2,
            is_single_layer_and_tiled: true,
            has_long_names: false,
            has_deep_data: true,
            has_multiple_layers: false
        };

        let mut data: Vec<u8> = Vec::new();
        requirements.write(&mut data).unwrap();
        let read = Requirements::read(&mut data.as_slice()).unwrap();
        assert_eq!(requirements, read);
    }

    #[test]
    fn round_trip(){
        let header = Header {
            channels: ChannelList::new(smallvec![
                    ChannelDescription {
                        name: Text::from("main"),
                        sample_type: SampleType::U32,
                        quantize_linearly: false,
                        sampling: Vec2(1, 1)
                    }
                ],
            ),
            compression: Compression::Uncompressed,
            line_order: LineOrder::Increasing,
            deep_data_version: Some(1),
            chunk_count: compute_chunk_count(Compression::Uncompressed, Vec2(2000, 333), BlockDescription::ScanLines),
            max_samples_per_pixel: Some(4),
            shared_attributes: ImageAttributes {
                pixel_aspect: 3.0,
                .. ImageAttributes::new(IntegerBounds {
                    position: Vec2(2,1),
                    size: Vec2(11, 9)
                })
            },

            blocks: BlockDescription::ScanLines,
            deep: false,
            layer_size: Vec2(2000, 333),
            own_attributes: LayerAttributes {
                layer_name: Some(Text::from("test name lol")),
                layer_position: Vec2(3, -5),
                screen_window_center: Vec2(0.3, 99.0),
                screen_window_width: 0.19,
                .. Default::default()
            }
        };

        let meta = MetaData {
            requirements: Requirements {
                file_format_version: 2,
                is_single_layer_and_tiled: false,
                has_long_names: false,
                has_deep_data: false,
                has_multiple_layers: false
            },
            headers: smallvec![ header ],
        };


        let mut data: Vec<u8> = Vec::new();
        MetaData::write_validating_to_buffered(&mut data, meta.headers.as_slice(), true).unwrap();
        let meta2 = MetaData::read_from_buffered(data.as_slice(), false).unwrap();
        MetaData::validate(meta2.headers.as_slice(), true).unwrap();
        assert_eq!(meta, meta2);
    }

    #[test]
    fn infer_low_requirements() {
        let header_version_1_short_names = Header {
            channels: ChannelList::new(smallvec![
                    ChannelDescription {
                        name: Text::from("main"),
                        sample_type: SampleType::U32,
                        quantize_linearly: false,
                        sampling: Vec2(1, 1)
                    }
                ],
            ),
            compression: Compression::Uncompressed,
            line_order: LineOrder::Increasing,
            deep_data_version: Some(1),
            chunk_count: compute_chunk_count(Compression::Uncompressed, Vec2(2000, 333), BlockDescription::ScanLines),
            max_samples_per_pixel: Some(4),
            shared_attributes: ImageAttributes {
                pixel_aspect: 3.0,
                .. ImageAttributes::new(IntegerBounds {
                    position: Vec2(2,1),
                    size: Vec2(11, 9)
                })
            },
            blocks: BlockDescription::ScanLines,
            deep: false,
            layer_size: Vec2(2000, 333),
            own_attributes: LayerAttributes {
                other: vec![
                    (Text::try_from("x").unwrap(), AttributeValue::F32(3.0)),
                    (Text::try_from("y").unwrap(), AttributeValue::F32(-1.0)),
                ].into_iter().collect(),
                .. Default::default()
            }
        };

        let low_requirements = MetaData::validate(
            &[header_version_1_short_names], true
        ).unwrap();

        assert_eq!(low_requirements.has_long_names, false);
        assert_eq!(low_requirements.file_format_version, 2); // always have version 2
        assert_eq!(low_requirements.has_deep_data, false);
        assert_eq!(low_requirements.has_multiple_layers, false);
    }

    #[test]
    fn infer_high_requirements() {
        let header_version_2_long_names = Header {
            channels: ChannelList::new(
                smallvec![
                    ChannelDescription {
                        name: Text::new_or_panic("main"),
                        sample_type: SampleType::U32,
                        quantize_linearly: false,
                        sampling: Vec2(1, 1)
                    }
                ],
            ),
            compression: Compression::Uncompressed,
            line_order: LineOrder::Increasing,
            deep_data_version: Some(1),
            chunk_count: compute_chunk_count(Compression::Uncompressed, Vec2(2000, 333), BlockDescription::ScanLines),
            max_samples_per_pixel: Some(4),
            shared_attributes: ImageAttributes {
                pixel_aspect: 3.0,
                .. ImageAttributes::new(IntegerBounds {
                    position: Vec2(2,1),
                    size: Vec2(11, 9)
                })
            },
            blocks: BlockDescription::ScanLines,
            deep: false,
            layer_size: Vec2(2000, 333),
            own_attributes: LayerAttributes {
                layer_name: Some(Text::new_or_panic("oasdasoidfj")),
                other: vec![
                    (Text::new_or_panic("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"), AttributeValue::F32(3.0)),
                    (Text::new_or_panic("y"), AttributeValue::F32(-1.0)),
                ].into_iter().collect(),
                .. Default::default()
            }
        };

        let mut layer_2 = header_version_2_long_names.clone();
        layer_2.own_attributes.layer_name = Some(Text::new_or_panic("anythingelse"));

        let low_requirements = MetaData::validate(
            &[header_version_2_long_names, layer_2], true
        ).unwrap();

        assert_eq!(low_requirements.has_long_names, true);
        assert_eq!(low_requirements.file_format_version, 2);
        assert_eq!(low_requirements.has_deep_data, false);
        assert_eq!(low_requirements.has_multiple_layers, true);
    }
}

