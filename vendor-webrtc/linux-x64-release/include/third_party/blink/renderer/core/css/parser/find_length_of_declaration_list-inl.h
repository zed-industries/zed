// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FIND_LENGTH_OF_DECLARATION_LIST_INL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FIND_LENGTH_OF_DECLARATION_LIST_INL_H_

// This file contains SIMD code to try to heuristically detect
// the length of a CSS declaration block. We use this during parsing
// in order to skip over them quickly (we don't need to parse
// all the properties until the first time something actually matches
// the selector). This is akin to setting a BlockGuard and then
// immediately calling SkipToEndOfBlock(), except that
//
//  a) It is much, much faster (something like 10x), since we don't
//     need to run the full tokenizer.
//  b) It is allowed to error out if there are some cases that are
//     too complicated for it to understand (e.g. cases that would
//     require simulating the entire block stack).
//  c) It knows to detect nested rules, and also similarly error out.
//     All of them have to involve { in some shape or form, so that
//     is a fairly easy check (except that we ignore it within strings).
//
// We _don't_ support these cases (i.e., we just error out), which
// we've empirically found to be rare within declaration blocks:
//
//   - Escaping using \ (possible, but requires counting whether
//     we have an even or odd number of them).
//   - [ and ] (would require the block stack).
//   - Extraneous ) (possible, but adds complications and would be rare)
//   - CSS comments (would require interactions with string parsing).
//   - ' within " or " within ' (complex, see below).
//
// The entry point is FindLengthOfDeclarationList(), which returns
// the number of bytes until the block's ending }, exclusive.
// Returns 0 if some kind of error occurred, which means the caller
// will need to parse the block the normal (slow) way.
//
// On x86, we are a bit hampered by our instruction set support;
// it would be nice to have e.g. AVX2, or possibly a smaller subset
// that includes PSHUFB and/or PCLMULQDQ. However, we do fairly well
// already with base SSE2. On Arm (at least on M1, which is very wide),
// it would help slightly to unroll 2x manually to extract more ILP;
// there are many paths where there are long dependency chains.

#ifdef __SSE2__
#include <immintrin.h>
#elif defined(__ARM_NEON__)
#include <arm_neon.h>
#endif

#include "base/compiler_specific.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"

#ifdef __SSE2__

// Loads the 16 next UTF-16 runes, but any character that is >= 256
// will be converted into 0xFF, so that it has no chance of matching
// anything that we care about below. This maps elegantly to a
// saturating pack, since our values are in little-endian already.
static inline __m128i LoadAndCollapseHighBytes(const UChar* ptr) {
  __m128i x1 = _mm_loadu_si128(reinterpret_cast<const __m128i*>(ptr));
  __m128i x2 =
      _mm_loadu_si128(UNSAFE_TODO(reinterpret_cast<const __m128i*>(ptr + 8)));
  return _mm_packus_epi16(x1, x2);
}

// For LChar, this is trivial; just load the bytes as-is.
static inline __m128i LoadAndCollapseHighBytes(const LChar* ptr) {
  return _mm_loadu_si128(reinterpret_cast<const __m128i*>(ptr));
}

template <class CharType>
ALWAYS_INLINE static size_t FindLengthOfDeclarationList(
    const base::span<CharType> chars) {
  const CharType* begin = chars.data();
  const CharType* end = base::to_address(chars.end());
  // If the previous block ended with quote status (see below),
  // the lowest byte of this will be all-ones.
  __m128i prev_quoted = _mm_setzero_si128();

  // Similarly, the lowest byte of this will be the number
  // of open parentheses from the previous block.
  __m128i prev_parens = _mm_setzero_si128();

  const CharType* ptr = begin;
  while (UNSAFE_TODO(ptr + 17) <= end) {
    __m128i x = LoadAndCollapseHighBytes(ptr);
    __m128i next_x = LoadAndCollapseHighBytes(UNSAFE_TODO(ptr + 1));

    // We don't want deal with escaped characters within strings,
    // and they are generally rare, so if we see any backslashes,
    // just error out. Note that backslashes are _not_ masked out
    // by being inside strings, but the others are.
    const __m128i eq_backslash = _mm_cmpeq_epi8(x, _mm_set1_epi8('\\'));

    // We need to mask out any characters that are part of strings.
    // Strings are both started and ended with quotation marks.
    // We do a prefix-xor to spread out “are we in a string”
    // status from each quotation mark to the next stop. The standard
    // trick to do this would be PCLMULQDQ, but that requires CPU support
    // that we don't have (see PrefixXORAVX2()).
    //
    // Thus, we'll simply do a prefix xor-sum instead. Note that we
    // need to factor in the quote status from the previous block;
    // if it had an unclosed quote (whether from a mark in its own
    // block, or carried on from a previous one), we need to
    // include that.
    //
    // There is a complication here. Both " and ' are allowed as
    // quotes, so we need to track both. But if one is within the
    // other, it does not count as a quote and should not start
    // a new span (this is similar to having a comment start token
    // within a string). We don't support this case (it seems hard
    // to do branch-free with SIMD), but we need to detect it.
    // Thus, what we do is that instead of just having all-ones
    // or all-zeros for each byte, we actually use and propagate
    // the byte value of the quote mark itself through the XOR cascade.
    // This leads to four values any byte can take:
    //
    //  - 0x00: We are not within quotes.
    //  - 0x22: We are within double quotes.
    //  - 0x27: We are within single quotes.
    //  - 0x05: We have overlapping single and double quote spans.
    //
    // 0x22 and 0x27 have the same meaning, namely “we're within quotes,
    // and need to mask out all special character handling” (i.e., convert
    // to 0xFF). But 0x05 means that we have ' within " or " within '
    // (mixed quotes), and this is a case we're not prepared to take on.
    // Thus, we treat this as an error condition and abort. If you have
    // shuffles (e.g., in the AVX2 path below), there's a very surprising
    // technique based on group theory that could be used to reliably
    // deal with this case; see:
    //
    //   https://chromium-review.googlesource.com/c/chromium/src/+/5592448
    //
    // However, it is notably slower, and this computation is definitely
    // on the critical path, so we settle for just detection.
    const __m128i eq_single_quote = _mm_cmpeq_epi8(x, _mm_set1_epi8('\''));
    const __m128i eq_double_quote = _mm_cmpeq_epi8(x, _mm_set1_epi8('"'));
    __m128i quoted = x & (eq_single_quote | eq_double_quote);
    quoted ^= prev_quoted;
    quoted ^= _mm_slli_si128(quoted, 1);
    quoted ^= _mm_slli_si128(quoted, 2);
    quoted ^= _mm_slli_si128(quoted, 4);
    quoted ^= _mm_slli_si128(quoted, 8);
    const __m128i mixed_quote =
        _mm_cmpeq_epi8(quoted, _mm_set1_epi8('\'' ^ '"'));
    const __m128i is_quoted = _mm_cmpgt_epi8(quoted, _mm_setzero_si128());

    // Unescaped newlines within quotes are not allowed; they terminate
    // the string. We don't want to complicate our handling beyond
    // detecting it, so we treat it as an error and abort.
    // \r, \n and \f all count as newlines in this regard
    // (see IsCSSNewLine()).
    const __m128i eq_newline = _mm_cmpeq_epi8(x, _mm_set1_epi8('\n')) |
                               _mm_cmpeq_epi8(x, _mm_set1_epi8('\r')) |
                               _mm_cmpeq_epi8(x, _mm_set1_epi8('\f'));
    const __m128i quoted_newline = is_quoted & eq_newline;

    // Now we have a mask of bytes that are inside quotes
    // (which happens to include the first quote, though
    // we don't really care). E.g., if we have this string,
    // our mask will be:
    //
    //    abc"def"ghij"kl"m...
    //    00011110000011100...
    //
    // We can use this to simply ignore things inside strings.
    // (We don't need to mask next_x as it's only used for comments;
    // masking x will be sufficient.)
    x = _mm_andnot_si128(is_quoted, x);

    // Look for start of comments; successive / and * characters.
    // We don't support them, as they are fairly rare and we'd need to
    // do similar masking as with strings.
    const __m128i comment_start = _mm_cmpeq_epi8(x, _mm_set1_epi8('/')) &
                                  _mm_cmpeq_epi8(next_x, _mm_set1_epi8('*'));

    // Parentheses within values are fairly common, e.g. due to var(),
    // calc() and similar. We need to keep track of how many of them we have
    // (since we disallow [ and {, they are the only thing that can be
    // on the block stack), because if we see a } when we have a non-zero
    // number of parens open, that } doesn't actually count as block end.
    //
    // Thus, we count opening paren as +1 and closing paren as -1
    // (comparisons always come out with -1, so we simply substitute),
    // and then run a prefix sum across the block, similar to what we
    // did above with prefix-xor. This gives the status in any byte.
    // E.g., say that we have
    //
    //   c a l c  (  ( 1 + 2  ) *  ( 3 + 4  )
    //   0 0 0 0 +1 +1 0 0 0 -1 0 +1 0 0 0 -1
    //
    // After the prefix sum, we have
    //   c a l c  (  ( 1 + 2  ) *  ( 3 + 4  )
    //   0 0 0 0  1  2 2 2 2  1 1  2 2 2 2  1
    //
    // plus the count from the previous block (if any). In other words,
    // if we were to see a } at the end of this block, we would know
    // that we wouldn't be in balance and we'd have to error out.
    //
    // NOTE: If we have an overflow (more than 128 levels of parens)
    // or underflow (right-paren without matching underflow) at any point
    // in the string, the top bit of the relevant byte in parens will be set.
    // The technically correct thing to do in CSS parsing with underflow
    // is to ignore the underflow, and we could have simulated that by
    // using saturating adds and starting at -128 (or +127). However,
    // it would require slightly more constants to load and possibly be
    // slightly more icky for NEON, so we don't bother. In any case,
    // we simply reuse the top bit as overflow detection (we OR it into
    // the general “we need to stop” mask below), even though
    // we could probably have gone all the way to accept 255 with an
    // extra comparison.
    const __m128i opening_paren = _mm_cmpeq_epi8(x, _mm_set1_epi8('('));
    const __m128i closing_paren = _mm_cmpeq_epi8(x, _mm_set1_epi8(')'));
    __m128i parens =
        _mm_sub_epi8(_mm_add_epi8(prev_parens, closing_paren), opening_paren);
    parens = _mm_add_epi8(parens, _mm_slli_si128(parens, 1));
    parens = _mm_add_epi8(parens, _mm_slli_si128(parens, 2));
    parens = _mm_add_epi8(parens, _mm_slli_si128(parens, 4));
    parens = _mm_add_epi8(parens, _mm_slli_si128(parens, 8));

    // We don't support the full block stack, so [ must go (we don't
    // need to worry about ], as it will just be ignored when there are
    // no ] to match against).
    //
    // Also, we cannot do lazy parsing of nested rules, so opening braces
    // need to abort. With SSSE3 or AVX2, we could use PSHUFB to do several
    // comparisons at the same time, but since they share low nibble,
    // none of the fastest tricks apply, and a simple series of comparisons
    // should be faster.
    //
    // [ and { happen to be 0x20 apart in ASCII, so we can do with one
    // less comparison.
    const __m128i opening_block =
        _mm_cmpeq_epi8(x | _mm_set1_epi8(0x20), _mm_set1_epi8('{'));

    // Right braces mean (successful) EOF.
    const __m128i eq_rightbrace = _mm_cmpeq_epi8(x, _mm_set1_epi8('}'));

    // We generally combine all of the end-parsing situations together
    // and figure out afterwards what the first one was, to determine
    // the return value.
    const __m128i must_end = eq_backslash | mixed_quote | quoted_newline |
                             opening_block | comment_start | eq_rightbrace |
                             parens;
    if (_mm_movemask_epi8(must_end) != 0) {
      unsigned idx = __builtin_ctz(_mm_movemask_epi8(must_end));
      UNSAFE_TODO(ptr += idx);
      if (*ptr == '}') {
        // Check that we have balanced parens at the end point
        // (the paren counter is zero).
        uint16_t mask =
            _mm_movemask_epi8(_mm_cmpeq_epi8(parens, _mm_setzero_si128()));
        if (((mask >> idx) & 1) == 0) {
          return 0;
        } else {
          return ptr - begin;
        }
      } else {
        // Ended due to something that was not successful EOB.
        return 0;
      }
    }

    UNSAFE_TODO(ptr += 16);
    prev_quoted = _mm_srli_si128(quoted, 15);
    prev_parens = _mm_srli_si128(parens, 15);
  }
  return 0;  // Premature EOF; we cannot SIMD any further.
}

// The AVX2 version is 50–65% faster than SSE2, depending on CPU;
// partially from wider vectors, and partially from increased
// instruction availability. The caller must call
// FindLengthOfDeclarationListAVX2() themselves if relevant.
//
// Like with NEON below, we'll only really document the differences
// with the SSE2 version. AVX2 is generally a 2x128-bit instruction set,
// much like NEON is a 2x64-bit instruction set, which will necessitate
// some of the same strategies.

__attribute__((target("avx2"))) static inline __m256i
LoadAndCollapseHighBytesAVX2(const UChar* ptr) {
  __m256i x1 = _mm256_loadu_si256(reinterpret_cast<const __m256i*>(ptr));
  __m256i x2 = _mm256_loadu_si256(
      reinterpret_cast<const __m256i*>(UNSAFE_TODO(ptr + 16)));
  __m256i packed = _mm256_packus_epi16(x1, x2);

  // AVX2 pack is per-lane (two separate 16 -> 8 packs),
  // so to get the right order, we'll need a lane-crossing permute.
  return _mm256_permute4x64_epi64(packed, _MM_SHUFFLE(3, 1, 2, 0));
}

__attribute__((target("avx2"))) static inline __m256i
LoadAndCollapseHighBytesAVX2(const LChar* ptr) {
  return _mm256_loadu_si256(reinterpret_cast<const __m256i*>(ptr));
}

// Similar to NEON, our parenthesis cascade doesn't cross the 128-bit lanes
// (shifts are not 256-bit, but rather two separate 128-bit shifts), so we'll
// need a final operation to propagate the highest element of the low lane
// into all the elements in the high lane. The compiler converts this to
// a nice combination of shuffles and permutations.
__attribute__((target("avx2"))) static inline __m256i BroadcastToHigh(
    __m256i x) {
  uint8_t b = _mm256_extract_epi8(x, 15);
  return _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, b, b,
                          b, b, b, b, b, b, b, b, b, b, b, b, b, b);
}

// For the prefix-xor cascade, we can abuse the carryless multiplication
// function found in modern CPUs (there are really none that support AVX2 but
// not this). Essentially, it does a binary multiplication, except that adds are
// replaced with XORs, which means we can multiply with 1111111... to get
// exactly what we want. Note that the upper bits will contain junk that we may
// need to remove later.
//
// Also note that doing this operation over both halves of a 256-bit register
// is a much newer extension, but we only really need it over a 32-bit value.
// We go through an integer register to convert the 256-bit values to 32
// single bits (if we had eight bits per byte, they would be masking each other
// out anyway), and then immediately bump upwards again to a 128-bit register
// for the multiplication. Note that we return that 128-bit register; since we
// want the value _both_ in an integer register (it lets us do more work
// in parallel with the parenthesis cascade) _and_ in a vector register
// (since we need to use it to mask out bytes before said cascade), we let
// the caller do the conversion.
__attribute__((target("avx2,pclmul"))) ALWAYS_INLINE static __m128i
PrefixXORAVX2(__m256i x, uint64_t prev) {
  uint64_t bitmask = _mm256_movemask_epi8(x) ^ prev;
  __m128i all_ones = _mm_set1_epi8(0xff);
  return _mm_clmulepi64_si128(_mm_set_epi64x(0ULL, bitmask), all_ones, 0);
}

// Once PrefixXORAVX2() has created a bit mask, we need to convert that back
// to a byte mask. This is an adapted version of
//
//   https://stackoverflow.com/questions/21622212/how-to-perform-the-inverse-of-mm256-movemask-epi8-vpmovmskb
//
// except that we take in the input value in the bottom 32 bits of a vector
// register, which gives less transfer back and forth through the integer
// registers. Clang figures out a fairly fast way of computing vmask using
// shuffles.
__attribute__((target("avx2"))) ALWAYS_INLINE static __m256i MaskToAVX2(
    __m128i mask) {
  __m256i vmask = _mm256_set1_epi32(_mm_extract_epi32(mask, 0));
  const __m256i shuffle =
      _mm256_setr_epi64x(0x0000000000000000, 0x0101010101010101,
                         0x0202020202020202, 0x0303030303030303);
  vmask = _mm256_shuffle_epi8(vmask, shuffle);
  const __m256i bit_mask = _mm256_set1_epi64x(0x7fbfdfeff7fbfdfe);
  vmask = _mm256_or_si256(vmask, bit_mask);
  return _mm256_cmpeq_epi8(vmask, _mm256_set1_epi64x(-1));
}

template <class CharType>
__attribute__((target("avx2,pclmul"))) ALWAYS_INLINE static size_t
FindLengthOfDeclarationListAVX2(base::span<const CharType> chars) {
  const CharType* begin = chars.data();
  const CharType* end = base::to_address(chars.end());
  uint64_t prev_single_quote = 0;
  uint64_t prev_double_quote = 0;
  __m256i prev_parens = _mm256_setzero_si256();

  const CharType* ptr = begin;
  while (UNSAFE_TODO(ptr + 33) <= end) {
    __m256i x = LoadAndCollapseHighBytesAVX2(ptr);
    __m256i next_x = LoadAndCollapseHighBytesAVX2(UNSAFE_TODO(ptr + 1));

    const __m256i eq_backslash = _mm256_cmpeq_epi8(x, _mm256_set1_epi8('\\'));

    // See PrefixXORAVX2() for information on how we compute the prefix-xor.
    // Note that we now have separate computations for single and double quotes,
    // since they are bit masks and not full bytes, but they can go in parallel
    // just fine.
    const __m256i eq_single_quote =
        _mm256_cmpeq_epi8(x, _mm256_set1_epi8('\''));
    const __m256i eq_double_quote = _mm256_cmpeq_epi8(x, _mm256_set1_epi8('"'));
    __m128i prefix_single_quote =
        PrefixXORAVX2(eq_single_quote, prev_single_quote);
    __m128i prefix_double_quote =
        PrefixXORAVX2(eq_double_quote, prev_double_quote);
    uint32_t single_quote_bitmask = _mm_cvtsi128_si32(prefix_single_quote);
    uint32_t double_quote_bitmask = _mm_cvtsi128_si32(prefix_double_quote);
    uint32_t mixed_quote = single_quote_bitmask & double_quote_bitmask;
    uint32_t quoted_bitmask = single_quote_bitmask | double_quote_bitmask;

    // We need to convert this back into a byte mask so that we can mask out
    // parens within quotes.
    __m256i quoted_mask = MaskToAVX2(prefix_single_quote | prefix_double_quote);

    // Since we have PSHUFB, and the values we're looking for (0x0a, 0x0c, 0x0d)
    // all share a nibble (the upper nibble is always zero), we can use a
    // shuffle plus compare instead of three compares and ORs. Basically this is
    // the test `x == table[x & 0x0f]` (modulo some PSHUFB behavior with the top
    // bit of x, that we don't need to worry about here), where we engineer
    // table[] so that this only holds true for the three values of x we care
    // about.
    const __m256i eq_newline_table =
        _mm256_setr_epi64x(0x0000000000000001, 0x00000d0c000a0000,
                           0x0000000000000000, 0x0000000000000000);
    const __m256i eq_newline =
        _mm256_cmpeq_epi8(x, _mm256_shuffle_epi8(eq_newline_table, x));
    const __m256i quoted_newline = quoted_mask & eq_newline;

    const __m256i comment_start =
        _mm256_cmpeq_epi8(x, _mm256_set1_epi8('/')) &
        _mm256_cmpeq_epi8(next_x, _mm256_set1_epi8('*'));

    // Like in NEON, we need to shuffle the low values over into the high
    // 128-bit lane (see BroadcastToHigh()), and we choose to do prev_parens
    // as a shuffle after the cascade instead of a single value before
    // for the same reasons.
    const __m256i opening_paren = _mm256_cmpeq_epi8(x, _mm256_set1_epi8('('));
    const __m256i closing_paren = _mm256_cmpeq_epi8(x, _mm256_set1_epi8(')'));
    __m256i parens =
        _mm256_sub_epi8(closing_paren, opening_paren) & ~quoted_mask;
    parens = _mm256_add_epi8(parens, _mm256_slli_si256(parens, 1));
    parens = _mm256_add_epi8(parens, _mm256_slli_si256(parens, 2));
    parens = _mm256_add_epi8(parens, _mm256_slli_si256(parens, 4));
    parens = _mm256_add_epi8(parens, _mm256_slli_si256(parens, 8));
    parens = _mm256_add_epi8(parens, BroadcastToHigh(parens));
    parens = _mm256_add_epi8(parens, prev_parens);

    const __m256i opening_block =
        _mm256_cmpeq_epi8(x | _mm256_set1_epi8(0x20), _mm256_set1_epi8('{'));
    const __m256i eq_rightbrace = _mm256_cmpeq_epi8(x, _mm256_set1_epi8('}'));
    uint64_t must_end =
        (_mm256_movemask_epi8(opening_block | comment_start | eq_rightbrace) &
         ~quoted_bitmask) |
        mixed_quote |
        _mm256_movemask_epi8(parens | eq_backslash | quoted_newline);
    if (must_end != 0) {
      unsigned idx = __builtin_ctzll(must_end);
      UNSAFE_TODO(ptr += idx);
      if (*ptr == '}') {
        uint32_t mask = _mm256_movemask_epi8(
            _mm256_cmpeq_epi8(parens, _mm256_setzero_si256()));
        if (((mask >> idx) & 1) == 0) {
          return 0;
        } else {
          return ptr - begin;
        }
      } else {
        // Ended due to something that was not successful EOB.
        return 0;
      }
    }

    UNSAFE_TODO(ptr += 32);

    // We keep prev_*_quote as integers, unlike in SSE2; there's no need
    // to waste cross-lane shifts on them.
    prev_single_quote = uint32_t(single_quote_bitmask) >> 31;
    prev_double_quote = uint32_t(double_quote_bitmask) >> 31;

    prev_parens = _mm256_set1_epi8(((__v32qi)parens)[31]);
  }
  return 0;
}

__attribute__((target("avx2,pclmul"))) inline size_t
FindLengthOfDeclarationListAVX2(StringView str) {
  if (str.Is8Bit()) {
    return FindLengthOfDeclarationListAVX2(str.Span8());
  } else {
    return FindLengthOfDeclarationListAVX2(str.Span16());
  }
}

#elif defined(__ARM_NEON__)

static inline uint8x16_t LoadAndCollapseHighBytes(const UChar* ptr) {
  uint8x16_t x1;
  uint8x16_t x2;
  UNSAFE_TODO({
    memcpy(&x1, ptr, sizeof(x1));
    memcpy(&x2, ptr + 8, sizeof(x2));
  });
  return vreinterpretq_u8_u64(
      vcombine_u64(vreinterpret_u64_u8(vqmovn_u16(vreinterpretq_u16_u8(x1))),
                   vreinterpret_u64_u8(vqmovn_u16(vreinterpretq_u16_u8(x2)))));
}
static inline uint8x16_t LoadAndCollapseHighBytes(const LChar* ptr) {
  uint8x16_t ret;
  UNSAFE_TODO(memcpy(&ret, ptr, sizeof(ret)));
  return ret;
}

// The NEON implementation follows basically the same pattern as the
// SSE2 implementation; comments will be added only where they differ
// substantially.
//
// For A64, we _do_ have access to the PMULL instruction (the NEON
// equivalent of PCLMULQDQ), but it's supposedly slow, so we use
// the same XOR-shift cascade.
template <class CharType>
ALWAYS_INLINE static size_t FindLengthOfDeclarationList(
    base::span<const CharType> chars) {
  const CharType* begin = chars.data();
  const CharType* end = base::to_address(chars.end());
  // Since NEON doesn't have a natural way of moving the last element
  // to the first slot (shift right by 15 _bytes_), but _does_ have
  // fairly cheap broadcasting (unlike SSE2 without SSSE3), we use
  // a slightly different convention: The prev_* elements hold the
  // last element in _all_ lanes, and is then applied _after_
  // the prefix sum/prefix XOR. This would create havoc with
  // saturating operations, but works well when they commute.
  uint8x16_t prev_quoted = vdupq_n_u8(0);
  uint8x16_t prev_parens = vdupq_n_u8(0);

  const CharType* ptr = begin;
  while (UNSAFE_TODO(ptr + 17) <= end) {
    uint8x16_t x = LoadAndCollapseHighBytes(ptr);
    const uint8x16_t next_x = LoadAndCollapseHighBytes(UNSAFE_TODO(ptr + 1));
    const uint8x16_t eq_backslash = x == '\\';
    const uint8x16_t eq_double_quote = x == '"';
    const uint8x16_t eq_single_quote = x == '\'';
    uint8x16_t quoted = x & (eq_double_quote | eq_single_quote);

    // NEON doesn't have 128-bit bytewise shifts like SSE2 have.
    // We thus need to do the algorithm separately in 64-bit halves,
    // then to a separate duplication step to transfer the result
    // from the highest element of the bottom half to all elements
    // of the top half. (The alternative would be to use TBL
    // instructions to simulate the shifts, but they can be slow
    // on mobile CPUs.)
    quoted ^=
        vreinterpretq_u8_u64(vshlq_n_u64(vreinterpretq_u64_u8(quoted), 8));
    quoted ^=
        vreinterpretq_u8_u64(vshlq_n_u64(vreinterpretq_u64_u8(quoted), 16));
    quoted ^=
        vreinterpretq_u8_u64(vshlq_n_u64(vreinterpretq_u64_u8(quoted), 32));
    quoted ^= vreinterpretq_u8_u64(vcombine_u64(
        vdup_n_u64(0),
        vreinterpret_u64_u8(vdup_lane_u8(
            vreinterpret_u8_u64(vget_low_u64(vreinterpretq_u64_u8(quoted))),
            7))));
    quoted ^= prev_quoted;
    const uint8x16_t mixed_quote = quoted == static_cast<char>('\'' ^ '"');

    const uint8x16_t is_quoted = quoted > vdupq_n_u8(0);
    // The VTBL instruction returns zero for indexes outside [0,16), so we
    // don't need to be clever at all here, unlike with AVX2.
#ifdef ARCH_CPU_ARM64
    const uint8x16_t eq_newline = vqtbl1q_u8(
        uint8x16_t{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0, 0xff, 0xff, 0, 0}, x);
#else
    const uint8x8x2_t eq_newline_table{0, 0, 0,    0, 0,    0,    0, 0,
                                       0, 0, 0xff, 0, 0xff, 0xff, 0, 0};
    const uint8x16_t eq_newline =
        vcombine_s8(vtbl2_u8(eq_newline_table, vget_low_s8(x)),
                    vtbl2_u8(eq_newline_table, vget_high_s8(x)));
#endif
    const uint8x16_t quoted_newline = is_quoted & eq_newline;

    x &= ~is_quoted;

    const uint8x16_t comment_start = (x == '/') & (next_x == '*');
    const uint8x16_t opening_paren = x == '(';
    const uint8x16_t closing_paren = x == ')';
    uint8x16_t parens = closing_paren - opening_paren;
    parens +=
        vreinterpretq_u8_u64(vshlq_n_u64(vreinterpretq_u64_u8(parens), 8));
    parens +=
        vreinterpretq_u8_u64(vshlq_n_u64(vreinterpretq_u64_u8(parens), 16));
    parens +=
        vreinterpretq_u8_u64(vshlq_n_u64(vreinterpretq_u64_u8(parens), 32));
    parens += vreinterpretq_u8_u64(vcombine_u64(
        vdup_n_u64(0),
        vreinterpret_u64_u8(vdup_lane_u8(
            vreinterpret_u8_u64(vget_low_u64(vreinterpretq_u64_u8(parens))),
            7))));
    parens += prev_parens;

    // The VSHRN trick below doesn't guarantee the use of the top bit
    // the same way PMOVMSKB does, so we can't just use the parens value
    // directly for overflow check. We could compare directly against 255
    // here, but it's nice to have exactly the same behavior as on Intel,
    // so we do a signed shift to just replicate the top bit into the entire
    // byte. (Supposedly, this also has one cycle better throughput on
    // some CPUs.)
    const uint8x16_t parens_overflow =
        vreinterpretq_u8_s8(vshrq_n_s8(vreinterpretq_s8_u8(parens), 7));

    const uint8x16_t opening_block = (x | vdupq_n_u8(0x20)) == '{';
    const uint8x16_t eq_rightbrace = x == '}';
    uint8x16_t must_end = eq_backslash | mixed_quote | quoted_newline |
                          opening_block | comment_start | eq_rightbrace |
                          parens_overflow;

    // https://community.arm.com/arm-community-blogs/b/infrastructure-solutions-blog/posts/porting-x86-vector-bitmask-optimizations-to-arm-neon
    uint64_t must_end_narrowed = vget_lane_u64(
        vreinterpret_u64_u8(vshrn_n_u16(vreinterpretq_u16_u8(must_end), 4)), 0);
    if (must_end_narrowed != 0) {
      unsigned idx = __builtin_ctzll(must_end_narrowed) >> 2;
      UNSAFE_TODO(ptr += idx);
      if (*ptr == '}') {
        // Since we don't have cheap PMOVMSKB, and this is not on
        // the most critical path, we just chicken out here and let
        // the compiler spill the value to the stack, where we can
        // do a normal indexing.
        if (parens[idx] != 0) {
          return 0;
        } else {
          return ptr - begin;
        }
      } else {
        return 0;
      }
    }

    // As mentioned above, broadcast instead of shifting.
    UNSAFE_TODO(ptr += 16);
    prev_quoted = vdupq_lane_u8(
        vreinterpret_u8_u64(vget_high_u64(vreinterpretq_u64_u8(quoted))), 7);
    prev_parens = vdupq_lane_u8(
        vreinterpret_u8_u64(vget_high_u64(vreinterpretq_u64_u8(parens))), 7);
  }
  return 0;
}

#else

// If we have neither SSE2 nor NEON, we simply return 0 immediately.
// We will then never use lazy parsing.
template <class CharType>
ALWAYS_INLINE static size_t FindLengthOfDeclarationList(
    base::span<const CharType> chars) {
  return 0;
}

#endif

ALWAYS_INLINE static size_t FindLengthOfDeclarationList(StringView str) {
  if (str.Is8Bit()) {
    return FindLengthOfDeclarationList(str.Span8());
  } else {
    return FindLengthOfDeclarationList(str.Span16());
  }
}

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FIND_LENGTH_OF_DECLARATION_LIST_INL_H_
