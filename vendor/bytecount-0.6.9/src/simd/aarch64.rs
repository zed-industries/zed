use core::arch::aarch64::{
    uint8x16_t, uint8x16x4_t, vaddlvq_u8, vandq_u8, vceqq_u8, vdupq_n_u8, vld1q_u8, vld1q_u8_x4,
    vsubq_u8,
};

const MASK: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255,
];

#[target_feature(enable = "neon")]
unsafe fn u8x16_from_offset(slice: &[u8], offset: usize) -> uint8x16_t {
    debug_assert!(
        offset + 16 <= slice.len(),
        "{} + 16 ≥ {}",
        offset,
        slice.len()
    );
    vld1q_u8(slice.as_ptr().add(offset) as *const _) // TODO: does this need to be aligned?
}

#[target_feature(enable = "neon")]
unsafe fn u8x16_x4_from_offset(slice: &[u8], offset: usize) -> uint8x16x4_t {
    debug_assert!(
        offset + 64 <= slice.len(),
        "{} + 64 ≥ {}",
        offset,
        slice.len()
    );
    vld1q_u8_x4(slice.as_ptr().add(offset) as *const _)
}

#[target_feature(enable = "neon")]
unsafe fn sum(u8s: uint8x16_t) -> usize {
    vaddlvq_u8(u8s) as usize
}

unsafe fn sum4(u1: uint8x16_t, u2: uint8x16_t, u3: uint8x16_t, u4: uint8x16_t) -> usize {
    ((vaddlvq_u8(u1) + vaddlvq_u8(u2)) + (vaddlvq_u8(u3) + vaddlvq_u8(u4))) as usize
}

#[target_feature(enable = "neon")]
pub unsafe fn chunk_count(haystack: &[u8], needle: u8) -> usize {
    assert!(haystack.len() >= 16);

    let mut offset = 0;
    let mut count = 0;

    let needles = vdupq_n_u8(needle);

    // 16320
    while haystack.len() >= offset + 64 * 255 {
        let (mut count1, mut count2, mut count3, mut count4) =
            (vdupq_n_u8(0), vdupq_n_u8(0), vdupq_n_u8(0), vdupq_n_u8(0));
        for _ in 0..255 {
            let uint8x16x4_t(h1, h2, h3, h4) = u8x16_x4_from_offset(haystack, offset);
            count1 = vsubq_u8(count1, vceqq_u8(h1, needles));
            count2 = vsubq_u8(count2, vceqq_u8(h2, needles));
            count3 = vsubq_u8(count3, vceqq_u8(h3, needles));
            count4 = vsubq_u8(count4, vceqq_u8(h4, needles));
            offset += 64;
        }
        count += sum4(count1, count2, count3, count4);
    }

    // 64
    let (mut count1, mut count2, mut count3, mut count4) =
        (vdupq_n_u8(0), vdupq_n_u8(0), vdupq_n_u8(0), vdupq_n_u8(0));
    for _ in 0..(haystack.len() - offset) / 64 {
        let uint8x16x4_t(h1, h2, h3, h4) = u8x16_x4_from_offset(haystack, offset);
        count1 = vsubq_u8(count1, vceqq_u8(h1, needles));
        count2 = vsubq_u8(count2, vceqq_u8(h2, needles));
        count3 = vsubq_u8(count3, vceqq_u8(h3, needles));
        count4 = vsubq_u8(count4, vceqq_u8(h4, needles));
        offset += 64;
    }
    count += sum4(count1, count2, count3, count4);

    let mut counts = vdupq_n_u8(0);
    // 16
    for i in 0..(haystack.len() - offset) / 16 {
        counts = vsubq_u8(
            counts,
            vceqq_u8(u8x16_from_offset(haystack, offset + i * 16), needles),
        );
    }
    if haystack.len() % 16 != 0 {
        counts = vsubq_u8(
            counts,
            vandq_u8(
                vceqq_u8(u8x16_from_offset(haystack, haystack.len() - 16), needles),
                u8x16_from_offset(&MASK, haystack.len() % 16),
            ),
        );
    }
    count + sum(counts)
}

#[target_feature(enable = "neon")]
unsafe fn is_following_utf8_byte(u8s: uint8x16_t) -> uint8x16_t {
    vceqq_u8(
        vandq_u8(u8s, vdupq_n_u8(0b1100_0000)),
        vdupq_n_u8(0b1000_0000),
    )
}

#[target_feature(enable = "neon")]
pub unsafe fn chunk_num_chars(utf8_chars: &[u8]) -> usize {
    assert!(utf8_chars.len() >= 16);

    let mut offset = 0;
    let mut count = 0;

    // 4080
    while utf8_chars.len() >= offset + 64 * 255 {
        let (mut count1, mut count2, mut count3, mut count4) =
            (vdupq_n_u8(0), vdupq_n_u8(0), vdupq_n_u8(0), vdupq_n_u8(0));

        for _ in 0..255 {
            let uint8x16x4_t(h1, h2, h3, h4) = u8x16_x4_from_offset(utf8_chars, offset);
            count1 = vsubq_u8(count1,is_following_utf8_byte(h1));
            count2 = vsubq_u8(count2,is_following_utf8_byte(h2));
            count3 = vsubq_u8(count3,is_following_utf8_byte(h3));
            count4 = vsubq_u8(count4,is_following_utf8_byte(h4));
            offset += 64;
        }
        count += sum4(count1, count2, count3, count4);
    }

    // 4080
    let (mut count1, mut count2, mut count3, mut count4) =
        (vdupq_n_u8(0), vdupq_n_u8(0), vdupq_n_u8(0), vdupq_n_u8(0));
        for _ in 0..(utf8_chars.len() - offset) / 64 {
            let uint8x16x4_t(h1, h2, h3, h4) = u8x16_x4_from_offset(utf8_chars, offset);
            count1 = vsubq_u8(count1, is_following_utf8_byte(h1));
            count2 = vsubq_u8(count2, is_following_utf8_byte(h2));
            count3 = vsubq_u8(count3, is_following_utf8_byte(h3));
            count4 = vsubq_u8(count4, is_following_utf8_byte(h4));
            offset += 64;
        }
        count += sum4(count1, count2, count3, count4);
    // 16
    let mut counts = vdupq_n_u8(0);
    for i in 0..(utf8_chars.len() - offset) / 16 {
        counts = vsubq_u8(
            counts,
            is_following_utf8_byte(u8x16_from_offset(utf8_chars, offset + i * 16)),
        );
    }
    if utf8_chars.len() % 16 != 0 {
        counts = vsubq_u8(
            counts,
            vandq_u8(
                is_following_utf8_byte(u8x16_from_offset(utf8_chars, utf8_chars.len() - 16)),
                u8x16_from_offset(&MASK, utf8_chars.len() % 16),
            ),
        );
    }
    count += sum(counts);

    utf8_chars.len() - count
}
