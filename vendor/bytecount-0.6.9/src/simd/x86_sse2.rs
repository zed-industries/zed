#[cfg(target_arch = "x86")]
use std::arch::x86::{
    __m128i,
    _mm_and_si128,
    _mm_cmpeq_epi8,
    _mm_cvtsi128_si32,
    _mm_loadu_si128,
    _mm_sad_epu8,
    _mm_set1_epi8,
    _mm_setzero_si128,
    _mm_shuffle_epi32,
    _mm_sub_epi8,
    _mm_xor_si128,
};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{
    __m128i,
    _mm_and_si128,
    _mm_cmpeq_epi8,
    _mm_cvtsi128_si32,
    _mm_loadu_si128,
    _mm_sad_epu8,
    _mm_set1_epi8,
    _mm_setzero_si128,
    _mm_shuffle_epi32,
    _mm_sub_epi8,
    _mm_xor_si128,
};

#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set1_epu8(a: u8) -> __m128i {
    _mm_set1_epi8(a as i8)
}

#[target_feature(enable = "sse2")]
pub unsafe fn mm_cmpneq_epi8(a: __m128i, b: __m128i) -> __m128i {
    _mm_xor_si128(_mm_cmpeq_epi8(a, b), _mm_set1_epi8(-1))
}

const MASK: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

#[target_feature(enable = "sse2")]
unsafe fn mm_from_offset(slice: &[u8], offset: usize) -> __m128i {
    _mm_loadu_si128(slice.as_ptr().offset(offset as isize) as *const _)
}

#[target_feature(enable = "sse2")]
unsafe fn sum(u8s: &__m128i) -> usize {
    let sums = _mm_sad_epu8(*u8s, _mm_setzero_si128());
    (_mm_cvtsi128_si32(sums) + _mm_cvtsi128_si32(_mm_shuffle_epi32(sums, 0xaa))) as usize
}

#[target_feature(enable = "sse2")]
pub unsafe fn chunk_count(haystack: &[u8], needle: u8) -> usize {
    assert!(haystack.len() >= 16);

    let mut offset = 0;
    let mut count = 0;

    let needles = _mm_set1_epu8(needle);

    // 4080
    while haystack.len() >= offset + 16 * 255 {
        let mut counts = _mm_setzero_si128();
        for _ in 0..255 {
            counts = _mm_sub_epi8(
                counts,
                _mm_cmpeq_epi8(mm_from_offset(haystack, offset), needles)
            );
            offset += 16;
        }
        count += sum(&counts);
    }

    // 2048
    if haystack.len() >= offset + 16 * 128 {
        let mut counts = _mm_setzero_si128();
        for _ in 0..128 {
            counts = _mm_sub_epi8(
                counts,
                _mm_cmpeq_epi8(mm_from_offset(haystack, offset), needles)
            );
            offset += 16;
        }
        count += sum(&counts);
    }

    // 16
    let mut counts = _mm_setzero_si128();
    for i in 0..(haystack.len() - offset) / 16 {
        counts = _mm_sub_epi8(
            counts,
            _mm_cmpeq_epi8(mm_from_offset(haystack, offset + i * 16), needles)
        );
    }
    if haystack.len() % 16 != 0 {
        counts = _mm_sub_epi8(
            counts,
            _mm_and_si128(
                _mm_cmpeq_epi8(mm_from_offset(haystack, haystack.len() - 16), needles),
                                  mm_from_offset(&MASK, haystack.len() % 16)
            )
        );
    }
    count += sum(&counts);

    count
}

#[target_feature(enable = "sse2")]
unsafe fn is_leading_utf8_byte(u8s: __m128i) -> __m128i {
    mm_cmpneq_epi8(_mm_and_si128(u8s, _mm_set1_epu8(0b1100_0000)), _mm_set1_epu8(0b1000_0000))
}

#[target_feature(enable = "sse2")]
pub unsafe fn chunk_num_chars(utf8_chars: &[u8]) -> usize {
    assert!(utf8_chars.len() >= 16);

    let mut offset = 0;
    let mut count = 0;

    // 4080
    while utf8_chars.len() >= offset + 16 * 255 {
        let mut counts = _mm_setzero_si128();

        for _ in 0..255 {
            counts = _mm_sub_epi8(
                counts,
                is_leading_utf8_byte(mm_from_offset(utf8_chars, offset))
            );
            offset += 16;
        }
        count += sum(&counts);
    }

    // 2048
    if utf8_chars.len() >= offset + 16 * 128 {
        let mut counts = _mm_setzero_si128();
        for _ in 0..128 {
            counts = _mm_sub_epi8(
                counts,
                is_leading_utf8_byte(mm_from_offset(utf8_chars, offset))
            );
            offset += 16;
        }
        count += sum(&counts);
    }

    // 16
    let mut counts = _mm_setzero_si128();
    for i in 0..(utf8_chars.len() - offset) / 16 {
        counts = _mm_sub_epi8(
            counts,
            is_leading_utf8_byte(mm_from_offset(utf8_chars, offset + i * 16))
        );
    }
    if utf8_chars.len() % 16 != 0 {
        counts = _mm_sub_epi8(
            counts,
            _mm_and_si128(
                is_leading_utf8_byte(mm_from_offset(utf8_chars, utf8_chars.len() - 16)),
                                     mm_from_offset(&MASK,      utf8_chars.len() % 16)
            )
        );
    }
    count += sum(&counts);

    count
}
