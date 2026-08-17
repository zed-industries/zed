use std::arch::x86_64::{
    __m256i, _mm256_and_si256, _mm256_cmpeq_epi8, _mm256_extract_epi64, _mm256_loadu_si256,
    _mm256_sad_epu8, _mm256_set1_epi8, _mm256_setzero_si256, _mm256_sub_epi8, _mm256_xor_si256,
};

#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_set1_epu8(a: u8) -> __m256i {
    _mm256_set1_epi8(a as i8)
}

#[target_feature(enable = "avx2")]
pub unsafe fn mm256_cmpneq_epi8(a: __m256i, b: __m256i) -> __m256i {
    _mm256_xor_si256(_mm256_cmpeq_epi8(a, b), _mm256_set1_epi8(-1))
}

const MASK: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

#[target_feature(enable = "avx2")]
unsafe fn mm256_from_offset(slice: &[u8], offset: usize) -> __m256i {
    _mm256_loadu_si256(slice.as_ptr().add(offset) as *const _)
}

#[target_feature(enable = "avx2")]
unsafe fn sum(u8s: &__m256i) -> usize {
    let sums = _mm256_sad_epu8(*u8s, _mm256_setzero_si256());
    (_mm256_extract_epi64(sums, 0)
        + _mm256_extract_epi64(sums, 1)
        + _mm256_extract_epi64(sums, 2)
        + _mm256_extract_epi64(sums, 3)) as usize
}

#[target_feature(enable = "avx2")]
pub unsafe fn chunk_count(haystack: &[u8], needle: u8) -> usize {
    assert!(haystack.len() >= 32);

    let mut offset = 0;
    let mut count = 0;

    let needles = _mm256_set1_epu8(needle);

    // 8160
    while haystack.len() >= offset + 32 * 255 {
        let mut counts = _mm256_setzero_si256();
        for _ in 0..255 {
            counts = _mm256_sub_epi8(
                counts,
                _mm256_cmpeq_epi8(mm256_from_offset(haystack, offset), needles),
            );
            offset += 32;
        }
        count += sum(&counts);
    }

    // 4096
    if haystack.len() >= offset + 32 * 128 {
        let mut counts = _mm256_setzero_si256();
        for _ in 0..128 {
            counts = _mm256_sub_epi8(
                counts,
                _mm256_cmpeq_epi8(mm256_from_offset(haystack, offset), needles),
            );
            offset += 32;
        }
        count += sum(&counts);
    }

    // 32
    let mut counts = _mm256_setzero_si256();
    for i in 0..(haystack.len() - offset) / 32 {
        counts = _mm256_sub_epi8(
            counts,
            _mm256_cmpeq_epi8(mm256_from_offset(haystack, offset + i * 32), needles),
        );
    }
    if haystack.len() % 32 != 0 {
        counts = _mm256_sub_epi8(
            counts,
            _mm256_and_si256(
                _mm256_cmpeq_epi8(mm256_from_offset(haystack, haystack.len() - 32), needles),
                mm256_from_offset(&MASK, haystack.len() % 32),
            ),
        );
    }
    count += sum(&counts);

    count
}

#[target_feature(enable = "avx2")]
unsafe fn is_leading_utf8_byte(u8s: __m256i) -> __m256i {
    mm256_cmpneq_epi8(
        _mm256_and_si256(u8s, _mm256_set1_epu8(0b1100_0000)),
        _mm256_set1_epu8(0b1000_0000),
    )
}

#[target_feature(enable = "avx2")]
pub unsafe fn chunk_num_chars(utf8_chars: &[u8]) -> usize {
    assert!(utf8_chars.len() >= 32);

    let mut offset = 0;
    let mut count = 0;

    // 8160
    while utf8_chars.len() >= offset + 32 * 255 {
        let mut counts = _mm256_setzero_si256();

        for _ in 0..255 {
            counts = _mm256_sub_epi8(
                counts,
                is_leading_utf8_byte(mm256_from_offset(utf8_chars, offset)),
            );
            offset += 32;
        }
        count += sum(&counts);
    }

    // 4096
    if utf8_chars.len() >= offset + 32 * 128 {
        let mut counts = _mm256_setzero_si256();
        for _ in 0..128 {
            counts = _mm256_sub_epi8(
                counts,
                is_leading_utf8_byte(mm256_from_offset(utf8_chars, offset)),
            );
            offset += 32;
        }
        count += sum(&counts);
    }

    // 32
    let mut counts = _mm256_setzero_si256();
    for i in 0..(utf8_chars.len() - offset) / 32 {
        counts = _mm256_sub_epi8(
            counts,
            is_leading_utf8_byte(mm256_from_offset(utf8_chars, offset + i * 32)),
        );
    }
    if utf8_chars.len() % 32 != 0 {
        counts = _mm256_sub_epi8(
            counts,
            _mm256_and_si256(
                is_leading_utf8_byte(mm256_from_offset(utf8_chars, utf8_chars.len() - 32)),
                mm256_from_offset(&MASK, utf8_chars.len() % 32),
            ),
        );
    }
    count += sum(&counts);

    count
}
