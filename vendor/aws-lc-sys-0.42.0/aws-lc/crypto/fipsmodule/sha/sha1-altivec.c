// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

// Altivec-optimized SHA1 in C. This is tested on ppc64le only.
//
// References:
// https://software.intel.com/en-us/articles/improving-the-performance-of-the-secure-hash-algorithm-1
// http://arctic.org/~dean/crypto/sha1.html
//
// This code used the generic SHA-1 from OpenSSL as a basis and AltiVec
// optimisations were added on top.

#include <openssl/sha.h>

#if defined(OPENSSL_PPC64LE)

#include <altivec.h>

static uint32_t rotate(uint32_t a, int n) { return (a << n) | (a >> (32 - n)); }

typedef vector unsigned int vec_uint32_t;
typedef vector unsigned char vec_uint8_t;

// Vector constants
static const vec_uint8_t k_swap_endianness = {3,  2,  1, 0, 7,  6,  5,  4,
                                              11, 10, 9, 8, 15, 14, 13, 12};

// Shift amounts for byte and bit shifts and rotations
static const vec_uint8_t k_4_bytes = {32, 32, 32, 32, 32, 32, 32, 32,
                                      32, 32, 32, 32, 32, 32, 32, 32};
static const vec_uint8_t k_12_bytes = {96, 96, 96, 96, 96, 96, 96, 96,
                                       96, 96, 96, 96, 96, 96, 96, 96};

#define K_00_19 0x5a827999UL
#define K_20_39 0x6ed9eba1UL
#define K_40_59 0x8f1bbcdcUL
#define K_60_79 0xca62c1d6UL

// Vector versions of the above.
static const vec_uint32_t K_00_19_x_4 = {K_00_19, K_00_19, K_00_19, K_00_19};
static const vec_uint32_t K_20_39_x_4 = {K_20_39, K_20_39, K_20_39, K_20_39};
static const vec_uint32_t K_40_59_x_4 = {K_40_59, K_40_59, K_40_59, K_40_59};
static const vec_uint32_t K_60_79_x_4 = {K_60_79, K_60_79, K_60_79, K_60_79};

// vector message scheduling: compute message schedule for round i..i+3 where i
// is divisible by 4. We return the schedule w[i..i+3] as a vector. In
// addition, we also precompute sum w[i..+3] and an additive constant K. This
// is done to offload some computation of f() in the integer execution units.
//
// Byte shifting code below may not be correct for big-endian systems.
static vec_uint32_t sched_00_15(vec_uint32_t *pre_added, const void *data,
                                vec_uint32_t k) {
  const vector unsigned char unaligned_data =
    vec_vsx_ld(0, (const unsigned char*) data);
  const vec_uint32_t v = (vec_uint32_t) unaligned_data;
  const vec_uint32_t w = vec_perm(v, v, k_swap_endianness);
  vec_st(w + k, 0, pre_added);
  return w;
}

// Compute w[i..i+3] using these steps for i in [16, 20, 24, 28]
//
// w'[i  ]  = (w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16]) <<< 1
// w'[i+1]  = (w[i-2] ^ w[i-7] ^ w[i-13] ^ w[i-15]) <<< 1
// w'[i+2]  = (w[i-1] ^ w[i-6] ^ w[i-12] ^ w[i-14]) <<< 1
// w'[i+3]  = (     0 ^ w[i-5] ^ w[i-11] ^ w[i-13]) <<< 1
//
// w[  i] = w'[  i]
// w[i+1] = w'[i+1]
// w[i+2] = w'[i+2]
// w[i+3] = w'[i+3] ^ (w'[i] <<< 1)
static vec_uint32_t sched_16_31(vec_uint32_t *pre_added, vec_uint32_t minus_4,
                                vec_uint32_t minus_8, vec_uint32_t minus_12,
                                vec_uint32_t minus_16, vec_uint32_t k) {
  const vec_uint32_t minus_3 = vec_sro(minus_4, k_4_bytes);
  const vec_uint32_t minus_14 = vec_sld((minus_12), (minus_16), 8);
  const vec_uint32_t k_1_bit = vec_splat_u32(1);
  const vec_uint32_t w_prime =
      vec_rl(minus_3 ^ minus_8 ^ minus_14 ^ minus_16, k_1_bit);
  const vec_uint32_t w =
      w_prime ^ vec_rl(vec_slo(w_prime, k_12_bytes), k_1_bit);
  vec_st(w + k, 0, pre_added);
  return w;
}

// Compute w[i..i+3] using this relation for i in [32, 36, 40 ... 76]
// w[i] = (w[i-6] ^ w[i-16] ^ w[i-28] ^ w[i-32]), 2) <<< 2
static vec_uint32_t sched_32_79(vec_uint32_t *pre_added, vec_uint32_t minus_4,
                                vec_uint32_t minus_8, vec_uint32_t minus_16,
                                vec_uint32_t minus_28, vec_uint32_t minus_32,
                                vec_uint32_t k) {
  const vec_uint32_t minus_6 = vec_sld(minus_4, minus_8, 8);
  const vec_uint32_t k_2_bits = vec_splat_u32(2);
  const vec_uint32_t w =
      vec_rl(minus_6 ^ minus_16 ^ minus_28 ^ minus_32, k_2_bits);
  vec_st(w + k, 0, pre_added);
  return w;
}

// As pointed out by Wei Dai <weidai@eskimo.com>, F() below can be simplified
// to the code in F_00_19. Wei attributes these optimisations to Peter
// Gutmann's SHS code, and he attributes it to Rich Schroeppel. #define
// F(x,y,z) (((x) & (y))  |  ((~(x)) & (z))) I've just become aware of another
// tweak to be made, again from Wei Dai, in F_40_59, (x&a)|(y&a) -> (x|y)&a
#define F_00_19(b, c, d) ((((c) ^ (d)) & (b)) ^ (d))
#define F_20_39(b, c, d) ((b) ^ (c) ^ (d))
#define F_40_59(b, c, d) (((b) & (c)) | (((b) | (c)) & (d)))
#define F_60_79(b, c, d) F_20_39(b, c, d)

// We pre-added the K constants during message scheduling.
#define BODY_00_19(i, a, b, c, d, e, f)                         \
  do {                                                          \
    (f) = w[i] + (e) + rotate((a), 5) + F_00_19((b), (c), (d)); \
    (b) = rotate((b), 30);                                      \
  } while (0)

#define BODY_20_39(i, a, b, c, d, e, f)                         \
  do {                                                          \
    (f) = w[i] + (e) + rotate((a), 5) + F_20_39((b), (c), (d)); \
    (b) = rotate((b), 30);                                      \
  } while (0)

#define BODY_40_59(i, a, b, c, d, e, f)                         \
  do {                                                          \
    (f) = w[i] + (e) + rotate((a), 5) + F_40_59((b), (c), (d)); \
    (b) = rotate((b), 30);                                      \
  } while (0)

#define BODY_60_79(i, a, b, c, d, e, f)                         \
  do {                                                          \
    (f) = w[i] + (e) + rotate((a), 5) + F_60_79((b), (c), (d)); \
    (b) = rotate((b), 30);                                      \
  } while (0)

void sha1_block_data_order(uint32_t *state, const uint8_t *data, size_t num) {
  uint32_t A, B, C, D, E, T;

  A = state[0];
  B = state[1];
  C = state[2];
  D = state[3];
  E = state[4];

  for (;;) {
    vec_uint32_t vw[20];
    const uint32_t *w = (const uint32_t *)&vw;

    vec_uint32_t k = K_00_19_x_4;
    const vec_uint32_t w0 = sched_00_15(vw + 0, data + 0, k);
    BODY_00_19(0, A, B, C, D, E, T);
    BODY_00_19(1, T, A, B, C, D, E);
    BODY_00_19(2, E, T, A, B, C, D);
    BODY_00_19(3, D, E, T, A, B, C);

    const vec_uint32_t w4 = sched_00_15(vw + 1, data + 16, k);
    BODY_00_19(4, C, D, E, T, A, B);
    BODY_00_19(5, B, C, D, E, T, A);
    BODY_00_19(6, A, B, C, D, E, T);
    BODY_00_19(7, T, A, B, C, D, E);

    const vec_uint32_t w8 = sched_00_15(vw + 2, data + 32, k);
    BODY_00_19(8, E, T, A, B, C, D);
    BODY_00_19(9, D, E, T, A, B, C);
    BODY_00_19(10, C, D, E, T, A, B);
    BODY_00_19(11, B, C, D, E, T, A);

    const vec_uint32_t w12 = sched_00_15(vw + 3, data + 48, k);
    BODY_00_19(12, A, B, C, D, E, T);
    BODY_00_19(13, T, A, B, C, D, E);
    BODY_00_19(14, E, T, A, B, C, D);
    BODY_00_19(15, D, E, T, A, B, C);

    const vec_uint32_t w16 = sched_16_31(vw + 4, w12, w8, w4, w0, k);
    BODY_00_19(16, C, D, E, T, A, B);
    BODY_00_19(17, B, C, D, E, T, A);
    BODY_00_19(18, A, B, C, D, E, T);
    BODY_00_19(19, T, A, B, C, D, E);

    k = K_20_39_x_4;
    const vec_uint32_t w20 = sched_16_31(vw + 5, w16, w12, w8, w4, k);
    BODY_20_39(20, E, T, A, B, C, D);
    BODY_20_39(21, D, E, T, A, B, C);
    BODY_20_39(22, C, D, E, T, A, B);
    BODY_20_39(23, B, C, D, E, T, A);

    const vec_uint32_t w24 = sched_16_31(vw + 6, w20, w16, w12, w8, k);
    BODY_20_39(24, A, B, C, D, E, T);
    BODY_20_39(25, T, A, B, C, D, E);
    BODY_20_39(26, E, T, A, B, C, D);
    BODY_20_39(27, D, E, T, A, B, C);

    const vec_uint32_t w28 = sched_16_31(vw + 7, w24, w20, w16, w12, k);
    BODY_20_39(28, C, D, E, T, A, B);
    BODY_20_39(29, B, C, D, E, T, A);
    BODY_20_39(30, A, B, C, D, E, T);
    BODY_20_39(31, T, A, B, C, D, E);

    const vec_uint32_t w32 = sched_32_79(vw + 8, w28, w24, w16, w4, w0, k);
    BODY_20_39(32, E, T, A, B, C, D);
    BODY_20_39(33, D, E, T, A, B, C);
    BODY_20_39(34, C, D, E, T, A, B);
    BODY_20_39(35, B, C, D, E, T, A);

    const vec_uint32_t w36 = sched_32_79(vw + 9, w32, w28, w20, w8, w4, k);
    BODY_20_39(36, A, B, C, D, E, T);
    BODY_20_39(37, T, A, B, C, D, E);
    BODY_20_39(38, E, T, A, B, C, D);
    BODY_20_39(39, D, E, T, A, B, C);

    k = K_40_59_x_4;
    const vec_uint32_t w40 = sched_32_79(vw + 10, w36, w32, w24, w12, w8, k);
    BODY_40_59(40, C, D, E, T, A, B);
    BODY_40_59(41, B, C, D, E, T, A);
    BODY_40_59(42, A, B, C, D, E, T);
    BODY_40_59(43, T, A, B, C, D, E);

    const vec_uint32_t w44 = sched_32_79(vw + 11, w40, w36, w28, w16, w12, k);
    BODY_40_59(44, E, T, A, B, C, D);
    BODY_40_59(45, D, E, T, A, B, C);
    BODY_40_59(46, C, D, E, T, A, B);
    BODY_40_59(47, B, C, D, E, T, A);

    const vec_uint32_t w48 = sched_32_79(vw + 12, w44, w40, w32, w20, w16, k);
    BODY_40_59(48, A, B, C, D, E, T);
    BODY_40_59(49, T, A, B, C, D, E);
    BODY_40_59(50, E, T, A, B, C, D);
    BODY_40_59(51, D, E, T, A, B, C);

    const vec_uint32_t w52 = sched_32_79(vw + 13, w48, w44, w36, w24, w20, k);
    BODY_40_59(52, C, D, E, T, A, B);
    BODY_40_59(53, B, C, D, E, T, A);
    BODY_40_59(54, A, B, C, D, E, T);
    BODY_40_59(55, T, A, B, C, D, E);

    const vec_uint32_t w56 = sched_32_79(vw + 14, w52, w48, w40, w28, w24, k);
    BODY_40_59(56, E, T, A, B, C, D);
    BODY_40_59(57, D, E, T, A, B, C);
    BODY_40_59(58, C, D, E, T, A, B);
    BODY_40_59(59, B, C, D, E, T, A);

    k = K_60_79_x_4;
    const vec_uint32_t w60 = sched_32_79(vw + 15, w56, w52, w44, w32, w28, k);
    BODY_60_79(60, A, B, C, D, E, T);
    BODY_60_79(61, T, A, B, C, D, E);
    BODY_60_79(62, E, T, A, B, C, D);
    BODY_60_79(63, D, E, T, A, B, C);

    const vec_uint32_t w64 = sched_32_79(vw + 16, w60, w56, w48, w36, w32, k);
    BODY_60_79(64, C, D, E, T, A, B);
    BODY_60_79(65, B, C, D, E, T, A);
    BODY_60_79(66, A, B, C, D, E, T);
    BODY_60_79(67, T, A, B, C, D, E);

    const vec_uint32_t w68 = sched_32_79(vw + 17, w64, w60, w52, w40, w36, k);
    BODY_60_79(68, E, T, A, B, C, D);
    BODY_60_79(69, D, E, T, A, B, C);
    BODY_60_79(70, C, D, E, T, A, B);
    BODY_60_79(71, B, C, D, E, T, A);

    const vec_uint32_t w72 = sched_32_79(vw + 18, w68, w64, w56, w44, w40, k);
    BODY_60_79(72, A, B, C, D, E, T);
    BODY_60_79(73, T, A, B, C, D, E);
    BODY_60_79(74, E, T, A, B, C, D);
    BODY_60_79(75, D, E, T, A, B, C);

    // We don't use the last value
    (void)sched_32_79(vw + 19, w72, w68, w60, w48, w44, k);
    BODY_60_79(76, C, D, E, T, A, B);
    BODY_60_79(77, B, C, D, E, T, A);
    BODY_60_79(78, A, B, C, D, E, T);
    BODY_60_79(79, T, A, B, C, D, E);

    const uint32_t mask = 0xffffffffUL;
    state[0] = (state[0] + E) & mask;
    state[1] = (state[1] + T) & mask;
    state[2] = (state[2] + A) & mask;
    state[3] = (state[3] + B) & mask;
    state[4] = (state[4] + C) & mask;

    data += 64;
    if (--num == 0) {
      break;
    }

    A = state[0];
    B = state[1];
    C = state[2];
    D = state[3];
    E = state[4];
  }
}

#endif  // OPENSSL_PPC64LE

#undef K_00_19
#undef K_20_39
#undef K_40_59
#undef K_60_79
#undef F_00_19
#undef F_20_39
#undef F_40_59
#undef F_60_79
#undef BODY_00_19
#undef BODY_20_39
#undef BODY_40_59
#undef BODY_60_79
