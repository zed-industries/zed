//! PNG decoder.

use alloc::vec::Vec;

/// PNG magic bytes.
pub const SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

/// Errors that can occur during decoding.
#[derive(Debug)]
pub enum DecodeError {
    /// The file format was not supported.
    UnsupportedFileFormat,
    /// Conversion into the requested format failed.
    ConversionFailed,
    /// Invalid signature in an image.
    InvalidSignature,
    /// The file contains a pixel format this is not supported.
    UnsupportedPixelFormat,
    /// The file enables a feature that is not supported.
    UnsupportedFeature,
    /// Some size limit was exceeded.
    LimitExceeded,
    /// Some index into the image was out of bounds.
    IndexOutOfBounds,
    /// Some portion of the file was corrupt.
    CorruptData,
    /// An "end of file" was reached prematurely.
    UnexpectedEof,
}

impl From<yazi::Error> for DecodeError {
    fn from(_: yazi::Error) -> Self {
        Self::CorruptData
    }
}

/// The possible color types for a PNG image.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ColorType {
    Greyscale = 0,
    GreyscaleAlpha = 4,
    Indexed = 3,
    TrueColor = 2,
    TrueColorAlpha = 6,
}

/// The PNG header.
#[derive(Copy, Clone, Debug)]
pub struct Header {
    pub width: u32,
    pub height: u32,
    pub color_type: ColorType,
    pub depth: u8,
    pub interlaced: bool,
}

impl Header {
    /// Attempts to decode a PNG header from the specified buffer.
    pub fn decode(png: &[u8]) -> Option<Self> {
        const IHDR: u32 = 73 << 24 | 72 << 16 | 68 << 8 | 82;
        if png.len() < 33 || !check_signature(png) {
            return None;
        }
        let mut o = 8;
        let len = get_u32be(png, o);
        if len != 13 || get_u32be(png, o + 4) != IHDR {
            return None;
        }
        o += 4;
        let width = get_u32be(png, o + 4);
        let height = get_u32be(png, o + 8);
        let depth = png[o + 12];
        let color_type = png[o + 13];
        let compression_method = png[o + 14];
        let filter_method = png[o + 15];
        let interlace_method = png[o + 16];
        if compression_method != 0
            || filter_method != 0
            || (interlace_method != 0 && interlace_method != 1)
        {
            return None;
        }
        let _crc = get_u32be(png, o + 17);
        let d = depth;
        use ColorType::*;
        let color_type = match color_type {
            0 => Greyscale,
            2 => TrueColor,
            3 => Indexed,
            4 => GreyscaleAlpha,
            6 => TrueColorAlpha,
            _ => return None,
        };
        match color_type {
            Greyscale | Indexed => {
                if d != 1 && d != 2 && d != 4 && d != 8 && d != 16 {
                    return None;
                }
                if color_type == Indexed && d == 16 {
                    return None;
                }
            }
            TrueColor | TrueColorAlpha | GreyscaleAlpha => {
                if d != 8 && d != 16 {
                    return None;
                }
            }
        };
        Some(Self {
            width,
            height,
            color_type,
            depth,
            interlaced: interlace_method != 0,
        })
    }
}

/// Returns true if the specified buffer might represent a PNG image.
pub fn check_signature(png: &[u8]) -> bool {
    if png.len() >= 8 {
        for i in 0..8 {
            if png[i] != SIGNATURE[i] {
                return false;
            }
        }
        return true;
    }
    false
}

pub fn decode(
    data: &[u8],
    scratch: &mut Vec<u8>,
    target: &mut [u8],
) -> Result<(u32, u32, bool), DecodeError> {
    let mut state = State::new(data, scratch)?;
    let w = state.header.width;
    let h = state.header.height;
    if w == 0 || h == 0 {
        return Ok((w, h, false));
    }
    let decomp_len = scratch.len();
    scratch.resize(decomp_len + state.extra_bytes, 0);
    let (decomp, extra) = scratch.split_at_mut(decomp_len);
    if target.len() < (w * h * 4) as usize {
        return Err(DecodeError::LimitExceeded);
    }
    state.trunc_16 = true;
    state.expand_alpha = true;
    decode_data::<EmitRgba8>(&mut state, decomp, extra, target).ok_or(DecodeError::CorruptData)?;
    Ok((w, h, state.has_alpha))
}

struct State<'a> {
    header: Header,
    data: &'a [u8],
    palette: &'a [u8],
    trans: &'a [u8],
    gamma: Option<f32>,
    effective_depth: u8,
    bpp: usize,
    channels: usize,
    pitch: usize,
    extra_bytes: usize,
    bwidth: usize,
    has_alpha: bool,
    trunc_16: bool,
    expand_alpha: bool,
}

impl<'a> State<'a> {
    fn new(data: &'a [u8], decomp: &mut Vec<u8>) -> Result<Self, DecodeError> {
        let header = Header::decode(data).ok_or(DecodeError::CorruptData)?;
        let mut this = Self {
            header,
            data,
            palette: &[],
            trans: &[],
            gamma: None,
            effective_depth: header.depth,
            bpp: 0,
            channels: 0,
            pitch: 0,
            extra_bytes: 0,
            bwidth: 0,
            has_alpha: false,
            trunc_16: false,
            expand_alpha: false,
        };
        let w = header.width as usize;
        let h = header.height as usize;
        if w == 0 || h == 0 {
            return Ok(this);
        }
        use ColorType::*;
        let (channels, has_alpha) = match header.color_type {
            TrueColor => (3, false),
            TrueColorAlpha => (4, true),
            GreyscaleAlpha => (2, true),
            Greyscale => (1, false),
            Indexed => (1, false),
        };
        this.has_alpha = has_alpha;
        this.bpp = this.header.depth as usize * channels;
        this.pitch = (w * this.bpp + 7) / 8;
        this.bwidth = (this.bpp + 7) / 8;
        this.extra_bytes = this.pitch * 2 + w * 8;
        decomp.clear();
        decomp.reserve(this.extra_bytes + (this.pitch + 1) * h);
        let limit = data.len();
        let mut offset = 33;
        let mut dec = yazi::Decoder::new();
        dec.set_format(yazi::Format::Zlib);
        let mut stream = dec.stream_into_vec(decomp);
        loop {
            if offset + 8 > limit {
                return Err(DecodeError::CorruptData);
            }
            let len = get_u32be(data, offset) as usize;
            offset += 4;
            let ty = get_u32be(data, offset);
            offset += 4;
            if offset + len > limit {
                return Err(DecodeError::CorruptData);
            }
            let bytes = data
                .get(offset..offset + len)
                .ok_or(DecodeError::CorruptData)?;
            const PLTE: u32 = chunk_name(b"PLTE");
            const TRNS: u32 = chunk_name(b"tRNS");
            const IDAT: u32 = chunk_name(b"IDAT");
            const GAMA: u32 = chunk_name(b"gAMA");
            const IEND: u32 = chunk_name(b"IEND");
            match ty {
                PLTE => this.palette = bytes,
                TRNS => this.trans = bytes,
                IDAT => {
                    stream.write(bytes)?;
                }
                GAMA => {
                    if bytes.len() > 4 && this.gamma.is_none() {
                        this.gamma = Some(get_u32be(bytes, 0) as f32 / 100000.);
                    }
                }
                IEND => break,
                _ => {}
            }
            offset += len + 4;
        }
        stream.finish()?;
        if header.color_type == Indexed {
            if this.palette.is_empty() {
                return Err(DecodeError::CorruptData);
            }
            if !this.trans.is_empty() {
                this.has_alpha = true;
            }
        }
        Ok(this)
    }
}

fn decode_data<E: Emit>(
    state: &mut State,
    decomp: &mut [u8],
    extra: &mut [u8],
    target: &mut [u8],
) -> Option<()> {
    let has_palette = !state.palette.is_empty();
    let w = state.header.width as usize;
    let h = state.header.height as usize;
    let (mut line, extra) = extra.split_at_mut(state.pitch);
    let (mut prev_line, out_line) = extra.split_at_mut(state.pitch);
    let depth = state.header.depth;
    let bpp = state.bpp;
    let pitch = state.pitch;
    let bwidth = state.bwidth;
    let trunc_16 = state.trunc_16;
    if state.header.interlaced {
        const ROW_START: [u8; 7] = [0, 0, 4, 0, 2, 0, 1];
        const ROW_INCREMENT: [u8; 7] = [8, 8, 8, 4, 4, 2, 2];
        const COL_START: [u8; 7] = [0, 4, 0, 2, 0, 1, 0];
        const COL_INCREMENT: [u8; 7] = [8, 8, 4, 4, 2, 2, 1];
        let mut pass = 0;
        let mut y = 0;
        let mut offset = 0;
        loop {
            let cols = match pass {
                0 => (w + 7) / 8,
                1 => (w + 3) / 8,
                2 => (w + 3) / 4,
                3 => (w + 1) / 4,
                4 => (w + 1) / 2,
                5 => w / 2,
                6 => w,
                _ => return None,
            };
            if cols == 0 {
                pass += 1;
                continue;
            }
            let start = COL_START[pass] as usize;
            let inc = COL_INCREMENT[pass] as usize;
            let row_inc = ROW_INCREMENT[pass] as usize;
            while y < h {
                let pitch = (cols * bpp + 7) / 8;
                let end = offset + pitch + 1;
                let source = decomp.get(offset..end)?;
                offset = end;
                let ty = source[0];
                defilter(
                    ty,
                    source.get(1..)?,
                    line.get_mut(..pitch)?,
                    prev_line.get(..pitch)?,
                    bwidth,
                )?;
                if depth == 8 {
                    E::emit(state, line, target, start, y, w, inc, cols)?;
                } else {
                    normalize(line, out_line, depth, has_palette, cols, trunc_16)?;
                    E::emit(state, out_line, target, start, y, w, inc, cols)?;
                }
                core::mem::swap(&mut prev_line, &mut line);
                y += row_inc;
            }
            if pass == 6 {
                break;
            }
            pass += 1;
            y = ROW_START[pass] as usize;
            for byte in prev_line.iter_mut() {
                *byte = 0;
            }
        }
    } else if depth == 8 {
        for y in 0..h {
            let offset = y * (pitch + 1);
            let end = offset + pitch + 1;
            let source = decomp.get(offset..end)?;
            let ty = *source.get(0)?;
            defilter(ty, source.get(1..)?, line, prev_line, bwidth)?;
            E::emit(state, line, target, 0, y, w, 1, w)?;
            core::mem::swap(&mut prev_line, &mut line);
        }
    } else {
        for y in 0..h {
            let offset = y * (pitch + 1);
            let end = offset + pitch + 1;
            let source = decomp.get(offset..end)?;
            let ty = *source.get(0)?;
            defilter(ty, source.get(1..)?, line, prev_line, bwidth)?;
            normalize(line, out_line, depth, has_palette, w, trunc_16)?;
            E::emit(state, out_line, target, 0, y, w, 1, w)?;
            core::mem::swap(&mut prev_line, &mut line);
        }
    }
    Some(())
}

#[cfg(target_endian = "little")]
const IS_LITTLE_ENDIAN: bool = true;

#[cfg(not(target_endian = "little"))]
const IS_LITTLE_ENDIAN: bool = false;

fn defilter(ty: u8, source: &[u8], dest: &mut [u8], last: &[u8], bwidth: usize) -> Option<()> {
    let len = source.len();
    match ty {
        0 => {
            dest.copy_from_slice(source);
        }
        1 => {
            dest.get_mut(..bwidth)?
                .copy_from_slice(source.get(..bwidth)?);
            for i in bwidth..len {
                dest[i] = source[i].wrapping_add(dest[i - bwidth]);
            }
        }
        2 => {
            for ((dest, source), last) in dest.iter_mut().zip(source.iter()).zip(last.iter()) {
                *dest = source.wrapping_add(*last);
            }
        }
        3 => {
            for i in 0..bwidth {
                dest[i] = source[i].wrapping_add((last[i] as u32 / 2) as u8);
            }
            for i in bwidth..len {
                dest[i] = source[i].wrapping_add(
                    ((dest[i - bwidth] as u32).wrapping_add(last[i] as u32) / 2) as u8,
                );
            }
        }
        4 => {
            for i in 0..bwidth {
                dest[i] = source[i].wrapping_add(last[i]);
            }
            for i in bwidth..len {
                dest[i] =
                    source[i].wrapping_add(paeth(dest[i - bwidth], last[i], last[i - bwidth]));
            }
        }
        _ => return None,
    }
    Some(())
}

fn normalize(
    source: &[u8],
    dest: &mut [u8],
    depth: u8,
    palette: bool,
    width: usize,
    trunc_16: bool,
) -> Option<()> {
    match depth {
        16 => {
            if trunc_16 {
                for (i, d) in dest.iter_mut().enumerate() {
                    *d = source[i * 2];
                }
            } else if IS_LITTLE_ENDIAN {
                for (s, d) in source.chunks(2).zip(dest.chunks_mut(2)) {
                    d[1] = s[0];
                    d[0] = s[1];
                }
            } else {
                dest.copy_from_slice(source);
            }
        }
        8 => {
            dest.copy_from_slice(source);
        }
        4 => {
            let conv = if !palette { 17 } else { 1 };
            for (i, d) in dest.get_mut(..width)?.iter_mut().enumerate() {
                *d = (source[i / 2] >> (4 - i % 2 * 4) & 15) * conv;
            }
        }
        2 => {
            let conv = if !palette { 85 } else { 1 };
            for (i, d) in dest.get_mut(..width)?.iter_mut().enumerate() {
                *d = (source[i / 4] >> (6 - i % 4 * 2) & 3) * conv;
            }
        }
        1 => {
            let conv = if !palette { 255 } else { 1 };
            for (i, d) in dest.get_mut(..width)?.iter_mut().enumerate() {
                *d = (source[i / 8] >> (7 - i % 8) & 1) * conv;
            }
        }
        _ => {}
    }
    Some(())
}

trait Emit {
    fn emit(
        state: &State,
        source: &[u8],
        image: &mut [u8],
        x: usize,
        y: usize,
        width: usize,
        inc: usize,
        len: usize,
    ) -> Option<()>;
}

struct EmitRgba8;

impl Emit for EmitRgba8 {
    fn emit(
        state: &State,
        source: &[u8],
        image: &mut [u8],
        x: usize,
        y: usize,
        width: usize,
        inc: usize,
        len: usize,
    ) -> Option<()> {
        use ColorType::*;
        let src = source;
        let mut out = y * width * 4 + x * 4;
        let mut i = 0;
        match state.header.color_type {
            Indexed => {
                let palette = state.palette;
                let trans = state.trans;
                let palette_len = palette.len();
                let trans_len = trans.len();
                for _ in 0..len {
                    let t = src[i] as usize;
                    let p = t * 3;
                    if p + 2 >= palette_len {
                        image[out] = 0;
                        image[out + 1] = 0;
                        image[out + 2] = 0;
                    } else {
                        image[out] = palette[p];
                        image[out + 1] = palette[p + 1];
                        image[out + 2] = palette[p + 2];
                    }
                    if t >= trans_len {
                        image[out + 3] = 255;
                    } else {
                        image[out + 3] = trans[t];
                    }
                    i += 1;
                    out += 4 * inc;
                }
            }
            TrueColor => {
                for _ in 0..len {
                    image[out] = src[i];
                    image[out + 1] = src[i + 1];
                    image[out + 2] = src[i + 2];
                    image[out + 3] = 255;
                    i += 3;
                    out += 4 * inc;
                }
            }
            TrueColorAlpha => {
                for _ in 0..len {
                    image[out] = src[i];
                    image[out + 1] = src[i + 1];
                    image[out + 2] = src[i + 2];
                    image[out + 3] = src[i + 3];
                    i += 4;
                    out += 4 * inc;
                }
            }
            Greyscale => {
                for c in src[..len].iter().copied() {
                    image[out] = c;
                    image[out + 1] = c;
                    image[out + 2] = c;
                    image[out + 3] = 255;
                    out += 4 * inc;
                }
            }
            GreyscaleAlpha => {
                let mut i = 0;
                for _ in 0..len {
                    let c = src[i];
                    image[out] = c;
                    image[out + 1] = c;
                    image[out + 2] = c;
                    image[out + 3] = src[i + 1];
                    i += 2;
                    out += 4 * inc;
                }
            }
        }
        Some(())
    }
}

#[inline(always)]
fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let pa = ((b as i32).wrapping_sub(c as i32)).abs();
    let pb = ((a as i32).wrapping_sub(c as i32)).abs();
    let pc = ((a as i32)
        .wrapping_add(b as i32)
        .wrapping_sub(c as i32)
        .wrapping_sub(c as i32))
    .abs();
    if pc < pa && pc < pb {
        c
    } else if pb < pa {
        b
    } else {
        a
    }
}

fn get_u32be(buf: &[u8], offset: usize) -> u32 {
    (buf[offset] as u32) << 24
        | (buf[offset + 1] as u32) << 16
        | (buf[offset + 2] as u32) << 8
        | buf[offset + 3] as u32
}

const fn chunk_name(bytes: &[u8; 4]) -> u32 {
    (bytes[0] as u32) << 24 | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 | bytes[3] as u32
}
