use core::arch::wasm32::*;

const MASK: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255,
];

#[target_feature(enable = "simd128")]
unsafe fn u8x16_from_offset(slice: &[u8], offset: usize) -> v128 {
    debug_assert!(
        offset + 16 <= slice.len(),
        "{} + 16 ≥ {}",
        offset,
        slice.len()
    );
    v128_load(slice.as_ptr().add(offset) as *const _)
}

// Load four 16-byte vectors from a slice at a given offset.
// This function assumes that the slice has at least 64 bytes available from the offset.
#[target_feature(enable = "simd128")]
unsafe fn u8x16x4_from_offset(slice: &[u8], offset: usize) -> (v128, v128, v128, v128) {
    debug_assert!(
        offset + 64 <= slice.len(),
        "{} + 64 ≥ {}",
        offset,
        slice.len()
    );
    (
        v128_load(slice.as_ptr().add(offset + 0) as *const _),
        v128_load(slice.as_ptr().add(offset + 16) as *const _),
        v128_load(slice.as_ptr().add(offset + 32) as *const _),
        v128_load(slice.as_ptr().add(offset + 48) as *const _),
    )
}

// TODO: We might want to amortize some additions by
// keeping in multiple u16s and u32s respectively for a few ns
#[target_feature(enable = "simd128")]
unsafe fn sum(u8s: v128) -> usize {
    let u16s = u16x8_extadd_pairwise_u8x16(u8s);
    let u32s = u32x4_extadd_pairwise_u16x8(u16s);
    let (u1, u2, u3, u4) = (
        u32x4_extract_lane::<0>(u32s),
        u32x4_extract_lane::<1>(u32s),
        u32x4_extract_lane::<2>(u32s),
        u32x4_extract_lane::<3>(u32s),
    );
    ((u1 + u2) + (u3 + u4)) as usize
}

#[target_feature(enable = "simd128")]
unsafe fn sum4(u1: v128, u2: v128, u3: v128, u4: v128) -> usize {
    // sum < (2^2 * 2^3 * 2^8 = 2^13) < 2^16, therefore no overflow here
    let u16s = u16x8_add(
        u16x8_add(
            u16x8_extadd_pairwise_u8x16(u1),
            u16x8_extadd_pairwise_u8x16(u2),
        ),
        u16x8_add(
            u16x8_extadd_pairwise_u8x16(u3),
            u16x8_extadd_pairwise_u8x16(u4),
        ),
    );
    let u32s = u32x4_extadd_pairwise_u16x8(u16s);
    let (u1, u2, u3, u4) = (
        u32x4_extract_lane::<0>(u32s),
        u32x4_extract_lane::<1>(u32s),
        u32x4_extract_lane::<2>(u32s),
        u32x4_extract_lane::<3>(u32s),
    );
    ((u1 + u2) + (u3 + u4)) as usize
}

#[target_feature(enable = "simd128")]
pub unsafe fn chunk_count(haystack: &[u8], needle: u8) -> usize {
    let needles = u8x16_splat(needle);
    let mut count = 0;
    let mut offset = 0;

    while haystack.len() >= offset + 64 * 255 {
        let (mut count1, mut count2, mut count3, mut count4) = (
            u8x16_splat(0),
            u8x16_splat(0),
            u8x16_splat(0),
            u8x16_splat(0),
        );
        for _ in 0..255 {
            let (h1, h2, h3, h4) = u8x16x4_from_offset(haystack, offset);
            count1 = u8x16_sub(count1, u8x16_eq(h1, needles));
            count2 = u8x16_sub(count2, u8x16_eq(h2, needles));
            count3 = u8x16_sub(count3, u8x16_eq(h3, needles));
            count4 = u8x16_sub(count4, u8x16_eq(h4, needles));
            offset += 64;
        }
        count += sum4(count1, count2, count3, count4);
    }

    // 64
    let (mut count1, mut count2, mut count3, mut count4) = (
        u8x16_splat(0),
        u8x16_splat(0),
        u8x16_splat(0),
        u8x16_splat(0),
    );
    for _ in 0..(haystack.len() - offset) / 64 {
        let (h1, h2, h3, h4) = u8x16x4_from_offset(haystack, offset);
        count1 = u8x16_sub(count1, u8x16_eq(h1, needles));
        count2 = u8x16_sub(count2, u8x16_eq(h2, needles));
        count3 = u8x16_sub(count3, u8x16_eq(h3, needles));
        count4 = u8x16_sub(count4, u8x16_eq(h4, needles));
        offset += 64;
    }
    count += sum4(count1, count2, count3, count4);

    let mut counts = u8x16_splat(0);
    // 16
    for i in 0..(haystack.len() - offset) / 16 {
        counts = u8x16_sub(
            counts,
            u8x16_eq(u8x16_from_offset(haystack, offset + i * 16), needles),
        );
    }
    if haystack.len() % 16 != 0 {
        counts = u8x16_sub(
            counts,
            v128_and(
                u8x16_eq(u8x16_from_offset(haystack, haystack.len() - 16), needles),
                u8x16_from_offset(&MASK, haystack.len() % 16),
            ),
        );
    }
    count + sum(counts)
}

#[target_feature(enable = "simd128")]
unsafe fn is_leading_utf8_byte(u8s: v128) -> v128 {
    u8x16_ne(
        v128_and(u8s, u8x16_splat(0b1100_0000)),
        u8x16_splat(0b1000_0000),
    )
}

#[target_feature(enable = "simd128")]
pub unsafe fn chunk_num_chars(utf8_chars: &[u8]) -> usize {
    assert!(utf8_chars.len() >= 16);

    let mut offset = 0;
    let mut count = 0;

    // 4080
    while utf8_chars.len() >= offset + 64 * 255 {
        let (mut count1, mut count2, mut count3, mut count4) = (
            u8x16_splat(0),
            u8x16_splat(0),
            u8x16_splat(0),
            u8x16_splat(0),
        );

        for _ in 0..255 {
            let (h1, h2, h3, h4) = u8x16x4_from_offset(utf8_chars, offset);
            count1 = u8x16_sub(count1, is_leading_utf8_byte(h1));
            count2 = u8x16_sub(count2, is_leading_utf8_byte(h2));
            count3 = u8x16_sub(count3, is_leading_utf8_byte(h3));
            count4 = u8x16_sub(count4, is_leading_utf8_byte(h4));
            offset += 64;
        }
        count += sum4(count1, count2, count3, count4);
    }

    // 4080
    let (mut count1, mut count2, mut count3, mut count4) = (
        u8x16_splat(0),
        u8x16_splat(0),
        u8x16_splat(0),
        u8x16_splat(0),
    );
    for _ in 0..(utf8_chars.len() - offset) / 64 {
        let (h1, h2, h3, h4) = u8x16x4_from_offset(utf8_chars, offset);
        count1 = u8x16_sub(count1, is_leading_utf8_byte(h1));
        count2 = u8x16_sub(count2, is_leading_utf8_byte(h2));
        count3 = u8x16_sub(count3, is_leading_utf8_byte(h3));
        count4 = u8x16_sub(count4, is_leading_utf8_byte(h4));
        offset += 64;
    }
    count += sum4(count1, count2, count3, count4);

    // 16
    let mut counts = u8x16_splat(0);
    for i in 0..(utf8_chars.len() - offset) / 16 {
        counts = u8x16_sub(
            counts,
            is_leading_utf8_byte(u8x16_from_offset(utf8_chars, offset + i * 16)),
        );
    }
    if utf8_chars.len() % 16 != 0 {
        counts = u8x16_sub(
            counts,
            v128_and(
                is_leading_utf8_byte(u8x16_from_offset(utf8_chars, utf8_chars.len() - 16)),
                u8x16_from_offset(&MASK, utf8_chars.len() % 16),
            ),
        );
    }
    count += sum(counts);

    count
}
