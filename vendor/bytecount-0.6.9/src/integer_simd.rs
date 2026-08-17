#[cfg(not(feature = "runtime-dispatch-simd"))]
use core::{mem, ptr, usize};
#[cfg(feature = "runtime-dispatch-simd")]
use std::{mem, ptr, usize};

fn splat(byte: u8) -> usize {
    let lo = usize::MAX / 0xFF;
    lo * byte as usize
}

unsafe fn usize_load_unchecked(bytes: &[u8], offset: usize) -> usize {
    let mut output = 0;
    ptr::copy_nonoverlapping(
        bytes.as_ptr().add(offset),
        &mut output as *mut usize as *mut u8,
        mem::size_of::<usize>(),
    );
    output
}

fn bytewise_equal(lhs: usize, rhs: usize) -> usize {
    let lo = usize::MAX / 0xFF;
    let hi = lo << 7;

    let x = lhs ^ rhs;
    !((((x & !hi) + !hi) | x) >> 7) & lo
}

fn sum_usize(values: usize) -> usize {
    let every_other_byte_lo = usize::MAX / 0xFFFF;
    let every_other_byte = every_other_byte_lo * 0xFF;

    // Pairwise reduction to avoid overflow on next step.
    let pair_sum: usize = (values & every_other_byte) + ((values >> 8) & every_other_byte);

    // Multiplication results in top two bytes holding sum.
    pair_sum.wrapping_mul(every_other_byte_lo) >> ((mem::size_of::<usize>() - 2) * 8)
}

fn is_leading_utf8_byte(values: usize) -> usize {
    // a leading UTF-8 byte is one which does not start with the bits 10.
    ((!values >> 7) | (values >> 6)) & splat(1)
}

pub fn chunk_count(haystack: &[u8], needle: u8) -> usize {
    let chunksize = mem::size_of::<usize>();
    assert!(haystack.len() >= chunksize);

    unsafe {
        let mut offset = 0;
        let mut count = 0;

        let needles = splat(needle);

        // 2040
        while haystack.len() >= offset + chunksize * 255 {
            let mut counts = 0;
            for _ in 0..255 {
                counts += bytewise_equal(usize_load_unchecked(haystack, offset), needles);
                offset += chunksize;
            }
            count += sum_usize(counts);
        }

        // 8
        let mut counts = 0;
        for i in 0..(haystack.len() - offset) / chunksize {
            counts += bytewise_equal(
                usize_load_unchecked(haystack, offset + i * chunksize),
                needles,
            );
        }
        if haystack.len() % 8 != 0 {
            let mask = usize::from_le(!(!0 >> ((haystack.len() % chunksize) * 8)));
            counts += bytewise_equal(
                usize_load_unchecked(haystack, haystack.len() - chunksize),
                needles,
            ) & mask;
        }
        count += sum_usize(counts);

        count
    }
}

pub fn chunk_num_chars(utf8_chars: &[u8]) -> usize {
    let chunksize = mem::size_of::<usize>();
    assert!(utf8_chars.len() >= chunksize);

    unsafe {
        let mut offset = 0;
        let mut count = 0;

        // 2040
        while utf8_chars.len() >= offset + chunksize * 255 {
            let mut counts = 0;
            for _ in 0..255 {
                counts += is_leading_utf8_byte(usize_load_unchecked(utf8_chars, offset));
                offset += chunksize;
            }
            count += sum_usize(counts);
        }

        // 8
        let mut counts = 0;
        for i in 0..(utf8_chars.len() - offset) / chunksize {
            counts +=
                is_leading_utf8_byte(usize_load_unchecked(utf8_chars, offset + i * chunksize));
        }
        if utf8_chars.len() % 8 != 0 {
            let mask = usize::from_le(!(!0 >> ((utf8_chars.len() % chunksize) * 8)));
            counts += is_leading_utf8_byte(usize_load_unchecked(
                utf8_chars,
                utf8_chars.len() - chunksize,
            )) & mask;
        }
        count += sum_usize(counts);

        count
    }
}
