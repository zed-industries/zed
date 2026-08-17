#include <emmintrin.h>
#include <smmintrin.h>
#include <tmmintrin.h>

static FORCE_INLINE_ATTR inline size_t utf8_range_ValidateUTF8Simd(
    const char* data_original, const char* data, const char* end,
    int return_position) {
  /* This code checks that utf-8 ranges are structurally valid 16 bytes at once
   * using superscalar instructions.
   * The mapping between ranges of codepoint and their corresponding utf-8
   * sequences is below.
   */

  /*
   * U+0000...U+007F     00...7F
   * U+0080...U+07FF     C2...DF 80...BF
   * U+0800...U+0FFF     E0      A0...BF 80...BF
   * U+1000...U+CFFF     E1...EC 80...BF 80...BF
   * U+D000...U+D7FF     ED      80...9F 80...BF
   * U+E000...U+FFFF     EE...EF 80...BF 80...BF
   * U+10000...U+3FFFF   F0      90...BF 80...BF 80...BF
   * U+40000...U+FFFFF   F1...F3 80...BF 80...BF 80...BF
   * U+100000...U+10FFFF F4      80...8F 80...BF 80...BF
   */

  /* First we compute the type for each byte, as given by the table below.
   * This type will be used as an index later on.
   */

  /*
   * Index  Min Max Byte Type
   *  0     00  7F  Single byte sequence
   *  1,2,3 80  BF  Second, third and fourth byte for many of the sequences.
   *  4     A0  BF  Second byte after E0
   *  5     80  9F  Second byte after ED
   *  6     90  BF  Second byte after F0
   *  7     80  8F  Second byte after F4
   *  8     C2  F4  First non ASCII byte
   *  9..15 7F  80  Invalid byte
   */

  /* After the first step we compute the index for all bytes, then we permute
     the bytes according to their indices to check the ranges from the range
     table.
   * The range for a given type can be found in the range_min_table and
     range_max_table, the range for type/index X is in range_min_table[X] ...
     range_max_table[X].
   */

  /* Algorithm:
   * Put index zero to all bytes.
   * Find all non ASCII characters, give them index 8.
   * For each tail byte in a codepoint sequence, give it an index corresponding
     to the 1 based index from the end.
   * If the first byte of the codepoint is in the [C0...DF] range, we write
     index 1 in the following byte.
   * If the first byte of the codepoint is in the range [E0...EF], we write
     indices 2 and 1 in the next two bytes.
   * If the first byte of the codepoint is in the range [F0...FF] we write
     indices 3,2,1 into the next three bytes.
   * For finding the number of bytes we need to look at high nibbles (4 bits)
     and do the lookup from the table, it can be done with shift by 4 + shuffle
     instructions. We call it `first_len`.
   * Then we shift first_len by 8 bits to get the indices of the 2nd bytes.
   * Saturating sub 1 and shift by 8 bits to get the indices of the 3rd bytes.
   * Again to get the indices of the 4th bytes.
   * Take OR of all that 4 values and check within range.
   */
  /* For example:
   * input       C3 80 68 E2 80 20 A6 F0 A0 80 AC 20 F0 93 80 80
   * first_len   1  0  0  2  0  0  0  3  0  0  0  0  3  0  0  0
   * 1st byte    8  0  0  8  0  0  0  8  0  0  0  0  8  0  0  0
   * 2nd byte    0  1  0  0  2  0  0  0  3  0  0  0  0  3  0  0 // Shift + sub
   * 3rd byte    0  0  0  0  0  1  0  0  0  2  0  0  0  0  2  0 // Shift + sub
   * 4th byte    0  0  0  0  0  0  0  0  0  0  1  0  0  0  0  1 // Shift + sub
   * Index       8  1  0  8  2  1  0  8  3  2  1  0  8  3  2  1 // OR of results
   */

  /* Checking for errors:
   * Error checking is done by looking up the high nibble (4 bits) of each byte
     against an error checking table.
   * Because the lookup value for the second byte depends of the value of the
     first byte in codepoint, we use saturated operations to adjust the index.
   * Specifically we need to add 2 for E0, 3 for ED, 3 for F0 and 4 for F4 to
     match the correct index.
       * If we subtract from all bytes EF then EO -> 241, ED -> 254, F0 -> 1,
         F4 -> 5
       * Do saturating sub 240, then E0 -> 1, ED -> 14 and we can do lookup to
         match the adjustment
       * Add saturating 112, then F0 -> 113, F4 -> 117, all that were > 16 will
         be more 128 and lookup in ef_fe_table will return 0 but for F0
         and F4 it will be 4 and 5 accordingly
   */
  /*
   * Then just check the appropriate ranges with greater/smaller equal
     instructions. Check tail with a naive algorithm.
   * To save from previous 16 byte checks we just align previous_first_len to
     get correct continuations of the codepoints.
   */

  /*
   * Map high nibble of "First Byte" to legal character length minus 1
   * 0x00 ~ 0xBF --> 0
   * 0xC0 ~ 0xDF --> 1
   * 0xE0 ~ 0xEF --> 2
   * 0xF0 ~ 0xFF --> 3
   */
  const __m128i first_len_table =
      _mm_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 3);

  /* Map "First Byte" to 8-th item of range table (0xC2 ~ 0xF4) */
  const __m128i first_range_table =
      _mm_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8);

  /*
   * Range table, map range index to min and max values
   */
  const __m128i range_min_table =
      _mm_setr_epi8(0x00, 0x80, 0x80, 0x80, 0xA0, 0x80, 0x90, 0x80, 0xC2, 0x7F,
                    0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F);

  const __m128i range_max_table =
      _mm_setr_epi8(0x7F, 0xBF, 0xBF, 0xBF, 0xBF, 0x9F, 0xBF, 0x8F, 0xF4, 0x80,
                    0x80, 0x80, 0x80, 0x80, 0x80, 0x80);

  /*
   * Tables for fast handling of four special First Bytes(E0,ED,F0,F4), after
   * which the Second Byte are not 80~BF. It contains "range index adjustment".
   * +------------+---------------+------------------+----------------+
   * | First Byte | original range| range adjustment | adjusted range |
   * +------------+---------------+------------------+----------------+
   * | E0         | 2             | 2                | 4              |
   * +------------+---------------+------------------+----------------+
   * | ED         | 2             | 3                | 5              |
   * +------------+---------------+------------------+----------------+
   * | F0         | 3             | 3                | 6              |
   * +------------+---------------+------------------+----------------+
   * | F4         | 4             | 4                | 8              |
   * +------------+---------------+------------------+----------------+
   */

  /* df_ee_table[1] -> E0, df_ee_table[14] -> ED as ED - E0 = 13 */
  // The values represent the adjustment in the Range Index table for a correct
  // index.
  const __m128i df_ee_table =
      _mm_setr_epi8(0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0);

  /* ef_fe_table[1] -> F0, ef_fe_table[5] -> F4, F4 - F0 = 4 */
  // The values represent the adjustment in the Range Index table for a correct
  // index.
  const __m128i ef_fe_table =
      _mm_setr_epi8(0, 3, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);

  __m128i prev_input = _mm_set1_epi8(0);
  __m128i prev_first_len = _mm_set1_epi8(0);
  __m128i error = _mm_set1_epi8(0);

  while (end - data >= 16) {
    const __m128i input = _mm_loadu_si128((const __m128i*)(data));

    /* high_nibbles = input >> 4 */
    const __m128i high_nibbles =
        _mm_and_si128(_mm_srli_epi16(input, 4), _mm_set1_epi8(0x0F));

    /* first_len = legal character length minus 1 */
    /* 0 for 00~7F, 1 for C0~DF, 2 for E0~EF, 3 for F0~FF */
    /* first_len = first_len_table[high_nibbles] */
    __m128i first_len = _mm_shuffle_epi8(first_len_table, high_nibbles);

    /* First Byte: set range index to 8 for bytes within 0xC0 ~ 0xFF */
    /* range = first_range_table[high_nibbles] */
    __m128i range = _mm_shuffle_epi8(first_range_table, high_nibbles);

    /* Second Byte: set range index to first_len */
    /* 0 for 00~7F, 1 for C0~DF, 2 for E0~EF, 3 for F0~FF */
    /* range |= (first_len, prev_first_len) << 1 byte */
    range = _mm_or_si128(range, _mm_alignr_epi8(first_len, prev_first_len, 15));

    /* Third Byte: set range index to saturate_sub(first_len, 1) */
    /* 0 for 00~7F, 0 for C0~DF, 1 for E0~EF, 2 for F0~FF */
    __m128i tmp1;
    __m128i tmp2;
    /* tmp1 = saturate_sub(first_len, 1) */
    tmp1 = _mm_subs_epu8(first_len, _mm_set1_epi8(1));
    /* tmp2 = saturate_sub(prev_first_len, 1) */
    tmp2 = _mm_subs_epu8(prev_first_len, _mm_set1_epi8(1));
    /* range |= (tmp1, tmp2) << 2 bytes */
    range = _mm_or_si128(range, _mm_alignr_epi8(tmp1, tmp2, 14));

    /* Fourth Byte: set range index to saturate_sub(first_len, 2) */
    /* 0 for 00~7F, 0 for C0~DF, 0 for E0~EF, 1 for F0~FF */
    /* tmp1 = saturate_sub(first_len, 2) */
    tmp1 = _mm_subs_epu8(first_len, _mm_set1_epi8(2));
    /* tmp2 = saturate_sub(prev_first_len, 2) */
    tmp2 = _mm_subs_epu8(prev_first_len, _mm_set1_epi8(2));
    /* range |= (tmp1, tmp2) << 3 bytes */
    range = _mm_or_si128(range, _mm_alignr_epi8(tmp1, tmp2, 13));

    /*
     * Now we have below range indices calculated
     * Correct cases:
     * - 8 for C0~FF
     * - 3 for 1st byte after F0~FF
     * - 2 for 1st byte after E0~EF or 2nd byte after F0~FF
     * - 1 for 1st byte after C0~DF or 2nd byte after E0~EF or
     *         3rd byte after F0~FF
     * - 0 for others
     * Error cases:
     *   >9 for non ascii First Byte overlapping
     *   E.g., F1 80 C2 90 --> 8 3 10 2, where 10 indicates error
     */

    /* Adjust Second Byte range for special First Bytes(E0,ED,F0,F4) */
    /* Overlaps lead to index 9~15, which are illegal in range table */
    __m128i shift1;
    __m128i pos;
    __m128i range2;
    /* shift1 = (input, prev_input) << 1 byte */
    shift1 = _mm_alignr_epi8(input, prev_input, 15);
    pos = _mm_sub_epi8(shift1, _mm_set1_epi8(0xEF));
    /*
     * shift1:  | EF  F0 ... FE | FF  00  ... ...  DE | DF  E0 ... EE |
     * pos:     | 0   1      15 | 16  17           239| 240 241    255|
     * pos-240: | 0   0      0  | 0   0            0  | 0   1      15 |
     * pos+112: | 112 113    127|       >= 128        |     >= 128    |
     */
    tmp1 = _mm_subs_epu8(pos, _mm_set1_epi8(-16));
    range2 = _mm_shuffle_epi8(df_ee_table, tmp1);
    tmp2 = _mm_adds_epu8(pos, _mm_set1_epi8(112));
    range2 = _mm_add_epi8(range2, _mm_shuffle_epi8(ef_fe_table, tmp2));

    range = _mm_add_epi8(range, range2);

    /* Load min and max values per calculated range index */
    __m128i min_range = _mm_shuffle_epi8(range_min_table, range);
    __m128i max_range = _mm_shuffle_epi8(range_max_table, range);

    /* Check value range */
    if (return_position) {
      error = _mm_cmplt_epi8(input, min_range);
      error = _mm_or_si128(error, _mm_cmpgt_epi8(input, max_range));
      /* 5% performance drop from this conditional branch */
      if (!_mm_testz_si128(error, error)) {
        break;
      }
    } else {
      error = _mm_or_si128(error, _mm_cmplt_epi8(input, min_range));
      error = _mm_or_si128(error, _mm_cmpgt_epi8(input, max_range));
    }

    prev_input = input;
    prev_first_len = first_len;

    data += 16;
  }
  /* If we got to the end, we don't need to skip any bytes backwards */
  if (return_position && data == data_original) {
    return utf8_range_ValidateUTF8Naive(data, end, return_position);
  }
  /* Find previous codepoint (not 80~BF) */
  data -= utf8_range_CodepointSkipBackwards(_mm_extract_epi32(prev_input, 3));
  if (return_position) {
    return (data - data_original) +
           utf8_range_ValidateUTF8Naive(data, end, return_position);
  }
  /* Test if there was any error */
  if (!_mm_testz_si128(error, error)) {
    return 0;
  }
  /* Check the tail */
  return utf8_range_ValidateUTF8Naive(data, end, return_position);
}
