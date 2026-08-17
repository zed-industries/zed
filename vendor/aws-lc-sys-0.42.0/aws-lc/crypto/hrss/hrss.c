// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/hrss.h>

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include <openssl/bn.h>
#include <openssl/hmac.h>
#include <openssl/mem.h>
#include <openssl/rand.h>
#include <openssl/sha.h>

#if defined(_MSC_VER)
#define RESTRICT
#else
#define RESTRICT restrict
#endif

#include "../internal.h"
#include "internal.h"

#if defined(OPENSSL_SSE2)
#include <emmintrin.h>
#endif

#if (defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64)) && defined(__ARM_NEON)
#include <arm_neon.h>
#endif

// This is an implementation of [HRSS], but with a KEM transformation based on
// [SXY]. The primary references are:

// HRSS: https://eprint.iacr.org/2017/667.pdf
// HRSSNIST:
// https://csrc.nist.gov/CSRC/media/Projects/Post-Quantum-Cryptography/documents/round-1/submissions/NTRU_HRSS_KEM.zip
// SXY: https://eprint.iacr.org/2017/1005.pdf
// NTRUTN14:
// https://assets.onboardsecurity.com/static/downloads/NTRU/resources/NTRUTech014.pdf
// NTRUCOMP: https://eprint.iacr.org/2018/1174
// SAFEGCD: https://gcd.cr.yp.to/papers.html#safegcd


// Vector operations.
//
// A couple of functions in this file can use vector operations to meaningful
// effect. If we're building for a target that has a supported vector unit,
// |HRSS_HAVE_VECTOR_UNIT| will be defined and |vec_t| will be typedefed to a
// 128-bit vector. The following functions abstract over the differences between
// NEON and SSE2 for implementing some vector operations.

// TODO: MSVC can likely also be made to work with vector operations, but ^ must
// be replaced with _mm_xor_si128, etc.
#if defined(OPENSSL_SSE2) && (defined(__clang__) || !defined(_MSC_VER))

#define HRSS_HAVE_VECTOR_UNIT
typedef __m128i vec_t;

// vec_capable returns one iff the current platform supports SSE2.
static int vec_capable(void) { return 1; }

// vec_add performs a pair-wise addition of four uint16s from |a| and |b|.
static inline vec_t vec_add(vec_t a, vec_t b) { return _mm_add_epi16(a, b); }

// vec_sub performs a pair-wise subtraction of four uint16s from |a| and |b|.
static inline vec_t vec_sub(vec_t a, vec_t b) { return _mm_sub_epi16(a, b); }

// vec_mul multiplies each uint16_t in |a| by |b| and returns the resulting
// vector.
static inline vec_t vec_mul(vec_t a, uint16_t b) {
  return _mm_mullo_epi16(a, _mm_set1_epi16(b));
}

// vec_fma multiplies each uint16_t in |b| by |c|, adds the result to |a|, and
// returns the resulting vector.
static inline vec_t vec_fma(vec_t a, vec_t b, uint16_t c) {
  return _mm_add_epi16(a, _mm_mullo_epi16(b, _mm_set1_epi16(c)));
}

// vec3_rshift_word right-shifts the 24 uint16_t's in |v| by one uint16.
static inline void vec3_rshift_word(vec_t v[3]) {
  // Intel's left and right shifting is backwards compared to the order in
  // memory because they're based on little-endian order of words (and not just
  // bytes). So the shifts in this function will be backwards from what one
  // might expect.
  const __m128i carry0 = _mm_srli_si128(v[0], 14);
  v[0] = _mm_slli_si128(v[0], 2);

  const __m128i carry1 = _mm_srli_si128(v[1], 14);
  v[1] = _mm_slli_si128(v[1], 2);
  v[1] |= carry0;

  v[2] = _mm_slli_si128(v[2], 2);
  v[2] |= carry1;
}

// vec4_rshift_word right-shifts the 32 uint16_t's in |v| by one uint16.
static inline void vec4_rshift_word(vec_t v[4]) {
  // Intel's left and right shifting is backwards compared to the order in
  // memory because they're based on little-endian order of words (and not just
  // bytes). So the shifts in this function will be backwards from what one
  // might expect.
  const __m128i carry0 = _mm_srli_si128(v[0], 14);
  v[0] = _mm_slli_si128(v[0], 2);

  const __m128i carry1 = _mm_srli_si128(v[1], 14);
  v[1] = _mm_slli_si128(v[1], 2);
  v[1] |= carry0;

  const __m128i carry2 = _mm_srli_si128(v[2], 14);
  v[2] = _mm_slli_si128(v[2], 2);
  v[2] |= carry1;

  v[3] = _mm_slli_si128(v[3], 2);
  v[3] |= carry2;
}

// vec_merge_3_5 takes the final three uint16_t's from |left|, appends the first
// five from |right|, and returns the resulting vector.
static inline vec_t vec_merge_3_5(vec_t left, vec_t right) {
  return _mm_srli_si128(left, 10) | _mm_slli_si128(right, 6);
}

// poly3_vec_lshift1 left-shifts the 768 bits in |a_s|, and in |a_a|, by one
// bit.
static inline void poly3_vec_lshift1(vec_t a_s[6], vec_t a_a[6]) {
  vec_t carry_s = {0};
  vec_t carry_a = {0};

  for (int i = 0; i < 6; i++) {
    vec_t next_carry_s = _mm_srli_epi64(a_s[i], 63);
    a_s[i] = _mm_slli_epi64(a_s[i], 1);
    a_s[i] |= _mm_slli_si128(next_carry_s, 8);
    a_s[i] |= carry_s;
    carry_s = _mm_srli_si128(next_carry_s, 8);

    vec_t next_carry_a = _mm_srli_epi64(a_a[i], 63);
    a_a[i] = _mm_slli_epi64(a_a[i], 1);
    a_a[i] |= _mm_slli_si128(next_carry_a, 8);
    a_a[i] |= carry_a;
    carry_a = _mm_srli_si128(next_carry_a, 8);
  }
}

// poly3_vec_rshift1 right-shifts the 768 bits in |a_s|, and in |a_a|, by one
// bit.
static inline void poly3_vec_rshift1(vec_t a_s[6], vec_t a_a[6]) {
  vec_t carry_s = {0};
  vec_t carry_a = {0};

  for (int i = 5; i >= 0; i--) {
    const vec_t next_carry_s = _mm_slli_epi64(a_s[i], 63);
    a_s[i] = _mm_srli_epi64(a_s[i], 1);
    a_s[i] |= _mm_srli_si128(next_carry_s, 8);
    a_s[i] |= carry_s;
    carry_s = _mm_slli_si128(next_carry_s, 8);

    const vec_t next_carry_a = _mm_slli_epi64(a_a[i], 63);
    a_a[i] = _mm_srli_epi64(a_a[i], 1);
    a_a[i] |= _mm_srli_si128(next_carry_a, 8);
    a_a[i] |= carry_a;
    carry_a = _mm_slli_si128(next_carry_a, 8);
  }
}

// vec_broadcast_bit duplicates the least-significant bit in |a| to all bits in
// a vector and returns the result.
static inline vec_t vec_broadcast_bit(vec_t a) {
  return _mm_shuffle_epi32(_mm_srai_epi32(_mm_slli_epi64(a, 63), 31), 0x55);
}

// vec_get_word returns the |i|th uint16_t in |v|. (This is a macro because the
// compiler requires that |i| be a compile-time constant.)
#define vec_get_word(v, i) _mm_extract_epi16(v, i)

#elif (defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64)) && defined(__ARM_NEON)

#define HRSS_HAVE_VECTOR_UNIT
// The Intel vector type |__m128i| is automatically aligned to a 16-byte
// boundary. But, I can't find any information about |uint16x8_t| should
// necessarily be aligned to a 16-byte boundary. We verify 16-byte alignment in
// |poly_mul_vec()|, so try to force alignment here.
typedef uint16x8_t vec_t __attribute__ ((aligned (16)));

// These functions perform the same actions as the SSE2 function of the same
// name, above.

static int vec_capable(void) { return CRYPTO_is_NEON_capable(); }

static inline vec_t vec_add(vec_t a, vec_t b) { return a + b; }

static inline vec_t vec_sub(vec_t a, vec_t b) { return a - b; }

static inline vec_t vec_mul(vec_t a, uint16_t b) { return vmulq_n_u16(a, b); }

static inline vec_t vec_fma(vec_t a, vec_t b, uint16_t c) {
  return vmlaq_n_u16(a, b, c);
}

static inline void vec3_rshift_word(vec_t v[3]) {
  const uint16x8_t kZero = {0};
  v[2] = vextq_u16(v[1], v[2], 7);
  v[1] = vextq_u16(v[0], v[1], 7);
  v[0] = vextq_u16(kZero, v[0], 7);
}

static inline void vec4_rshift_word(vec_t v[4]) {
  const uint16x8_t kZero = {0};
  v[3] = vextq_u16(v[2], v[3], 7);
  v[2] = vextq_u16(v[1], v[2], 7);
  v[1] = vextq_u16(v[0], v[1], 7);
  v[0] = vextq_u16(kZero, v[0], 7);
}

static inline vec_t vec_merge_3_5(vec_t left, vec_t right) {
  return vextq_u16(left, right, 5);
}

static inline uint16_t vec_get_word(vec_t v, unsigned i) {
  return v[i];
}

#if !defined(OPENSSL_AARCH64)

static inline vec_t vec_broadcast_bit(vec_t a) {
  a = (vec_t)vshrq_n_s16(((int16x8_t)a) << 15, 15);
  return vdupq_lane_u16(vget_low_u16(a), 0);
}

static inline void poly3_vec_lshift1(vec_t a_s[6], vec_t a_a[6]) {
  vec_t carry_s = {0};
  vec_t carry_a = {0};
  const vec_t kZero = {0};

  for (int i = 0; i < 6; i++) {
    vec_t next_carry_s = a_s[i] >> 15;
    a_s[i] <<= 1;
    a_s[i] |= vextq_u16(kZero, next_carry_s, 7);
    a_s[i] |= carry_s;
    carry_s = vextq_u16(next_carry_s, kZero, 7);

    vec_t next_carry_a = a_a[i] >> 15;
    a_a[i] <<= 1;
    a_a[i] |= vextq_u16(kZero, next_carry_a, 7);
    a_a[i] |= carry_a;
    carry_a = vextq_u16(next_carry_a, kZero, 7);
  }
}

static inline void poly3_vec_rshift1(vec_t a_s[6], vec_t a_a[6]) {
  vec_t carry_s = {0};
  vec_t carry_a = {0};
  const vec_t kZero = {0};

  for (int i = 5; i >= 0; i--) {
    vec_t next_carry_s = a_s[i] << 15;
    a_s[i] >>= 1;
    a_s[i] |= vextq_u16(next_carry_s, kZero, 1);
    a_s[i] |= carry_s;
    carry_s = vextq_u16(kZero, next_carry_s, 1);

    vec_t next_carry_a = a_a[i] << 15;
    a_a[i] >>= 1;
    a_a[i] |= vextq_u16(next_carry_a, kZero, 1);
    a_a[i] |= carry_a;
    carry_a = vextq_u16(kZero, next_carry_a, 1);
  }
}

#endif  // !OPENSSL_AARCH64

#endif  // (ARM || AARCH64) && NEON

// Polynomials in this scheme have N terms.
// #define N 701

// Underlying data types and arithmetic operations.
// ------------------------------------------------

// Binary polynomials.

// poly2 represents a degree-N polynomial over GF(2). The words are in little-
// endian order, i.e. the coefficient of x^0 is the LSB of the first word. The
// final word is only partially used since N is not a multiple of the word size.

// Defined in internal.h:
// struct poly2 {
//  crypto_word_t v[WORDS_PER_POLY];
// };

OPENSSL_UNUSED static void hexdump(const void *void_in, size_t len) {
  const uint8_t *in = (const uint8_t *)void_in;
  for (size_t i = 0; i < len; i++) {
    printf("%02x", in[i]);
  }
  printf("\n");
}

static void poly2_zero(struct poly2 *p) {
  OPENSSL_memset(&p->v[0], 0, sizeof(crypto_word_t) * WORDS_PER_POLY);
}

// word_reverse returns |in| with the bits in reverse order.
static crypto_word_t word_reverse(crypto_word_t in) {
#if defined(OPENSSL_64_BIT)
  static const crypto_word_t kMasks[6] = {
    UINT64_C(0x5555555555555555),
    UINT64_C(0x3333333333333333),
    UINT64_C(0x0f0f0f0f0f0f0f0f),
    UINT64_C(0x00ff00ff00ff00ff),
    UINT64_C(0x0000ffff0000ffff),
    UINT64_C(0x00000000ffffffff),
  };
#else
  static const crypto_word_t kMasks[5] = {
    0x55555555,
    0x33333333,
    0x0f0f0f0f,
    0x00ff00ff,
    0x0000ffff,
  };
#endif

  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kMasks); i++) {
    in = ((in >> (1 << i)) & kMasks[i]) | ((in & kMasks[i]) << (1 << i));
  }

  return in;
}

// lsb_to_all replicates the least-significant bit of |v| to all bits of the
// word. This is used in bit-slicing operations to make a vector from a fixed
// value.
static crypto_word_t lsb_to_all(crypto_word_t v) { return 0u - (v & 1); }

// poly2_mod_phiN reduces |p| by Φ(N).
static void poly2_mod_phiN(struct poly2 *p) {
  // m is the term at x^700, replicated to every bit.
  const crypto_word_t m =
      lsb_to_all(p->v[WORDS_PER_POLY - 1] >> (BITS_IN_LAST_WORD - 1));
  for (size_t i = 0; i < WORDS_PER_POLY; i++) {
    p->v[i] ^= m;
  }
  p->v[WORDS_PER_POLY - 1] &= (UINT64_C(1) << (BITS_IN_LAST_WORD - 1)) - 1;
}

// poly2_reverse_700 reverses the order of the first 700 bits of |in| and writes
// the result to |out|.
static void poly2_reverse_700(struct poly2 *out, const struct poly2 *in) {
  struct poly2 t;
  for (size_t i = 0; i < WORDS_PER_POLY; i++) {
    t.v[i] = word_reverse(in->v[i]);
  }

  static const size_t shift = BITS_PER_WORD - ((N-1) % BITS_PER_WORD);
  for (size_t i = 0; i < WORDS_PER_POLY-1; i++) {
    out->v[i] = t.v[WORDS_PER_POLY-1-i] >> shift;
    out->v[i] |= t.v[WORDS_PER_POLY-2-i] << (BITS_PER_WORD - shift);
  }
  out->v[WORDS_PER_POLY-1] = t.v[0] >> shift;
}

// poly2_cswap exchanges the values of |a| and |b| if |swap| is all ones.
static void poly2_cswap(struct poly2 *a, struct poly2 *b, crypto_word_t swap) {
  for (size_t i = 0; i < WORDS_PER_POLY; i++) {
    const crypto_word_t sum = swap & (a->v[i] ^ b->v[i]);
    a->v[i] ^= sum;
    b->v[i] ^= sum;
  }
}

// poly2_fmadd sets |out| to |out| + |in| * m, where m is either
// |CONSTTIME_TRUE_W| or |CONSTTIME_FALSE_W|.
static void poly2_fmadd(struct poly2 *out, const struct poly2 *in,
                        crypto_word_t m) {
  for (size_t i = 0; i < WORDS_PER_POLY; i++) {
    out->v[i] ^= in->v[i] & m;
  }
}

// poly2_lshift1 left-shifts |p| by one bit.
static void poly2_lshift1(struct poly2 *p) {
  crypto_word_t carry = 0;
  for (size_t i = 0; i < WORDS_PER_POLY; i++) {
    const crypto_word_t next_carry = p->v[i] >> (BITS_PER_WORD - 1);
    p->v[i] <<= 1;
    p->v[i] |= carry;
    carry = next_carry;
  }
}

// poly2_rshift1 right-shifts |p| by one bit.
static void poly2_rshift1(struct poly2 *p) {
  crypto_word_t carry = 0;
  for (size_t i = WORDS_PER_POLY - 1; i < WORDS_PER_POLY; i--) {
    const crypto_word_t next_carry = p->v[i] & 1;
    p->v[i] >>= 1;
    p->v[i] |= carry << (BITS_PER_WORD - 1);
    carry = next_carry;
  }
}

// poly2_clear_top_bits clears the bits in the final word that are only for
// alignment.
static void poly2_clear_top_bits(struct poly2 *p) {
  p->v[WORDS_PER_POLY - 1] &= (UINT64_C(1) << BITS_IN_LAST_WORD) - 1;
}

// poly2_top_bits_are_clear returns one iff the extra bits in the final words of
// |p| are zero.
static int poly2_top_bits_are_clear(const struct poly2 *p) {
  return (p->v[WORDS_PER_POLY - 1] &
          ~((UINT64_C(1) << BITS_IN_LAST_WORD) - 1)) == 0;
}

// Ternary polynomials.

// poly3 represents a degree-N polynomial over GF(3). Each coefficient is
// bitsliced across the |s| and |a| arrays, like this:
//
//   s  |  a  | value
//  -----------------
//   0  |  0  | 0
//   0  |  1  | 1
//   1  |  1  | -1 (aka 2)
//   1  |  0  | <invalid>
//
// ('s' is for sign, and 'a' is the absolute value.)
//
// Once bitsliced as such, the following circuits can be used to implement
// addition and multiplication mod 3:
//
//   (s3, a3) = (s1, a1) × (s2, a2)
//   a3 = a1 ∧ a2
//   s3 = (s1 ⊕ s2) ∧ a3
//
//   (s3, a3) = (s1, a1) + (s2, a2)
//   t = s1 ⊕ a2
//   s3 = t ∧ (s2 ⊕ a1)
//   a3 = (a1 ⊕ a2) ∨ (t ⊕ s2)
//
//   (s3, a3) = (s1, a1) - (s2, a2)
//   t = a1 ⊕ a2
//   s3 = (s1 ⊕ a2) ∧ (t ⊕ s2)
//   a3 = t ∨ (s1 ⊕ s2)
//
// Negating a value just involves XORing s by a.
//
// struct poly3 {
//   struct poly2 s, a;
// };

OPENSSL_UNUSED static void poly3_print(const struct poly3 *in) {
  struct poly3 p;
  OPENSSL_memcpy(&p, in, sizeof(p));
  p.s.v[WORDS_PER_POLY - 1] &= ((crypto_word_t)1 << BITS_IN_LAST_WORD) - 1;
  p.a.v[WORDS_PER_POLY - 1] &= ((crypto_word_t)1 << BITS_IN_LAST_WORD) - 1;

  printf("{[");
  for (unsigned i = 0; i < WORDS_PER_POLY; i++) {
    if (i) {
      printf(" ");
    }
    printf(BN_HEX_FMT2, p.s.v[i]);
  }
  printf("] [");
  for (unsigned i = 0; i < WORDS_PER_POLY; i++) {
    if (i) {
      printf(" ");
    }
    printf(BN_HEX_FMT2, p.a.v[i]);
  }
  printf("]}\n");
}

static void poly3_zero(struct poly3 *p) {
  poly2_zero(&p->s);
  poly2_zero(&p->a);
}

// poly3_reverse_700 reverses the order of the first 700 terms of |in| and
// writes them to |out|.
static void poly3_reverse_700(struct poly3 *out, const struct poly3 *in) {
  poly2_reverse_700(&out->a, &in->a);
  poly2_reverse_700(&out->s, &in->s);
}

// poly3_word_mul sets (|out_s|, |out_a|) to (|s1|, |a1|) × (|s2|, |a2|).
static void poly3_word_mul(crypto_word_t *out_s, crypto_word_t *out_a,
                           const crypto_word_t s1, const crypto_word_t a1,
                           const crypto_word_t s2, const crypto_word_t a2) {
  *out_a = a1 & a2;
  *out_s = (s1 ^ s2) & *out_a;
}

// poly3_word_add sets (|out_s|, |out_a|) to (|s1|, |a1|) + (|s2|, |a2|).
static void poly3_word_add(crypto_word_t *out_s, crypto_word_t *out_a,
                           const crypto_word_t s1, const crypto_word_t a1,
                           const crypto_word_t s2, const crypto_word_t a2) {
  const crypto_word_t t = s1 ^ a2;
  *out_s = t & (s2 ^ a1);
  *out_a = (a1 ^ a2) | (t ^ s2);
}

// poly3_word_sub sets (|out_s|, |out_a|) to (|s1|, |a1|) - (|s2|, |a2|).
static void poly3_word_sub(crypto_word_t *out_s, crypto_word_t *out_a,
                           const crypto_word_t s1, const crypto_word_t a1,
                           const crypto_word_t s2, const crypto_word_t a2) {
  const crypto_word_t t = a1 ^ a2;
  *out_s = (s1 ^ a2) & (t ^ s2);
  *out_a = t | (s1 ^ s2);
}

// poly3_mul_const sets |p| to |p|×m, where m = (ms, ma).
static void poly3_mul_const(struct poly3 *p, crypto_word_t ms,
                            crypto_word_t ma) {
  ms = lsb_to_all(ms);
  ma = lsb_to_all(ma);

  for (size_t i = 0; i < WORDS_PER_POLY; i++) {
    poly3_word_mul(&p->s.v[i], &p->a.v[i], p->s.v[i], p->a.v[i], ms, ma);
  }
}

// poly3_fmadd sets |out| to |out| - |in|×m, where m is (ms, ma).
static void poly3_fmsub(struct poly3 *RESTRICT out,
                        const struct poly3 *RESTRICT in, crypto_word_t ms,
                        crypto_word_t ma) {
  crypto_word_t product_s, product_a;
  for (size_t i = 0; i < WORDS_PER_POLY; i++) {
    poly3_word_mul(&product_s, &product_a, in->s.v[i], in->a.v[i], ms, ma);
    poly3_word_sub(&out->s.v[i], &out->a.v[i], out->s.v[i], out->a.v[i],
                   product_s, product_a);
  }
}

// final_bit_to_all replicates the bit in the final position of the last word to
// all the bits in the word.
static crypto_word_t final_bit_to_all(crypto_word_t v) {
  return lsb_to_all(v >> (BITS_IN_LAST_WORD - 1));
}

// poly3_top_bits_are_clear returns one iff the extra bits in the final words of
// |p| are zero.
OPENSSL_UNUSED static int poly3_top_bits_are_clear(const struct poly3 *p) {
  return poly2_top_bits_are_clear(&p->s) && poly2_top_bits_are_clear(&p->a);
}

// poly3_mod_phiN reduces |p| by Φ(N).
static void poly3_mod_phiN(struct poly3 *p) {
  // In order to reduce by Φ(N) we subtract by the value of the greatest
  // coefficient.
  const crypto_word_t factor_s = final_bit_to_all(p->s.v[WORDS_PER_POLY - 1]);
  const crypto_word_t factor_a = final_bit_to_all(p->a.v[WORDS_PER_POLY - 1]);

  for (size_t i = 0; i < WORDS_PER_POLY; i++) {
    poly3_word_sub(&p->s.v[i], &p->a.v[i], p->s.v[i], p->a.v[i], factor_s,
                   factor_a);
  }

  poly2_clear_top_bits(&p->s);
  poly2_clear_top_bits(&p->a);
}

static void poly3_cswap(struct poly3 *a, struct poly3 *b, crypto_word_t swap) {
  poly2_cswap(&a->s, &b->s, swap);
  poly2_cswap(&a->a, &b->a, swap);
}

static void poly3_lshift1(struct poly3 *p) {
  poly2_lshift1(&p->s);
  poly2_lshift1(&p->a);
}

static void poly3_rshift1(struct poly3 *p) {
  poly2_rshift1(&p->s);
  poly2_rshift1(&p->a);
}

// poly3_span represents a pointer into a poly3.
struct poly3_span {
  crypto_word_t *s;
  crypto_word_t *a;
};

// poly3_span_add adds |n| words of values from |a| and |b| and writes the
// result to |out|.
static void poly3_span_add(const struct poly3_span *out,
                           const struct poly3_span *a,
                           const struct poly3_span *b, size_t n) {
  for (size_t i = 0; i < n; i++) {
    poly3_word_add(&out->s[i], &out->a[i], a->s[i], a->a[i], b->s[i], b->a[i]);
  }
}

// poly3_span_sub subtracts |n| words of |b| from |n| words of |a|.
static void poly3_span_sub(const struct poly3_span *a,
                           const struct poly3_span *b, size_t n) {
  for (size_t i = 0; i < n; i++) {
    poly3_word_sub(&a->s[i], &a->a[i], a->s[i], a->a[i], b->s[i], b->a[i]);
  }
}

// poly3_mul_aux is a recursive function that multiplies |n| words from |a| and
// |b| and writes 2×|n| words to |out|. Each call uses 2*ceil(n/2) elements of
// |scratch| and the function recurses, except if |n| == 1, when |scratch| isn't
// used and the recursion stops. For |n| in {11, 22}, the transitive total
// amount of |scratch| needed happens to be 2n+2.
static void poly3_mul_aux(const struct poly3_span *out,
                          const struct poly3_span *scratch,
                          const struct poly3_span *a,
                          const struct poly3_span *b, size_t n) {
  if (n == 1) {
    crypto_word_t r_s_low = 0, r_s_high = 0, r_a_low = 0, r_a_high = 0;
    crypto_word_t b_s = b->s[0], b_a = b->a[0];
    const crypto_word_t a_s = a->s[0], a_a = a->a[0];

    for (size_t i = 0; i < BITS_PER_WORD; i++) {
      // Multiply (s, a) by the next value from (b_s, b_a).
      crypto_word_t m_s, m_a;
      poly3_word_mul(&m_s, &m_a, a_s, a_a, lsb_to_all(b_s), lsb_to_all(b_a));
      b_s >>= 1;
      b_a >>= 1;

      if (i == 0) {
        // Special case otherwise the code tries to shift by BITS_PER_WORD
        // below, which is undefined.
        r_s_low = m_s;
        r_a_low = m_a;
        continue;
      }

      // Shift the multiplication result to the correct position.
      const crypto_word_t m_s_low = m_s << i;
      const crypto_word_t m_s_high = m_s >> (BITS_PER_WORD - i);
      const crypto_word_t m_a_low = m_a << i;
      const crypto_word_t m_a_high = m_a >> (BITS_PER_WORD - i);

      // Add into the result.
      poly3_word_add(&r_s_low, &r_a_low, r_s_low, r_a_low, m_s_low, m_a_low);
      poly3_word_add(&r_s_high, &r_a_high, r_s_high, r_a_high, m_s_high,
                     m_a_high);
    }

    out->s[0] = r_s_low;
    out->s[1] = r_s_high;
    out->a[0] = r_a_low;
    out->a[1] = r_a_high;
    return;
  }

  // Karatsuba multiplication.
  // https://en.wikipedia.org/wiki/Karatsuba_algorithm

  // When |n| is odd, the two "halves" will have different lengths. The first
  // is always the smaller.
  const size_t low_len = n / 2;
  const size_t high_len = n - low_len;
  const struct poly3_span a_high = {&a->s[low_len], &a->a[low_len]};
  const struct poly3_span b_high = {&b->s[low_len], &b->a[low_len]};

  // Store a_1 + a_0 in the first half of |out| and b_1 + b_0 in the second
  // half.
  const struct poly3_span a_cross_sum = *out;
  const struct poly3_span b_cross_sum = {&out->s[high_len], &out->a[high_len]};
  poly3_span_add(&a_cross_sum, a, &a_high, low_len);
  poly3_span_add(&b_cross_sum, b, &b_high, low_len);
  if (high_len != low_len) {
    a_cross_sum.s[low_len] = a_high.s[low_len];
    a_cross_sum.a[low_len] = a_high.a[low_len];
    b_cross_sum.s[low_len] = b_high.s[low_len];
    b_cross_sum.a[low_len] = b_high.a[low_len];
  }

  const struct poly3_span child_scratch = {&scratch->s[2 * high_len],
                                           &scratch->a[2 * high_len]};
  const struct poly3_span out_mid = {&out->s[low_len], &out->a[low_len]};
  const struct poly3_span out_high = {&out->s[2 * low_len],
                                      &out->a[2 * low_len]};

  // Calculate (a_1 + a_0) × (b_1 + b_0) and write to scratch buffer.
  poly3_mul_aux(scratch, &child_scratch, &a_cross_sum, &b_cross_sum, high_len);
  // Calculate a_1 × b_1.
  poly3_mul_aux(&out_high, &child_scratch, &a_high, &b_high, high_len);
  // Calculate a_0 × b_0.
  poly3_mul_aux(out, &child_scratch, a, b, low_len);

  // Subtract those last two products from the first.
  poly3_span_sub(scratch, out, low_len * 2);
  poly3_span_sub(scratch, &out_high, high_len * 2);

  // Add the middle product into the output.
  poly3_span_add(&out_mid, &out_mid, scratch, high_len * 2);
}

// HRSS_poly3_mul sets |*out| to |x|×|y| mod Φ(N).
void HRSS_poly3_mul(struct poly3 *out, const struct poly3 *x,
                    const struct poly3 *y) {
  crypto_word_t prod_s[WORDS_PER_POLY * 2];
  crypto_word_t prod_a[WORDS_PER_POLY * 2];
  crypto_word_t scratch_s[WORDS_PER_POLY * 2 + 2];
  crypto_word_t scratch_a[WORDS_PER_POLY * 2 + 2];
  const struct poly3_span prod_span = {prod_s, prod_a};
  const struct poly3_span scratch_span = {scratch_s, scratch_a};
  const struct poly3_span x_span = {(crypto_word_t *)x->s.v,
                                    (crypto_word_t *)x->a.v};
  const struct poly3_span y_span = {(crypto_word_t *)y->s.v,
                                    (crypto_word_t *)y->a.v};

  poly3_mul_aux(&prod_span, &scratch_span, &x_span, &y_span, WORDS_PER_POLY);

  // |prod| needs to be reduced mod (𝑥^n - 1), which just involves adding the
  // upper-half to the lower-half. However, N is 701, which isn't a multiple of
  // BITS_PER_WORD, so the upper-half vectors all have to be shifted before
  // being added to the lower-half.
  for (size_t i = 0; i < WORDS_PER_POLY; i++) {
    crypto_word_t v_s = prod_s[WORDS_PER_POLY + i - 1] >> BITS_IN_LAST_WORD;
    v_s |= prod_s[WORDS_PER_POLY + i] << (BITS_PER_WORD - BITS_IN_LAST_WORD);
    crypto_word_t v_a = prod_a[WORDS_PER_POLY + i - 1] >> BITS_IN_LAST_WORD;
    v_a |= prod_a[WORDS_PER_POLY + i] << (BITS_PER_WORD - BITS_IN_LAST_WORD);

    poly3_word_add(&out->s.v[i], &out->a.v[i], prod_s[i], prod_a[i], v_s, v_a);
  }

  poly3_mod_phiN(out);
}

#if defined(HRSS_HAVE_VECTOR_UNIT) && !defined(OPENSSL_AARCH64)

// poly3_vec_cswap swaps (|a_s|, |a_a|) and (|b_s|, |b_a|) if |swap| is
// |0xff..ff|. Otherwise, |swap| must be zero.
static inline void poly3_vec_cswap(vec_t a_s[6], vec_t a_a[6], vec_t b_s[6],
                                   vec_t b_a[6], const vec_t swap) {
  for (int i = 0; i < 6; i++) {
    const vec_t sum_s = swap & (a_s[i] ^ b_s[i]);
    a_s[i] ^= sum_s;
    b_s[i] ^= sum_s;

    const vec_t sum_a = swap & (a_a[i] ^ b_a[i]);
    a_a[i] ^= sum_a;
    b_a[i] ^= sum_a;
  }
}

// poly3_vec_fmsub subtracts (|ms|, |ma|) × (|b_s|, |b_a|) from (|a_s|, |a_a|).
static inline void poly3_vec_fmsub(vec_t a_s[6], vec_t a_a[6], vec_t b_s[6],
                                   vec_t b_a[6], const vec_t ms,
                                   const vec_t ma) {
  for (int i = 0; i < 6; i++) {
    // See the bitslice formula, above.
    const vec_t s = b_s[i];
    const vec_t a = b_a[i];
    const vec_t product_a = a & ma;
    const vec_t product_s = (s ^ ms) & product_a;

    const vec_t out_s = a_s[i];
    const vec_t out_a = a_a[i];
    const vec_t t = out_a ^ product_a;
    a_s[i] = (out_s ^ product_a) & (t ^ product_s);
    a_a[i] = t | (out_s ^ product_s);
  }
}

// poly3_invert_vec sets |*out| to |in|^-1, i.e. such that |out|×|in| == 1 mod
// Φ(N).
static void poly3_invert_vec(struct poly3 *out, const struct poly3 *in) {
  // This algorithm is taken from section 7.1 of [SAFEGCD].
  const vec_t kZero = {0};
  const vec_t kOne = {1};
  static const uint8_t kBottomSixtyOne[sizeof(vec_t)] = {
      0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x1f};

  vec_t v_s[6], v_a[6], r_s[6], r_a[6], f_s[6], f_a[6], g_s[6], g_a[6];
  // v = 0
  memset(&v_s, 0, sizeof(v_s));
  memset(&v_a, 0, sizeof(v_a));
  // r = 1
  memset(&r_s, 0, sizeof(r_s));
  memset(&r_a, 0, sizeof(r_a));
  r_a[0] = kOne;
  // f = all ones.
  memset(f_s, 0, sizeof(f_s));
  memset(f_a, 0xff, 5 * sizeof(vec_t));
  memcpy(&f_a[5], kBottomSixtyOne, sizeof(kBottomSixtyOne));
  // g is the reversal of |in|.
  struct poly3 in_reversed;
  poly3_reverse_700(&in_reversed, in);
  g_s[5] = kZero;
  memcpy(&g_s, &in_reversed.s.v, WORDS_PER_POLY * sizeof(crypto_word_t));
  g_a[5] = kZero;
  memcpy(&g_a, &in_reversed.a.v, WORDS_PER_POLY * sizeof(crypto_word_t));

  int delta = 1;

  for (size_t i = 0; i < (2*(N-1)) - 1; i++) {
    poly3_vec_lshift1(v_s, v_a);

    const crypto_word_t delta_sign_bit = (delta >> (sizeof(delta) * 8 - 1)) & 1;
    const crypto_word_t delta_is_non_negative = delta_sign_bit - 1;
    const crypto_word_t delta_is_non_zero = ~constant_time_is_zero_w(delta);
    const vec_t g_has_constant_term = vec_broadcast_bit(g_a[0]);
    const vec_t mask_w =
        {delta_is_non_negative & delta_is_non_zero};
    const vec_t mask = vec_broadcast_bit(mask_w) & g_has_constant_term;

    const vec_t c_a = vec_broadcast_bit(f_a[0] & g_a[0]);
    const vec_t c_s = vec_broadcast_bit((f_s[0] ^ g_s[0]) & c_a);

    // SSE2 is an Intel thing. So, checking for |OPENSSL_SSE2| should avoid
    // hitting the Arm specific path under the #else.
#if defined(OPENSSL_SSE2)
    // This is necessary because older versions of GCC, such as version 4.1.2,
    // do not support accessing individual elements of the __m128i type
    alignas(16) uint64_t mask_tmp[2];
    _mm_store_si128((void*) mask_tmp, mask);
    delta = constant_time_select_int(lsb_to_all(mask_tmp[0]), -delta, delta);
#else
    delta = constant_time_select_int(lsb_to_all(mask[0]), -delta, delta);
#endif
    delta++;

    poly3_vec_cswap(f_s, f_a, g_s, g_a, mask);
    poly3_vec_fmsub(g_s, g_a, f_s, f_a, c_s, c_a);
    poly3_vec_rshift1(g_s, g_a);

    poly3_vec_cswap(v_s, v_a, r_s, r_a, mask);
    poly3_vec_fmsub(r_s, r_a, v_s, v_a, c_s, c_a);
  }

  assert(delta == 0);
  memcpy(out->s.v, v_s, WORDS_PER_POLY * sizeof(crypto_word_t));
  memcpy(out->a.v, v_a, WORDS_PER_POLY * sizeof(crypto_word_t));
  poly3_mul_const(out, vec_get_word(f_s[0], 0), vec_get_word(f_a[0], 0));
  poly3_reverse_700(out, out);
}

#endif  // HRSS_HAVE_VECTOR_UNIT

// HRSS_poly3_invert sets |*out| to |in|^-1, i.e. such that |out|×|in| == 1 mod
// Φ(N).
void HRSS_poly3_invert(struct poly3 *out, const struct poly3 *in) {
  // The vector version of this function seems slightly slower on AArch64, but
  // is useful on ARMv7 and x86-64.
#if defined(HRSS_HAVE_VECTOR_UNIT) && !defined(OPENSSL_AARCH64)
  if (vec_capable()) {
    poly3_invert_vec(out, in);
    return;
  }
#endif

  // This algorithm is taken from section 7.1 of [SAFEGCD].
  struct poly3 v, r, f, g;
  // v = 0
  poly3_zero(&v);
  // r = 1
  poly3_zero(&r);
  r.a.v[0] = 1;
  // f = all ones.
  OPENSSL_memset(&f.s, 0, sizeof(struct poly2));
  OPENSSL_memset(&f.a, 0xff, sizeof(struct poly2));
  f.a.v[WORDS_PER_POLY - 1] >>= BITS_PER_WORD - BITS_IN_LAST_WORD;
  // g is the reversal of |in|.
  poly3_reverse_700(&g, in);
  int delta = 1;

  for (size_t i = 0; i < (2*(N-1)) - 1; i++) {
    poly3_lshift1(&v);

    const crypto_word_t delta_sign_bit = (delta >> (sizeof(delta) * 8 - 1)) & 1;
    const crypto_word_t delta_is_non_negative = delta_sign_bit - 1;
    const crypto_word_t delta_is_non_zero = ~constant_time_is_zero_w(delta);
    const crypto_word_t g_has_constant_term = lsb_to_all(g.a.v[0]);
    const crypto_word_t mask =
        g_has_constant_term & delta_is_non_negative & delta_is_non_zero;

    crypto_word_t c_s, c_a;
    poly3_word_mul(&c_s, &c_a, f.s.v[0], f.a.v[0], g.s.v[0], g.a.v[0]);
    c_s = lsb_to_all(c_s);
    c_a = lsb_to_all(c_a);

    delta = constant_time_select_int(mask, -delta, delta);
    delta++;

    poly3_cswap(&f, &g, mask);
    poly3_fmsub(&g, &f, c_s, c_a);
    poly3_rshift1(&g);

    poly3_cswap(&v, &r, mask);
    poly3_fmsub(&r, &v, c_s, c_a);
  }

  assert(delta == 0);
  poly3_mul_const(&v, f.s.v[0], f.a.v[0]);
  poly3_reverse_700(out, &v);
}

// Polynomials in Q.

// Coefficients are reduced mod Q. (Q is clearly not prime, therefore the
// coefficients do not form a field.)
#define Q 8192

// VECS_PER_POLY is the number of 128-bit vectors needed to represent a
// polynomial.
#define COEFFICIENTS_PER_VEC (sizeof(vec_t) / sizeof(uint16_t))
#define VECS_PER_POLY ((N + COEFFICIENTS_PER_VEC - 1) / COEFFICIENTS_PER_VEC)

// poly represents a polynomial with coefficients mod Q. Note that, while Q is a
// power of two, this does not operate in GF(Q). That would be a binary field
// but this is simply mod Q. Thus the coefficients are not a field.
//
// Coefficients are ordered little-endian, thus the coefficient of x^0 is the
// first element of the array.
struct poly {
  alignas(16) uint16_t v[N+3];
};

#if defined(HRSS_HAVE_VECTOR_UNIT)
struct poly_vec {
  // N + 3 = 704, which is a multiple of 64 and thus aligns things, esp for
  // the vector code.
  vec_t vectors[VECS_PER_POLY];
};

static void poly_vec2poly(struct poly *p, const struct poly_vec *pv)
{
  OPENSSL_memcpy(p, pv, sizeof(*p));
}
static void poly2poly_vec(struct poly_vec *pv, const struct poly *p)
{
  OPENSSL_memcpy(pv, p, sizeof(*p));
}
#endif

// poly_normalize zeros out the excess elements of |x| which are included only
// for alignment.
static void poly_normalize(struct poly *x) {
  OPENSSL_memset(&x->v[N], 0, 3 * sizeof(uint16_t));
}

// poly_assert_normalized asserts that the excess elements of |x| are zeroed out
// for the cases that case. (E.g. |poly_mul_vec|.)
static void poly_assert_normalized(const struct poly *x) {
  assert(x->v[N] == 0);
  assert(x->v[N + 1] == 0);
  assert(x->v[N + 2] == 0);
}

OPENSSL_UNUSED static void poly_print(const struct poly *p) {
  printf("[");
  for (unsigned i = 0; i < N; i++) {
    if (i) {
      printf(" ");
    }
    printf("%d", p->v[i]);
  }
  printf("]\n");
}

// POLY_MUL_SCRATCH contains space for the working variables needed by
// |poly_mul|. The contents afterwards may be discarded, but the object may also
// be reused with future |poly_mul| calls to save heap allocations.
//
// This object must have 32-byte alignment.
struct POLY_MUL_SCRATCH {
  union {
    // This is used by |poly_mul_novec|.
    struct {
      uint16_t prod[2 * N];
      uint16_t scratch[1318];
    } novec;

#if defined(HRSS_HAVE_VECTOR_UNIT)
    // This is used by |poly_mul_vec|.
    struct {
      vec_t prod[VECS_PER_POLY * 2];
      vec_t scratch[172];
    } vec;
#endif

#if defined(POLY_RQ_MUL_ASM)
    // This is the space used by |poly_Rq_mul|.
    uint8_t rq[POLY_MUL_RQ_SCRATCH_SPACE];
#endif
  } u;
};

#if defined(HRSS_HAVE_VECTOR_UNIT)

// poly_mul_vec_aux is a recursive function that multiplies |n| words from |a|
// and |b| and writes 2×|n| words to |out|. Each call uses 2*ceil(n/2) elements
// of |scratch| and the function recurses, except if |n| < 3, when |scratch|
// isn't used and the recursion stops. If |n| == |VECS_PER_POLY| then |scratch|
// needs 172 elements.
static void poly_mul_vec_aux(vec_t *restrict out, vec_t *restrict scratch,
                             const vec_t *restrict a, const vec_t *restrict b,
                             const size_t n) {
  // In [HRSS], the technique they used for polynomial multiplication is
  // described: they start with Toom-4 at the top level and then two layers of
  // Karatsuba. Karatsuba is a specific instance of the general Toom–Cook
  // decomposition, which splits an input n-ways and produces 2n-1
  // multiplications of those parts. So, starting with 704 coefficients (rounded
  // up from 701 to have more factors of two), Toom-4 gives seven
  // multiplications of degree-174 polynomials. Each round of Karatsuba (which
  // is Toom-2) increases the number of multiplications by a factor of three
  // while halving the size of the values being multiplied. So two rounds gives
  // 63 multiplications of degree-44 polynomials. Then they (I think) form
  // vectors by gathering all 63 coefficients of each power together, for each
  // input, and doing more rounds of Karatsuba on the vectors until they bottom-
  // out somewhere with schoolbook multiplication.
  //
  // I tried something like that for NEON. NEON vectors are 128 bits so hold
  // eight coefficients. I wrote a function that did Karatsuba on eight
  // multiplications at the same time, using such vectors, and a Go script that
  // decomposed from degree-704, with Karatsuba in non-transposed form, until it
  // reached multiplications of degree-44. It batched up those 81
  // multiplications into lots of eight with a single one left over (which was
  // handled directly).
  //
  // It worked, but it was significantly slower than the dumb algorithm used
  // below. Potentially that was because I misunderstood how [HRSS] did it, or
  // because Clang is bad at generating good code from NEON intrinsics on ARMv7.
  // (Which is true: the code generated by Clang for the below is pretty crap.)
  //
  // This algorithm is much simpler. It just does Karatsuba decomposition all
  // the way down and never transposes. When it gets down to degree-16 or
  // degree-24 values, they are multiplied using schoolbook multiplication and
  // vector intrinsics. The vector operations form each of the eight phase-
  // shifts of one of the inputs, point-wise multiply, and then add into the
  // result at the correct place. This means that 33% (degree-16) or 25%
  // (degree-24) of the multiplies and adds are wasted, but it does ok.
  if (n == 2) {
    vec_t result[4];
    vec_t vec_a[3];
    static const vec_t kZero = {0};
    vec_a[0] = a[0];
    vec_a[1] = a[1];
    vec_a[2] = kZero;

    result[0] = vec_mul(vec_a[0], vec_get_word(b[0], 0));
    result[1] = vec_mul(vec_a[1], vec_get_word(b[0], 0));

    result[1] = vec_fma(result[1], vec_a[0], vec_get_word(b[1], 0));
    result[2] = vec_mul(vec_a[1], vec_get_word(b[1], 0));
    result[3] = kZero;

    vec3_rshift_word(vec_a);

#define BLOCK(x, y)                                                      \
  do {                                                                   \
    result[x + 0] =                                                      \
        vec_fma(result[x + 0], vec_a[0], vec_get_word(b[y / 8], y % 8)); \
    result[x + 1] =                                                      \
        vec_fma(result[x + 1], vec_a[1], vec_get_word(b[y / 8], y % 8)); \
    result[x + 2] =                                                      \
        vec_fma(result[x + 2], vec_a[2], vec_get_word(b[y / 8], y % 8)); \
  } while (0)

    BLOCK(0, 1);
    BLOCK(1, 9);

    vec3_rshift_word(vec_a);

    BLOCK(0, 2);
    BLOCK(1, 10);

    vec3_rshift_word(vec_a);

    BLOCK(0, 3);
    BLOCK(1, 11);

    vec3_rshift_word(vec_a);

    BLOCK(0, 4);
    BLOCK(1, 12);

    vec3_rshift_word(vec_a);

    BLOCK(0, 5);
    BLOCK(1, 13);

    vec3_rshift_word(vec_a);

    BLOCK(0, 6);
    BLOCK(1, 14);

    vec3_rshift_word(vec_a);

    BLOCK(0, 7);
    BLOCK(1, 15);

#undef BLOCK

    memcpy(out, result, sizeof(result));
    return;
  }

  if (n == 3) {
    vec_t result[6];
    vec_t vec_a[4];
    static const vec_t kZero = {0};
    vec_a[0] = a[0];
    vec_a[1] = a[1];
    vec_a[2] = a[2];
    vec_a[3] = kZero;

    result[0] = vec_mul(a[0], vec_get_word(b[0], 0));
    result[1] = vec_mul(a[1], vec_get_word(b[0], 0));
    result[2] = vec_mul(a[2], vec_get_word(b[0], 0));

#define BLOCK_PRE(x, y)                                                  \
  do {                                                                   \
    result[x + 0] =                                                      \
        vec_fma(result[x + 0], vec_a[0], vec_get_word(b[y / 8], y % 8)); \
    result[x + 1] =                                                      \
        vec_fma(result[x + 1], vec_a[1], vec_get_word(b[y / 8], y % 8)); \
    result[x + 2] = vec_mul(vec_a[2], vec_get_word(b[y / 8], y % 8));    \
  } while (0)

    BLOCK_PRE(1, 8);
    BLOCK_PRE(2, 16);

    result[5] = kZero;

    vec4_rshift_word(vec_a);

#define BLOCK(x, y)                                                      \
  do {                                                                   \
    result[x + 0] =                                                      \
        vec_fma(result[x + 0], vec_a[0], vec_get_word(b[y / 8], y % 8)); \
    result[x + 1] =                                                      \
        vec_fma(result[x + 1], vec_a[1], vec_get_word(b[y / 8], y % 8)); \
    result[x + 2] =                                                      \
        vec_fma(result[x + 2], vec_a[2], vec_get_word(b[y / 8], y % 8)); \
    result[x + 3] =                                                      \
        vec_fma(result[x + 3], vec_a[3], vec_get_word(b[y / 8], y % 8)); \
  } while (0)

    BLOCK(0, 1);
    BLOCK(1, 9);
    BLOCK(2, 17);

    vec4_rshift_word(vec_a);

    BLOCK(0, 2);
    BLOCK(1, 10);
    BLOCK(2, 18);

    vec4_rshift_word(vec_a);

    BLOCK(0, 3);
    BLOCK(1, 11);
    BLOCK(2, 19);

    vec4_rshift_word(vec_a);

    BLOCK(0, 4);
    BLOCK(1, 12);
    BLOCK(2, 20);

    vec4_rshift_word(vec_a);

    BLOCK(0, 5);
    BLOCK(1, 13);
    BLOCK(2, 21);

    vec4_rshift_word(vec_a);

    BLOCK(0, 6);
    BLOCK(1, 14);
    BLOCK(2, 22);

    vec4_rshift_word(vec_a);

    BLOCK(0, 7);
    BLOCK(1, 15);
    BLOCK(2, 23);

#undef BLOCK
#undef BLOCK_PRE

    memcpy(out, result, sizeof(result));

    return;
  }

  // Karatsuba multiplication.
  // https://en.wikipedia.org/wiki/Karatsuba_algorithm

  // When |n| is odd, the two "halves" will have different lengths. The first is
  // always the smaller.
  const size_t low_len = n / 2;
  const size_t high_len = n - low_len;
  const vec_t *a_high = &a[low_len];
  const vec_t *b_high = &b[low_len];

  // Store a_1 + a_0 in the first half of |out| and b_1 + b_0 in the second
  // half.
  for (size_t i = 0; i < low_len; i++) {
    out[i] = vec_add(a_high[i], a[i]);
    out[high_len + i] = vec_add(b_high[i], b[i]);
  }
  if (high_len != low_len) {
    out[low_len] = a_high[low_len];
    out[high_len + low_len] = b_high[low_len];
  }

  vec_t *const child_scratch = &scratch[2 * high_len];
  // Calculate (a_1 + a_0) × (b_1 + b_0) and write to scratch buffer.
  poly_mul_vec_aux(scratch, child_scratch, out, &out[high_len], high_len);
  // Calculate a_1 × b_1.
  poly_mul_vec_aux(&out[low_len * 2], child_scratch, a_high, b_high, high_len);
  // Calculate a_0 × b_0.
  poly_mul_vec_aux(out, child_scratch, a, b, low_len);

  // Subtract those last two products from the first.
  for (size_t i = 0; i < low_len * 2; i++) {
    scratch[i] = vec_sub(scratch[i], vec_add(out[i], out[low_len * 2 + i]));
  }
  if (low_len != high_len) {
    scratch[low_len * 2] = vec_sub(scratch[low_len * 2], out[low_len * 4]);
    scratch[low_len * 2 + 1] =
        vec_sub(scratch[low_len * 2 + 1], out[low_len * 4 + 1]);
  }

  // Add the middle product into the output.
  for (size_t i = 0; i < high_len * 2; i++) {
    out[low_len + i] = vec_add(out[low_len + i], scratch[i]);
  }
}

// poly_mul_vec sets |*out| to |x|×|y| mod (𝑥^n - 1).
static void poly_mul_vec(struct POLY_MUL_SCRATCH *scratch, struct poly *out,
                         const struct poly *x, const struct poly *y) {
  OPENSSL_STATIC_ASSERT(sizeof(out->v) == sizeof(vec_t) * VECS_PER_POLY,
                        struct_poly_is_the_wrong_size)
  OPENSSL_STATIC_ASSERT(alignof(struct poly) == alignof(vec_t),
                        struct_poly_has_incorrect_alignment)

  poly_assert_normalized(x);
  poly_assert_normalized(y);

  struct poly_vec x_vec = {{{0}}};
  struct poly_vec y_vec = {{{0}}};
  poly2poly_vec(&x_vec, x);
  poly2poly_vec(&y_vec, y);

  vec_t *const prod = scratch->u.vec.prod;
  vec_t *const aux_scratch = scratch->u.vec.scratch;
  poly_mul_vec_aux(prod, aux_scratch, x_vec.vectors, y_vec.vectors, VECS_PER_POLY);

  // |prod| needs to be reduced mod (𝑥^n - 1), which just involves adding the
  // upper-half to the lower-half. However, N is 701, which isn't a multiple of
  // the vector size, so the upper-half vectors all have to be shifted before
  // being added to the lower-half.
  struct poly_vec out_vecs = {{{0}}};

  for (size_t i = 0; i < VECS_PER_POLY; i++) {
    const vec_t prev = prod[VECS_PER_POLY - 1 + i];
    const vec_t this = prod[VECS_PER_POLY + i];
    out_vecs.vectors[i] = vec_add(prod[i], vec_merge_3_5(prev, this));
  }

  poly_vec2poly(out, &out_vecs);
  OPENSSL_memset(&out->v[N], 0, 3 * sizeof(uint16_t));
}

#endif  // HRSS_HAVE_VECTOR_UNIT

// poly_mul_novec_aux writes the product of |a| and |b| to |out|, using
// |scratch| as scratch space. It'll use Karatsuba if the inputs are large
// enough to warrant it. Each call uses 2*ceil(n/2) elements of |scratch| and
// the function recurses, except if |n| < 64, when |scratch| isn't used and the
// recursion stops. If |n| == |N| then |scratch| needs 1318 elements.
static void poly_mul_novec_aux(uint16_t *out, uint16_t *scratch,
                               const uint16_t *a, const uint16_t *b, size_t n) {
  static const size_t kSchoolbookLimit = 64;
  if (n < kSchoolbookLimit) {
    OPENSSL_memset(out, 0, sizeof(uint16_t) * n * 2);
    for (size_t i = 0; i < n; i++) {
      for (size_t j = 0; j < n; j++) {
        out[i + j] += (unsigned) a[i] * b[j];
      }
    }

    return;
  }

  // Karatsuba multiplication.
  // https://en.wikipedia.org/wiki/Karatsuba_algorithm

  // When |n| is odd, the two "halves" will have different lengths. The
  // first is always the smaller.
  const size_t low_len = n / 2;
  const size_t high_len = n - low_len;
  const uint16_t *const a_high = &a[low_len];
  const uint16_t *const b_high = &b[low_len];

  for (size_t i = 0; i < low_len; i++) {
    out[i] = a_high[i] + a[i];
    out[high_len + i] = b_high[i] + b[i];
  }
  if (high_len != low_len) {
    out[low_len] = a_high[low_len];
    out[high_len + low_len] = b_high[low_len];
  }

  uint16_t *const child_scratch = &scratch[2 * high_len];
  poly_mul_novec_aux(scratch, child_scratch, out, &out[high_len], high_len);
  poly_mul_novec_aux(&out[low_len * 2], child_scratch, a_high, b_high,
                     high_len);
  poly_mul_novec_aux(out, child_scratch, a, b, low_len);

  for (size_t i = 0; i < low_len * 2; i++) {
    scratch[i] -= out[i] + out[low_len * 2 + i];
  }
  if (low_len != high_len) {
    scratch[low_len * 2] -= out[low_len * 4];
    assert(out[low_len * 4 + 1] == 0);
  }

  for (size_t i = 0; i < high_len * 2; i++) {
    out[low_len + i] += scratch[i];
  }
}

// poly_mul_novec sets |*out| to |x|×|y| mod (𝑥^n - 1).
static void poly_mul_novec(struct POLY_MUL_SCRATCH *scratch, struct poly *out,
                           const struct poly *x, const struct poly *y) {
  uint16_t *const prod = scratch->u.novec.prod;
  uint16_t *const aux_scratch = scratch->u.novec.scratch;
  poly_mul_novec_aux(prod, aux_scratch, x->v, y->v, N);

  for (size_t i = 0; i < N; i++) {
    out->v[i] = prod[i] + prod[i + N];
  }
  OPENSSL_memset(&out->v[N], 0, 3 * sizeof(uint16_t));
}

static void poly_mul(struct POLY_MUL_SCRATCH *scratch, struct poly *r,
                     const struct poly *a, const struct poly *b) {
#if defined(POLY_RQ_MUL_ASM)
  if (CRYPTO_is_AVX2_capable()) {
    poly_Rq_mul(r->v, a->v, b->v, scratch->u.rq);
    poly_normalize(r);
  } else
#endif

#if defined(HRSS_HAVE_VECTOR_UNIT)
  if (vec_capable()) {
    poly_mul_vec(scratch, r, a, b);
  } else
#endif

  // Fallback, non-vector case.
  {
    poly_mul_novec(scratch, r, a, b);
  }

  poly_assert_normalized(r);
}

// poly_mul_x_minus_1 sets |p| to |p|×(𝑥 - 1) mod (𝑥^n - 1).
static void poly_mul_x_minus_1(struct poly *p) {
  // Multiplying by (𝑥 - 1) means negating each coefficient and adding in
  // the value of the previous one.
  const uint16_t orig_final_coefficient = p->v[N - 1];

  for (size_t i = N - 1; i > 0; i--) {
    p->v[i] = p->v[i - 1] - p->v[i];
  }
  p->v[0] = orig_final_coefficient - p->v[0];
}

// poly_mod_phiN sets |p| to |p| mod Φ(N).
static void poly_mod_phiN(struct poly *p) {
  const uint16_t coeff700 = p->v[N - 1];

  for (unsigned i = 0; i < N; i++) {
    p->v[i] -= coeff700;
  }
}

// poly_clamp reduces each coefficient mod Q.
static void poly_clamp(struct poly *p) {
  for (unsigned i = 0; i < N; i++) {
    p->v[i] &= Q - 1;
  }
}


// Conversion functions
// --------------------

// poly2_from_poly sets |*out| to |in| mod 2.
static void poly2_from_poly(struct poly2 *out, const struct poly *in) {
  crypto_word_t *words = out->v;
  unsigned shift = 0;
  crypto_word_t word = 0;

  for (unsigned i = 0; i < N; i++) {
    word >>= 1;
    word |= (crypto_word_t)(in->v[i] & 1) << (BITS_PER_WORD - 1);
    shift++;

    if (shift == BITS_PER_WORD) {
      *words = word;
      words++;
      word = 0;
      shift = 0;
    }
  }

  word >>= BITS_PER_WORD - shift;
  *words = word;
}

// mod3 treats |a| as a signed number and returns |a| mod 3.
static uint16_t mod3(int16_t a) {
  const int16_t q = ((int32_t)a * 21845) >> 16;
  int16_t ret = a - 3 * q;
  // At this point, |ret| is in {0, 1, 2, 3} and that needs to be mapped to {0,
  // 1, 2, 0}.
  return ret & ((ret & (ret >> 1)) - 1);
}

// poly3_from_poly sets |*out| to |in|.
static void poly3_from_poly(struct poly3 *out, const struct poly *in) {
  crypto_word_t *words_s = out->s.v;
  crypto_word_t *words_a = out->a.v;
  crypto_word_t s = 0;
  crypto_word_t a = 0;
  unsigned shift = 0;

  for (unsigned i = 0; i < N; i++) {
    // This duplicates the 13th bit upwards to the top of the uint16,
    // essentially treating it as a sign bit and converting into a signed int16.
    // The signed value is reduced mod 3, yielding {0, 1, 2}.
    const uint16_t v = mod3((int16_t)(in->v[i] << 3) >> 3);
    s >>= 1;
    const crypto_word_t s_bit = (crypto_word_t)(v & 2) << (BITS_PER_WORD - 2);
    s |= s_bit;
    a >>= 1;
    a |= s_bit | (crypto_word_t)(v & 1) << (BITS_PER_WORD - 1);
    shift++;

    if (shift == BITS_PER_WORD) {
      *words_s = s;
      words_s++;
      *words_a = a;
      words_a++;
      s = a = 0;
      shift = 0;
    }
  }

  s >>= BITS_PER_WORD - shift;
  a >>= BITS_PER_WORD - shift;
  *words_s = s;
  *words_a = a;
}

// poly3_from_poly_checked sets |*out| to |in|, which has coefficients in {0, 1,
// Q-1}. It returns a mask indicating whether all coefficients were found to be
// in that set.
static crypto_word_t poly3_from_poly_checked(struct poly3 *out,
                                             const struct poly *in) {
  crypto_word_t *words_s = out->s.v;
  crypto_word_t *words_a = out->a.v;
  crypto_word_t s = 0;
  crypto_word_t a = 0;
  unsigned shift = 0;
  crypto_word_t ok = CONSTTIME_TRUE_W;

  for (unsigned i = 0; i < N; i++) {
    const uint16_t v = in->v[i];
    // Maps {0, 1, Q-1} to {0, 1, 2}.
    uint16_t mod3 = v & 3;
    mod3 ^= mod3 >> 1;
    const uint16_t expected = (uint16_t)((~((mod3 >> 1) - 1)) | mod3) % Q;
    ok &= constant_time_eq_w(v, expected);

    s >>= 1;
    const crypto_word_t s_bit = (crypto_word_t)(mod3 & 2)
                                << (BITS_PER_WORD - 2);
    s |= s_bit;
    a >>= 1;
    a |= s_bit | (crypto_word_t)(mod3 & 1) << (BITS_PER_WORD - 1);
    shift++;

    if (shift == BITS_PER_WORD) {
      *words_s = s;
      words_s++;
      *words_a = a;
      words_a++;
      s = a = 0;
      shift = 0;
    }
  }

  s >>= BITS_PER_WORD - shift;
  a >>= BITS_PER_WORD - shift;
  *words_s = s;
  *words_a = a;

  return ok;
}

static void poly_from_poly2(struct poly *out, const struct poly2 *in) {
  const crypto_word_t *words = in->v;
  unsigned shift = 0;
  crypto_word_t word = *words;

  for (unsigned i = 0; i < N; i++) {
    out->v[i] = word & 1;
    word >>= 1;
    shift++;

    if (shift == BITS_PER_WORD) {
      words++;
      word = *words;
      shift = 0;
    }
  }

  poly_normalize(out);
}

static void poly_from_poly3(struct poly *out, const struct poly3 *in) {
  const crypto_word_t *words_s = in->s.v;
  const crypto_word_t *words_a = in->a.v;
  crypto_word_t word_s = ~(*words_s);
  crypto_word_t word_a = *words_a;
  unsigned shift = 0;

  for (unsigned i = 0; i < N; i++) {
    out->v[i] = (uint16_t)(word_s & 1) - 1;
    out->v[i] |= word_a & 1;
    word_s >>= 1;
    word_a >>= 1;
    shift++;

    if (shift == BITS_PER_WORD) {
      words_s++;
      words_a++;
      word_s = ~(*words_s);
      word_a = *words_a;
      shift = 0;
    }
  }

  poly_normalize(out);
}

// Polynomial inversion
// --------------------

// poly_invert_mod2 sets |*out| to |in^-1| (i.e. such that |*out|×|in| = 1 mod
// Φ(N)), all mod 2. This isn't useful in itself, but is part of doing inversion
// mod Q.
static void poly_invert_mod2(struct poly *out, const struct poly *in) {
  // This algorithm is taken from section 7.1 of [SAFEGCD].
  struct poly2 v, r, f, g;

  // v = 0
  poly2_zero(&v);
  // r = 1
  poly2_zero(&r);
  r.v[0] = 1;
  // f = all ones.
  OPENSSL_memset(&f, 0xff, sizeof(struct poly2));
  f.v[WORDS_PER_POLY - 1] >>= BITS_PER_WORD - BITS_IN_LAST_WORD;
  // g is the reversal of |in|.
  poly2_from_poly(&g, in);
  poly2_mod_phiN(&g);
  poly2_reverse_700(&g, &g);
  int delta = 1;

  for (size_t i = 0; i < (2*(N-1)) - 1; i++) {
    poly2_lshift1(&v);

    const crypto_word_t delta_sign_bit = (delta >> (sizeof(delta) * 8 - 1)) & 1;
    const crypto_word_t delta_is_non_negative = delta_sign_bit - 1;
    const crypto_word_t delta_is_non_zero = ~constant_time_is_zero_w(delta);
    const crypto_word_t g_has_constant_term = lsb_to_all(g.v[0]);
    const crypto_word_t mask =
        g_has_constant_term & delta_is_non_negative & delta_is_non_zero;

    const crypto_word_t c = lsb_to_all(f.v[0] & g.v[0]);

    delta = constant_time_select_int(mask, -delta, delta);
    delta++;

    poly2_cswap(&f, &g, mask);
    poly2_fmadd(&g, &f, c);
    poly2_rshift1(&g);

    poly2_cswap(&v, &r, mask);
    poly2_fmadd(&r, &v, c);
  }

  assert(delta == 0);
  assert(f.v[0] & 1);
  poly2_reverse_700(&v, &v);
  poly_from_poly2(out, &v);
  poly_assert_normalized(out);
}

// poly_invert sets |*out| to |in^-1| (i.e. such that |*out|×|in| = 1 mod Φ(N)).
static void poly_invert(struct POLY_MUL_SCRATCH *scratch, struct poly *out,
                        const struct poly *in) {
  // Inversion mod Q, which is done based on the result of inverting mod
  // 2. See [NTRUTN14] paper, bottom of page two.
  struct poly a, *b, tmp;

  // a = -in.
  for (unsigned i = 0; i < N; i++) {
    a.v[i] = -in->v[i];
  }
  poly_normalize(&a);

  // b = in^-1 mod 2.
  b = out;
  poly_invert_mod2(b, in);

  // We are working mod Q=2**13 and we need to iterate ceil(log_2(13))
  // times, which is four.
  for (unsigned i = 0; i < 4; i++) {
    poly_mul(scratch, &tmp, &a, b);
    tmp.v[0] += 2;
    poly_mul(scratch, b, b, &tmp);
  }

  poly_assert_normalized(out);
}

// Marshal and unmarshal functions for various basic types.
// --------------------------------------------------------

#define POLY_BYTES 1138

// poly_marshal serialises all but the final coefficient of |in| to |out|.
static void poly_marshal(uint8_t out[POLY_BYTES], const struct poly *in) {
  const uint16_t *p = in->v;

  for (size_t i = 0; i < N / 8; i++) {
    out[0] = p[0];
    out[1] = (0x1f & (p[0] >> 8)) | ((p[1] & 0x07) << 5);
    out[2] = p[1] >> 3;
    out[3] = (3 & (p[1] >> 11)) | ((p[2] & 0x3f) << 2);
    out[4] = (0x7f & (p[2] >> 6)) | ((p[3] & 0x01) << 7);
    out[5] = p[3] >> 1;
    out[6] = (0xf & (p[3] >> 9)) | ((p[4] & 0x0f) << 4);
    out[7] = p[4] >> 4;
    out[8] = (1 & (p[4] >> 12)) | ((p[5] & 0x7f) << 1);
    out[9] = (0x3f & (p[5] >> 7)) | ((p[6] & 0x03) << 6);
    out[10] = p[6] >> 2;
    out[11] = (7 & (p[6] >> 10)) | ((p[7] & 0x1f) << 3);
    out[12] = p[7] >> 5;

    p += 8;
    out += 13;
  }

  // There are four remaining values.
  out[0] = p[0];
  out[1] = (0x1f & (p[0] >> 8)) | ((p[1] & 0x07) << 5);
  out[2] = p[1] >> 3;
  out[3] = (3 & (p[1] >> 11)) | ((p[2] & 0x3f) << 2);
  out[4] = (0x7f & (p[2] >> 6)) | ((p[3] & 0x01) << 7);
  out[5] = p[3] >> 1;
  out[6] = 0xf & (p[3] >> 9);
}

// poly_unmarshal parses the output of |poly_marshal| and sets |out| such that
// all but the final coefficients match, and the final coefficient is calculated
// such that evaluating |out| at one results in zero. It returns one on success
// or zero if |in| is an invalid encoding.
static int poly_unmarshal(struct poly *out, const uint8_t in[POLY_BYTES]) {
  uint16_t *p = out->v;

  for (size_t i = 0; i < N / 8; i++) {
    p[0] = (uint16_t)(in[0]) | (uint16_t)(in[1] & 0x1f) << 8;
    p[1] = (uint16_t)(in[1] >> 5) | (uint16_t)(in[2]) << 3 |
           (uint16_t)(in[3] & 3) << 11;
    p[2] = (uint16_t)(in[3] >> 2) | (uint16_t)(in[4] & 0x7f) << 6;
    p[3] = (uint16_t)(in[4] >> 7) | (uint16_t)(in[5]) << 1 |
           (uint16_t)(in[6] & 0xf) << 9;
    p[4] = (uint16_t)(in[6] >> 4) | (uint16_t)(in[7]) << 4 |
           (uint16_t)(in[8] & 1) << 12;
    p[5] = (uint16_t)(in[8] >> 1) | (uint16_t)(in[9] & 0x3f) << 7;
    p[6] = (uint16_t)(in[9] >> 6) | (uint16_t)(in[10]) << 2 |
           (uint16_t)(in[11] & 7) << 10;
    p[7] = (uint16_t)(in[11] >> 3) | (uint16_t)(in[12]) << 5;

    p += 8;
    in += 13;
  }

  // There are four coefficients remaining.
  p[0] = (uint16_t)(in[0]) | (uint16_t)(in[1] & 0x1f) << 8;
  p[1] = (uint16_t)(in[1] >> 5) | (uint16_t)(in[2]) << 3 |
         (uint16_t)(in[3] & 3) << 11;
  p[2] = (uint16_t)(in[3] >> 2) | (uint16_t)(in[4] & 0x7f) << 6;
  p[3] = (uint16_t)(in[4] >> 7) | (uint16_t)(in[5]) << 1 |
         (uint16_t)(in[6] & 0xf) << 9;

  for (unsigned i = 0; i < N - 1; i++) {
    out->v[i] = (int16_t)(out->v[i] << 3) >> 3;
  }

  // There are four unused bits in the last byte. We require them to be zero.
  if ((in[6] & 0xf0) != 0) {
    return 0;
  }

  // Set the final coefficient as specifed in [HRSSNIST] 1.9.2 step 6.
  uint32_t sum = 0;
  for (size_t i = 0; i < N - 1; i++) {
    sum += out->v[i];
  }

  out->v[N - 1] = (uint16_t)(0u - sum);
  poly_normalize(out);

  return 1;
}

// mod3_from_modQ maps {0, 1, Q-1, 65535} -> {0, 1, 2, 2}. Note that |v| may
// have an invalid value when processing attacker-controlled inputs.
static uint16_t mod3_from_modQ(uint16_t v) {
  v &= 3;
  return v ^ (v >> 1);
}

// poly_marshal_mod3 marshals |in| to |out| where the coefficients of |in| are
// all in {0, 1, Q-1, 65535} and |in| is mod Φ(N). (Note that coefficients may
// have invalid values when processing attacker-controlled inputs.)
static void poly_marshal_mod3(uint8_t out[HRSS_POLY3_BYTES],
                              const struct poly *in) {
  const uint16_t *coeffs = in->v;

  // Only 700 coefficients are marshaled because in[700] must be zero.
  assert(coeffs[N-1] == 0);

  for (size_t i = 0; i < HRSS_POLY3_BYTES; i++) {
    const uint16_t coeffs0 = mod3_from_modQ(coeffs[0]);
    const uint16_t coeffs1 = mod3_from_modQ(coeffs[1]);
    const uint16_t coeffs2 = mod3_from_modQ(coeffs[2]);
    const uint16_t coeffs3 = mod3_from_modQ(coeffs[3]);
    const uint16_t coeffs4 = mod3_from_modQ(coeffs[4]);
    out[i] = coeffs0 + coeffs1 * 3 + coeffs2 * 9 + coeffs3 * 27 + coeffs4 * 81;
    coeffs += 5;
  }
}

// HRSS-specific functions
// -----------------------

// poly_short_sample samples a vector of values in {0xffff (i.e. -1), 0, 1}.
// This is the same action as the algorithm in [HRSSNIST] section 1.8.1, but
// with HRSS-SXY the sampling algorithm is now a private detail of the
// implementation (previously it had to match between two parties). This
// function uses that freedom to implement a flatter distribution of values.
static void poly_short_sample(struct poly *out,
                              const uint8_t in[HRSS_SAMPLE_BYTES]) {
  OPENSSL_STATIC_ASSERT(HRSS_SAMPLE_BYTES == N - 1,
                        HRSS_SAMPLE_BYTES_incorrect)
  for (size_t i = 0; i < N - 1; i++) {
    uint16_t v = mod3(in[i]);
    // Map {0, 1, 2} -> {0, 1, 0xffff}
    v |= ((v >> 1) ^ 1) - 1;
    out->v[i] = v;
  }
  out->v[N - 1] = 0;
  poly_normalize(out);
}

// poly_short_sample_plus performs the T+ sample as defined in [HRSSNIST],
// section 1.8.2.
static void poly_short_sample_plus(struct poly *out,
                                   const uint8_t in[HRSS_SAMPLE_BYTES]) {
  poly_short_sample(out, in);

  // sum (and the product in the for loop) will overflow. But that's fine
  // because |sum| is bound by +/- (N-2), and N < 2^15 so it works out.
  uint16_t sum = 0;
  for (unsigned i = 0; i < N - 2; i++) {
    sum += (unsigned) out->v[i] * out->v[i + 1];
  }

  // If the sum is negative, flip the sign of even-positioned coefficients. (See
  // page 8 of [HRSS].)
  sum = ((int16_t) sum) >> 15;
  const uint16_t scale = sum | (~sum & 1);
  for (unsigned i = 0; i < N; i += 2) {
    out->v[i] = (unsigned) out->v[i] * scale;
  }
  poly_assert_normalized(out);
}

// poly_lift computes the function discussed in [HRSS], appendix B.
static void poly_lift(struct poly *out, const struct poly *a) {
  // We wish to calculate a/(𝑥-1) mod Φ(N) over GF(3), where Φ(N) is the
  // Nth cyclotomic polynomial, i.e. 1 + 𝑥 + … + 𝑥^700 (since N is prime).

  // 1/(𝑥-1) has a fairly basic structure that we can exploit to speed this up:
  //
  // R.<x> = PolynomialRing(GF(3)…)
  // inv = R.cyclotomic_polynomial(1).inverse_mod(R.cyclotomic_polynomial(n))
  // list(inv)[:15]
  //   [1, 0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2]
  //
  // This three-element pattern of coefficients repeats for the whole
  // polynomial.
  //
  // Next define the overbar operator such that z̅ = z[0] +
  // reverse(z[1:]). (Index zero of a polynomial here is the coefficient
  // of the constant term. So index one is the coefficient of 𝑥 and so
  // on.)
  //
  // A less odd way to define this is to see that z̅ negates the indexes,
  // so z̅[0] = z[-0], z̅[1] = z[-1] and so on.
  //
  // The use of z̅ is that, when working mod (𝑥^701 - 1), vz[0] = <v,
  // z̅>, vz[1] = <v, 𝑥z̅>, …. (Where <a, b> is the inner product: the sum
  // of the point-wise products.) Although we calculated the inverse mod
  // Φ(N), we can work mod (𝑥^N - 1) and reduce mod Φ(N) at the end.
  // (That's because (𝑥^N - 1) is a multiple of Φ(N).)
  //
  // When working mod (𝑥^N - 1), multiplication by 𝑥 is a right-rotation
  // of the list of coefficients.
  //
  // Thus we can consider what the pattern of z̅, 𝑥z̅, 𝑥^2z̅, … looks like:
  //
  // def reverse(xs):
  //   suffix = list(xs[1:])
  //   suffix.reverse()
  //   return [xs[0]] + suffix
  //
  // def rotate(xs):
  //   return [xs[-1]] + xs[:-1]
  //
  // zoverbar = reverse(list(inv) + [0])
  // xzoverbar = rotate(reverse(list(inv) + [0]))
  // x2zoverbar = rotate(rotate(reverse(list(inv) + [0])))
  //
  // zoverbar[:15]
  //   [1, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1]
  // xzoverbar[:15]
  //   [0, 1, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0]
  // x2zoverbar[:15]
  //   [2, 0, 1, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2]
  //
  // (For a formula for z̅, see lemma two of appendix B.)
  //
  // After the first three elements have been taken care of, all then have
  // a repeating three-element cycle. The next value (𝑥^3z̅) involves
  // three rotations of the first pattern, thus the three-element cycle
  // lines up. However, the discontinuity in the first three elements
  // obviously moves to a different position. Consider the difference
  // between 𝑥^3z̅ and z̅:
  //
  // [x-y for (x,y) in zip(zoverbar, x3zoverbar)][:15]
  //    [0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
  //
  // This pattern of differences is the same for all elements, although it
  // obviously moves right with the rotations.
  //
  // From this, we reach algorithm eight of appendix B.

  // Handle the first three elements of the inner products.
  out->v[0] = a->v[0] + a->v[2];
  out->v[1] = a->v[1];
  out->v[2] = -a->v[0] + a->v[2];

  // s0, s1, s2 are added into out->v[0], out->v[1], and out->v[2],
  // respectively. We do not compute s1 because it's just -(s0 + s1).
  uint16_t s0 = 0, s2 = 0;
  for (size_t i = 3; i < 699; i += 3) {
    s0 += -a->v[i] + a->v[i + 2];
    // s1 += a->v[i] - a->v[i + 1];
    s2 += a->v[i + 1] - a->v[i + 2];
  }

  // Handle the fact that the three-element pattern doesn't fill the
  // polynomial exactly (since 701 isn't a multiple of three).
  s0 -= a->v[699];
  // s1 += a->v[699] - a->v[700];
  s2 += a->v[700];

  // Note that s0 + s1 + s2 = 0.
  out->v[0] += s0;
  out->v[1] -= (s0 + s2); // = s1
  out->v[2] += s2;

  // Calculate the remaining inner products by taking advantage of the
  // fact that the pattern repeats every three cycles and the pattern of
  // differences moves with the rotation.
  for (size_t i = 3; i < N; i++) {
    out->v[i] = (out->v[i - 3] - (a->v[i - 2] + a->v[i - 1] + a->v[i]));
  }

  // Reduce mod Φ(N) by subtracting a multiple of out[700] from every
  // element and convert to mod Q. (See above about adding twice as
  // subtraction.)
  const crypto_word_t v = out->v[700];
  for (unsigned i = 0; i < N; i++) {
    const uint16_t vi_mod3 = mod3(out->v[i] - v);
    // Map {0, 1, 2} to {0, 1, 0xffff}.
    out->v[i] = (~((vi_mod3 >> 1) - 1)) | vi_mod3;
  }

  poly_mul_x_minus_1(out);
  poly_normalize(out);
}

struct public_key {
  struct poly ph;
};

struct private_key {
  struct poly3 f, f_inverse;
  struct poly ph_inverse;
  uint8_t hmac_key[32];
};

// public_key_from_external converts an external public key pointer into an
// internal one. Externally the alignment is only specified to be eight bytes
// but we need 16-byte alignment. We could annotate the external struct with
// that alignment but we can only assume that malloced pointers are 8-byte
// aligned in any case. (Even if the underlying malloc returns values with
// 16-byte alignment, |OPENSSL_malloc| will store an 8-byte size prefix and mess
// that up.)
static struct public_key *public_key_from_external(
    struct HRSS_public_key *ext) {
  OPENSSL_STATIC_ASSERT(
      sizeof(struct HRSS_public_key) >= sizeof(struct public_key) + 15,
      HRSS_public_key_too_small)

  return align_pointer(ext->opaque, 16);
}

// private_key_from_external does the same thing as |public_key_from_external|,
// but for private keys. See the comment on that function about alignment
// issues.
static struct private_key *private_key_from_external(
    struct HRSS_private_key *ext) {
  OPENSSL_STATIC_ASSERT(
      sizeof(struct HRSS_private_key) >= sizeof(struct private_key) + 15,
      HRSS_private_key_too_small)

  return align_pointer(ext->opaque, 16);
}

// malloc_align32 returns a pointer to |size| bytes of 32-byte-aligned heap and
// sets |*out_ptr| to a value that can be passed to |OPENSSL_free| to release
// it. It returns NULL if out of memory.
static void *malloc_align32(void **out_ptr, size_t size) {
  void *ptr = OPENSSL_malloc(size + 31);
  if (!ptr) {
    *out_ptr = NULL;
    return NULL;
  }

  *out_ptr = ptr;
  return align_pointer(ptr, 32);
}

int HRSS_generate_key(
    struct HRSS_public_key *out_pub, struct HRSS_private_key *out_priv,
    const uint8_t in[HRSS_SAMPLE_BYTES + HRSS_SAMPLE_BYTES + 32]) {
  struct public_key *pub = public_key_from_external(out_pub);
  struct private_key *priv = private_key_from_external(out_priv);

  struct vars {
    struct POLY_MUL_SCRATCH scratch;
    struct poly f;
    struct poly pg_phi1;
    struct poly pfg_phi1;
    struct poly pfg_phi1_inverse;
  };

  void *malloc_ptr;
  struct vars *const vars = malloc_align32(&malloc_ptr, sizeof(struct vars));
  if (!vars) {
    // If the caller ignores the return value the output will still be safe.
    // The private key output is randomised in case it's later passed to
    // |HRSS_encap|.
    memset(out_pub, 0, sizeof(struct HRSS_public_key));
    AWSLC_ABORT_IF_NOT_ONE(RAND_bytes((uint8_t*) out_priv, sizeof(struct HRSS_private_key)));
    return 0;
  }

#if !defined(NDEBUG)
  OPENSSL_memset(vars, 0xff, sizeof(struct vars));
#endif

  OPENSSL_memcpy(priv->hmac_key, in + 2 * HRSS_SAMPLE_BYTES,
                 sizeof(priv->hmac_key));

  poly_short_sample_plus(&vars->f, in);
  poly3_from_poly(&priv->f, &vars->f);
  HRSS_poly3_invert(&priv->f_inverse, &priv->f);

  // pg_phi1 is p (i.e. 3) × g × Φ(1) (i.e. 𝑥-1).
  poly_short_sample_plus(&vars->pg_phi1, in + HRSS_SAMPLE_BYTES);
  for (unsigned i = 0; i < N; i++) {
    vars->pg_phi1.v[i] *= 3;
  }
  poly_mul_x_minus_1(&vars->pg_phi1);

  poly_mul(&vars->scratch, &vars->pfg_phi1, &vars->f, &vars->pg_phi1);

  poly_invert(&vars->scratch, &vars->pfg_phi1_inverse, &vars->pfg_phi1);

  poly_mul(&vars->scratch, &pub->ph, &vars->pfg_phi1_inverse, &vars->pg_phi1);
  poly_mul(&vars->scratch, &pub->ph, &pub->ph, &vars->pg_phi1);
  poly_clamp(&pub->ph);

  poly_mul(&vars->scratch, &priv->ph_inverse, &vars->pfg_phi1_inverse,
           &vars->f);
  poly_mul(&vars->scratch, &priv->ph_inverse, &priv->ph_inverse, &vars->f);
  poly_clamp(&priv->ph_inverse);

  OPENSSL_free(malloc_ptr);
  return 1;
}

static const char kSharedKey[] = "shared key";

int HRSS_encap(uint8_t out_ciphertext[POLY_BYTES], uint8_t out_shared_key[32],
               const struct HRSS_public_key *in_pub,
               const uint8_t in[HRSS_SAMPLE_BYTES + HRSS_SAMPLE_BYTES]) {
  const struct public_key *pub =
      public_key_from_external((struct HRSS_public_key *)in_pub);

  struct vars {
    struct POLY_MUL_SCRATCH scratch;
    struct poly m, r, m_lifted;
    struct poly prh_plus_m;
    SHA256_CTX hash_ctx;
    uint8_t m_bytes[HRSS_POLY3_BYTES];
    uint8_t r_bytes[HRSS_POLY3_BYTES];
  };

  void *malloc_ptr;
  struct vars *const vars = malloc_align32(&malloc_ptr, sizeof(struct vars));
  if (!vars) {
    // If the caller ignores the return value the output will still be safe.
    // The private key output is randomised in case it's used to encrypt and
    // transmit something.
    memset(out_ciphertext, 0, POLY_BYTES);
    AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(out_shared_key, 32));
    return 0;
  }

#if !defined(NDEBUG)
  OPENSSL_memset(vars, 0xff, sizeof(struct vars));
#endif

  poly_short_sample(&vars->m, in);
  poly_short_sample(&vars->r, in + HRSS_SAMPLE_BYTES);
  poly_lift(&vars->m_lifted, &vars->m);

  poly_mul(&vars->scratch, &vars->prh_plus_m, &vars->r, &pub->ph);
  for (unsigned i = 0; i < N; i++) {
    vars->prh_plus_m.v[i] += vars->m_lifted.v[i];
  }

  poly_marshal(out_ciphertext, &vars->prh_plus_m);

  poly_marshal_mod3(vars->m_bytes, &vars->m);
  poly_marshal_mod3(vars->r_bytes, &vars->r);

  SHA256_Init(&vars->hash_ctx);
  SHA256_Update(&vars->hash_ctx, kSharedKey, sizeof(kSharedKey));
  SHA256_Update(&vars->hash_ctx, vars->m_bytes, sizeof(vars->m_bytes));
  SHA256_Update(&vars->hash_ctx, vars->r_bytes, sizeof(vars->r_bytes));
  SHA256_Update(&vars->hash_ctx, out_ciphertext, POLY_BYTES);
  SHA256_Final(out_shared_key, &vars->hash_ctx);

  OPENSSL_free(malloc_ptr);
  return 1;
}

int HRSS_decap(uint8_t out_shared_key[HRSS_KEY_BYTES],
                const struct HRSS_private_key *in_priv,
                const uint8_t *ciphertext, size_t ciphertext_len) {
  const struct private_key *priv =
      private_key_from_external((struct HRSS_private_key *)in_priv);

  struct vars {
    struct POLY_MUL_SCRATCH scratch;
    uint8_t masked_key[SHA256_CBLOCK];
    SHA256_CTX hash_ctx;
    struct poly c;
    struct poly f, cf;
    struct poly3 cf3, m3;
    struct poly m, m_lifted;
    struct poly r;
    struct poly3 r3;
    uint8_t expected_ciphertext[HRSS_CIPHERTEXT_BYTES];
    uint8_t m_bytes[HRSS_POLY3_BYTES];
    uint8_t r_bytes[HRSS_POLY3_BYTES];
    uint8_t shared_key[32];
  };

  void *malloc_ptr;
  struct vars *const vars = malloc_align32(&malloc_ptr, sizeof(struct vars));
  if (!vars) {
    // If the caller ignores the return value the output will still be safe.
    // The private key output is randomised in case it's used to encrypt and
    // transmit something.
    AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(out_shared_key, HRSS_KEY_BYTES));
    return 0;
  }

#if !defined(NDEBUG)
  OPENSSL_memset(vars, 0xff, sizeof(struct vars));
#endif

  // This is HMAC, expanded inline rather than using the |HMAC| function so that
  // we can avoid dealing with possible allocation failures and so keep this
  // function infallible.
  OPENSSL_STATIC_ASSERT(sizeof(priv->hmac_key) <= sizeof(vars->masked_key),
                        HRSS_HMAC_key_larger_than_SHA_256_block_size)

  for (size_t i = 0; i < sizeof(priv->hmac_key); i++) {
    vars->masked_key[i] = priv->hmac_key[i] ^ 0x36;
  }
  OPENSSL_memset(vars->masked_key + sizeof(priv->hmac_key), 0x36,
                 sizeof(vars->masked_key) - sizeof(priv->hmac_key));

  SHA256_Init(&vars->hash_ctx);
  SHA256_Update(&vars->hash_ctx, vars->masked_key, sizeof(vars->masked_key));
  SHA256_Update(&vars->hash_ctx, ciphertext, ciphertext_len);
  uint8_t inner_digest[SHA256_DIGEST_LENGTH];
  SHA256_Final(inner_digest, &vars->hash_ctx);

  for (size_t i = 0; i < sizeof(priv->hmac_key); i++) {
    vars->masked_key[i] ^= (0x5c ^ 0x36);
  }
  OPENSSL_memset(vars->masked_key + sizeof(priv->hmac_key), 0x5c,
                 sizeof(vars->masked_key) - sizeof(priv->hmac_key));

  SHA256_Init(&vars->hash_ctx);
  SHA256_Update(&vars->hash_ctx, vars->masked_key, sizeof(vars->masked_key));
  SHA256_Update(&vars->hash_ctx, inner_digest, sizeof(inner_digest));
  OPENSSL_STATIC_ASSERT(HRSS_KEY_BYTES == SHA256_DIGEST_LENGTH,
                        HRSS_shared_key_length_incorrect)
  SHA256_Final(out_shared_key, &vars->hash_ctx);

  // If the ciphertext is publicly invalid then a random shared key is still
  // returned to simply the logic of the caller, but this path is not constant
  // time.
  if (ciphertext_len != HRSS_CIPHERTEXT_BYTES ||
      !poly_unmarshal(&vars->c, ciphertext)) {
    goto out;
  }

  poly_from_poly3(&vars->f, &priv->f);
  poly_mul(&vars->scratch, &vars->cf, &vars->c, &vars->f);
  poly3_from_poly(&vars->cf3, &vars->cf);
  // Note that cf3 is not reduced mod Φ(N). That reduction is deferred.
  HRSS_poly3_mul(&vars->m3, &vars->cf3, &priv->f_inverse);

  poly_from_poly3(&vars->m, &vars->m3);
  poly_lift(&vars->m_lifted, &vars->m);

  for (unsigned i = 0; i < N; i++) {
    vars->r.v[i] = vars->c.v[i] - vars->m_lifted.v[i];
  }
  poly_normalize(&vars->r);
  poly_mul(&vars->scratch, &vars->r, &vars->r, &priv->ph_inverse);
  poly_mod_phiN(&vars->r);
  poly_clamp(&vars->r);

  crypto_word_t ok = poly3_from_poly_checked(&vars->r3, &vars->r);

  // [NTRUCOMP] section 5.1 includes ReEnc2 and a proof that it's valid. Rather
  // than do an expensive |poly_mul|, it rebuilds |c'| from |c - lift(m)|
  // (called |b|) with:
  //   t = (−b(1)/N) mod Q
  //   c' = b + tΦ(N) + lift(m) mod Q
  //
  // When polynomials are transmitted, the final coefficient is omitted and
  // |poly_unmarshal| sets it such that f(1) == 0. Thus c(1) == 0. Also,
  // |poly_lift| multiplies the result by (x-1) and therefore evaluating a
  // lifted polynomial at 1 is also zero. Thus lift(m)(1) == 0 and so
  // (c - lift(m))(1) == 0.
  //
  // Although we defer the reduction above, |b| is conceptually reduced mod
  // Φ(N). In order to do that reduction one subtracts |c[N-1]| from every
  // coefficient. Therefore b(1) = -c[N-1]×N. The value of |t|, above, then is
  // just recovering |c[N-1]|, and adding tΦ(N) is simply undoing the reduction.
  // Therefore b + tΦ(N) + lift(m) = c by construction and we don't need to
  // recover |c| at all so long as we do the checks in
  // |poly3_from_poly_checked|.
  //
  // The |poly_marshal| here then is just confirming that |poly_unmarshal| is
  // strict and could be omitted.

  OPENSSL_STATIC_ASSERT(HRSS_CIPHERTEXT_BYTES == POLY_BYTES,
                        ciphertext_is_the_wrong_size)
  assert(ciphertext_len == sizeof(vars->expected_ciphertext));
  poly_marshal(vars->expected_ciphertext, &vars->c);

  poly_marshal_mod3(vars->m_bytes, &vars->m);
  poly_marshal_mod3(vars->r_bytes, &vars->r);

  ok &= constant_time_is_zero_w(
      CRYPTO_memcmp(ciphertext, vars->expected_ciphertext,
                    sizeof(vars->expected_ciphertext)));

  SHA256_Init(&vars->hash_ctx);
  SHA256_Update(&vars->hash_ctx, kSharedKey, sizeof(kSharedKey));
  SHA256_Update(&vars->hash_ctx, vars->m_bytes, sizeof(vars->m_bytes));
  SHA256_Update(&vars->hash_ctx, vars->r_bytes, sizeof(vars->r_bytes));
  SHA256_Update(&vars->hash_ctx, vars->expected_ciphertext,
                sizeof(vars->expected_ciphertext));
  SHA256_Final(vars->shared_key, &vars->hash_ctx);

  for (unsigned i = 0; i < sizeof(vars->shared_key); i++) {
    out_shared_key[i] =
        constant_time_select_8(ok, vars->shared_key[i], out_shared_key[i]);
  }

out:
  OPENSSL_free(malloc_ptr);
  return 1;
}

void HRSS_marshal_public_key(uint8_t out[HRSS_PUBLIC_KEY_BYTES],
                             const struct HRSS_public_key *in_pub) {
  const struct public_key *pub =
      public_key_from_external((struct HRSS_public_key *)in_pub);
  poly_marshal(out, &pub->ph);
}

int HRSS_parse_public_key(struct HRSS_public_key *out,
                          const uint8_t in[HRSS_PUBLIC_KEY_BYTES]) {
  struct public_key *pub = public_key_from_external(out);
  if (!poly_unmarshal(&pub->ph, in)) {
    return 0;
  }
  OPENSSL_memset(&pub->ph.v[N], 0, 3 * sizeof(uint16_t));
  return 1;
}
