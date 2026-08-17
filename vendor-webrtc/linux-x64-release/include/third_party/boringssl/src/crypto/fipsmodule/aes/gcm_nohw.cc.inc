// Copyright 2019 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <openssl/base.h>

#include "../../internal.h"
#include "internal.h"

#if !defined(BORINGSSL_HAS_UINT128) && defined(OPENSSL_SSE2)
#include <emmintrin.h>
#endif


// This file contains a constant-time implementation of GHASH based on the notes
// in https://bearssl.org/constanttime.html#ghash-for-gcm and the reduction
// algorithm described in
// https://crypto.stanford.edu/RealWorldCrypto/slides/gueron.pdf.
//
// Unlike the BearSSL notes, we use uint128_t in the 64-bit implementation. Our
// primary compilers (clang, clang-cl, and gcc) all support it. MSVC will run
// the 32-bit implementation, but we can use its intrinsics if necessary.

#if defined(BORINGSSL_HAS_UINT128)

static void gcm_mul64_nohw(uint64_t *out_lo, uint64_t *out_hi, uint64_t a,
                           uint64_t b) {
  // One term every four bits means the largest term is 64/4 = 16, which barely
  // overflows into the next term. Using one term every five bits would cost 25
  // multiplications instead of 16. It is faster to mask off the bottom four
  // bits of |a|, giving a largest term of 60/4 = 15, and apply the bottom bits
  // separately.
  uint64_t a0 = a & UINT64_C(0x1111111111111110);
  uint64_t a1 = a & UINT64_C(0x2222222222222220);
  uint64_t a2 = a & UINT64_C(0x4444444444444440);
  uint64_t a3 = a & UINT64_C(0x8888888888888880);

  uint64_t b0 = b & UINT64_C(0x1111111111111111);
  uint64_t b1 = b & UINT64_C(0x2222222222222222);
  uint64_t b2 = b & UINT64_C(0x4444444444444444);
  uint64_t b3 = b & UINT64_C(0x8888888888888888);

  uint128_t c0 = (a0 * (uint128_t)b0) ^ (a1 * (uint128_t)b3) ^
                 (a2 * (uint128_t)b2) ^ (a3 * (uint128_t)b1);
  uint128_t c1 = (a0 * (uint128_t)b1) ^ (a1 * (uint128_t)b0) ^
                 (a2 * (uint128_t)b3) ^ (a3 * (uint128_t)b2);
  uint128_t c2 = (a0 * (uint128_t)b2) ^ (a1 * (uint128_t)b1) ^
                 (a2 * (uint128_t)b0) ^ (a3 * (uint128_t)b3);
  uint128_t c3 = (a0 * (uint128_t)b3) ^ (a1 * (uint128_t)b2) ^
                 (a2 * (uint128_t)b1) ^ (a3 * (uint128_t)b0);

  // Multiply the bottom four bits of |a| with |b|.
  uint64_t a0_mask = UINT64_C(0) - (a & 1);
  uint64_t a1_mask = UINT64_C(0) - ((a >> 1) & 1);
  uint64_t a2_mask = UINT64_C(0) - ((a >> 2) & 1);
  uint64_t a3_mask = UINT64_C(0) - ((a >> 3) & 1);
  uint128_t extra = (a0_mask & b) ^ ((uint128_t)(a1_mask & b) << 1) ^
                    ((uint128_t)(a2_mask & b) << 2) ^
                    ((uint128_t)(a3_mask & b) << 3);

  *out_lo = (((uint64_t)c0) & UINT64_C(0x1111111111111111)) ^
            (((uint64_t)c1) & UINT64_C(0x2222222222222222)) ^
            (((uint64_t)c2) & UINT64_C(0x4444444444444444)) ^
            (((uint64_t)c3) & UINT64_C(0x8888888888888888)) ^ ((uint64_t)extra);
  *out_hi = (((uint64_t)(c0 >> 64)) & UINT64_C(0x1111111111111111)) ^
            (((uint64_t)(c1 >> 64)) & UINT64_C(0x2222222222222222)) ^
            (((uint64_t)(c2 >> 64)) & UINT64_C(0x4444444444444444)) ^
            (((uint64_t)(c3 >> 64)) & UINT64_C(0x8888888888888888)) ^
            ((uint64_t)(extra >> 64));
}

#elif defined(OPENSSL_SSE2)

static __m128i gcm_mul32_nohw(uint32_t a, uint32_t b) {
  // One term every four bits means the largest term is 32/4 = 8, which does not
  // overflow into the next term.
  __m128i aa = _mm_setr_epi32(a, 0, a, 0);
  __m128i bb = _mm_setr_epi32(b, 0, b, 0);

  __m128i a0a0 =
      _mm_and_si128(aa, _mm_setr_epi32(0x11111111, 0, 0x11111111, 0));
  __m128i a2a2 =
      _mm_and_si128(aa, _mm_setr_epi32(0x44444444, 0, 0x44444444, 0));
  __m128i b0b1 =
      _mm_and_si128(bb, _mm_setr_epi32(0x11111111, 0, 0x22222222, 0));
  __m128i b2b3 =
      _mm_and_si128(bb, _mm_setr_epi32(0x44444444, 0, 0x88888888, 0));

  __m128i c0c1 =
      _mm_xor_si128(_mm_mul_epu32(a0a0, b0b1), _mm_mul_epu32(a2a2, b2b3));
  __m128i c2c3 =
      _mm_xor_si128(_mm_mul_epu32(a2a2, b0b1), _mm_mul_epu32(a0a0, b2b3));

  __m128i a1a1 =
      _mm_and_si128(aa, _mm_setr_epi32(0x22222222, 0, 0x22222222, 0));
  __m128i a3a3 =
      _mm_and_si128(aa, _mm_setr_epi32(0x88888888, 0, 0x88888888, 0));
  __m128i b3b0 =
      _mm_and_si128(bb, _mm_setr_epi32(0x88888888, 0, 0x11111111, 0));
  __m128i b1b2 =
      _mm_and_si128(bb, _mm_setr_epi32(0x22222222, 0, 0x44444444, 0));

  c0c1 = _mm_xor_si128(c0c1, _mm_mul_epu32(a1a1, b3b0));
  c0c1 = _mm_xor_si128(c0c1, _mm_mul_epu32(a3a3, b1b2));
  c2c3 = _mm_xor_si128(c2c3, _mm_mul_epu32(a3a3, b3b0));
  c2c3 = _mm_xor_si128(c2c3, _mm_mul_epu32(a1a1, b1b2));

  c0c1 = _mm_and_si128(
      c0c1, _mm_setr_epi32(0x11111111, 0x11111111, 0x22222222, 0x22222222));
  c2c3 = _mm_and_si128(
      c2c3, _mm_setr_epi32(0x44444444, 0x44444444, 0x88888888, 0x88888888));

  c0c1 = _mm_xor_si128(c0c1, c2c3);
  // c0 ^= c1
  c0c1 = _mm_xor_si128(c0c1, _mm_srli_si128(c0c1, 8));
  return c0c1;
}

static void gcm_mul64_nohw(uint64_t *out_lo, uint64_t *out_hi, uint64_t a,
                           uint64_t b) {
  uint32_t a0 = a & 0xffffffff;
  uint32_t a1 = a >> 32;
  uint32_t b0 = b & 0xffffffff;
  uint32_t b1 = b >> 32;
  // Karatsuba multiplication.
  __m128i lo = gcm_mul32_nohw(a0, b0);
  __m128i hi = gcm_mul32_nohw(a1, b1);
  __m128i mid = gcm_mul32_nohw(a0 ^ a1, b0 ^ b1);
  mid = _mm_xor_si128(mid, lo);
  mid = _mm_xor_si128(mid, hi);
  __m128i ret = _mm_unpacklo_epi64(lo, hi);
  mid = _mm_slli_si128(mid, 4);
  mid = _mm_and_si128(mid, _mm_setr_epi32(0, 0xffffffff, 0xffffffff, 0));
  ret = _mm_xor_si128(ret, mid);
  memcpy(out_lo, &ret, 8);
  memcpy(out_hi, ((char*)&ret) + 8, 8);
}

#else  // !BORINGSSL_HAS_UINT128 && !OPENSSL_SSE2

static uint64_t gcm_mul32_nohw(uint32_t a, uint32_t b) {
  // One term every four bits means the largest term is 32/4 = 8, which does not
  // overflow into the next term.
  uint32_t a0 = a & 0x11111111;
  uint32_t a1 = a & 0x22222222;
  uint32_t a2 = a & 0x44444444;
  uint32_t a3 = a & 0x88888888;

  uint32_t b0 = b & 0x11111111;
  uint32_t b1 = b & 0x22222222;
  uint32_t b2 = b & 0x44444444;
  uint32_t b3 = b & 0x88888888;

  uint64_t c0 = (a0 * (uint64_t)b0) ^ (a1 * (uint64_t)b3) ^
                (a2 * (uint64_t)b2) ^ (a3 * (uint64_t)b1);
  uint64_t c1 = (a0 * (uint64_t)b1) ^ (a1 * (uint64_t)b0) ^
                (a2 * (uint64_t)b3) ^ (a3 * (uint64_t)b2);
  uint64_t c2 = (a0 * (uint64_t)b2) ^ (a1 * (uint64_t)b1) ^
                (a2 * (uint64_t)b0) ^ (a3 * (uint64_t)b3);
  uint64_t c3 = (a0 * (uint64_t)b3) ^ (a1 * (uint64_t)b2) ^
                (a2 * (uint64_t)b1) ^ (a3 * (uint64_t)b0);

  return (c0 & UINT64_C(0x1111111111111111)) |
         (c1 & UINT64_C(0x2222222222222222)) |
         (c2 & UINT64_C(0x4444444444444444)) |
         (c3 & UINT64_C(0x8888888888888888));
}

static void gcm_mul64_nohw(uint64_t *out_lo, uint64_t *out_hi, uint64_t a,
                           uint64_t b) {
  uint32_t a0 = a & 0xffffffff;
  uint32_t a1 = a >> 32;
  uint32_t b0 = b & 0xffffffff;
  uint32_t b1 = b >> 32;
  // Karatsuba multiplication.
  uint64_t lo = gcm_mul32_nohw(a0, b0);
  uint64_t hi = gcm_mul32_nohw(a1, b1);
  uint64_t mid = gcm_mul32_nohw(a0 ^ a1, b0 ^ b1) ^ lo ^ hi;
  *out_lo = lo ^ (mid << 32);
  *out_hi = hi ^ (mid >> 32);
}

#endif  // BORINGSSL_HAS_UINT128

void gcm_init_nohw(u128 Htable[16], const uint64_t Xi[2]) {
  // We implement GHASH in terms of POLYVAL, as described in RFC 8452. This
  // avoids a shift by 1 in the multiplication, needed to account for bit
  // reversal losing a bit after multiplication, that is,
  // rev128(X) * rev128(Y) = rev255(X*Y).
  //
  // Per Appendix A, we run mulX_POLYVAL. Note this is the same transformation
  // applied by |gcm_init_clmul|, etc. Note |Xi| has already been byteswapped.
  //
  // See also slide 16 of
  // https://crypto.stanford.edu/RealWorldCrypto/slides/gueron.pdf
  Htable[0].lo = Xi[1];
  Htable[0].hi = Xi[0];

  uint64_t carry = Htable[0].hi >> 63;
  carry = 0u - carry;

  Htable[0].hi <<= 1;
  Htable[0].hi |= Htable[0].lo >> 63;
  Htable[0].lo <<= 1;

  // The irreducible polynomial is 1 + x^121 + x^126 + x^127 + x^128, so we
  // conditionally add 0xc200...0001.
  Htable[0].lo ^= carry & 1;
  Htable[0].hi ^= carry & UINT64_C(0xc200000000000000);

  // This implementation does not use the rest of |Htable|.
}

static void gcm_polyval_nohw(uint64_t Xi[2], const u128 *H) {
  // Karatsuba multiplication. The product of |Xi| and |H| is stored in |r0|
  // through |r3|. Note there is no byte or bit reversal because we are
  // evaluating POLYVAL.
  uint64_t r0, r1;
  gcm_mul64_nohw(&r0, &r1, Xi[0], H->lo);
  uint64_t r2, r3;
  gcm_mul64_nohw(&r2, &r3, Xi[1], H->hi);
  uint64_t mid0, mid1;
  gcm_mul64_nohw(&mid0, &mid1, Xi[0] ^ Xi[1], H->hi ^ H->lo);
  mid0 ^= r0 ^ r2;
  mid1 ^= r1 ^ r3;
  r2 ^= mid1;
  r1 ^= mid0;

  // Now we multiply our 256-bit result by x^-128 and reduce. |r2| and
  // |r3| shifts into position and we must multiply |r0| and |r1| by x^-128. We
  // have:
  //
  //       1 = x^121 + x^126 + x^127 + x^128
  //  x^-128 = x^-7 + x^-2 + x^-1 + 1
  //
  // This is the GHASH reduction step, but with bits flowing in reverse.

  // The x^-7, x^-2, and x^-1 terms shift bits past x^0, which would require
  // another reduction steps. Instead, we gather the excess bits, incorporate
  // them into |r0| and |r1| and reduce once. See slides 17-19
  // of https://crypto.stanford.edu/RealWorldCrypto/slides/gueron.pdf.
  r1 ^= (r0 << 63) ^ (r0 << 62) ^ (r0 << 57);

  // 1
  r2 ^= r0;
  r3 ^= r1;

  // x^-1
  r2 ^= r0 >> 1;
  r2 ^= r1 << 63;
  r3 ^= r1 >> 1;

  // x^-2
  r2 ^= r0 >> 2;
  r2 ^= r1 << 62;
  r3 ^= r1 >> 2;

  // x^-7
  r2 ^= r0 >> 7;
  r2 ^= r1 << 57;
  r3 ^= r1 >> 7;

  Xi[0] = r2;
  Xi[1] = r3;
}

void gcm_gmult_nohw(uint8_t Xi[16], const u128 Htable[16]) {
  uint64_t swapped[2];
  swapped[0] = CRYPTO_load_u64_be(Xi + 8);
  swapped[1] = CRYPTO_load_u64_be(Xi);
  gcm_polyval_nohw(swapped, &Htable[0]);
  CRYPTO_store_u64_be(Xi, swapped[1]);
  CRYPTO_store_u64_be(Xi + 8, swapped[0]);
}

void gcm_ghash_nohw(uint8_t Xi[16], const u128 Htable[16], const uint8_t *inp,
                    size_t len) {
  uint64_t swapped[2];
  swapped[0] = CRYPTO_load_u64_be(Xi + 8);
  swapped[1] = CRYPTO_load_u64_be(Xi);

  while (len >= 16) {
    swapped[0] ^= CRYPTO_load_u64_be(inp + 8);
    swapped[1] ^= CRYPTO_load_u64_be(inp);
    gcm_polyval_nohw(swapped, &Htable[0]);
    inp += 16;
    len -= 16;
  }

  CRYPTO_store_u64_be(Xi, swapped[1]);
  CRYPTO_store_u64_be(Xi + 8, swapped[0]);
}
