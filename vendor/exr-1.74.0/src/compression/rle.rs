use super::*;
use super::optimize_bytes::*;
use super::Error;
use super::Result;

// inspired by  https://github.com/openexr/openexr/blob/master/OpenEXR/IlmImf/ImfRle.cpp

const MIN_RUN_LENGTH : usize = 3;
const MAX_RUN_LENGTH : usize = 127;


pub fn decompress_bytes(
    channels: &ChannelList,
    compressed_le: ByteVec,
    rectangle: IntegerBounds,
    expected_byte_size: usize,
    pedantic: bool,
) -> Result<ByteVec> {
    let mut remaining_le = compressed_le.as_slice();
    let mut decompressed_le = Vec::with_capacity(expected_byte_size.min(8*2048));

    while !remaining_le.is_empty() && decompressed_le.len() != expected_byte_size {
        let count = take_1(&mut remaining_le)? as i8 as i32;

        if count < 0 {
            // take the next '-count' bytes as-is
            let values = take_n(&mut remaining_le, (-count) as usize)?;
            decompressed_le.extend_from_slice(values);
        }
        else {
            // repeat the next value 'count + 1' times
            let value = take_1(&mut remaining_le)?;
            decompressed_le.resize(decompressed_le.len() + count as usize + 1, value);
        }
    }

    if pedantic && !remaining_le.is_empty() {
        return Err(Error::invalid("data amount"));
    }

    differences_to_samples(&mut decompressed_le);
    interleave_byte_blocks(&mut decompressed_le);
    super::convert_little_endian_to_current(decompressed_le, channels, rectangle) // TODO no alloc
}

pub fn compress_bytes(channels: &ChannelList, uncompressed_ne: ByteVec, rectangle: IntegerBounds) -> Result<ByteVec> {
    // see https://github.com/AcademySoftwareFoundation/openexr/blob/3bd93f85bcb74c77255f28cdbb913fdbfbb39dfe/OpenEXR/IlmImf/ImfTiledOutputFile.cpp#L750-L842
    let mut data_le = super::convert_current_to_little_endian(uncompressed_ne, channels, rectangle)?;// TODO no alloc

    separate_bytes_fragments(&mut data_le);
    samples_to_differences(&mut data_le);

    let mut compressed_le = Vec::with_capacity(data_le.len());
    let mut run_start = 0;
    let mut run_end = 1;

    while run_start < data_le.len() {
        while
            run_end < data_le.len()
                && data_le[run_start] == data_le[run_end]
                && (run_end - run_start) as i32 - 1 < MAX_RUN_LENGTH as i32
            {
                run_end += 1;
            }

        if run_end - run_start >= MIN_RUN_LENGTH {
            compressed_le.push(((run_end - run_start) as i32 - 1) as u8);
            compressed_le.push(data_le[run_start]);
            run_start = run_end;

        } else {
            while
                run_end < data_le.len() && (
                    (run_end + 1 >= data_le.len() || data_le[run_end] != data_le[run_end + 1])
                        || (run_end + 2 >= data_le.len() || data_le[run_end + 1] != data_le[run_end + 2])
                ) && run_end - run_start < MAX_RUN_LENGTH
                {
                    run_end += 1;
                }

            compressed_le.push((run_start as i32 - run_end as i32) as u8);
            compressed_le.extend_from_slice(&data_le[run_start .. run_end]);

            run_start = run_end;
            run_end += 1;
        }
    }

    Ok(compressed_le)
}

fn take_1(slice: &mut &[u8]) -> Result<u8> {
    if !slice.is_empty() {
        let result = slice[0];
        *slice = &slice[1..];
        Ok(result)

    } else {
        Err(Error::invalid("compressed data"))
    }
}

fn take_n<'s>(slice: &mut &'s [u8], n: usize) -> Result<&'s [u8]> {
    if n <= slice.len() {
        let (front, back) = slice.split_at(n);
        *slice = back;
        Ok(front)

    } else {
        Err(Error::invalid("compressed data"))
    }
}
