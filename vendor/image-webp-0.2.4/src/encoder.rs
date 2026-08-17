//! Encoding of WebP images.
use std::collections::BinaryHeap;
use std::io::{self, Write};
use std::slice::ChunksExact;

use quick_error::quick_error;

/// Color type of the image.
///
/// Note that the WebP format doesn't have a concept of color type. All images are encoded as RGBA
/// and some decoders may treat them as such. This enum is used to indicate the color type of the
/// input data provided to the encoder, which can help improve compression ratio.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorType {
    /// Opaque image with a single luminance byte per pixel.
    L8,
    /// Image with a luminance and alpha byte per pixel.
    La8,
    /// Opaque image with a red, green, and blue byte per pixel.
    Rgb8,
    /// Image with a red, green, blue, and alpha byte per pixel.
    Rgba8,
}

quick_error! {
    /// Error that can occur during encoding.
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum EncodingError {
        /// An IO error occurred.
        IoError(err: io::Error) {
            from()
            display("IO error: {}", err)
            source(err)
        }

        /// The image dimensions are not allowed by the WebP format.
        InvalidDimensions {
            display("Invalid dimensions")
        }
    }
}

struct BitWriter<W> {
    writer: W,
    buffer: u64,
    nbits: u8,
}

impl<W: Write> BitWriter<W> {
    fn write_bits(&mut self, bits: u64, nbits: u8) -> io::Result<()> {
        debug_assert!(nbits <= 64);

        self.buffer |= bits << self.nbits;
        self.nbits += nbits;

        if self.nbits >= 64 {
            self.writer.write_all(&self.buffer.to_le_bytes())?;
            self.nbits -= 64;
            self.buffer = bits.checked_shr(u32::from(nbits - self.nbits)).unwrap_or(0);
        }
        debug_assert!(self.nbits < 64);
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.nbits % 8 != 0 {
            self.write_bits(0, 8 - self.nbits % 8)?;
        }
        if self.nbits > 0 {
            self.writer
                .write_all(&self.buffer.to_le_bytes()[..self.nbits as usize / 8])
                .unwrap();
            self.buffer = 0;
            self.nbits = 0;
        }
        Ok(())
    }
}

fn write_single_entry_huffman_tree<W: Write>(w: &mut BitWriter<W>, symbol: u8) -> io::Result<()> {
    w.write_bits(1, 2)?;
    if symbol <= 1 {
        w.write_bits(0, 1)?;
        w.write_bits(u64::from(symbol), 1)?;
    } else {
        w.write_bits(1, 1)?;
        w.write_bits(u64::from(symbol), 8)?;
    }
    Ok(())
}

fn build_huffman_tree(
    frequencies: &[u32],
    lengths: &mut [u8],
    codes: &mut [u16],
    length_limit: u8,
) -> bool {
    assert_eq!(frequencies.len(), lengths.len());
    assert_eq!(frequencies.len(), codes.len());

    if frequencies.iter().filter(|&&f| f > 0).count() <= 1 {
        lengths.fill(0);
        codes.fill(0);
        return false;
    }

    #[derive(Eq, PartialEq, Copy, Clone, Debug)]
    struct Item(u32, u16);
    impl Ord for Item {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.0.cmp(&self.0)
        }
    }
    impl PartialOrd for Item {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    // Build a huffman tree
    let mut internal_nodes = Vec::new();
    let mut nodes = BinaryHeap::from_iter(
        frequencies
            .iter()
            .enumerate()
            .filter(|(_, &frequency)| frequency > 0)
            .map(|(i, &frequency)| Item(frequency, i as u16)),
    );
    while nodes.len() > 1 {
        let Item(frequency1, index1) = nodes.pop().unwrap();
        let mut root = nodes.peek_mut().unwrap();
        internal_nodes.push((index1, root.1));
        *root = Item(
            frequency1 + root.0,
            internal_nodes.len() as u16 + frequencies.len() as u16 - 1,
        );
    }

    // Walk the tree to assign code lengths
    lengths.fill(0);
    let mut stack = Vec::new();
    stack.push((nodes.pop().unwrap().1, 0));
    while let Some((node, depth)) = stack.pop() {
        let node = node as usize;
        if node < frequencies.len() {
            lengths[node] = depth as u8;
        } else {
            let (left, right) = internal_nodes[node - frequencies.len()];
            stack.push((left, depth + 1));
            stack.push((right, depth + 1));
        }
    }

    // Limit the codes to length length_limit
    let mut max_length = 0;
    for &length in lengths.iter() {
        max_length = max_length.max(length);
    }
    if max_length > length_limit {
        let mut counts = [0u32; 16];
        for &length in lengths.iter() {
            counts[length.min(length_limit) as usize] += 1;
        }

        let mut total = 0;
        for (i, count) in counts
            .iter()
            .enumerate()
            .skip(1)
            .take(length_limit as usize)
        {
            total += count << (length_limit as usize - i);
        }

        while total > 1u32 << length_limit {
            let mut i = length_limit as usize - 1;
            while counts[i] == 0 {
                i -= 1;
            }
            counts[i] -= 1;
            counts[length_limit as usize] -= 1;
            counts[i + 1] += 2;
            total -= 1;
        }

        // assign new lengths
        let mut len = length_limit;
        let mut indexes = frequencies.iter().copied().enumerate().collect::<Vec<_>>();
        indexes.sort_unstable_by_key(|&(_, frequency)| frequency);
        for &(i, frequency) in &indexes {
            if frequency > 0 {
                while counts[len as usize] == 0 {
                    len -= 1;
                }
                lengths[i] = len;
                counts[len as usize] -= 1;
            }
        }
    }

    // Assign codes
    codes.fill(0);
    let mut code = 0u32;
    for len in 1..=length_limit {
        for (i, &length) in lengths.iter().enumerate() {
            if length == len {
                codes[i] = (code as u16).reverse_bits() >> (16 - len);
                code += 1;
            }
        }
        code <<= 1;
    }
    assert_eq!(code, 2 << length_limit);

    true
}

fn write_huffman_tree<W: Write>(
    w: &mut BitWriter<W>,
    frequencies: &[u32],
    lengths: &mut [u8],
    codes: &mut [u16],
) -> io::Result<()> {
    if !build_huffman_tree(frequencies, lengths, codes, 15) {
        let symbol = frequencies
            .iter()
            .position(|&frequency| frequency > 0)
            .unwrap_or(0);
        return write_single_entry_huffman_tree(w, symbol as u8);
    }

    let mut code_length_lengths = [0u8; 16];
    let mut code_length_codes = [0u16; 16];
    let mut code_length_frequencies = [0u32; 16];
    for &length in lengths.iter() {
        code_length_frequencies[length as usize] += 1;
    }
    let single_code_length_length = !build_huffman_tree(
        &code_length_frequencies,
        &mut code_length_lengths,
        &mut code_length_codes,
        7,
    );

    const CODE_LENGTH_ORDER: [usize; 19] = [
        17, 18, 0, 1, 2, 3, 4, 5, 16, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    ];

    // Write the huffman tree
    w.write_bits(0, 1)?; // normal huffman tree
    w.write_bits(19 - 4, 4)?; // num_code_lengths - 4

    for i in CODE_LENGTH_ORDER {
        if i > 15 || code_length_frequencies[i] == 0 {
            w.write_bits(0, 3)?;
        } else if single_code_length_length {
            w.write_bits(1, 3)?;
        } else {
            w.write_bits(u64::from(code_length_lengths[i]), 3)?;
        }
    }

    match lengths.len() {
        256 => {
            w.write_bits(1, 1)?; // max_symbol is stored
            w.write_bits(3, 3)?; // max_symbol_nbits / 2 - 2
            w.write_bits(254, 8)?; // max_symbol - 2
        }
        280 => w.write_bits(0, 1)?,
        _ => unreachable!(),
    }

    // Write the huffman codes
    if !single_code_length_length {
        for &len in lengths.iter() {
            w.write_bits(
                u64::from(code_length_codes[len as usize]),
                code_length_lengths[len as usize],
            )?;
        }
    }

    Ok(())
}

const fn length_to_symbol(len: u16) -> (u16, u8) {
    let len = len - 1;
    let highest_bit = len.ilog2() as u16;
    let second_highest_bit = (len >> (highest_bit - 1)) & 1;
    let extra_bits = highest_bit - 1;
    let symbol = 2 * highest_bit + second_highest_bit;
    (symbol, extra_bits as u8)
}

#[inline(always)]
fn count_run(
    pixel: &[u8],
    it: &mut std::iter::Peekable<ChunksExact<u8>>,
    frequencies1: &mut [u32; 280],
) {
    let mut run_length = 0;
    while run_length < 4096 && it.peek() == Some(&pixel) {
        run_length += 1;
        it.next();
    }
    if run_length > 0 {
        if run_length <= 4 {
            let symbol = 256 + run_length - 1;
            frequencies1[symbol] += 1;
        } else {
            let (symbol, _extra_bits) = length_to_symbol(run_length as u16);
            frequencies1[256 + symbol as usize] += 1;
        }
    }
}

#[inline(always)]
fn write_run<W: Write>(
    w: &mut BitWriter<W>,
    pixel: &[u8],
    it: &mut std::iter::Peekable<ChunksExact<u8>>,
    codes1: &[u16; 280],
    lengths1: &[u8; 280],
) -> io::Result<()> {
    let mut run_length = 0;
    while run_length < 4096 && it.peek() == Some(&pixel) {
        run_length += 1;
        it.next();
    }
    if run_length > 0 {
        if run_length <= 4 {
            let symbol = 256 + run_length - 1;
            w.write_bits(u64::from(codes1[symbol]), lengths1[symbol])?;
        } else {
            let (symbol, extra_bits) = length_to_symbol(run_length as u16);
            w.write_bits(
                u64::from(codes1[256 + symbol as usize]),
                lengths1[256 + symbol as usize],
            )?;
            w.write_bits(
                (run_length as u64 - 1) & ((1 << extra_bits) - 1),
                extra_bits,
            )?;
        }
    }
    Ok(())
}

/// Allows fine-tuning some encoder parameters.
///
/// Pass to [`WebPEncoder::set_params()`].
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct EncoderParams {
    /// Use a predictor transform. Enabled by default.
    pub use_predictor_transform: bool,
}

impl Default for EncoderParams {
    fn default() -> Self {
        Self {
            use_predictor_transform: true,
        }
    }
}

/// Encode image data with the indicated color type.
///
/// # Panics
///
/// Panics if the image data is not of the indicated dimensions.
fn encode_frame<W: Write>(
    writer: W,
    data: &[u8],
    width: u32,
    height: u32,
    color: ColorType,
    params: EncoderParams,
) -> Result<(), EncodingError> {
    let w = &mut BitWriter {
        writer,
        buffer: 0,
        nbits: 0,
    };

    let (is_color, is_alpha, bytes_per_pixel) = match color {
        ColorType::L8 => (false, false, 1),
        ColorType::La8 => (false, true, 2),
        ColorType::Rgb8 => (true, false, 3),
        ColorType::Rgba8 => (true, true, 4),
    };

    assert_eq!(
        (u64::from(width) * u64::from(height)).saturating_mul(bytes_per_pixel),
        data.len() as u64
    );

    if width == 0 || width > 16384 || height == 0 || height > 16384 {
        return Err(EncodingError::InvalidDimensions);
    }

    w.write_bits(0x2f, 8)?; // signature
    w.write_bits(u64::from(width) - 1, 14)?;
    w.write_bits(u64::from(height) - 1, 14)?;

    w.write_bits(u64::from(is_alpha), 1)?; // alpha used
    w.write_bits(0x0, 3)?; // version

    // subtract green transform
    w.write_bits(0b101, 3)?;

    // predictor transform
    if params.use_predictor_transform {
        w.write_bits(0b111001, 6)?;
        w.write_bits(0x0, 1)?; // no color cache
        write_single_entry_huffman_tree(w, 2)?;
        for _ in 0..4 {
            write_single_entry_huffman_tree(w, 0)?;
        }
    }

    // transforms done
    w.write_bits(0x0, 1)?;

    // color cache
    w.write_bits(0x0, 1)?;

    // meta-huffman codes
    w.write_bits(0x0, 1)?;

    // expand to RGBA
    let mut pixels = match color {
        ColorType::L8 => data.iter().flat_map(|&p| [p, p, p, 255]).collect(),
        ColorType::La8 => data
            .chunks_exact(2)
            .flat_map(|p| [p[0], p[0], p[0], p[1]])
            .collect(),
        ColorType::Rgb8 => data
            .chunks_exact(3)
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect(),
        ColorType::Rgba8 => data.to_vec(),
    };

    // compute subtract green transform
    for pixel in pixels.chunks_exact_mut(4) {
        pixel[0] = pixel[0].wrapping_sub(pixel[1]);
        pixel[2] = pixel[2].wrapping_sub(pixel[1]);
    }

    // compute predictor transform
    if params.use_predictor_transform {
        let row_bytes = width as usize * 4;
        for y in (1..height as usize).rev() {
            let (prev, current) =
                pixels[(y - 1) * row_bytes..][..row_bytes * 2].split_at_mut(row_bytes);
            for (c, p) in current.iter_mut().zip(prev) {
                *c = c.wrapping_sub(*p);
            }
        }
        for i in (4..row_bytes).rev() {
            pixels[i] = pixels[i].wrapping_sub(pixels[i - 4]);
        }
        pixels[3] = pixels[3].wrapping_sub(255);
    }

    // compute frequencies
    let mut frequencies0 = [0u32; 256];
    let mut frequencies1 = [0u32; 280];
    let mut frequencies2 = [0u32; 256];
    let mut frequencies3 = [0u32; 256];
    let mut it = pixels.chunks_exact(4).peekable();
    match color {
        ColorType::L8 => {
            frequencies0[0] = 1;
            frequencies2[0] = 1;
            frequencies3[0] = 1;
            while let Some(pixel) = it.next() {
                frequencies1[pixel[1] as usize] += 1;
                count_run(pixel, &mut it, &mut frequencies1);
            }
        }
        ColorType::La8 => {
            frequencies0[0] = 1;
            frequencies2[0] = 1;
            while let Some(pixel) = it.next() {
                frequencies1[pixel[1] as usize] += 1;
                frequencies3[pixel[3] as usize] += 1;
                count_run(pixel, &mut it, &mut frequencies1);
            }
        }
        ColorType::Rgb8 => {
            frequencies3[0] = 1;
            while let Some(pixel) = it.next() {
                frequencies0[pixel[0] as usize] += 1;
                frequencies1[pixel[1] as usize] += 1;
                frequencies2[pixel[2] as usize] += 1;
                count_run(pixel, &mut it, &mut frequencies1);
            }
        }
        ColorType::Rgba8 => {
            while let Some(pixel) = it.next() {
                frequencies0[pixel[0] as usize] += 1;
                frequencies1[pixel[1] as usize] += 1;
                frequencies2[pixel[2] as usize] += 1;
                frequencies3[pixel[3] as usize] += 1;
                count_run(pixel, &mut it, &mut frequencies1);
            }
        }
    }

    // compute and write huffman codes
    let mut lengths0 = [0u8; 256];
    let mut lengths1 = [0u8; 280];
    let mut lengths2 = [0u8; 256];
    let mut lengths3 = [0u8; 256];
    let mut codes0 = [0u16; 256];
    let mut codes1 = [0u16; 280];
    let mut codes2 = [0u16; 256];
    let mut codes3 = [0u16; 256];
    write_huffman_tree(w, &frequencies1, &mut lengths1, &mut codes1)?;
    if is_color {
        write_huffman_tree(w, &frequencies0, &mut lengths0, &mut codes0)?;
        write_huffman_tree(w, &frequencies2, &mut lengths2, &mut codes2)?;
    } else {
        write_single_entry_huffman_tree(w, 0)?;
        write_single_entry_huffman_tree(w, 0)?;
    }
    if is_alpha {
        write_huffman_tree(w, &frequencies3, &mut lengths3, &mut codes3)?;
    } else if params.use_predictor_transform {
        write_single_entry_huffman_tree(w, 0)?;
    } else {
        write_single_entry_huffman_tree(w, 255)?;
    }
    write_single_entry_huffman_tree(w, 1)?;

    // Write image data
    let mut it = pixels.chunks_exact(4).peekable();
    match color {
        ColorType::L8 => {
            while let Some(pixel) = it.next() {
                w.write_bits(
                    u64::from(codes1[pixel[1] as usize]),
                    lengths1[pixel[1] as usize],
                )?;
                write_run(w, pixel, &mut it, &codes1, &lengths1)?;
            }
        }
        ColorType::La8 => {
            while let Some(pixel) = it.next() {
                let len1 = lengths1[pixel[1] as usize];
                let len3 = lengths3[pixel[3] as usize];

                let code = u64::from(codes1[pixel[1] as usize])
                    | (u64::from(codes3[pixel[3] as usize]) << len1);

                w.write_bits(code, len1 + len3)?;
                write_run(w, pixel, &mut it, &codes1, &lengths1)?;
            }
        }
        ColorType::Rgb8 => {
            while let Some(pixel) = it.next() {
                let len1 = lengths1[pixel[1] as usize];
                let len0 = lengths0[pixel[0] as usize];
                let len2 = lengths2[pixel[2] as usize];

                let code = u64::from(codes1[pixel[1] as usize])
                    | (u64::from(codes0[pixel[0] as usize]) << len1)
                    | (u64::from(codes2[pixel[2] as usize]) << (len1 + len0));

                w.write_bits(code, len1 + len0 + len2)?;
                write_run(w, pixel, &mut it, &codes1, &lengths1)?;
            }
        }
        ColorType::Rgba8 => {
            while let Some(pixel) = it.next() {
                let len1 = lengths1[pixel[1] as usize];
                let len0 = lengths0[pixel[0] as usize];
                let len2 = lengths2[pixel[2] as usize];
                let len3 = lengths3[pixel[3] as usize];

                let code = u64::from(codes1[pixel[1] as usize])
                    | (u64::from(codes0[pixel[0] as usize]) << len1)
                    | (u64::from(codes2[pixel[2] as usize]) << (len1 + len0))
                    | (u64::from(codes3[pixel[3] as usize]) << (len1 + len0 + len2));

                w.write_bits(code, len1 + len0 + len2 + len3)?;
                write_run(w, pixel, &mut it, &codes1, &lengths1)?;
            }
        }
    }

    w.flush()?;
    Ok(())
}

const fn chunk_size(inner_bytes: usize) -> u32 {
    if inner_bytes % 2 == 1 {
        (inner_bytes + 1) as u32 + 8
    } else {
        inner_bytes as u32 + 8
    }
}

fn write_chunk<W: Write>(mut w: W, name: &[u8], data: &[u8]) -> io::Result<()> {
    debug_assert!(name.len() == 4);

    w.write_all(name)?;
    w.write_all(&(data.len() as u32).to_le_bytes())?;
    w.write_all(data)?;
    if data.len() % 2 == 1 {
        w.write_all(&[0])?;
    }
    Ok(())
}

/// WebP Encoder.
pub struct WebPEncoder<W> {
    writer: W,
    icc_profile: Vec<u8>,
    exif_metadata: Vec<u8>,
    xmp_metadata: Vec<u8>,
    params: EncoderParams,
}

impl<W: Write> WebPEncoder<W> {
    /// Create a new encoder that writes its output to `w`.
    ///
    /// Only supports "VP8L" lossless encoding.
    pub fn new(w: W) -> Self {
        Self {
            writer: w,
            icc_profile: Vec::new(),
            exif_metadata: Vec::new(),
            xmp_metadata: Vec::new(),
            params: EncoderParams::default(),
        }
    }

    /// Set the ICC profile to use for the image.
    pub fn set_icc_profile(&mut self, icc_profile: Vec<u8>) {
        self.icc_profile = icc_profile;
    }

    /// Set the EXIF metadata to use for the image.
    pub fn set_exif_metadata(&mut self, exif_metadata: Vec<u8>) {
        self.exif_metadata = exif_metadata;
    }

    /// Set the XMP metadata to use for the image.
    pub fn set_xmp_metadata(&mut self, xmp_metadata: Vec<u8>) {
        self.xmp_metadata = xmp_metadata;
    }

    /// Set the `EncoderParams` to use.
    pub fn set_params(&mut self, params: EncoderParams) {
        self.params = params;
    }

    /// Encode image data with the indicated color type.
    ///
    /// # Panics
    ///
    /// Panics if the image data is not of the indicated dimensions.
    pub fn encode(
        mut self,
        data: &[u8],
        width: u32,
        height: u32,
        color: ColorType,
    ) -> Result<(), EncodingError> {
        let mut frame = Vec::new();
        encode_frame(&mut frame, data, width, height, color, self.params)?;

        // If the image has no metadata, it can be encoded with the "simple" WebP container format.
        if self.icc_profile.is_empty()
            && self.exif_metadata.is_empty()
            && self.xmp_metadata.is_empty()
        {
            self.writer.write_all(b"RIFF")?;
            self.writer
                .write_all(&(chunk_size(frame.len()) + 4).to_le_bytes())?;
            self.writer.write_all(b"WEBP")?;
            write_chunk(&mut self.writer, b"VP8L", &frame)?;
        } else {
            let mut total_bytes = 22 + chunk_size(frame.len());
            if !self.icc_profile.is_empty() {
                total_bytes += chunk_size(self.icc_profile.len());
            }
            if !self.exif_metadata.is_empty() {
                total_bytes += chunk_size(self.exif_metadata.len());
            }
            if !self.xmp_metadata.is_empty() {
                total_bytes += chunk_size(self.xmp_metadata.len());
            }

            let mut flags = 0;
            if !self.xmp_metadata.is_empty() {
                flags |= 1 << 2;
            }
            if !self.exif_metadata.is_empty() {
                flags |= 1 << 3;
            }
            if let ColorType::La8 | ColorType::Rgba8 = color {
                flags |= 1 << 4;
            }
            if !self.icc_profile.is_empty() {
                flags |= 1 << 5;
            }

            self.writer.write_all(b"RIFF")?;
            self.writer.write_all(&total_bytes.to_le_bytes())?;
            self.writer.write_all(b"WEBP")?;

            let mut vp8x = Vec::new();
            vp8x.write_all(&[flags])?; // flags
            vp8x.write_all(&[0; 3])?; // reserved
            vp8x.write_all(&(width - 1).to_le_bytes()[..3])?; // canvas width
            vp8x.write_all(&(height - 1).to_le_bytes()[..3])?; // canvas height
            write_chunk(&mut self.writer, b"VP8X", &vp8x)?;

            if !self.icc_profile.is_empty() {
                write_chunk(&mut self.writer, b"ICCP", &self.icc_profile)?;
            }

            write_chunk(&mut self.writer, b"VP8L", &frame)?;

            if !self.exif_metadata.is_empty() {
                write_chunk(&mut self.writer, b"EXIF", &self.exif_metadata)?;
            }

            if !self.xmp_metadata.is_empty() {
                write_chunk(&mut self.writer, b"XMP ", &self.xmp_metadata)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rand::RngCore;

    use super::*;

    #[test]
    fn write_webp() {
        let mut img = vec![0; 256 * 256 * 4];
        rand::thread_rng().fill_bytes(&mut img);

        let mut output = Vec::new();
        WebPEncoder::new(&mut output)
            .encode(&img, 256, 256, crate::ColorType::Rgba8)
            .unwrap();

        let mut decoder = crate::WebPDecoder::new(std::io::Cursor::new(output)).unwrap();
        let mut img2 = vec![0; 256 * 256 * 4];
        decoder.read_image(&mut img2).unwrap();
        assert_eq!(img, img2);
    }

    #[test]
    fn write_webp_exif() {
        let mut img = vec![0; 256 * 256 * 3];
        rand::thread_rng().fill_bytes(&mut img);

        let mut exif = vec![0; 10];
        rand::thread_rng().fill_bytes(&mut exif);

        let mut output = Vec::new();
        let mut encoder = WebPEncoder::new(&mut output);
        encoder.set_exif_metadata(exif.clone());
        encoder
            .encode(&img, 256, 256, crate::ColorType::Rgb8)
            .unwrap();

        let mut decoder = crate::WebPDecoder::new(std::io::Cursor::new(output)).unwrap();

        let mut img2 = vec![0; 256 * 256 * 3];
        decoder.read_image(&mut img2).unwrap();
        assert_eq!(img, img2);

        let exif2 = decoder.exif_metadata().unwrap();
        assert_eq!(Some(exif), exif2);
    }

    #[test]
    fn roundtrip_libwebp() {
        roundtrip_libwebp_params(EncoderParams::default());
        roundtrip_libwebp_params(EncoderParams {
            use_predictor_transform: false,
            ..Default::default()
        });
    }

    fn roundtrip_libwebp_params(params: EncoderParams) {
        println!("Testing {params:?}");

        let mut img = vec![0; 256 * 256 * 4];
        rand::thread_rng().fill_bytes(&mut img);

        let mut output = Vec::new();
        let mut encoder = WebPEncoder::new(&mut output);
        encoder.set_params(params.clone());
        encoder
            .encode(&img[..256 * 256 * 3], 256, 256, crate::ColorType::Rgb8)
            .unwrap();
        let decoded = webp::Decoder::new(&output).decode().unwrap();
        assert_eq!(img[..256 * 256 * 3], *decoded);

        let mut output = Vec::new();
        let mut encoder = WebPEncoder::new(&mut output);
        encoder.set_params(params.clone());
        encoder
            .encode(&img, 256, 256, crate::ColorType::Rgba8)
            .unwrap();
        let decoded = webp::Decoder::new(&output).decode().unwrap();
        assert_eq!(img, *decoded);

        let mut output = Vec::new();
        let mut encoder = WebPEncoder::new(&mut output);
        encoder.set_params(params.clone());
        encoder.set_icc_profile(vec![0; 10]);
        encoder
            .encode(&img, 256, 256, crate::ColorType::Rgba8)
            .unwrap();
        let decoded = webp::Decoder::new(&output).decode().unwrap();
        assert_eq!(img, *decoded);

        let mut output = Vec::new();
        let mut encoder = WebPEncoder::new(&mut output);
        encoder.set_params(params.clone());
        encoder.set_exif_metadata(vec![0; 10]);
        encoder
            .encode(&img, 256, 256, crate::ColorType::Rgba8)
            .unwrap();
        let decoded = webp::Decoder::new(&output).decode().unwrap();
        assert_eq!(img, *decoded);

        let mut output = Vec::new();
        let mut encoder = WebPEncoder::new(&mut output);
        encoder.set_params(params);
        encoder.set_xmp_metadata(vec![0; 7]);
        encoder.set_icc_profile(vec![0; 8]);
        encoder.set_icc_profile(vec![0; 9]);
        encoder
            .encode(&img, 256, 256, crate::ColorType::Rgba8)
            .unwrap();
        let decoded = webp::Decoder::new(&output).decode().unwrap();
        assert_eq!(img, *decoded);
    }
}
