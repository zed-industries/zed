#include <arm_neon.h>

/* This code is almost the same as SSE implementation, please reference
 * utf8-range-sse.inc for detailed explanation.
 * The only difference is the range adjustment step. NEON code is more
 * straightforward.
 */

static FORCE_INLINE_ATTR inline size_t utf8_range_ValidateUTF8Simd(
    const char* data_original, const char* data, const char* end,
    int return_position) {
  const uint8x16_t first_len_tbl = {
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 3,
  };
  const uint8x16_t first_range_tbl = {
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8,
  };
  const uint8x16_t range_min_tbl = {
      0x00, 0x80, 0x80, 0x80, 0xA0, 0x80, 0x90, 0x80,
      0xC2, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
  };
  const uint8x16_t range_max_tbl = {
      0x7F, 0xBF, 0xBF, 0xBF, 0xBF, 0x9F, 0xBF, 0x8F,
      0xF4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  };
  /* Range adjustment in NEON uint8x16x2 table. Note that lanes are interleaved
   * in register. The table below is plotted vertically to ease understanding.
   * The 1st column is for E0~EF, 2nd column for F0~FF.
   */
  // clang-format off
  const uint8_t range_adjust_tbl_data[] = {
    /* index -> 0~15  16~31 <- index */
    /*  E0 -> */ 2,     3, /* <- F0  */
                 0,     0,
                 0,     0,
                 0,     0,
                 0,     4, /* <- F4  */
                 0,     0,
                 0,     0,
                 0,     0,
                 0,     0,
                 0,     0,
                 0,     0,
                 0,     0,
                 0,     0,
    /*  ED -> */ 3,     0,
                 0,     0,
                 0,     0,
  };
  // clang-format on
  const uint8x16x2_t range_adjust_tbl = vld2q_u8(range_adjust_tbl_data);

  const uint8x16_t const_1 = vdupq_n_u8(1);
  const uint8x16_t const_2 = vdupq_n_u8(2);
  const uint8x16_t const_e0 = vdupq_n_u8(0xE0);

  uint8x16_t prev_input = vdupq_n_u8(0);
  uint8x16_t prev_first_len = vdupq_n_u8(0);
  uint8x16_t error = vdupq_n_u8(0);

  while (end - data >= 16) {
    const uint8x16_t input = vld1q_u8((const uint8_t*)data);

    const uint8x16_t high_nibbles = vshrq_n_u8(input, 4);

    const uint8x16_t first_len = vqtbl1q_u8(first_len_tbl, high_nibbles);

    uint8x16_t range = vqtbl1q_u8(first_range_tbl, high_nibbles);

    range = vorrq_u8(range, vextq_u8(prev_first_len, first_len, 15));

    uint8x16_t shift2 = vextq_u8(prev_first_len, first_len, 14);
    shift2 = vqsubq_u8(shift2, const_1);
    range = vorrq_u8(range, shift2);

    uint8x16_t shift3 = vextq_u8(prev_first_len, first_len, 13);
    shift3 = vqsubq_u8(shift3, const_2);
    range = vorrq_u8(range, shift3);

    uint8x16_t shift1 = vextq_u8(prev_input, input, 15);
    shift1 = vsubq_u8(shift1, const_e0);
    range = vaddq_u8(range, vqtbl2q_u8(range_adjust_tbl, shift1));

    const uint8x16_t min_range = vqtbl1q_u8(range_min_tbl, range);
    const uint8x16_t max_range = vqtbl1q_u8(range_max_tbl, range);

    if (return_position) {
      error = vcltq_u8(input, min_range);
      error = vorrq_u8(error, vcgtq_u8(input, max_range));
      if (vmaxvq_u32(vreinterpretq_u32_u8(error))) {
        break;
      }
    } else {
      error = vorrq_u8(error, vcltq_u8(input, min_range));
      error = vorrq_u8(error, vcgtq_u8(input, max_range));
    }

    prev_input = input;
    prev_first_len = first_len;

    data += 16;
  }

  if (return_position && data == data_original) {
    return utf8_range_ValidateUTF8Naive(data, end, return_position);
  }
  const int32_t prev = vgetq_lane_s32(vreinterpretq_s32_u8(prev_input), 3);
  data -= utf8_range_CodepointSkipBackwards(prev);
  if (return_position) {
    return (data - data_original) +
           utf8_range_ValidateUTF8Naive(data, end, return_position);
  }
  if (vmaxvq_u32(vreinterpretq_u32_u8(error))) {
    return 0;
  }
  return utf8_range_ValidateUTF8Naive(data, end, return_position);
}
