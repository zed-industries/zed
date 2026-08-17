
//! Contains the compression attribute definition
//! and methods to compress and decompress data.


// private modules make non-breaking changes easier
mod zip;
mod rle;
mod piz;
mod pxr24;
mod b44;


use std::convert::TryInto;
use crate::meta::attribute::{IntegerBounds, SampleType, ChannelList};
use crate::error::{Result, Error, usize_to_i32, UnitResult};
use crate::meta::header::Header;


/// A byte vector.
pub type ByteVec = Vec<u8>;

/// A byte slice.
pub type Bytes<'s> = &'s [u8];

/// Specifies which compression method to use.
/// Use uncompressed data for fastest loading and writing speeds.
/// Use RLE compression for fast loading and writing with slight memory savings.
/// Use ZIP compression for slow processing with large memory savings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Compression {

    /// Store uncompressed values.
    /// Produces large files that can be read and written very quickly.
    /// Consider using RLE instead, as it provides some compression with almost equivalent speed.
    Uncompressed,

    /// Produces slightly smaller files
    /// that can still be read and written rather quickly.
    /// The compressed file size is usually between 60 and 75 percent of the uncompressed size.
    /// Works best for images with large flat areas, such as masks and abstract graphics.
    /// This compression method is lossless.
    RLE,

    /// Uses ZIP compression to compress each line. Slowly produces small images
    /// which can be read with moderate speed. This compression method is lossless.
    /// Might be slightly faster but larger than `ZIP16´.
    ZIP1,  // TODO ZIP { individual_lines: bool, compression_level: Option<u8> }  // TODO specify zip compression level?

    /// Uses ZIP compression to compress blocks of 16 lines. Slowly produces small images
    /// which can be read with moderate speed. This compression method is lossless.
    /// Might be slightly slower but smaller than `ZIP1´.
    ZIP16, // TODO collapse with ZIP1

    /// PIZ compression works well for noisy and natural images. Works better with larger tiles.
    /// Only supported for flat images, but not for deep data.
    /// This compression method is lossless.
    // A wavelet transform is applied to the pixel data, and the result is Huffman-
    // encoded. This scheme tends to provide the best compression ratio for the types of
    // images that are typically processed at Industrial Light & Magic. Files are
    // compressed and decompressed at roughly the same speed. For photographic
    // images with film grain, the files are reduced to between 35 and 55 percent of their
    // uncompressed size.
    // PIZ compression works well for scan-line based files, and also for tiled files with
    // large tiles, but small tiles do not shrink much. (PIZ-compressed data start with a
    // relatively long header; if the input to the compressor is short, adding the header
    // tends to offset any size reduction of the input.)
    PIZ,

    /// Like `ZIP1`, but reduces precision of `f32` images to `f24`.
    /// Therefore, this is lossless compression for `f16` and `u32` data, lossy compression for `f32` data.
    /// This compression method works well for depth
    /// buffers and similar images, where the possible range of values is very large, but
    /// where full 32-bit floating-point accuracy is not necessary. Rounding improves
    /// compression significantly by eliminating the pixels' 8 least significant bits, which
    /// tend to be very noisy, and therefore difficult to compress.
    /// This produces really small image files. Only supported for flat images, not for deep data.
    // After reducing 32-bit floating-point data to 24 bits by rounding (while leaving 16-bit
    // floating-point data unchanged), differences between horizontally adjacent pixels
    // are compressed with zlib, similar to ZIP. PXR24 compression preserves image
    // channels of type HALF and UINT exactly, but the relative error of FLOAT data
    // increases to about ???.
    PXR24, // TODO specify zip compression level?

    /// This is a lossy compression method for f16 images.
    /// It's the predecessor of the `B44A` compression,
    /// which has improved compression rates for uniformly colored areas.
    /// You should probably use `B44A` instead of the plain `B44`.
    ///
    /// Only supported for flat images, not for deep data.
    // lossy 4-by-4 pixel block compression,
    // flat fields are compressed more
    // Channels of type HALF are split into blocks of four by four pixels or 32 bytes. Each
    // block is then packed into 14 bytes, reducing the data to 44 percent of their
    // uncompressed size. When B44 compression is applied to RGB images in
    // combination with luminance/chroma encoding (see below), the size of the
    // compressed pixels is about 22 percent of the size of the original RGB data.
    // Channels of type UINT or FLOAT are not compressed.
    // Decoding is fast enough to allow real-time playback of B44-compressed OpenEXR
    // image sequences on commodity hardware.
    // The size of a B44-compressed file depends on the number of pixels in the image,
    // but not on the data in the pixels. All images with the same resolution and the same
    // set of channels have the same size. This can be advantageous for systems that
    // support real-time playback of image sequences; the predictable file size makes it
    // easier to allocate space on storage media efficiently.
    // B44 compression is only supported for flat images.
    B44, // TODO B44 { optimize_uniform_areas: bool }

    /// This is a lossy compression method for f16 images.
    /// All f32 and u32 channels will be stored without compression.
    /// All the f16 pixels are divided into 4x4 blocks.
    /// Each block is then compressed as a whole.
    ///
    /// The 32 bytes of a block will require only ~14 bytes after compression,
    /// independent of the actual pixel contents. With chroma subsampling,
    /// a block will be compressed to ~7 bytes.
    /// Uniformly colored blocks will be compressed to ~3 bytes.
    ///
    /// The 512 bytes of an f32 block will not be compressed at all.
    ///
    /// Should be fast enough for realtime playback.
    /// Only supported for flat images, not for deep data.
    B44A, // TODO collapse with B44

    /// __This lossy compression is not yet supported by this implementation.__
    // lossy DCT based compression, in blocks
    // of 32 scanlines. More efficient for partial buffer access.
    DWAA(Option<f32>), // TODO does this have a default value? make this non optional? default Compression Level setting is 45.0

    /// __This lossy compression is not yet supported by this implementation.__
    // lossy DCT based compression, in blocks
    // of 256 scanlines. More efficient space
    // wise and faster to decode full frames
    // than DWAA_COMPRESSION.
    DWAB(Option<f32>), // TODO collapse with DWAA. default Compression Level setting is 45.0

    /// __This lossy compression is not yet supported by this implementation.__
    // High-Throughput JPEG 2000 (32 lines)
    HTJ2K32,

    /// __This lossy compression is not yet supported by this implementation.__
    // High-Throughput JPEG 2000 (256 lines)
    HTJ2K256,
}

impl std::fmt::Display for Compression {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} compression", match self {
            Compression::Uncompressed => "no",
            Compression::RLE => "rle",
            Compression::ZIP1 => "zip line",
            Compression::ZIP16 => "zip block",
            Compression::B44 => "b44",
            Compression::B44A => "b44a",
            Compression::DWAA(_) => "dwaa",
            Compression::DWAB(_) => "dwab",
            Compression::PIZ => "piz",
            Compression::PXR24 => "pxr24",
            Compression::HTJ2K32 => "ht j2k 32",
            Compression::HTJ2K256 => "ht j2k 256",
        })
    }
}



impl Compression {

    /// Compress the image section, converting from native endian into with little-endian format.
    pub fn compress_image_section_to_le(self, header: &Header, uncompressed_native_endian: ByteVec, pixel_section: IntegerBounds) -> Result<ByteVec> {
        let max_tile_size = header.max_block_pixel_size();

        assert!(pixel_section.validate(Some(max_tile_size)).is_ok(), "decompress tile coordinate bug");
        if header.deep { assert!(self.supports_deep_data()) }

        use self::Compression::*;
        let compressed_little_endian = match self {
            Uncompressed => {
                return convert_current_to_little_endian(
                    uncompressed_native_endian, &header.channels, pixel_section
                )
            },

            // we need to clone here, because we might have to fallback to the uncompressed data later (when compressed data is larger than raw data)
            ZIP16 => zip::compress_bytes(&header.channels, uncompressed_native_endian.clone(), pixel_section),
            ZIP1 => zip::compress_bytes(&header.channels, uncompressed_native_endian.clone(), pixel_section),
            RLE => rle::compress_bytes(&header.channels, uncompressed_native_endian.clone(), pixel_section),
            PIZ => piz::compress(&header.channels, uncompressed_native_endian.clone(), pixel_section),
            PXR24 => pxr24::compress(&header.channels, uncompressed_native_endian.clone(), pixel_section),
            B44 => b44::compress(&header.channels, uncompressed_native_endian.clone(), pixel_section, false),
            B44A => b44::compress(&header.channels, uncompressed_native_endian.clone(), pixel_section, true),
            _ => return Err(Error::unsupported(format!("yet unimplemented compression method: {}", self)))
        };

        let compressed_little_endian = compressed_little_endian.map_err(|_|
            Error::invalid(format!("pixels cannot be compressed ({})", self))
        )?;

        if self == Uncompressed || compressed_little_endian.len() < uncompressed_native_endian.len() {
            // only write compressed if it actually is smaller than raw
            Ok(compressed_little_endian)
        }
        else {
            // if we do not use compression, manually convert uncompressed data
            convert_current_to_little_endian(uncompressed_native_endian, &header.channels, pixel_section)
        }
    }

    /// Decompress the image section from bytes of little-endian format, returning native-endian format.
    pub fn decompress_image_section_from_le(self, header: &Header, compressed_le: ByteVec, pixel_section: IntegerBounds, pedantic: bool) -> Result<ByteVec> {
        let max_tile_size = header.max_block_pixel_size();

        assert!(pixel_section.validate(Some(max_tile_size)).is_ok(), "decompress tile coordinate bug");
        if header.deep { assert!(self.supports_deep_data()) }

        let expected_byte_size = pixel_section.size.area() * header.channels.bytes_per_pixel; // FIXME this needs to account for subsampling anywhere

        // note: always true where self == Uncompressed
        if compressed_le.len() == expected_byte_size {
            // the compressed data was larger than the raw data, so the small raw data has been written
            convert_little_endian_to_current(compressed_le, &header.channels, pixel_section)
        }
        else {
            use self::Compression::*;
            let bytes_ne = match self {
                Uncompressed => convert_little_endian_to_current(compressed_le, &header.channels, pixel_section),
                ZIP16 => zip::decompress_bytes(&header.channels, compressed_le, pixel_section, expected_byte_size, pedantic),
                ZIP1 => zip::decompress_bytes(&header.channels, compressed_le, pixel_section, expected_byte_size, pedantic),
                RLE => rle::decompress_bytes(&header.channels, compressed_le, pixel_section, expected_byte_size, pedantic),
                PIZ => piz::decompress(&header.channels, compressed_le, pixel_section, expected_byte_size, pedantic),
                PXR24 => pxr24::decompress(&header.channels, compressed_le, pixel_section, expected_byte_size, pedantic),
                B44 | B44A => b44::decompress(&header.channels, compressed_le, pixel_section, expected_byte_size, pedantic),
                _ => return Err(Error::unsupported(format!("yet unimplemented compression method: {}", self)))
            };

            // map all errors to compression errors
            let bytes_ne = bytes_ne
                .map_err(|decompression_error| match decompression_error {
                    Error::NotSupported(message) =>
                        Error::unsupported(format!("yet unimplemented compression special case ({})", message)),

                    error => Error::invalid(format!(
                        "compressed {:?} data ({})",
                        self, error.to_string()
                    )),
                })?;

            if bytes_ne.len() != expected_byte_size {
                Err(Error::invalid("decompressed data"))
            }

            else { Ok(bytes_ne) }
        }
    }

    /// For scan line images and deep scan line images, one or more scan lines may be
    /// stored together as a scan line block. The number of scan lines per block
    /// depends on how the pixel data are compressed.
    pub fn scan_lines_per_block(self) -> usize {
        use self::Compression::*;
        match self {
            Uncompressed | RLE     | ZIP1              => 1,
            ZIP16   | PXR24                            => 16,
            PIZ     | B44   | B44A | DWAA(_) | HTJ2K32 => 32,
            DWAB(_) | HTJ2K256                         => 256,
        }
    }

    /// Deep data can only be compressed using RLE or ZIP compression.
    pub fn supports_deep_data(self) -> bool {
        use self::Compression::*;
        match self {
            Uncompressed | RLE | ZIP1 =>
                true,

            ZIP16 | PXR24 | PIZ | B44 | B44A |
            DWAA(_) | DWAB(_) | HTJ2K256 | HTJ2K32 =>
                false,
        }
    }

    /// Most compression methods will reconstruct the exact pixel bytes,
    /// but some might throw away unimportant data for specific types of samples.
    pub fn is_lossless_for(self, sample_type: SampleType) -> bool {
        use self::Compression::*;
        match self {
            PXR24 => sample_type != SampleType::F32, // pxr reduces f32 to f24
            B44 | B44A => sample_type != SampleType::F16, // b44 only compresses f16 values, others are left uncompressed
            Uncompressed | RLE | ZIP1 | ZIP16 | PIZ | HTJ2K32 | HTJ2K256 => true,
            DWAB(_) | DWAA(_) => false,
        }
    }

    /// Most compression methods will reconstruct the exact pixel bytes,
    /// but some might throw away unimportant data in some cases.
    pub fn may_loose_data(self) -> bool {
        use self::Compression::*;
        match self {
            Uncompressed | RLE | ZIP1 | ZIP16 | PIZ | HTJ2K32 | HTJ2K256 => false,
            PXR24 | B44 | B44A | DWAB(_) | DWAA(_) => true,
        }
    }

    /// Most compression methods will reconstruct the exact pixel bytes,
    /// but some might replace NaN with zeroes.
    /// This might also depend on the sample type of the pixels.
    /// Even a compression method that supports NaN might change the bit patterns of those NaNs.
    pub fn supports_nan(self) -> bool {
        use self::Compression::*;
        match self {
            B44A | DWAB(_) | DWAA(_) => false,
            Uncompressed | PXR24 | RLE | ZIP1 | ZIP16 | PIZ | B44 | HTJ2K32 | HTJ2K256 => true,
        }
    }

    /// Most compression methods will reconstruct the exact pixel and NaN bits,
    /// but some might replace NaN bits with other NaN bits.
    /// This might also depend on the sample type of the pixels.
    pub fn preserves_nan_bits(self) -> bool {
        use self::Compression::*;
        match self {
            B44A | PXR24 | DWAB(_) | DWAA(_) => false,
            B44 | Uncompressed | RLE | ZIP1 | ZIP16 | PIZ | HTJ2K32 | HTJ2K256 => true
        }
    }

}

// see https://github.com/AcademySoftwareFoundation/openexr/blob/6a9f8af6e89547bcd370ae3cec2b12849eee0b54/OpenEXR/IlmImf/ImfMisc.cpp#L1456-L1541

#[allow(unused)] // allows the extra parameters to be unused
fn convert_current_to_little_endian(mut bytes: ByteVec, channels: &ChannelList, rectangle: IntegerBounds) -> Result<ByteVec> {
    #[cfg(target_endian = "big")]
    reverse_block_endianness(&mut bytes, channels, rectangle)?;

    Ok(bytes)
}

#[allow(unused)] // allows the extra parameters to be unused
fn convert_little_endian_to_current(mut bytes: ByteVec, channels: &ChannelList, rectangle: IntegerBounds) -> Result<ByteVec> {
    #[cfg(target_endian = "big")]
    reverse_block_endianness(&mut bytes, channels, rectangle)?;

    Ok(bytes)
}

#[allow(unused)] // unused when on little endian system
fn reverse_block_endianness(bytes: &mut [u8], channels: &ChannelList, rectangle: IntegerBounds) -> UnitResult {
    let mut remaining_bytes: &mut [u8] = bytes;

    for y in rectangle.position.y() .. rectangle.end().y() {
        for channel in &channels.list {
            let line_is_subsampled = mod_p(y, usize_to_i32(channel.sampling.y(), "sampling")?) != 0;
            if line_is_subsampled { continue; }

            let sample_count = rectangle.size.width() / channel.sampling.x();

            match channel.sample_type {
                SampleType::F16 =>
                    remaining_bytes = convert_byte_chunks(reverse_2_bytes, 2, remaining_bytes, sample_count),

                SampleType::F32 =>
                    remaining_bytes = convert_byte_chunks(reverse_4_bytes, 4, remaining_bytes, sample_count),

                SampleType::U32 =>
                    remaining_bytes = convert_byte_chunks(reverse_4_bytes, 4, remaining_bytes, sample_count),
            }
        }
    }

    // Converts groups of bytes (e.g. 2 bytes), as many groups as specified. Returns a slice of the remaining bytes.
    #[inline]
    fn convert_byte_chunks(convert_single_value: fn(&mut[u8]), batch_size: usize, bytes: &mut [u8], batch_count: usize) -> &mut [u8] {
        let (line_bytes, rest) = bytes.split_at_mut(batch_count * batch_size);
        let value_byte_chunks = line_bytes.chunks_exact_mut(batch_size);

        for value_bytes in value_byte_chunks {
            convert_single_value(value_bytes);
        }

        rest
    }

    debug_assert!(remaining_bytes.is_empty(), "not all bytes were converted to little endian");
    Ok(())
}

#[inline]
fn reverse_2_bytes(bytes: &mut [u8]){
    // this code seems like it could be optimized easily by the compiler
    let two_bytes: [u8; 2] = bytes.try_into().expect("invalid byte count");
    bytes.copy_from_slice(&[two_bytes[1], two_bytes[0]]);
}

#[inline]
fn reverse_4_bytes(bytes: &mut [u8]){
    let four_bytes: [u8; 4] = bytes.try_into().expect("invalid byte count");
    bytes.copy_from_slice(&[four_bytes[3], four_bytes[2], four_bytes[1], four_bytes[0]]);
}

#[inline]
fn div_p (x: i32, y: i32) -> i32 {
    if x >= 0 {
        if y >= 0 { x  / y }
        else { -(x  / -y) }
    }
    else {
        if y >= 0 { -((y-1-x) / y) }
        else { (-y-1-x) / -y }
    }
}

#[inline]
fn mod_p(x: i32, y: i32) -> i32 {
    x - y * div_p(x, y)
}

/// A collection of functions used to prepare data for compression.
mod optimize_bytes {

    /// Integrate over all differences to the previous value in order to reconstruct sample values.
    pub fn differences_to_samples(buffer: &mut [u8]) {
        // The naive implementation is very simple:
        //
        // for index in 1..buffer.len() {
        //    buffer[index] = (buffer[index - 1] as i32 + buffer[index] as i32 - 128) as u8;
        // }
        //
        // But we process elements in pairs to take advantage of instruction-level parallelism.
        // When computations within a pair do not depend on each other, they can be processed in parallel.
        // Since this function is responsible for a very large chunk of execution time,
        // this tweak alone improves decoding performance of RLE images by 20%.
        if let Some(first) = buffer.get(0) {
            let mut previous = *first as i16;
            for chunk in &mut buffer[1..].chunks_exact_mut(2) {
                // no bounds checks here due to indices and chunk size being constant
                let diff0 = chunk[0] as i16;
                let diff1 = chunk[1] as i16;
                // these two computations do not depend on each other, unlike in the naive version,
                // so they can be executed by the CPU in parallel via instruction-level parallelism
                let sample0 = (previous + diff0 - 128) as u8;
                let sample1 = (previous + diff0 + diff1 - 128 * 2) as u8;
                chunk[0] = sample0;
                chunk[1] = sample1;
                previous = sample1 as i16;
            }
            // handle the remaining element at the end not processed by the loop over pairs, if present
            for elem in &mut buffer[1..].chunks_exact_mut(2).into_remainder().iter_mut() {
                let sample = (previous + *elem as i16 - 128) as u8;
                *elem = sample;
                previous = sample as i16;
            }
        }
    }

    /// Derive over all values in order to produce differences to the previous value.
    pub fn samples_to_differences(buffer: &mut [u8]){
        // naive version:
        // for index in (1..buffer.len()).rev() {
        //     buffer[index] = (buffer[index] as i32 - buffer[index - 1] as i32 + 128) as u8;
        // }
        //
        // But we process elements in batches to take advantage of autovectorization.
        // If the target platform has no vector instructions (e.g. 32-bit ARM without `-C target-cpu=native`)
        // this will instead take advantage of instruction-level parallelism.
        if let Some(first) = buffer.get(0) {
            let mut previous = *first as i16;
            // Chunk size is 16 because we process bytes (8 bits),
            // and 8*16 = 128 bits is the size of a typical SIMD register.
            // Even WASM has 128-bit SIMD registers.
            for chunk in &mut buffer[1..].chunks_exact_mut(16) {
                // no bounds checks here due to indices and chunk size being constant
                let sample0 = chunk[0] as i16;
                let sample1 = chunk[1] as i16;
                let sample2 = chunk[2] as i16;
                let sample3 = chunk[3] as i16;
                let sample4 = chunk[4] as i16;
                let sample5 = chunk[5] as i16;
                let sample6 = chunk[6] as i16;
                let sample7 = chunk[7] as i16;
                let sample8 = chunk[8] as i16;
                let sample9 = chunk[9] as i16;
                let sample10 = chunk[10] as i16;
                let sample11 = chunk[11] as i16;
                let sample12 = chunk[12] as i16;
                let sample13 = chunk[13] as i16;
                let sample14 = chunk[14] as i16;
                let sample15 = chunk[15] as i16;
                // Unlike in decoding, computations in here are truly independent from each other,
                // which enables the compiler to vectorize this loop.
                // Even if the target platform has no vector instructions,
                // so using more parallelism doesn't imply doing more work,
                // and we're not really limited in how wide we can go.
                chunk[0] = (sample0 - previous + 128) as u8;
                chunk[1] = (sample1 - sample0 + 128) as u8;
                chunk[2] = (sample2 - sample1 + 128) as u8;
                chunk[3] = (sample3 - sample2 + 128) as u8;
                chunk[4] = (sample4 - sample3 + 128) as u8;
                chunk[5] = (sample5 - sample4 + 128) as u8;
                chunk[6] = (sample6 - sample5 + 128) as u8;
                chunk[7] = (sample7 - sample6 + 128) as u8;
                chunk[8] = (sample8 - sample7 + 128) as u8;
                chunk[9] = (sample9 - sample8 + 128) as u8;
                chunk[10] = (sample10 - sample9 + 128) as u8;
                chunk[11] = (sample11 - sample10 + 128) as u8;
                chunk[12] = (sample12 - sample11 + 128) as u8;
                chunk[13] = (sample13 - sample12 + 128) as u8;
                chunk[14] = (sample14 - sample13 + 128) as u8;
                chunk[15] = (sample15 - sample14 + 128) as u8;
                previous = sample15;
            }
            // Handle the remaining element at the end not processed by the loop over batches, if present
            // This is what the iterator-based version of this function would look like without vectorization
            for elem in &mut buffer[1..].chunks_exact_mut(16).into_remainder().iter_mut() {
                let diff = (*elem as i16 - previous + 128) as u8;
                previous = *elem as i16;
                *elem = diff;
            }
        }
    }

    use std::cell::Cell;
    thread_local! {
        // A buffer for reusing between invocations of interleaving and deinterleaving.
        // Allocating memory is cheap, but zeroing or otherwise initializing it is not.
        // Doing it hundreds of times (once per block) would be expensive.
        // This optimization brings down the time spent in interleaving from 15% to 5%.
        static SCRATCH_SPACE: Cell<Vec<u8>> = Cell::new(Vec::new());
    }

    fn with_reused_buffer<F>(length: usize, mut func: F) where F: FnMut(&mut [u8]) {
        SCRATCH_SPACE.with(|scratch_space| {
            // reuse a buffer if we've already initialized one
            let mut buffer = scratch_space.take();
            if buffer.len() < length {
                // Efficiently create a zeroed Vec by requesting zeroed memory from the OS.
                // This is slightly faster than a `memcpy()` plus `memset()` that would happen otherwise,
                // but is not a big deal either way since it's not a hot codepath.
                buffer = vec![0u8; length];
            }

            // call the function
            func(&mut buffer[..length]);

            // save the internal buffer for reuse
            scratch_space.set(buffer);
        });
    }

    /// Interleave the bytes such that the second half of the array is every other byte.
    pub fn interleave_byte_blocks(separated: &mut [u8]) {
        with_reused_buffer(separated.len(), |interleaved| {

            // Split the two halves that we are going to interleave.
            let (first_half, second_half) = separated.split_at((separated.len() + 1) / 2);
            // The first half can be 1 byte longer than the second if the length of the input is odd,
            // but the loop below only processes numbers in pairs.
            // To handle it, preserve the last element of the first slice, to be handled after the loop.
            let first_half_last = first_half.last();
            // Truncate the first half to match the lenght of the second one; more optimizer-friendly
            let first_half_iter = &first_half[..second_half.len()];

            // Main loop that performs the interleaving
            for ((first, second), interleaved) in first_half_iter.iter().zip(second_half.iter())
                .zip(interleaved.chunks_exact_mut(2)) {
                    // The length of each chunk is known to be 2 at compile time,
                    // and each index is also a constant.
                    // This allows the compiler to remove the bounds checks.
                    interleaved[0] = *first;
                    interleaved[1] = *second;
            }

            // If the length of the slice was odd, restore the last element of the first half that we saved
            if interleaved.len() % 2 == 1 {
                if let Some(value) = first_half_last {
                    // we can unwrap() here because we just checked that the lenght is non-zero:
                    // `% 2 == 1` will fail for zero
                    *interleaved.last_mut().unwrap() = *value;
                }
            }

            // write out the results
            separated.copy_from_slice(&interleaved);
        });
    }

    /// Separate the bytes such that the second half contains every other byte.
    /// This performs deinterleaving - the inverse of interleaving.
    pub fn separate_bytes_fragments(source: &mut [u8]) {
        with_reused_buffer(source.len(), |separated| {

            // Split the two halves that we are going to interleave.
            let (first_half, second_half) = separated.split_at_mut((source.len() + 1) / 2);
            // The first half can be 1 byte longer than the second if the length of the input is odd,
            // but the loop below only processes numbers in pairs.
            // To handle it, preserve the last element of the input, to be handled after the loop.
            let last = source.last();
            let first_half_iter = &mut first_half[..second_half.len()];

            // Main loop that performs the deinterleaving
            for ((first, second), interleaved) in first_half_iter.iter_mut().zip(second_half.iter_mut())
                .zip(source.chunks_exact(2)) {
                    // The length of each chunk is known to be 2 at compile time,
                    // and each index is also a constant.
                    // This allows the compiler to remove the bounds checks.
                    *first = interleaved[0];
                    *second = interleaved[1];
            }

            // If the length of the slice was odd, restore the last element of the input that we saved
            if source.len() % 2 == 1 {
                if let Some(value) = last {
                    // we can unwrap() here because we just checked that the lenght is non-zero:
                    // `% 2 == 1` will fail for zero
                    *first_half.last_mut().unwrap() = *value;
                }
            }

            // write out the results
            source.copy_from_slice(&separated);
        });
    }


    #[cfg(test)]
    pub mod test {

        #[test]
        fn roundtrip_interleave(){
            let source = vec![ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ];
            let mut modified = source.clone();

            super::separate_bytes_fragments(&mut modified);
            super::interleave_byte_blocks(&mut modified);

            assert_eq!(source, modified);
        }

        #[test]
        fn roundtrip_derive(){
            let source = vec![ 0, 1, 2, 7, 4, 5, 6, 7, 13, 9, 10 ];
            let mut modified = source.clone();

            super::samples_to_differences(&mut modified);
            super::differences_to_samples(&mut modified);

            assert_eq!(source, modified);
        }

    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::meta::attribute::ChannelDescription;
    use crate::block::samples::IntoNativeSample;

    #[test]
    fn roundtrip_endianness_mixed_channels(){
        let a32 = ChannelDescription::new("A", SampleType::F32, true);
        let y16 = ChannelDescription::new("Y", SampleType::F16, true);
        let channels = ChannelList::new(smallvec![ a32, y16 ]);

        let data = vec![
            23582740683_f32.to_ne_bytes().as_slice(),
            35827420683_f32.to_ne_bytes().as_slice(),
            27406832358_f32.to_f16().to_ne_bytes().as_slice(),
            74062358283_f32.to_f16().to_ne_bytes().as_slice(),

            52582740683_f32.to_ne_bytes().as_slice(),
            45827420683_f32.to_ne_bytes().as_slice(),
            15406832358_f32.to_f16().to_ne_bytes().as_slice(),
            65062358283_f32.to_f16().to_ne_bytes().as_slice(),
        ].into_iter().flatten().map(|x| *x).collect();

        roundtrip_convert_endianness(
            data, &channels,
            IntegerBounds::from_dimensions((2, 2))
        );
    }

    fn roundtrip_convert_endianness(
        current_endian: ByteVec, channels: &ChannelList, rectangle: IntegerBounds
    ){
        let little_endian = convert_current_to_little_endian(
            current_endian.clone(), channels, rectangle
        ).unwrap();

        let current_endian_decoded = convert_little_endian_to_current(
            little_endian.clone(), channels, rectangle
        ).unwrap();

        assert_eq!(current_endian, current_endian_decoded, "endianness conversion failed");
    }
}