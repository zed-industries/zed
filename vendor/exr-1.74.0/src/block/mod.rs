//! This is the low-level interface for the raw blocks of an image.
//! See `exr::image` module for a high-level interface.
//!
//! Handle compressed and uncompressed pixel byte blocks. Includes compression and decompression,
//! and reading a complete image into blocks.
//!
//! Start with the `block::read(...)`
//! and `block::write(...)` functions.


pub mod writer;
pub mod reader;

pub mod lines;
pub mod samples;
pub mod chunk;


use std::io::{Read, Seek, Write};
use crate::error::{Result, UnitResult, Error, usize_to_i32};
use crate::meta::{Headers, MetaData, BlockDescription};
use crate::math::Vec2;
use crate::compression::ByteVec;
use crate::block::chunk::{CompressedBlock, CompressedTileBlock, CompressedScanLineBlock, Chunk, TileCoordinates};
use crate::meta::header::Header;
use crate::block::lines::{LineIndex, LineRef, LineSlice, LineRefMut};
use crate::meta::attribute::ChannelList;


/// Specifies where a block of pixel data should be placed in the actual image.
/// This is a globally unique identifier which
/// includes the layer, level index, and pixel location.
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct BlockIndex {

    /// Index of the layer.
    pub layer: usize,

    /// Index of the top left pixel from the block within the data window.
    pub pixel_position: Vec2<usize>,

    /// Number of pixels in this block, extending to the right and downwards.
    /// Stays the same across all resolution levels.
    pub pixel_size: Vec2<usize>,

    /// Index of the mip or rip level in the image.
    pub level: Vec2<usize>,
}

/// Contains a block of pixel data and where that data should be placed in the actual image.
/// The bytes must be encoded in native-endian format.
/// The conversion to little-endian format happens when converting to chunks (potentially in parallel).
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UncompressedBlock {

    /// Location of the data inside the image.
    pub index: BlockIndex,

    /// Uncompressed pixel values of the whole block.
    /// One or more scan lines may be stored together as a scan line block.
    /// This byte vector contains all pixel rows, one after another.
    /// For each line in the tile, for each channel, the row values are contiguous.
    /// Stores all samples of the first channel, then all samples of the second channel, and so on.
    /// This data is in native-endian format.
    pub data: ByteVec,
}

/// Immediately reads the meta data from the file.
/// Then, returns a reader that can be used to read all pixel blocks.
/// From the reader, you can pull each compressed chunk from the file.
/// Alternatively, you can create a decompressor, and pull the uncompressed data from it.
/// The reader is assumed to be buffered.
pub fn read<R: Read + Seek>(buffered_read: R, pedantic: bool) -> Result<self::reader::Reader<R>> {
    self::reader::Reader::read_from_buffered(buffered_read, pedantic)
}

/// Immediately writes the meta data to the file.
/// Then, calls a closure with a writer that can be used to write all pixel blocks.
/// In the closure, you can push compressed chunks directly into the writer.
/// Alternatively, you can create a compressor, wrapping the writer, and push the uncompressed data to it.
/// The writer is assumed to be buffered.
pub fn write<W: Write + Seek>(
    buffered_write: W, headers: Headers, compatibility_checks: bool,
    write_chunks: impl FnOnce(MetaData, &mut self::writer::ChunkWriter<W>) -> UnitResult
) -> UnitResult {
    self::writer::write_chunks_with(buffered_write, headers, compatibility_checks, write_chunks)
}




/// This iterator tells you the block indices of all blocks that must be in the image.
/// The order of the blocks depends on the `LineOrder` attribute
/// (unspecified line order is treated the same as increasing line order).
/// The blocks written to the file must be exactly in this order,
/// except for when the `LineOrder` is unspecified.
/// The index represents the block index, in increasing line order, within the header.
pub fn enumerate_ordered_header_block_indices(headers: &[Header]) -> impl '_ + Iterator<Item=(usize, BlockIndex)> {
    headers.iter().enumerate().flat_map(|(layer_index, header)|{
        header.enumerate_ordered_blocks().map(move |(index_in_header, tile)|{
            let data_indices = header.get_absolute_block_pixel_coordinates(tile.location).expect("tile coordinate bug");

            let block = BlockIndex {
                layer: layer_index,
                level: tile.location.level_index,
                pixel_position: data_indices.position.to_usize("data indices start").expect("data index bug"),
                pixel_size: data_indices.size,
            };

            (index_in_header, block)
        })
    })
}


impl UncompressedBlock {

    /// Decompress the possibly compressed chunk and returns an `UncompressedBlock`.
    // for uncompressed data, the ByteVec in the chunk is moved all the way
    #[inline]
    #[must_use]
    pub fn decompress_chunk(chunk: Chunk, meta_data: &MetaData, pedantic: bool) -> Result<Self> {
        let header: &Header = meta_data.headers.get(chunk.layer_index)
            .ok_or(Error::invalid("chunk layer index"))?;

        let tile_data_indices = header.get_block_data_indices(&chunk.compressed_block)?;
        let absolute_indices = header.get_absolute_block_pixel_coordinates(tile_data_indices)?;

        absolute_indices.validate(Some(header.layer_size))?;

        match chunk.compressed_block {
            CompressedBlock::Tile(CompressedTileBlock { compressed_pixels_le, .. }) |
            CompressedBlock::ScanLine(CompressedScanLineBlock { compressed_pixels_le, .. }) => {
                Ok(UncompressedBlock {
                    data: header.compression.decompress_image_section_from_le(header, compressed_pixels_le, absolute_indices, pedantic)?,
                    index: BlockIndex {
                        layer: chunk.layer_index,
                        pixel_position: absolute_indices.position.to_usize("data indices start")?,
                        level: tile_data_indices.level_index,
                        pixel_size: absolute_indices.size,
                    }
                })
            },

            _ => return Err(Error::unsupported("deep data not supported yet"))
        }
    }

    /// Consume this block by compressing it, returning a `Chunk`.
    // for uncompressed data, the ByteVec in the chunk is moved all the way
    #[inline]
    #[must_use]
    pub fn compress_to_chunk(self, headers: &[Header]) -> Result<Chunk> {
        let UncompressedBlock { data, index } = self;

        let header: &Header = headers.get(index.layer)
            .expect("block layer index bug");

        let expected_byte_size = header.channels.bytes_per_pixel * self.index.pixel_size.area(); // TODO sampling??
        if expected_byte_size != data.len() {
            panic!("get_line byte size should be {} but was {}", expected_byte_size, data.len());
        }

        let tile_coordinates = TileCoordinates {
            // FIXME this calculation should not be made here but elsewhere instead (in meta::header?)
            tile_index: index.pixel_position / header.max_block_pixel_size(), // TODO sampling??
            level_index: index.level,
        };

        let absolute_indices = header.get_absolute_block_pixel_coordinates(tile_coordinates)?;
        absolute_indices.validate(Some(header.layer_size))?;

        if !header.compression.may_loose_data() { debug_assert_eq!(
            &header.compression.decompress_image_section_from_le(
                header,
                header.compression.compress_image_section_to_le(header, data.clone(), absolute_indices)?,
                absolute_indices,
                true
            ).unwrap(),
            &data,
            "compression method not round trippin'"
        ); }

        let compressed_pixels_le = header.compression.compress_image_section_to_le(header, data, absolute_indices)?;

        Ok(Chunk {
            layer_index: index.layer,
            compressed_block : match header.blocks {
                BlockDescription::ScanLines => CompressedBlock::ScanLine(CompressedScanLineBlock {
                    compressed_pixels_le,

                    // FIXME this calculation should not be made here but elsewhere instead (in meta::header?)
                    y_coordinate: usize_to_i32(index.pixel_position.y(), "pixel index")? + header.own_attributes.layer_position.y(), // TODO sampling??
                }),

                BlockDescription::Tiles(_) => CompressedBlock::Tile(CompressedTileBlock {
                    compressed_pixels_le,
                    coordinates: tile_coordinates,
                }),
            }
        })
    }

    /// Iterate all the lines in this block.
    /// Each line contains the all samples for one of the channels.
    pub fn lines(&self, channels: &ChannelList) -> impl Iterator<Item=LineRef<'_>> {
        LineIndex::lines_in_block(self.index, channels)
            .map(move |(bytes, line)| LineSlice { location: line, value: &self.data[bytes] })
    }

    /* TODO pub fn lines_mut<'s>(&'s mut self, header: &Header) -> impl 's + Iterator<Item=LineRefMut<'s>> {
        LineIndex::lines_in_block(self.index, &header.channels)
            .map(move |(bytes, line)| LineSlice { location: line, value: &mut self.data[bytes] })
    }*/

    /*// TODO make iterator
    /// Call a closure for each line of samples in this uncompressed block.
    pub fn for_lines(
        &self, header: &Header,
        mut accept_line: impl FnMut(LineRef<'_>) -> UnitResult
    ) -> UnitResult {
        for (bytes, line) in LineIndex::lines_in_block(self.index, &header.channels) {
            let line_ref = LineSlice { location: line, value: &self.data[bytes] };
            accept_line(line_ref)?;
        }

        Ok(())
    }*/

    // TODO from iterator??
    /// Create an uncompressed block byte vector by requesting one line of samples after another.
    pub fn collect_block_data_from_lines(
        channels: &ChannelList, block_index: BlockIndex,
        mut extract_line: impl FnMut(LineRefMut<'_>)
    ) -> Vec<u8>
    {
        let byte_count = block_index.pixel_size.area() * channels.bytes_per_pixel;
        let mut block_bytes = vec![0_u8; byte_count];

        for (byte_range, line_index) in LineIndex::lines_in_block(block_index, channels) {
            extract_line(LineRefMut { // TODO subsampling
                value: &mut block_bytes[byte_range],
                location: line_index,
            });
        }

        block_bytes
    }

    /// Create an uncompressed block by requesting one line of samples after another.
    pub fn from_lines(
        channels: &ChannelList, block_index: BlockIndex,
        extract_line: impl FnMut(LineRefMut<'_>)
    ) -> Self {
        Self {
            index: block_index,
            data: Self::collect_block_data_from_lines(channels, block_index, extract_line)
        }
    }
}