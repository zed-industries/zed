// Copyright 2014 The BoringSSL Authors
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

#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include <openssl/base.h>
#include <openssl/bytestring.h>
#include <openssl/mem.h>
#include <openssl/rand.h>

#include "../../internal.h"
#include "../bcm_interface.h"
#include "../delocate.h"
#include "../keccak/internal.h"


namespace mlkem {
namespace {

namespace fips {
void ensure_keygen_self_test();
void ensure_encap_self_test();
void ensure_decap_self_test();
}  // namespace fips

// See
// https://csrc.nist.gov/pubs/fips/203/final

static void prf(uint8_t *out, size_t out_len, const uint8_t in[33]) {
  BORINGSSL_keccak(out, out_len, in, 33, boringssl_shake256);
}

// Section 4.1
void hash_h(uint8_t out[32], const uint8_t *in, size_t len) {
  BORINGSSL_keccak(out, 32, in, len, boringssl_sha3_256);
}

void hash_g(uint8_t out[64], const uint8_t *in, size_t len) {
  BORINGSSL_keccak(out, 64, in, len, boringssl_sha3_512);
}

// This is called `J` in the spec.
void kdf(uint8_t out[BCM_MLKEM_SHARED_SECRET_BYTES],
         const uint8_t failure_secret[32], const uint8_t *ciphertext,
         size_t ciphertext_len) {
  struct BORINGSSL_keccak_st st;
  BORINGSSL_keccak_init(&st, boringssl_shake256);
  BORINGSSL_keccak_absorb(&st, failure_secret, 32);
  BORINGSSL_keccak_absorb(&st, ciphertext, ciphertext_len);
  BORINGSSL_keccak_squeeze(&st, out, BCM_MLKEM_SHARED_SECRET_BYTES);
}

// Constants that are common across all sizes.
#define DEGREE 256
const size_t kBarrettMultiplier = 5039;
const unsigned kBarrettShift = 24;
static const uint16_t kPrime = 3329;
const int kLog2Prime = 12;
const uint16_t kHalfPrime = (/*kPrime=*/3329 - 1) / 2;
// kInverseDegree is 128^-1 mod 3329; 128 because kPrime does not have a 512th
// root of unity.
const uint16_t kInverseDegree = 3303;

// Rank-specific constants.
#define RANK768 3
static const int kDU768 = 10;
const int kDV768 = 4;
#define RANK1024 4
static const int kDU1024 = 11;
const int kDV1024 = 5;

constexpr size_t encoded_vector_size(int rank) {
  return (kLog2Prime * DEGREE / 8) * static_cast<size_t>(rank);
}

constexpr size_t encoded_public_key_size(int rank) {
  return encoded_vector_size(rank) + /*sizeof(rho)=*/32;
}

static_assert(encoded_public_key_size(RANK768) == BCM_MLKEM768_PUBLIC_KEY_BYTES,
              "");
static_assert(encoded_public_key_size(RANK1024) ==
                  BCM_MLKEM1024_PUBLIC_KEY_BYTES,
              "");

constexpr size_t compressed_vector_size(int rank) {
  // `if constexpr` isn't available in C++17.
  return (rank == RANK768 ? kDU768 : kDU1024) * static_cast<size_t>(rank) *
         DEGREE / 8;
}

constexpr size_t ciphertext_size(int rank) {
  return compressed_vector_size(rank) +
         (rank == RANK768 ? kDV768 : kDV1024) * DEGREE / 8;
}

static_assert(ciphertext_size(RANK768) == BCM_MLKEM768_CIPHERTEXT_BYTES, "");
static_assert(ciphertext_size(RANK1024) == BCM_MLKEM1024_CIPHERTEXT_BYTES, "");

typedef struct scalar {
  // On every function entry and exit, 0 <= c < kPrime.
  uint16_t c[DEGREE];
} scalar;

template <int RANK>
struct vector {
  scalar v[RANK];
};

template <int RANK>
struct matrix {
  scalar v[RANK][RANK];
};

// This bit of Python will be referenced in some of the following comments:
//
// p = 3329
//
// def bitreverse(i):
//     ret = 0
//     for n in range(7):
//         bit = i & 1
//         ret <<= 1
//         ret |= bit
//         i >>= 1
//     return ret

// kNTTRoots = [pow(17, bitreverse(i), p) for i in range(128)]
const uint16_t kNTTRoots[128] = {
    1,    1729, 2580, 3289, 2642, 630,  1897, 848,  1062, 1919, 193,  797,
    2786, 3260, 569,  1746, 296,  2447, 1339, 1476, 3046, 56,   2240, 1333,
    1426, 2094, 535,  2882, 2393, 2879, 1974, 821,  289,  331,  3253, 1756,
    1197, 2304, 2277, 2055, 650,  1977, 2513, 632,  2865, 33,   1320, 1915,
    2319, 1435, 807,  452,  1438, 2868, 1534, 2402, 2647, 2617, 1481, 648,
    2474, 3110, 1227, 910,  17,   2761, 583,  2649, 1637, 723,  2288, 1100,
    1409, 2662, 3281, 233,  756,  2156, 3015, 3050, 1703, 1651, 2789, 1789,
    1847, 952,  1461, 2687, 939,  2308, 2437, 2388, 733,  2337, 268,  641,
    1584, 2298, 2037, 3220, 375,  2549, 2090, 1645, 1063, 319,  2773, 757,
    2099, 561,  2466, 2594, 2804, 1092, 403,  1026, 1143, 2150, 2775, 886,
    1722, 1212, 1874, 1029, 2110, 2935, 885,  2154,
};

// kInverseNTTRoots = [pow(17, -bitreverse(i), p) for i in range(128)]
const uint16_t kInverseNTTRoots[128] = {
    1,    1600, 40,   749,  2481, 1432, 2699, 687,  1583, 2760, 69,   543,
    2532, 3136, 1410, 2267, 2508, 1355, 450,  936,  447,  2794, 1235, 1903,
    1996, 1089, 3273, 283,  1853, 1990, 882,  3033, 2419, 2102, 219,  855,
    2681, 1848, 712,  682,  927,  1795, 461,  1891, 2877, 2522, 1894, 1010,
    1414, 2009, 3296, 464,  2697, 816,  1352, 2679, 1274, 1052, 1025, 2132,
    1573, 76,   2998, 3040, 1175, 2444, 394,  1219, 2300, 1455, 2117, 1607,
    2443, 554,  1179, 2186, 2303, 2926, 2237, 525,  735,  863,  2768, 1230,
    2572, 556,  3010, 2266, 1684, 1239, 780,  2954, 109,  1292, 1031, 1745,
    2688, 3061, 992,  2596, 941,  892,  1021, 2390, 642,  1868, 2377, 1482,
    1540, 540,  1678, 1626, 279,  314,  1173, 2573, 3096, 48,   667,  1920,
    2229, 1041, 2606, 1692, 680,  2746, 568,  3312,
};

// kModRoots = [pow(17, 2*bitreverse(i) + 1, p) for i in range(128)]
const uint16_t kModRoots[128] = {
    17,   3312, 2761, 568,  583,  2746, 2649, 680,  1637, 1692, 723,  2606,
    2288, 1041, 1100, 2229, 1409, 1920, 2662, 667,  3281, 48,   233,  3096,
    756,  2573, 2156, 1173, 3015, 314,  3050, 279,  1703, 1626, 1651, 1678,
    2789, 540,  1789, 1540, 1847, 1482, 952,  2377, 1461, 1868, 2687, 642,
    939,  2390, 2308, 1021, 2437, 892,  2388, 941,  733,  2596, 2337, 992,
    268,  3061, 641,  2688, 1584, 1745, 2298, 1031, 2037, 1292, 3220, 109,
    375,  2954, 2549, 780,  2090, 1239, 1645, 1684, 1063, 2266, 319,  3010,
    2773, 556,  757,  2572, 2099, 1230, 561,  2768, 2466, 863,  2594, 735,
    2804, 525,  1092, 2237, 403,  2926, 1026, 2303, 1143, 2186, 2150, 1179,
    2775, 554,  886,  2443, 1722, 1607, 1212, 2117, 1874, 1455, 1029, 2300,
    2110, 1219, 2935, 394,  885,  2444, 2154, 1175,
};

// reduce_once reduces 0 <= x < 2*kPrime, mod kPrime.
uint16_t reduce_once(uint16_t x) {
  declassify_assert(x < 2 * kPrime);
  const uint16_t subtracted = x - kPrime;
  uint16_t mask = 0u - (subtracted >> 15);
  // Although this is a constant-time select, we omit a value barrier here.
  // Value barriers impede auto-vectorization (likely because it forces the
  // value to transit through a general-purpose register). On AArch64, this is a
  // difference of 2x.
  //
  // We usually add value barriers to selects because Clang turns consecutive
  // selects with the same condition into a branch instead of CMOV/CSEL. This
  // condition does not occur in ML-KEM, so omitting it seems to be safe so far,
  // but see |scalar_centered_binomial_distribution_eta_2_with_prf|.
  return (mask & x) | (~mask & subtracted);
}

// constant time reduce x mod kPrime using Barrett reduction. x must be less
// than kPrime + 2×kPrime².
static uint16_t reduce(uint32_t x) {
  declassify_assert(x < kPrime + 2u * kPrime * kPrime);
  uint64_t product = (uint64_t)x * kBarrettMultiplier;
  uint32_t quotient = (uint32_t)(product >> kBarrettShift);
  uint32_t remainder = x - quotient * kPrime;
  return reduce_once(remainder);
}

void scalar_zero(scalar *out) { OPENSSL_memset(out, 0, sizeof(*out)); }

template <int RANK>
void vector_zero(vector<RANK> *out) {
  OPENSSL_memset(out->v, 0, sizeof(scalar) * RANK);
}

// In place number theoretic transform of a given scalar.
// Note that MLKEM's kPrime 3329 does not have a 512th root of unity, so this
// transform leaves off the last iteration of the usual FFT code, with the 128
// relevant roots of unity being stored in |kNTTRoots|. This means the output
// should be seen as 128 elements in GF(3329^2), with the coefficients of the
// elements being consecutive entries in |s->c|.
static void scalar_ntt(scalar *s) {
  int offset = DEGREE;
  // `int` is used here because using `size_t` throughout caused a ~5% slowdown
  // with Clang 14 on Aarch64.
  for (int step = 1; step < DEGREE / 2; step <<= 1) {
    offset >>= 1;
    int k = 0;
    for (int i = 0; i < step; i++) {
      const uint32_t step_root = kNTTRoots[i + step];
      for (int j = k; j < k + offset; j++) {
        uint16_t odd = reduce(step_root * s->c[j + offset]);
        uint16_t even = s->c[j];
        s->c[j] = reduce_once(odd + even);
        s->c[j + offset] = reduce_once(even - odd + kPrime);
      }
      k += 2 * offset;
    }
  }
}

template <int RANK>
static void vector_ntt(vector<RANK> *a) {
  for (int i = 0; i < RANK; i++) {
    scalar_ntt(&a->v[i]);
  }
}

// In place inverse number theoretic transform of a given scalar, with pairs of
// entries of s->v being interpreted as elements of GF(3329^2). Just as with the
// number theoretic transform, this leaves off the first step of the normal iFFT
// to account for the fact that 3329 does not have a 512th root of unity, using
// the precomputed 128 roots of unity stored in |kInverseNTTRoots|.
void scalar_inverse_ntt(scalar *s) {
  int step = DEGREE / 2;
  // `int` is used here because using `size_t` throughout caused a ~5% slowdown
  // with Clang 14 on Aarch64.
  for (int offset = 2; offset < DEGREE; offset <<= 1) {
    step >>= 1;
    int k = 0;
    for (int i = 0; i < step; i++) {
      uint32_t step_root = kInverseNTTRoots[i + step];
      for (int j = k; j < k + offset; j++) {
        uint16_t odd = s->c[j + offset];
        uint16_t even = s->c[j];
        s->c[j] = reduce_once(odd + even);
        s->c[j + offset] = reduce(step_root * (even - odd + kPrime));
      }
      k += 2 * offset;
    }
  }
  for (int i = 0; i < DEGREE; i++) {
    s->c[i] = reduce(s->c[i] * kInverseDegree);
  }
}

template <int RANK>
void vector_inverse_ntt(vector<RANK> *a) {
  for (int i = 0; i < RANK; i++) {
    scalar_inverse_ntt(&a->v[i]);
  }
}

void scalar_add(scalar *lhs, const scalar *rhs) {
  for (int i = 0; i < DEGREE; i++) {
    lhs->c[i] = reduce_once(lhs->c[i] + rhs->c[i]);
  }
}

void scalar_sub(scalar *lhs, const scalar *rhs) {
  for (int i = 0; i < DEGREE; i++) {
    lhs->c[i] = reduce_once(lhs->c[i] - rhs->c[i] + kPrime);
  }
}

// Multiplying two scalars in the number theoretically transformed state. Since
// 3329 does not have a 512th root of unity, this means we have to interpret
// the 2*ith and (2*i+1)th entries of the scalar as elements of GF(3329)[X]/(X^2
// - 17^(2*bitreverse(i)+1)) The value of 17^(2*bitreverse(i)+1) mod 3329 is
// stored in the precomputed |kModRoots| table. Note that our Barrett transform
// only allows us to multipy two reduced numbers together, so we need some
// intermediate reduction steps, even if an uint64_t could hold 3 multiplied
// numbers.
void scalar_mult(scalar *out, const scalar *lhs, const scalar *rhs) {
  for (int i = 0; i < DEGREE / 2; i++) {
    uint32_t real_real = (uint32_t)lhs->c[2 * i] * rhs->c[2 * i];
    uint32_t img_img = (uint32_t)lhs->c[2 * i + 1] * rhs->c[2 * i + 1];
    uint32_t real_img = (uint32_t)lhs->c[2 * i] * rhs->c[2 * i + 1];
    uint32_t img_real = (uint32_t)lhs->c[2 * i + 1] * rhs->c[2 * i];
    out->c[2 * i] =
        reduce(real_real + (uint32_t)reduce(img_img) * kModRoots[i]);
    out->c[2 * i + 1] = reduce(img_real + real_img);
  }
}

template <int RANK>
void vector_add(vector<RANK> *lhs, const vector<RANK> *rhs) {
  for (int i = 0; i < RANK; i++) {
    scalar_add(&lhs->v[i], &rhs->v[i]);
  }
}

template <int RANK>
static void matrix_mult(vector<RANK> *out, const matrix<RANK> *m,
                        const vector<RANK> *a) {
  vector_zero(out);
  for (int i = 0; i < RANK; i++) {
    for (int j = 0; j < RANK; j++) {
      scalar product;
      scalar_mult(&product, &m->v[i][j], &a->v[j]);
      scalar_add(&out->v[i], &product);
    }
  }
}

template <int RANK>
void matrix_mult_transpose(vector<RANK> *out, const matrix<RANK> *m,
                           const vector<RANK> *a) {
  vector_zero(out);
  for (int i = 0; i < RANK; i++) {
    for (int j = 0; j < RANK; j++) {
      scalar product;
      scalar_mult(&product, &m->v[j][i], &a->v[j]);
      scalar_add(&out->v[i], &product);
    }
  }
}

template <int RANK>
void scalar_inner_product(scalar *out, const vector<RANK> *lhs,
                          const vector<RANK> *rhs) {
  scalar_zero(out);
  for (int i = 0; i < RANK; i++) {
    scalar product;
    scalar_mult(&product, &lhs->v[i], &rhs->v[i]);
    scalar_add(out, &product);
  }
}

// Algorithm 6 from the spec. Rejection samples a Keccak stream to get
// uniformly distributed elements. This is used for matrix expansion and only
// operates on public inputs.
static void scalar_from_keccak_vartime(scalar *out,
                                       struct BORINGSSL_keccak_st *keccak_ctx) {
  assert(keccak_ctx->squeeze_offset == 0);
  assert(keccak_ctx->rate_bytes == 168);
  static_assert(168 % 3 == 0, "block and coefficient boundaries do not align");

  int done = 0;
  while (done < DEGREE) {
    uint8_t block[168];
    BORINGSSL_keccak_squeeze(keccak_ctx, block, sizeof(block));
    for (size_t i = 0; i < sizeof(block) && done < DEGREE; i += 3) {
      uint16_t d1 = block[i] + 256 * (block[i + 1] % 16);
      uint16_t d2 = block[i + 1] / 16 + 16 * block[i + 2];
      if (d1 < kPrime) {
        out->c[done++] = d1;
      }
      if (d2 < kPrime && done < DEGREE) {
        out->c[done++] = d2;
      }
    }
  }
}

// Algorithm 7 from the spec, with eta fixed to two and the PRF call
// included. Creates binominally distributed elements by sampling 2*|eta| bits,
// and setting the coefficient to the count of the first bits minus the count of
// the second bits, resulting in a centered binomial distribution. Since eta is
// two this gives -2/2 with a probability of 1/16, -1/1 with probability 1/4,
// and 0 with probability 3/8.
void scalar_centered_binomial_distribution_eta_2_with_prf(
    scalar *out, const uint8_t input[33]) {
  uint8_t entropy[128];
  static_assert(sizeof(entropy) == 2 * /*kEta=*/2 * DEGREE / 8, "");
  prf(entropy, sizeof(entropy), input);

  for (int i = 0; i < DEGREE; i += 2) {
    uint8_t byte = entropy[i / 2];

    uint16_t value = (byte & 1) + ((byte >> 1) & 1);
    value -= ((byte >> 2) & 1) + ((byte >> 3) & 1);
    // Add |kPrime| if |value| underflowed. See |reduce_once| for a discussion
    // on why the value barrier is omitted. While this could have been written
    // reduce_once(value + kPrime), this is one extra addition and small range
    // of |value| tempts some versions of Clang to emit a branch.
    uint16_t mask = 0u - (value >> 15);
    out->c[i] = ((value + kPrime) & mask) | (value & ~mask);

    byte >>= 4;
    value = (byte & 1) + ((byte >> 1) & 1);
    value -= ((byte >> 2) & 1) + ((byte >> 3) & 1);
    // See above.
    mask = 0u - (value >> 15);
    out->c[i + 1] = ((value + kPrime) & mask) | (value & ~mask);
  }
}

// Generates a secret vector by using
// |scalar_centered_binomial_distribution_eta_2_with_prf|, using the given seed
// appending and incrementing |counter| for entry of the vector.
template <int RANK>
void vector_generate_secret_eta_2(vector<RANK> *out, uint8_t *counter,
                                  const uint8_t seed[32]) {
  uint8_t input[33];
  OPENSSL_memcpy(input, seed, 32);
  for (int i = 0; i < RANK; i++) {
    input[32] = (*counter)++;
    scalar_centered_binomial_distribution_eta_2_with_prf(&out->v[i], input);
  }
}

// Expands the matrix of a seed for key generation and for encaps-CPA.
template <int RANK>
void matrix_expand(matrix<RANK> *out, const uint8_t rho[32]) {
  uint8_t input[34];
  OPENSSL_memcpy(input, rho, 32);
  for (int i = 0; i < RANK; i++) {
    for (int j = 0; j < RANK; j++) {
      input[32] = i;
      input[33] = j;
      struct BORINGSSL_keccak_st keccak_ctx;
      BORINGSSL_keccak_init(&keccak_ctx, boringssl_shake128);
      BORINGSSL_keccak_absorb(&keccak_ctx, input, sizeof(input));
      scalar_from_keccak_vartime(&out->v[i][j], &keccak_ctx);
    }
  }
}

const uint8_t kMasks[8] = {0x01, 0x03, 0x07, 0x0f, 0x1f, 0x3f, 0x7f, 0xff};

void scalar_encode(uint8_t *out, const scalar *s, int bits) {
  assert(bits <= (int)sizeof(*s->c) * 8 && bits != 1);

  uint8_t out_byte = 0;
  int out_byte_bits = 0;

  for (int i = 0; i < DEGREE; i++) {
    uint16_t element = s->c[i];
    int element_bits_done = 0;

    while (element_bits_done < bits) {
      int chunk_bits = bits - element_bits_done;
      int out_bits_remaining = 8 - out_byte_bits;
      if (chunk_bits >= out_bits_remaining) {
        chunk_bits = out_bits_remaining;
        out_byte |= (element & kMasks[chunk_bits - 1]) << out_byte_bits;
        *out = out_byte;
        out++;
        out_byte_bits = 0;
        out_byte = 0;
      } else {
        out_byte |= (element & kMasks[chunk_bits - 1]) << out_byte_bits;
        out_byte_bits += chunk_bits;
      }

      element_bits_done += chunk_bits;
      element >>= chunk_bits;
    }
  }

  if (out_byte_bits > 0) {
    *out = out_byte;
  }
}

// scalar_encode_1 is |scalar_encode| specialised for |bits| == 1.
void scalar_encode_1(uint8_t out[32], const scalar *s) {
  for (int i = 0; i < DEGREE; i += 8) {
    uint8_t out_byte = 0;
    for (int j = 0; j < 8; j++) {
      out_byte |= (s->c[i + j] & 1) << j;
    }
    *out = out_byte;
    out++;
  }
}

// Encodes an entire vector into 32*|RANK|*|bits| bytes. Note that since 256
// (DEGREE) is divisible by 8, the individual vector entries will always fill a
// whole number of bytes, so we do not need to worry about bit packing here.
template <int RANK>
void vector_encode(uint8_t *out, const vector<RANK> *a, int bits) {
  for (int i = 0; i < RANK; i++) {
    scalar_encode(out + i * bits * DEGREE / 8, &a->v[i], bits);
  }
}

// scalar_decode parses |DEGREE * bits| bits from |in| into |DEGREE| values in
// |out|. It returns one on success and zero if any parsed value is >=
// |kPrime|.
int scalar_decode(scalar *out, const uint8_t *in, int bits) {
  assert(bits <= (int)sizeof(*out->c) * 8 && bits != 1);

  uint8_t in_byte = 0;
  int in_byte_bits_left = 0;

  for (int i = 0; i < DEGREE; i++) {
    uint16_t element = 0;
    int element_bits_done = 0;

    while (element_bits_done < bits) {
      if (in_byte_bits_left == 0) {
        in_byte = *in;
        in++;
        in_byte_bits_left = 8;
      }

      int chunk_bits = bits - element_bits_done;
      if (chunk_bits > in_byte_bits_left) {
        chunk_bits = in_byte_bits_left;
      }

      element |= (in_byte & kMasks[chunk_bits - 1]) << element_bits_done;
      in_byte_bits_left -= chunk_bits;
      in_byte >>= chunk_bits;

      element_bits_done += chunk_bits;
    }

    // An element is only out of range in the case of invalid input, in which
    // case it is okay to leak the comparison.
    if (constant_time_declassify_int(element >= kPrime)) {
      return 0;
    }
    out->c[i] = element;
  }

  return 1;
}

// scalar_decode_1 is |scalar_decode| specialised for |bits| == 1.
void scalar_decode_1(scalar *out, const uint8_t in[32]) {
  for (int i = 0; i < DEGREE; i += 8) {
    uint8_t in_byte = *in;
    in++;
    for (int j = 0; j < 8; j++) {
      out->c[i + j] = in_byte & 1;
      in_byte >>= 1;
    }
  }
}

// Decodes 32*|RANK|*|bits| bytes from |in| into |out|. It returns one on
// success or zero if any parsed value is >= |kPrime|.
template <int RANK>
static int vector_decode(vector<RANK> *out, const uint8_t *in, int bits) {
  for (int i = 0; i < RANK; i++) {
    if (!scalar_decode(&out->v[i], in + i * bits * DEGREE / 8, bits)) {
      return 0;
    }
  }
  return 1;
}

// Compresses (lossily) an input |x| mod 3329 into |bits| many bits by grouping
// numbers close to each other together. The formula used is
// round(2^|bits|/kPrime*x) mod 2^|bits|.
// Uses Barrett reduction to achieve constant time. Since we need both the
// remainder (for rounding) and the quotient (as the result), we cannot use
// |reduce| here, but need to do the Barrett reduction directly.
static uint16_t compress(uint16_t x, int bits) {
  uint32_t shifted = (uint32_t)x << bits;
  uint64_t product = (uint64_t)shifted * kBarrettMultiplier;
  uint32_t quotient = (uint32_t)(product >> kBarrettShift);
  uint32_t remainder = shifted - quotient * kPrime;

  // Adjust the quotient to round correctly:
  //   0 <= remainder <= kHalfPrime round to 0
  //   kHalfPrime < remainder <= kPrime + kHalfPrime round to 1
  //   kPrime + kHalfPrime < remainder < 2 * kPrime round to 2
  declassify_assert(remainder < 2u * kPrime);
  quotient += 1 & constant_time_lt_w(kHalfPrime, remainder);
  quotient += 1 & constant_time_lt_w(kPrime + kHalfPrime, remainder);
  return quotient & ((1 << bits) - 1);
}

// Decompresses |x| by using an equi-distant representative. The formula is
// round(kPrime/2^|bits|*x). Note that 2^|bits| being the divisor allows us to
// implement this logic using only bit operations.
uint16_t decompress(uint16_t x, int bits) {
  uint32_t product = (uint32_t)x * kPrime;
  uint32_t power = 1 << bits;
  // This is |product| % power, since |power| is a power of 2.
  uint32_t remainder = product & (power - 1);
  // This is |product| / power, since |power| is a power of 2.
  uint32_t lower = product >> bits;
  // The rounding logic works since the first half of numbers mod |power| have a
  // 0 as first bit, and the second half has a 1 as first bit, since |power| is
  // a power of 2. As a 12 bit number, |remainder| is always positive, so we
  // will shift in 0s for a right shift.
  return lower + (remainder >> (bits - 1));
}

static void scalar_compress(scalar *s, int bits) {
  for (int i = 0; i < DEGREE; i++) {
    s->c[i] = compress(s->c[i], bits);
  }
}

static void scalar_decompress(scalar *s, int bits) {
  for (int i = 0; i < DEGREE; i++) {
    s->c[i] = decompress(s->c[i], bits);
  }
}

template <int RANK>
void vector_compress(vector<RANK> *a, int bits) {
  for (int i = 0; i < RANK; i++) {
    scalar_compress(&a->v[i], bits);
  }
}

template <int RANK>
void vector_decompress(vector<RANK> *a, int bits) {
  for (int i = 0; i < RANK; i++) {
    scalar_decompress(&a->v[i], bits);
  }
}

template <int RANK>
struct public_key {
  vector<RANK> t;
  uint8_t rho[32];
  uint8_t public_key_hash[32];
  matrix<RANK> m;
};

template <int RANK>
struct private_key {
  struct public_key<RANK> pub;
  vector<RANK> s;
  uint8_t fo_failure_secret[32];
};

template <int RANK>
static void decrypt_cpa(
    uint8_t out[32], const struct private_key<RANK> *priv,
    const uint8_t ciphertext[BCM_MLKEM768_CIPHERTEXT_BYTES]) {
  constexpr int du = RANK == RANK768 ? kDU768 : kDU1024;
  constexpr int dv = RANK == RANK768 ? kDV768 : kDV1024;

  vector<RANK> u;
  vector_decode(&u, ciphertext, du);
  vector_decompress(&u, du);
  vector_ntt(&u);
  scalar v;
  scalar_decode(&v, ciphertext + compressed_vector_size(RANK), dv);
  scalar_decompress(&v, dv);
  scalar mask;
  scalar_inner_product(&mask, &priv->s, &u);
  scalar_inverse_ntt(&mask);
  scalar_sub(&v, &mask);
  scalar_compress(&v, 1);
  scalar_encode_1(out, &v);
}

template <int RANK>
static bcm_status mlkem_marshal_public_key(CBB *out,
                                           const struct public_key<RANK> *pub) {
  uint8_t *vector_output;
  if (!CBB_add_space(out, &vector_output, encoded_vector_size(RANK))) {
    return bcm_status::failure;
  }
  vector_encode(vector_output, &pub->t, kLog2Prime);
  if (!CBB_add_bytes(out, pub->rho, sizeof(pub->rho))) {
    return bcm_status::failure;
  }
  return bcm_status::approved;
}

template <int RANK>
void mlkem_generate_key_external_seed_no_self_test(
    uint8_t *out_encoded_public_key, private_key<RANK> *priv,
    const uint8_t seed[BCM_MLKEM_SEED_BYTES]) {
  uint8_t augmented_seed[33];
  OPENSSL_memcpy(augmented_seed, seed, 32);
  augmented_seed[32] = RANK;

  uint8_t hashed[64];
  hash_g(hashed, augmented_seed, sizeof(augmented_seed));
  const uint8_t *const rho = hashed;
  const uint8_t *const sigma = hashed + 32;
  // rho is public.
  CONSTTIME_DECLASSIFY(rho, 32);
  OPENSSL_memcpy(priv->pub.rho, hashed, sizeof(priv->pub.rho));
  matrix_expand(&priv->pub.m, rho);
  uint8_t counter = 0;
  vector_generate_secret_eta_2(&priv->s, &counter, sigma);
  vector_ntt(&priv->s);
  vector<RANK> error;
  vector_generate_secret_eta_2(&error, &counter, sigma);
  vector_ntt(&error);
  matrix_mult_transpose(&priv->pub.t, &priv->pub.m, &priv->s);
  vector_add(&priv->pub.t, &error);
  // t is part of the public key and thus is public.
  CONSTTIME_DECLASSIFY(&priv->pub.t, sizeof(priv->pub.t));

  CBB cbb;
  CBB_init_fixed(&cbb, out_encoded_public_key, encoded_public_key_size(RANK));
  if (!bcm_success(mlkem_marshal_public_key(&cbb, &priv->pub))) {
    abort();
  }

  hash_h(priv->pub.public_key_hash, out_encoded_public_key,
         encoded_public_key_size(RANK));
  OPENSSL_memcpy(priv->fo_failure_secret, seed + 32, 32);
}

template <int RANK>
void mlkem_generate_key_external_seed(
    uint8_t *out_encoded_public_key, private_key<RANK> *priv,
    const uint8_t seed[BCM_MLKEM_SEED_BYTES]) {
  fips::ensure_keygen_self_test();
  mlkem_generate_key_external_seed_no_self_test(out_encoded_public_key, priv,
                                                seed);
}

// Encrypts a message with given randomness to
// the ciphertext in |out|. Without applying the Fujisaki-Okamoto transform this
// would not result in a CCA secure scheme, since lattice schemes are vulnerable
// to decryption failure oracles.
template <int RANK>
void encrypt_cpa(uint8_t *out, const struct mlkem::public_key<RANK> *pub,
                 const uint8_t message[32], const uint8_t randomness[32]) {
  constexpr int du = RANK == RANK768 ? mlkem::kDU768 : mlkem::kDU1024;
  constexpr int dv = RANK == RANK768 ? mlkem::kDV768 : mlkem::kDV1024;

  uint8_t counter = 0;
  mlkem::vector<RANK> secret;
  vector_generate_secret_eta_2(&secret, &counter, randomness);
  vector_ntt(&secret);
  mlkem::vector<RANK> error;
  vector_generate_secret_eta_2(&error, &counter, randomness);
  uint8_t input[33];
  OPENSSL_memcpy(input, randomness, 32);
  input[32] = counter;
  mlkem::scalar scalar_error;
  scalar_centered_binomial_distribution_eta_2_with_prf(&scalar_error, input);
  mlkem::vector<RANK> u;
  matrix_mult(&u, &pub->m, &secret);
  vector_inverse_ntt(&u);
  vector_add(&u, &error);
  mlkem::scalar v;
  scalar_inner_product(&v, &pub->t, &secret);
  scalar_inverse_ntt(&v);
  scalar_add(&v, &scalar_error);
  mlkem::scalar expanded_message;
  scalar_decode_1(&expanded_message, message);
  scalar_decompress(&expanded_message, 1);
  scalar_add(&v, &expanded_message);
  vector_compress(&u, du);
  vector_encode(out, &u, du);
  scalar_compress(&v, dv);
  scalar_encode(out + mlkem::compressed_vector_size(RANK), &v, dv);
}

// See section 6.3
template <int RANK>
void mlkem_decap_no_self_test(
    uint8_t out_shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES],
    const uint8_t *ciphertext, const struct private_key<RANK> *priv) {
  uint8_t decrypted[64];
  decrypt_cpa(decrypted, priv, ciphertext);
  OPENSSL_memcpy(decrypted + 32, priv->pub.public_key_hash,
                 sizeof(decrypted) - 32);
  uint8_t key_and_randomness[64];
  hash_g(key_and_randomness, decrypted, sizeof(decrypted));
  constexpr size_t ciphertext_len = ciphertext_size(RANK);
  uint8_t expected_ciphertext[BCM_MLKEM1024_CIPHERTEXT_BYTES];
  static_assert(ciphertext_len <= sizeof(expected_ciphertext), "");
  encrypt_cpa(expected_ciphertext, &priv->pub, decrypted,
              key_and_randomness + 32);

  uint8_t failure_key[32];
  kdf(failure_key, priv->fo_failure_secret, ciphertext, ciphertext_len);

  uint8_t mask = constant_time_eq_int_8(
      CRYPTO_memcmp(ciphertext, expected_ciphertext, ciphertext_len), 0);
  for (int i = 0; i < BCM_MLKEM_SHARED_SECRET_BYTES; i++) {
    out_shared_secret[i] =
        constant_time_select_8(mask, key_and_randomness[i], failure_key[i]);
  }
}

template <int RANK>
void mlkem_decap(uint8_t out_shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES],
                 const uint8_t *ciphertext,
                 const struct private_key<RANK> *priv) {
  fips::ensure_decap_self_test();
  mlkem_decap_no_self_test(out_shared_secret, ciphertext, priv);
}

// mlkem_parse_public_key_no_hash parses |in| into |pub| but doesn't calculate
// the value of |pub->public_key_hash|.
template <int RANK>
int mlkem_parse_public_key_no_hash(struct public_key<RANK> *pub, CBS *in) {
  CBS t_bytes;
  if (!CBS_get_bytes(in, &t_bytes, encoded_vector_size(RANK)) ||
      !vector_decode(&pub->t, CBS_data(&t_bytes), kLog2Prime) ||
      !CBS_copy_bytes(in, pub->rho, sizeof(pub->rho))) {
    return 0;
  }
  matrix_expand(&pub->m, pub->rho);
  return 1;
}

template <int RANK>
int mlkem_parse_public_key(struct public_key<RANK> *pub, CBS *in) {
  CBS orig_in = *in;
  if (!mlkem_parse_public_key_no_hash(pub, in) ||  //
      CBS_len(in) != 0) {
    return 0;
  }
  hash_h(pub->public_key_hash, CBS_data(&orig_in), CBS_len(&orig_in));
  return 1;
}

template <int RANK>
int mlkem_parse_private_key(struct private_key<RANK> *priv, CBS *in) {
  CBS s_bytes;
  if (!CBS_get_bytes(in, &s_bytes, encoded_vector_size(RANK)) ||
      !vector_decode(&priv->s, CBS_data(&s_bytes), kLog2Prime) ||
      !mlkem_parse_public_key_no_hash(&priv->pub, in) ||
      !CBS_copy_bytes(in, priv->pub.public_key_hash,
                      sizeof(priv->pub.public_key_hash)) ||
      !CBS_copy_bytes(in, priv->fo_failure_secret,
                      sizeof(priv->fo_failure_secret)) ||
      CBS_len(in) != 0) {
    return 0;
  }
  return 1;
}

template <int RANK>
int mlkem_marshal_private_key(CBB *out, const struct private_key<RANK> *priv) {
  uint8_t *s_output;
  if (!CBB_add_space(out, &s_output, encoded_vector_size(RANK))) {
    return 0;
  }
  vector_encode(s_output, &priv->s, kLog2Prime);
  if (!bcm_success(mlkem_marshal_public_key(out, &priv->pub)) ||
      !CBB_add_bytes(out, priv->pub.public_key_hash,
                     sizeof(priv->pub.public_key_hash)) ||
      !CBB_add_bytes(out, priv->fo_failure_secret,
                     sizeof(priv->fo_failure_secret))) {
    return 0;
  }
  return 1;
}

struct public_key<RANK768> *public_key_768_from_external(
    const struct BCM_mlkem768_public_key *external) {
  static_assert(sizeof(struct BCM_mlkem768_public_key) >=
                    sizeof(struct public_key<RANK768>),
                "MLKEM public key is too small");
  static_assert(alignof(struct BCM_mlkem768_public_key) >=
                    alignof(struct public_key<RANK768>),
                "MLKEM public key alignment incorrect");
  return (struct public_key<RANK768> *)external;
}

static struct public_key<RANK1024> *public_key_1024_from_external(
    const struct BCM_mlkem1024_public_key *external) {
  static_assert(sizeof(struct BCM_mlkem1024_public_key) >=
                    sizeof(struct public_key<RANK1024>),
                "MLKEM1024 public key is too small");
  static_assert(alignof(struct BCM_mlkem1024_public_key) >=
                    alignof(struct public_key<RANK1024>),
                "MLKEM1024 public key alignment incorrect");
  return (struct public_key<RANK1024> *)external;
}

struct private_key<RANK768> *private_key_768_from_external(
    const struct BCM_mlkem768_private_key *external) {
  static_assert(sizeof(struct BCM_mlkem768_private_key) >=
                    sizeof(struct private_key<RANK768>),
                "MLKEM private key too small");
  static_assert(alignof(struct BCM_mlkem768_private_key) >=
                    alignof(struct private_key<RANK768>),
                "MLKEM private key alignment incorrect");
  return (struct private_key<RANK768> *)external;
}

struct private_key<RANK1024> *private_key_1024_from_external(
    const struct BCM_mlkem1024_private_key *external) {
  static_assert(sizeof(struct BCM_mlkem1024_private_key) >=
                    sizeof(struct private_key<RANK1024>),
                "MLKEM1024 private key too small");
  static_assert(alignof(struct BCM_mlkem1024_private_key) >=
                    alignof(struct private_key<RANK1024>),
                "MLKEM1024 private key alignment incorrect");
  return (struct private_key<RANK1024> *)external;
}

// See section 6.2.
template <int RANK>
void mlkem_encap_external_entropy_no_self_test(
    uint8_t *out_ciphertext,
    uint8_t out_shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES],
    const struct mlkem::public_key<RANK> *pub,
    const uint8_t entropy[BCM_MLKEM_ENCAP_ENTROPY]) {
  uint8_t input[64];
  OPENSSL_memcpy(input, entropy, BCM_MLKEM_ENCAP_ENTROPY);
  OPENSSL_memcpy(input + BCM_MLKEM_ENCAP_ENTROPY, pub->public_key_hash,
                 sizeof(input) - BCM_MLKEM_ENCAP_ENTROPY);
  uint8_t key_and_randomness[64];
  mlkem::hash_g(key_and_randomness, input, sizeof(input));
  encrypt_cpa(out_ciphertext, pub, entropy, key_and_randomness + 32);
  // The ciphertext is public.
  CONSTTIME_DECLASSIFY(out_ciphertext, mlkem::ciphertext_size(RANK));
  static_assert(BCM_MLKEM_SHARED_SECRET_BYTES == 32, "");
  memcpy(out_shared_secret, key_and_randomness, 32);
}

template <int RANK>
void mlkem_encap_external_entropy(
    uint8_t *out_ciphertext,
    uint8_t out_shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES],
    const struct mlkem::public_key<RANK> *pub,
    const uint8_t entropy[BCM_MLKEM_ENCAP_ENTROPY]) {
  fips::ensure_encap_self_test();
  mlkem_encap_external_entropy_no_self_test(out_ciphertext, out_shared_secret,
                                            pub, entropy);
}

namespace fips {

#include "fips_known_values.inc"

static int keygen_self_test() {
  uint8_t pub_key[BCM_MLKEM768_PUBLIC_KEY_BYTES];
  private_key<RANK768> priv;
  static_assert(sizeof(kTestEntropy) >= BCM_MLKEM_SEED_BYTES);
  mlkem_generate_key_external_seed_no_self_test(pub_key, &priv, kTestEntropy);
  CBB cbb;
  constexpr size_t kMarshaledPrivateKeySize = 2400;
  uint8_t priv_bytes[kMarshaledPrivateKeySize];
  CBB_init_fixed(&cbb, priv_bytes, sizeof(priv_bytes));
  static_assert(sizeof(kExpectedPrivateKeyBytes) == kMarshaledPrivateKeySize);
  static_assert(sizeof(kExpectedPublicKeyBytes) == sizeof(pub_key));
  if (!mlkem_marshal_private_key(&cbb, &priv) ||
      !BORINGSSL_check_test(kExpectedPrivateKeyBytes, priv_bytes,
                            sizeof(priv_bytes), "ML-KEM keygen private key") ||
      !BORINGSSL_check_test(kExpectedPublicKeyBytes, pub_key, sizeof(pub_key),
                            "ML-KEM keygen public key")) {
    return 0;
  }
  return 1;
}

static int encap_self_test() {
  CBS cbs;
  CBS_init(&cbs, kExpectedPublicKeyBytes, sizeof(kExpectedPublicKeyBytes));
  public_key<RANK768> pub;
  if (!mlkem_parse_public_key(&pub, &cbs)) {
    return 0;
  }
  uint8_t ciphertext[BCM_MLKEM768_CIPHERTEXT_BYTES];
  uint8_t shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES];
  static_assert(sizeof(kTestEntropy) >= BCM_MLKEM_ENCAP_ENTROPY);
  mlkem_encap_external_entropy_no_self_test(ciphertext, shared_secret, &pub,
                                            kTestEntropy);
  if (!BORINGSSL_check_test(ciphertext, kExpectedCiphertext, sizeof(ciphertext),
                            "ML-KEM encap ciphertext") ||
      !BORINGSSL_check_test(kExpectedSharedSecret, shared_secret,
                            sizeof(kExpectedSharedSecret),
                            "ML-KEM encap shared secret")) {
    return 0;
  }
  return 1;
}

static int decap_self_test() {
  CBS cbs;
  CBS_init(&cbs, kExpectedPrivateKeyBytes, sizeof(kExpectedPrivateKeyBytes));
  private_key<RANK768> priv;
  if (!mlkem_parse_private_key(&priv, &cbs)) {
    return 0;
  }
  uint8_t shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES];
  mlkem_decap_no_self_test(shared_secret, kExpectedCiphertext, &priv);
  static_assert(sizeof(kExpectedSharedSecret) == sizeof(shared_secret));
  if (!BORINGSSL_check_test(kExpectedSharedSecret, shared_secret,
                            sizeof(shared_secret),
                            "ML-KEM decap shared secret")) {
    return 0;
  }

  uint8_t implicit_rejection_shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES];
  static_assert(sizeof(kExpectedPrivateKeyBytes) >=
                sizeof(kExpectedCiphertext));
  mlkem_decap_no_self_test(implicit_rejection_shared_secret,
                           kExpectedPrivateKeyBytes, &priv);
  static_assert(sizeof(kExpectedImplicitRejectionSharedSecret) ==
                sizeof(implicit_rejection_shared_secret));
  if (!BORINGSSL_check_test(kExpectedImplicitRejectionSharedSecret,
                            implicit_rejection_shared_secret,
                            sizeof(implicit_rejection_shared_secret),
                            "ML-KEM decap implicit rejection shared secret")) {
    return 0;
  }
  return 1;
}

#if defined(BORINGSSL_FIPS)

DEFINE_STATIC_ONCE(g_mlkem_keygen_self_test_once)

void ensure_keygen_self_test(void) {
  CRYPTO_once(g_mlkem_keygen_self_test_once_bss_get(), []() {
    if (!keygen_self_test()) {
      BORINGSSL_FIPS_abort();
    }
  });
}

DEFINE_STATIC_ONCE(g_mlkem_encap_self_test_once)

void ensure_encap_self_test(void) {
  CRYPTO_once(g_mlkem_encap_self_test_once_bss_get(), []() {
    if (!encap_self_test()) {
      BORINGSSL_FIPS_abort();
    }
  });
}

DEFINE_STATIC_ONCE(g_mlkem_decap_self_test_once)

void ensure_decap_self_test(void) {
  CRYPTO_once(g_mlkem_decap_self_test_once_bss_get(), []() {
    if (!decap_self_test()) {
      BORINGSSL_FIPS_abort();
    }
  });
}

#else

void ensure_keygen_self_test(void) {}
void ensure_encap_self_test(void) {}
void ensure_decap_self_test(void) {}

#endif
}  // namespace fips

}  // namespace
}  // namespace mlkem

bcm_status BCM_mlkem768_check_fips(
    const struct BCM_mlkem768_private_key *private_key) {
  mlkem::private_key<RANK768> *priv =
      mlkem::private_key_768_from_external(private_key);

  const uint8_t entropy[BCM_MLKEM_ENCAP_ENTROPY] = {1, 2, 3, 4};
  uint8_t ciphertext[BCM_MLKEM768_CIPHERTEXT_BYTES];
  uint8_t shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES];
  mlkem_encap_external_entropy_no_self_test(ciphertext, shared_secret,
                                            &priv->pub, entropy);

  if (boringssl_fips_break_test("MLKEM_PWCT")) {
    shared_secret[0] ^= 1;
  }

  uint8_t shared_secret2[BCM_MLKEM_SHARED_SECRET_BYTES];
  mlkem::mlkem_decap_no_self_test(shared_secret2, ciphertext, priv);
  if (CRYPTO_memcmp(shared_secret, shared_secret2, sizeof(shared_secret)) !=
      0) {
    return bcm_status::failure;
  }
  return bcm_status::approved;
}

bcm_status BCM_mlkem768_generate_key_fips(
    uint8_t out_encoded_public_key[BCM_MLKEM768_PUBLIC_KEY_BYTES],
    uint8_t optional_out_seed[BCM_MLKEM_SEED_BYTES],
    struct BCM_mlkem768_private_key *out_private_key) {
  if (out_encoded_public_key == nullptr || out_private_key == nullptr) {
    return bcm_status::failure;
  }
  BCM_mlkem768_generate_key(out_encoded_public_key, optional_out_seed,
                            out_private_key);
  return BCM_mlkem768_check_fips(out_private_key);
}

bcm_infallible BCM_mlkem768_generate_key(
    uint8_t out_encoded_public_key[BCM_MLKEM768_PUBLIC_KEY_BYTES],
    uint8_t optional_out_seed[BCM_MLKEM_SEED_BYTES],
    struct BCM_mlkem768_private_key *out_private_key) {
  uint8_t seed[BCM_MLKEM_SEED_BYTES];
  BCM_rand_bytes(seed, sizeof(seed));
  CONSTTIME_SECRET(seed, sizeof(seed));
  if (optional_out_seed) {
    OPENSSL_memcpy(optional_out_seed, seed, sizeof(seed));
  }
  BCM_mlkem768_generate_key_external_seed(out_encoded_public_key,
                                          out_private_key, seed);
  return bcm_infallible::not_approved;
}

bcm_status BCM_mlkem768_private_key_from_seed(
    struct BCM_mlkem768_private_key *out_private_key, const uint8_t *seed,
    size_t seed_len) {
  if (seed_len != BCM_MLKEM_SEED_BYTES) {
    return bcm_status::failure;
  }

  uint8_t public_key_bytes[BCM_MLKEM768_PUBLIC_KEY_BYTES];
  BCM_mlkem768_generate_key_external_seed(public_key_bytes, out_private_key,
                                          seed);
  return bcm_status::not_approved;
}

bcm_status BCM_mlkem1024_check_fips(
    const struct BCM_mlkem1024_private_key *private_key) {
  mlkem::private_key<RANK1024> *priv =
      mlkem::private_key_1024_from_external(private_key);

  const uint8_t entropy[BCM_MLKEM_ENCAP_ENTROPY] = {1, 2, 3, 4};
  uint8_t ciphertext[BCM_MLKEM1024_CIPHERTEXT_BYTES];
  uint8_t shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES];
  mlkem_encap_external_entropy_no_self_test(ciphertext, shared_secret,
                                            &priv->pub, entropy);

  if (boringssl_fips_break_test("MLKEM_PWCT")) {
    shared_secret[0] ^= 1;
  }

  uint8_t shared_secret2[BCM_MLKEM_SHARED_SECRET_BYTES];
  mlkem::mlkem_decap_no_self_test(shared_secret2, ciphertext, priv);
  if (CRYPTO_memcmp(shared_secret, shared_secret2, sizeof(shared_secret)) !=
      0) {
    return bcm_status::failure;
  }
  return bcm_status::approved;
}

bcm_status BCM_mlkem1024_generate_key_fips(
    uint8_t out_encoded_public_key[BCM_MLKEM1024_PUBLIC_KEY_BYTES],
    uint8_t optional_out_seed[BCM_MLKEM_SEED_BYTES],
    struct BCM_mlkem1024_private_key *out_private_key) {
  if (out_encoded_public_key == nullptr || out_private_key == nullptr) {
    return bcm_status::failure;
  }
  BCM_mlkem1024_generate_key(out_encoded_public_key, optional_out_seed,
                             out_private_key);
  return BCM_mlkem1024_check_fips(out_private_key);
}

bcm_infallible BCM_mlkem1024_generate_key(
    uint8_t out_encoded_public_key[BCM_MLKEM1024_PUBLIC_KEY_BYTES],
    uint8_t optional_out_seed[BCM_MLKEM_SEED_BYTES],
    struct BCM_mlkem1024_private_key *out_private_key) {
  uint8_t seed[BCM_MLKEM_SEED_BYTES];
  BCM_rand_bytes(seed, sizeof(seed));
  CONSTTIME_SECRET(seed, sizeof(seed));
  if (optional_out_seed) {
    OPENSSL_memcpy(optional_out_seed, seed, sizeof(seed));
  }
  BCM_mlkem1024_generate_key_external_seed(out_encoded_public_key,
                                           out_private_key, seed);
  return bcm_infallible::not_approved;
}

bcm_status BCM_mlkem1024_private_key_from_seed(
    struct BCM_mlkem1024_private_key *out_private_key, const uint8_t *seed,
    size_t seed_len) {
  if (seed_len != BCM_MLKEM_SEED_BYTES) {
    return bcm_status::failure;
  }
  uint8_t public_key_bytes[BCM_MLKEM1024_PUBLIC_KEY_BYTES];
  BCM_mlkem1024_generate_key_external_seed(public_key_bytes, out_private_key,
                                           seed);
  return bcm_status::not_approved;
}

bcm_infallible BCM_mlkem768_generate_key_external_seed(
    uint8_t out_encoded_public_key[BCM_MLKEM768_PUBLIC_KEY_BYTES],
    struct BCM_mlkem768_private_key *out_private_key,
    const uint8_t seed[BCM_MLKEM_SEED_BYTES]) {
  mlkem::private_key<RANK768> *priv =
      mlkem::private_key_768_from_external(out_private_key);
  mlkem_generate_key_external_seed(out_encoded_public_key, priv, seed);
  return bcm_infallible::approved;
}

bcm_infallible BCM_mlkem1024_generate_key_external_seed(
    uint8_t out_encoded_public_key[BCM_MLKEM1024_PUBLIC_KEY_BYTES],
    struct BCM_mlkem1024_private_key *out_private_key,
    const uint8_t seed[BCM_MLKEM_SEED_BYTES]) {
  mlkem::private_key<RANK1024> *priv =
      mlkem::private_key_1024_from_external(out_private_key);
  mlkem_generate_key_external_seed(out_encoded_public_key, priv, seed);
  return bcm_infallible::approved;
}

bcm_infallible BCM_mlkem768_public_from_private(
    struct BCM_mlkem768_public_key *out_public_key,
    const struct BCM_mlkem768_private_key *private_key) {
  struct mlkem::public_key<RANK768> *const pub =
      mlkem::public_key_768_from_external(out_public_key);
  const struct mlkem::private_key<RANK768> *const priv =
      mlkem::private_key_768_from_external(private_key);
  *pub = priv->pub;
  return bcm_infallible::approved;
}

bcm_infallible BCM_mlkem1024_public_from_private(
    struct BCM_mlkem1024_public_key *out_public_key,
    const struct BCM_mlkem1024_private_key *private_key) {
  struct mlkem::public_key<RANK1024> *const pub =
      mlkem::public_key_1024_from_external(out_public_key);
  const struct mlkem::private_key<RANK1024> *const priv =
      mlkem::private_key_1024_from_external(private_key);
  *pub = priv->pub;
  return bcm_infallible::approved;
}

// Calls |MLKEM768_encap_external_entropy| with random bytes from
// |BCM_rand_bytes|
bcm_infallible BCM_mlkem768_encap(
    uint8_t out_ciphertext[BCM_MLKEM768_CIPHERTEXT_BYTES],
    uint8_t out_shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES],
    const struct BCM_mlkem768_public_key *public_key) {
  uint8_t entropy[BCM_MLKEM_ENCAP_ENTROPY];
  BCM_rand_bytes(entropy, BCM_MLKEM_ENCAP_ENTROPY);
  CONSTTIME_SECRET(entropy, BCM_MLKEM_ENCAP_ENTROPY);
  BCM_mlkem768_encap_external_entropy(out_ciphertext, out_shared_secret,
                                      public_key, entropy);
  return bcm_infallible::approved;
}

bcm_infallible BCM_mlkem1024_encap(
    uint8_t out_ciphertext[BCM_MLKEM1024_CIPHERTEXT_BYTES],
    uint8_t out_shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES],
    const struct BCM_mlkem1024_public_key *public_key) {
  uint8_t entropy[BCM_MLKEM_ENCAP_ENTROPY];
  BCM_rand_bytes(entropy, BCM_MLKEM_ENCAP_ENTROPY);
  CONSTTIME_SECRET(entropy, BCM_MLKEM_ENCAP_ENTROPY);
  BCM_mlkem1024_encap_external_entropy(out_ciphertext, out_shared_secret,
                                       public_key, entropy);
  return bcm_infallible::approved;
}

bcm_infallible BCM_mlkem768_encap_external_entropy(
    uint8_t out_ciphertext[BCM_MLKEM768_CIPHERTEXT_BYTES],
    uint8_t out_shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES],
    const struct BCM_mlkem768_public_key *public_key,
    const uint8_t entropy[BCM_MLKEM_ENCAP_ENTROPY]) {
  const struct mlkem::public_key<RANK768> *pub =
      mlkem::public_key_768_from_external(public_key);
  mlkem_encap_external_entropy(out_ciphertext, out_shared_secret, pub, entropy);
  return bcm_infallible::approved;
}

bcm_infallible BCM_mlkem1024_encap_external_entropy(
    uint8_t out_ciphertext[BCM_MLKEM1024_CIPHERTEXT_BYTES],
    uint8_t out_shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES],
    const struct BCM_mlkem1024_public_key *public_key,
    const uint8_t entropy[BCM_MLKEM_ENCAP_ENTROPY]) {
  const struct mlkem::public_key<RANK1024> *pub =
      mlkem::public_key_1024_from_external(public_key);
  mlkem_encap_external_entropy(out_ciphertext, out_shared_secret, pub, entropy);
  return bcm_infallible::approved;
}

bcm_status BCM_mlkem768_decap(
    uint8_t out_shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES],
    const uint8_t *ciphertext, size_t ciphertext_len,
    const struct BCM_mlkem768_private_key *private_key) {
  if (ciphertext_len != BCM_MLKEM768_CIPHERTEXT_BYTES) {
    BCM_rand_bytes(out_shared_secret, BCM_MLKEM_SHARED_SECRET_BYTES);
    return bcm_status::failure;
  }
  const struct mlkem::private_key<RANK768> *priv =
      mlkem::private_key_768_from_external(private_key);
  mlkem_decap(out_shared_secret, ciphertext, priv);
  return bcm_status::approved;
}

bcm_status BCM_mlkem1024_decap(
    uint8_t out_shared_secret[BCM_MLKEM_SHARED_SECRET_BYTES],
    const uint8_t *ciphertext, size_t ciphertext_len,
    const struct BCM_mlkem1024_private_key *private_key) {
  if (ciphertext_len != BCM_MLKEM1024_CIPHERTEXT_BYTES) {
    BCM_rand_bytes(out_shared_secret, BCM_MLKEM_SHARED_SECRET_BYTES);
    return bcm_status::failure;
  }
  const struct mlkem::private_key<RANK1024> *priv =
      mlkem::private_key_1024_from_external(private_key);
  mlkem_decap(out_shared_secret, ciphertext, priv);
  return bcm_status::approved;
}

bcm_status BCM_mlkem768_marshal_public_key(
    CBB *out, const struct BCM_mlkem768_public_key *public_key) {
  return mlkem_marshal_public_key(
      out, mlkem::public_key_768_from_external(public_key));
}

bcm_status BCM_mlkem1024_marshal_public_key(
    CBB *out, const struct BCM_mlkem1024_public_key *public_key) {
  return mlkem_marshal_public_key(
      out, mlkem::public_key_1024_from_external(public_key));
}

bcm_status BCM_mlkem768_parse_public_key(
    struct BCM_mlkem768_public_key *public_key, CBS *in) {
  struct mlkem::public_key<RANK768> *pub =
      mlkem::public_key_768_from_external(public_key);
  if (!mlkem_parse_public_key(pub, in)) {
    return bcm_status::failure;
  }
  return bcm_status::approved;
}

bcm_status BCM_mlkem1024_parse_public_key(
    struct BCM_mlkem1024_public_key *public_key, CBS *in) {
  struct mlkem::public_key<RANK1024> *pub =
      mlkem::public_key_1024_from_external(public_key);
  if (!mlkem_parse_public_key(pub, in)) {
    return bcm_status::failure;
  }
  return bcm_status::approved;
}

bcm_status BCM_mlkem768_marshal_private_key(
    CBB *out, const struct BCM_mlkem768_private_key *private_key) {
  const struct mlkem::private_key<RANK768> *const priv =
      mlkem::private_key_768_from_external(private_key);
  if (!mlkem_marshal_private_key(out, priv)) {
    return bcm_status::failure;
  }
  return bcm_status::approved;
}

bcm_status BCM_mlkem1024_marshal_private_key(
    CBB *out, const struct BCM_mlkem1024_private_key *private_key) {
  const struct mlkem::private_key<RANK1024> *const priv =
      mlkem::private_key_1024_from_external(private_key);
  if (!mlkem_marshal_private_key(out, priv)) {
    return bcm_status::failure;
  }
  return bcm_status::approved;
}

bcm_status BCM_mlkem768_parse_private_key(
    struct BCM_mlkem768_private_key *out_private_key, CBS *in) {
  struct mlkem::private_key<RANK768> *const priv =
      mlkem::private_key_768_from_external(out_private_key);
  if (!mlkem_parse_private_key(priv, in)) {
    return bcm_status::failure;
  }
  return bcm_status::approved;
}

bcm_status BCM_mlkem1024_parse_private_key(
    struct BCM_mlkem1024_private_key *out_private_key, CBS *in) {
  struct mlkem::private_key<RANK1024> *const priv =
      mlkem::private_key_1024_from_external(out_private_key);
  if (!mlkem_parse_private_key(priv, in)) {
    return bcm_status::failure;
  }
  return bcm_status::approved;
}

int boringssl_self_test_mlkem() {
  return mlkem::fips::keygen_self_test() && mlkem::fips::encap_self_test() &&
         mlkem::fips::decap_self_test();
}
