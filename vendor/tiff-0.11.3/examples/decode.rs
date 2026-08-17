use tiff::decoder::{BufferLayoutPreference, Decoder, DecodingBuffer, DecodingResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(image) = std::env::args_os().nth(1) else {
        eprintln!("Usage: decode FILE");
        return Ok(());
    };

    let file = std::fs::File::open(image)?;
    let io = std::io::BufReader::new(file);
    let mut reader = Decoder::new(io)?;

    let mut data = DecodingResult::I8(vec![]);

    for i in 0u32.. {
        let colortype = reader.colortype()?;
        let dimensions = reader.dimensions()?;
        let layout = reader.read_image_to_buffer(&mut data)?;

        debug_planes(i, &mut data, &layout, &colortype, dimensions);

        if !reader.more_images() {
            break;
        }

        reader.next_image()?
    }

    Ok(())
}

fn debug_planes(
    index: u32,
    data: &mut DecodingResult,
    layout: &BufferLayoutPreference,
    colortype: &tiff::ColorType,
    (width, height): (u32, u32),
) {
    let (depth, mut tupltype) = match colortype {
        // Note: we will expand, so this is in fact not BLACKANDWHITE
        tiff::ColorType::Gray(_) => (1, "GRAYSCALE"),
        tiff::ColorType::RGB(_) => (3, "RGB"),
        tiff::ColorType::RGBA(_) => (4, "RGB_ALPHA"),
        tiff::ColorType::Palette(_) => (1, "PALETTE"),
        tiff::ColorType::CMYK(_) => (4, "CMYK"),
        tiff::ColorType::Multiband { num_samples: 2, .. } => (2, "GRAYSCALE_ALPHA"),
        _ => {
            eprintln!("Unsupported color type for PAM output: {:?}", colortype);
            return;
        }
    };

    // Number of samples in a line and number of sample lines in the image, depending on whether
    // they are planar or not. This determines the layout of the image buffer when expanding bit
    // packed samples to a full byte (we support power-of-two sample sizes, positive and negative,
    // but only those).
    let (swidth, sheight) = if layout.planes > 1 {
        (
            u64::from(width) * u64::from(colortype.num_samples()),
            u64::from(height),
        )
    } else {
        (
            u64::from(width),
            u64::from(height) * u64::from(colortype.num_samples()),
        )
    };

    let mut fallback_buffer;
    let maxval = match data.as_buffer(0) {
        // `U8` is also our representation of bit-packed color. PAM uses the smallest number of
        // *bytes* needed to represent a sample but we must also adhere to maxval. We could explode
        // them here (e.g. represent 1-bit as `{0, 1}`) but we'd also set `maxval` accordingly and
        // this requires tupltype to be `BLACKANDWHITE` so introducing a bunch of complexity. If
        // you want to do that in your real code, go ahead.
        DecodingBuffer::U8(buf) => match colortype.bit_depth() {
            8 => u64::from(u8::MAX),
            4 => {
                fallback_buffer = vec![0; (swidth * sheight) as usize];
                expand_4bit(buf, swidth, &mut fallback_buffer);
                *data = DecodingResult::U8(fallback_buffer);
                u64::from(u8::MAX)
            }
            2 => {
                fallback_buffer = vec![0; (swidth * sheight) as usize];
                expand_2bit(buf, swidth, &mut fallback_buffer);
                *data = DecodingResult::U8(fallback_buffer);
                u64::from(u8::MAX)
            }
            1 => {
                fallback_buffer = vec![0; (swidth * sheight) as usize];
                expand_1bit(buf, swidth, &mut fallback_buffer);
                *data = DecodingResult::U8(fallback_buffer);
                u64::from(u8::MAX)
            }
            _ => {
                eprintln!(
                    "Unsupported bit depth for U8 buffer: {}",
                    colortype.bit_depth()
                );
                return;
            }
        },
        DecodingBuffer::U16(_) => u64::from(u16::MAX),
        DecodingBuffer::U32(_) => u64::from(u32::MAX),
        DecodingBuffer::U64(_) => u64::MAX,
        // Eh, this is a good effort I guess.
        DecodingBuffer::I8(items) => {
            items.iter_mut().for_each(|v| *v = 0.max(*v));
            u64::from(i8::MAX as u8)
        }
        DecodingBuffer::I16(items) => {
            items.iter_mut().for_each(|v| *v = 0.max(*v));
            u64::from(i16::MAX as u16)
        }
        DecodingBuffer::I32(items) => {
            items.iter_mut().for_each(|v| *v = 0.max(*v));
            u64::from(i32::MAX as u32)
        }
        DecodingBuffer::I64(items) => {
            items.iter_mut().for_each(|v| *v = 0.max(*v));
            i64::MAX as u64
        }
        // Not supported by PAM.
        DecodingBuffer::F16(_) | DecodingBuffer::F32(_) | DecodingBuffer::F64(_) => return,
    };

    let byte_len = if layout.planes > 1 {
        tupltype = "GRAYSCALE";
        layout.plane_stride.unwrap().get()
    } else {
        data.as_buffer(0).byte_len()
    };

    // Write out this image's planes to PAM files.
    let header = format!("P7\nWIDTH {width}\nHEIGHT {height}\nDEPTH {depth}\nMAXVAL {maxval}\nTUPLTYPE {tupltype}\nENDHDR\n");

    for (plane, plane_data) in data
        .as_buffer(0)
        .as_bytes_mut()
        .chunks_exact(byte_len)
        .enumerate()
    {
        let filename = format!("image_{index:03}_plane_{plane:03}.pam");
        let mut file = std::fs::File::create(filename).expect("Failed to create output file");

        use std::io::Write;
        file.write_all(header.as_bytes())
            .expect("Failed to write PAM header");
        file.write_all(plane_data)
            .expect("Failed to write PAM data");
    }
}

/// Put exactly the ith `SCALE`-bit chunk of `val` into the least significant bits.
#[inline(always)]
fn bits_extract<const SCALE: u8>(val: u8, i: u8) -> u8 {
    // Shift left to discard high bits, then right to bring desired bits down. This way also
    // conveniently avoids all cases of overflow.
    (val << (i * SCALE)) >> (8 - SCALE)
}

fn expand_1bit(from: &[u8], samples_per_padded: u64, into: &mut [u8]) {
    let stride = samples_per_padded.div_ceil(8);

    let from_row = from.chunks_exact(stride as usize);
    let into_row = into.chunks_exact_mut(samples_per_padded as usize);

    for (from_row, into_row) in from_row.zip(into_row) {
        for (into, &from) in into_row.chunks_mut(8).zip(from_row) {
            let data =
                core::array::from_fn::<u8, 8, _>(|i| 0xff * bits_extract::<1>(from, i as u8));
            into.copy_from_slice(&data[..into.len()]);
        }
    }
}

fn expand_2bit(from: &[u8], samples_per_padded: u64, into: &mut [u8]) {
    let stride = samples_per_padded.div_ceil(4);

    let from_row = from.chunks_exact(stride as usize);
    let into_row = into.chunks_exact_mut(samples_per_padded as usize);

    for (from_row, into_row) in from_row.zip(into_row) {
        for (into, &from) in into_row.chunks_mut(4).zip(from_row) {
            let data =
                core::array::from_fn::<u8, 4, _>(|i| 0xaa * bits_extract::<2>(from, i as u8));
            into.copy_from_slice(&data[..into.len()]);
        }
    }
}

fn expand_4bit(from: &[u8], samples_per_padded: u64, into: &mut [u8]) {
    let stride = samples_per_padded.div_ceil(2);

    let from_row = from.chunks_exact(stride as usize);
    let into_row = into.chunks_exact_mut(samples_per_padded as usize);

    for (from_row, into_row) in from_row.zip(into_row) {
        for (into, &from) in into_row.chunks_mut(2).zip(from_row) {
            let data =
                core::array::from_fn::<u8, 2, _>(|i| 0x88 * bits_extract::<4>(from, i as u8));
            into.copy_from_slice(&data[..into.len()]);
        }
    }
}
