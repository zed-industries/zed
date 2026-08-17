
//! Read and write already compressed pixel data blocks.
//! Does not include the process of compression and decompression.

use crate::meta::attribute::{IntegerBounds};

/// A generic block of pixel information.
/// Contains pixel data and an index to the corresponding header.
/// All pixel data in a file is split into a list of chunks.
/// Also contains positioning information that locates this
/// data block in the referenced layer.
/// The byte data is in little-endian format,
/// as these bytes will be written into the file directly.
#[derive(Debug, Clone)]
pub struct Chunk {

    /// The index of the layer that the block belongs to.
    /// This is required as the pixel data can appear in any order in a file.
    // PDF says u64, but source code seems to be i32
    pub layer_index: usize,

    /// The compressed pixel contents.
    /// This data is compressed and in little-endian format.
    pub compressed_block: CompressedBlock,
}

/// The raw, possibly compressed pixel data of a file.
/// Each layer in a file can have a different type.
/// Also contains positioning information that locates this
/// data block in the corresponding layer.
/// Exists inside a `Chunk`.
/// The byte data is in little-endian format,
/// as these bytes will be written into the file directly.
#[derive(Debug, Clone)]
pub enum CompressedBlock {

    /// Scan line blocks of flat data.
    ScanLine(CompressedScanLineBlock),

    /// Tiles of flat data.
    Tile(CompressedTileBlock),

    /// Scan line blocks of deep data.
    DeepScanLine(CompressedDeepScanLineBlock),

    /// Tiles of deep data.
    DeepTile(CompressedDeepTileBlock),
}

/// A `Block` of possibly compressed flat scan lines.
/// Corresponds to type attribute `scanlineimage`.
/// The byte data is in little-endian format,
/// as these bytes will be written into the file directly.
#[derive(Debug, Clone)]
pub struct CompressedScanLineBlock {

    /// The block's y coordinate is the pixel space y coordinate of the top scan line in the block.
    /// The top scan line block in the image is aligned with the top edge of the data window.
    pub y_coordinate: i32,

    /// One or more scan lines may be stored together as a scan line block.
    /// The number of scan lines per block depends on how the pixel data are compressed.
    /// For each line in the tile, for each channel, the row values are contiguous.
    /// This data is compressed and in little-endian format.
    pub compressed_pixels_le: Vec<u8>,
}

/// This `Block` is a tile of flat (non-deep) data.
/// Corresponds to type attribute `tiledimage`.
/// The byte data is in little-endian format,
/// as these bytes will be written into the file directly.
#[derive(Debug, Clone)]
pub struct CompressedTileBlock {

    /// The tile location.
    pub coordinates: TileCoordinates,

    /// One or more scan lines may be stored together as a scan line block.
    /// The number of scan lines per block depends on how the pixel data are compressed.
    /// For each line in the tile, for each channel, the row values are contiguous.
    /// This data is compressed and in little-endian format.
    pub compressed_pixels_le: Vec<u8>,
}

/// Indicates the position and resolution level of a `TileBlock` or `DeepTileBlock`.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct TileCoordinates {

    /// Index of the tile, not pixel position.
    pub tile_index: Vec2<usize>,

    /// Index of the Mip/Rip level.
    pub level_index: Vec2<usize>,
}

/// This `Block` consists of one or more deep scan lines.
/// Corresponds to type attribute `deepscanline`.
/// The byte data is in little-endian format,
/// as these bytes will be written into the file directly.
#[derive(Debug, Clone)]
pub struct CompressedDeepScanLineBlock {

    /// The block's y coordinate is the pixel space y coordinate of the top scan line in the block.
    /// The top scan line block in the image is aligned with the top edge of the data window.
    pub y_coordinate: i32,

    /// Count of samples.
    pub decompressed_sample_data_size: usize,

    /// The pixel offset table is a list of integers, one for each pixel column within the data window.
    /// Each entry in the table indicates the total number of samples required
    /// to store the pixel in it as well as all pixels to the left of it.
    pub compressed_pixel_offset_table: Vec<i8>,

    /// One or more scan lines may be stored together as a scan line block.
    /// The number of scan lines per block depends on how the pixel data are compressed.
    /// For each line in the tile, for each channel, the row values are contiguous.
    pub compressed_sample_data_le: Vec<u8>,
}

/// This `Block` is a tile of deep data.
/// Corresponds to type attribute `deeptile`.
/// The byte data is in little-endian format,
/// as these bytes will be written into the file directly.
#[derive(Debug, Clone)]
pub struct CompressedDeepTileBlock {

    /// The tile location.
    pub coordinates: TileCoordinates,

    /// Count of samples.
    pub decompressed_sample_data_size: usize,

    /// The pixel offset table is a list of integers, one for each pixel column within the data window.
    /// Each entry in the table indicates the total number of samples required
    /// to store the pixel in it as well as all pixels to the left of it.
    pub compressed_pixel_offset_table: Vec<i8>,

    /// One or more scan lines may be stored together as a scan line block.
    /// The number of scan lines per block depends on how the pixel data are compressed.
    /// For each line in the tile, for each channel, the row values are contiguous.
    pub compressed_sample_data_le: Vec<u8>,
}


use crate::io::*;

impl TileCoordinates {

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        i32::write_le(usize_to_i32(self.tile_index.x(), "tile x")?, write)?;
        i32::write_le(usize_to_i32(self.tile_index.y(), "tile y")?, write)?;
        i32::write_le(usize_to_i32(self.level_index.x(), "level x")?, write)?;
        i32::write_le(usize_to_i32(self.level_index.y(), "level y")?, write)?;
        Ok(())
    }

    /// Read the value without validating.
    pub fn read(read: &mut impl Read) -> Result<Self> {
        let tile_x = i32::read_le(read)?;
        let tile_y = i32::read_le(read)?;

        let level_x = i32::read_le(read)?;
        let level_y = i32::read_le(read)?;

        if level_x > 31 || level_y > 31 {
            // there can be at most 31 levels, because the largest level would have a size of 2^31,
            // which exceeds the maximum 32-bit integer value.
            return Err(Error::invalid("level index exceeding integer maximum"));
        }

        Ok(TileCoordinates {
            tile_index: Vec2(tile_x, tile_y).to_usize("tile coordinate index")?,
            level_index: Vec2(level_x, level_y).to_usize("tile coordinate level")?
        })
    }

    /// The indices which can be used to index into the arrays of a data window.
    /// These coordinates are only valid inside the corresponding one header.
    /// Will start at 0 and always be positive.
    pub fn to_data_indices(&self, tile_size: Vec2<usize>, max: Vec2<usize>) -> Result<IntegerBounds> {
        let x = self.tile_index.x() * tile_size.width();
        let y = self.tile_index.y() * tile_size.height();

        if x >= max.x() || y >= max.y() {
            Err(Error::invalid("tile index"))
        }
        else {
            Ok(IntegerBounds {
                position: Vec2(usize_to_i32(x, "tile x")?, usize_to_i32(y, "tile y")?),
                size: Vec2(
                    calculate_block_size(max.x(), tile_size.width(), x)?,
                    calculate_block_size(max.y(), tile_size.height(), y)?,
                ),
            })
        }
    }

    /// Absolute coordinates inside the global 2D space of a file, may be negative.
    pub fn to_absolute_indices(&self, tile_size: Vec2<usize>, data_window: IntegerBounds) -> Result<IntegerBounds> {
        let data = self.to_data_indices(tile_size, data_window.size)?;
        Ok(data.with_origin(data_window.position))
    }

    /// Returns if this is the original resolution or a smaller copy.
    pub fn is_largest_resolution_level(&self) -> bool {
        self.level_index == Vec2(0, 0)
    }
}



use crate::meta::{MetaData, BlockDescription, calculate_block_size};

impl CompressedScanLineBlock {

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        debug_assert_ne!(self.compressed_pixels_le.len(), 0, "empty blocks should not be put in the file bug");

        i32::write_le(self.y_coordinate, write)?;
        u8::write_i32_sized_slice_le(write, &self.compressed_pixels_le)?;
        Ok(())
    }

    /// Read the value without validating.
    pub fn read(read: &mut impl Read, max_block_byte_size: usize) -> Result<Self> {
        let y_coordinate = i32::read_le(read)?;
        let compressed_pixels_le = u8::read_i32_sized_vec_le(read, max_block_byte_size, Some(max_block_byte_size), "scan line block sample count")?;
        Ok(CompressedScanLineBlock { y_coordinate, compressed_pixels_le })
    }
}

impl CompressedTileBlock {

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        debug_assert_ne!(self.compressed_pixels_le.len(), 0, "empty blocks should not be put in the file bug");

        self.coordinates.write(write)?;
        u8::write_i32_sized_slice_le(write, &self.compressed_pixels_le)?;
        Ok(())
    }

    /// Read the value without validating.
    pub fn read(read: &mut impl Read, max_block_byte_size: usize) -> Result<Self> {
        let coordinates = TileCoordinates::read(read)?;
        let compressed_pixels_le = u8::read_i32_sized_vec_le(read, max_block_byte_size, Some(max_block_byte_size), "tile block sample count")?;
        Ok(CompressedTileBlock { coordinates, compressed_pixels_le })
    }
}

impl CompressedDeepScanLineBlock {

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        debug_assert_ne!(self.compressed_sample_data_le.len(), 0, "empty blocks should not be put in the file bug");

        i32::write_le(self.y_coordinate, write)?;
        u64::write_le(self.compressed_pixel_offset_table.len() as u64, write)?;
        u64::write_le(self.compressed_sample_data_le.len() as u64, write)?; // TODO just guessed
        u64::write_le(self.decompressed_sample_data_size as u64, write)?;
        i8::write_slice_le(write, &self.compressed_pixel_offset_table)?;
        u8::write_slice_le(write, &self.compressed_sample_data_le)?;
        Ok(())
    }

    /// Read the value without validating.
    pub fn read(read: &mut impl Read, max_block_byte_size: usize) -> Result<Self> {
        let y_coordinate = i32::read_le(read)?;
        let compressed_pixel_offset_table_size = u64_to_usize(u64::read_le(read)?, "deep table size")?;
        let compressed_sample_data_size = u64_to_usize(u64::read_le(read)?, "deep size")?;
        let decompressed_sample_data_size = u64_to_usize(u64::read_le(read)?, "raw deep size")?;

        // doc said i32, try u8
        let compressed_pixel_offset_table = i8::read_vec_le(
            read, compressed_pixel_offset_table_size,
            6 * u16::MAX as usize, Some(max_block_byte_size),
            "deep scan line block table size"
        )?;

        let compressed_sample_data_le = u8::read_vec_le(
            read, compressed_sample_data_size,
            6 * u16::MAX as usize, Some(max_block_byte_size),
            "deep scan line block sample count"
        )?;

        Ok(CompressedDeepScanLineBlock {
            y_coordinate,
            decompressed_sample_data_size,
            compressed_pixel_offset_table,
            compressed_sample_data_le,
        })
    }
}


impl CompressedDeepTileBlock {

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        debug_assert_ne!(self.compressed_sample_data_le.len(), 0, "empty blocks should not be put in the file bug");

        self.coordinates.write(write)?;
        u64::write_le(self.compressed_pixel_offset_table.len() as u64, write)?;
        u64::write_le(self.compressed_sample_data_le.len() as u64, write)?; // TODO just guessed
        u64::write_le(self.decompressed_sample_data_size as u64, write)?;
        i8::write_slice_le(write, &self.compressed_pixel_offset_table)?;
        u8::write_slice_le(write, &self.compressed_sample_data_le)?;
        Ok(())
    }

    /// Read the value without validating.
    pub fn read(read: &mut impl Read, hard_max_block_byte_size: usize) -> Result<Self> {
        let coordinates = TileCoordinates::read(read)?;
        let compressed_pixel_offset_table_size = u64_to_usize(u64::read_le(read)?,"deep table size")?;
        let compressed_sample_data_size = u64_to_usize(u64::read_le(read)?, "deep size")?; // TODO u64 just guessed
        let decompressed_sample_data_size = u64_to_usize(u64::read_le(read)?, "raw deep size")?;

        let compressed_pixel_offset_table = i8::read_vec_le(
            read, compressed_pixel_offset_table_size,
            6 * u16::MAX as usize, Some(hard_max_block_byte_size),
            "deep tile block table size"
        )?;

        let compressed_sample_data_le = u8::read_vec_le(
            read, compressed_sample_data_size,
            6 * u16::MAX as usize, Some(hard_max_block_byte_size),
            "deep tile block sample count"
        )?;

        Ok(CompressedDeepTileBlock {
            coordinates,
            decompressed_sample_data_size,
            compressed_pixel_offset_table,
            compressed_sample_data_le,
        })
    }
}

use crate::error::{UnitResult, Result, Error, u64_to_usize, usize_to_i32, i32_to_usize};
use crate::math::Vec2;

/// Validation of chunks is done while reading and writing the actual data. (For example in exr::full_image)
impl Chunk {

    /// Without validation, write this instance to the byte stream.
    pub fn write(&self, write: &mut impl Write, header_count: usize) -> UnitResult {
        debug_assert!(self.layer_index < header_count, "layer index bug"); // validation is done in full_image or simple_image

        if header_count != 1 {  usize_to_i32(self.layer_index, "layer index")?.write_le(write)?; }
        else { assert_eq!(self.layer_index, 0, "invalid header index for single layer file"); }

        match self.compressed_block {
            CompressedBlock::ScanLine     (ref value) => value.write(write),
            CompressedBlock::Tile         (ref value) => value.write(write),
            CompressedBlock::DeepScanLine (ref value) => value.write(write),
            CompressedBlock::DeepTile     (ref value) => value.write(write),
        }
    }

    /// Read the value without validating.
    pub fn read(read: &mut impl Read, meta_data: &MetaData) -> Result<Self> {
        let layer_number = i32_to_usize(
            if meta_data.requirements.is_multilayer() { i32::read_le(read)? } // documentation says u64, but is i32
            else { 0_i32 }, // reference the first header for single-layer images
            "chunk data part number"
        )?;

        if layer_number >= meta_data.headers.len() {
            return Err(Error::invalid("chunk data part number"));
        }

        let header = &meta_data.headers[layer_number];
        let max_block_byte_size = header.max_block_byte_size();

        let chunk = Chunk {
            layer_index: layer_number,
            compressed_block: match header.blocks {
                // flat data
                BlockDescription::ScanLines if !header.deep => CompressedBlock::ScanLine(CompressedScanLineBlock::read(read, max_block_byte_size)?),
                BlockDescription::Tiles(_) if !header.deep     => CompressedBlock::Tile(CompressedTileBlock::read(read, max_block_byte_size)?),

                // deep data
                BlockDescription::ScanLines   => CompressedBlock::DeepScanLine(CompressedDeepScanLineBlock::read(read, max_block_byte_size)?),
                BlockDescription::Tiles(_)    => CompressedBlock::DeepTile(CompressedDeepTileBlock::read(read, max_block_byte_size)?),
            },
        };

        Ok(chunk)
    }
}

