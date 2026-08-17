// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

// This implementation of poly1305 is by Andrew Moon
// (https://github.com/floodyberry/poly1305-donna) and released as public
// domain. It implements SIMD vectorization based on the algorithm described in
// http://cr.yp.to/papers.html#neoncrypto. Unrolled to 2 powers, i.e. 64 byte
// block size

#include <openssl/poly1305.h>

#include "../internal.h"


#if defined(BORINGSSL_HAS_UINT128) && defined(OPENSSL_X86_64)

#include <emmintrin.h>

typedef __m128i xmmi;

static const alignas(16) uint32_t poly1305_x64_sse2_message_mask[4] = {
    (1 << 26) - 1, 0, (1 << 26) - 1, 0};
static const alignas(16) uint32_t poly1305_x64_sse2_5[4] = {5, 0, 5, 0};
static const alignas(16) uint32_t poly1305_x64_sse2_1shl128[4] = {
    (1 << 24), 0, (1 << 24), 0};

static inline uint128_t add128(uint128_t a, uint128_t b) { return a + b; }

static inline uint128_t add128_64(uint128_t a, uint64_t b) { return a + b; }

static inline uint128_t mul64x64_128(uint64_t a, uint64_t b) {
  return (uint128_t)a * b;
}

static inline uint64_t lo128(uint128_t a) { return (uint64_t)a; }

static inline uint64_t shr128(uint128_t v, const int shift) {
  return (uint64_t)(v >> shift);
}

static inline uint64_t shr128_pair(uint64_t hi, uint64_t lo, const int shift) {
  return (uint64_t)((((uint128_t)hi << 64) | lo) >> shift);
}

typedef struct poly1305_power_t {
  union {
    xmmi v;
    uint64_t u[2];
    uint32_t d[4];
  } R20, R21, R22, R23, R24, S21, S22, S23, S24;
} poly1305_power;

typedef struct poly1305_state_internal_t {
  poly1305_power P[2]; /* 288 bytes, top 32 bit halves unused = 144
                          bytes of free storage */
  union {
    xmmi H[5];  //  80 bytes
    uint64_t HH[10];
  } u;
  // uint64_t r0,r1,r2;       [24 bytes]
  // uint64_t pad0,pad1;      [16 bytes]
  uint64_t started;        //   8 bytes
  uint64_t leftover;       //   8 bytes
  uint8_t buffer[64];      //  64 bytes
} poly1305_state_internal; /* 448 bytes total + 63 bytes for
                              alignment = 511 bytes raw */

OPENSSL_STATIC_ASSERT(
    sizeof(struct poly1305_state_internal_t) + 63 <= sizeof(poly1305_state),
    _poly1305_state_isn_t_large_enough_to_hold_aligned_poly1305_state_internal_t)

static inline poly1305_state_internal *poly1305_aligned_state(
    poly1305_state *state) {
  return (poly1305_state_internal *)(((uint64_t)state + 63) & ~63);
}

static inline size_t poly1305_min(size_t a, size_t b) {
  return (a < b) ? a : b;
}

void CRYPTO_poly1305_init(poly1305_state *state, const uint8_t key[32]) {
  poly1305_state_internal *st = poly1305_aligned_state(state);
  poly1305_power *p;
  uint64_t r0, r1, r2;
  uint64_t t0, t1;

  // clamp key
  t0 = CRYPTO_load_u64_le(key + 0);
  t1 = CRYPTO_load_u64_le(key + 8);
  r0 = t0 & 0xffc0fffffff;
  t0 >>= 44;
  t0 |= t1 << 20;
  r1 = t0 & 0xfffffc0ffff;
  t1 >>= 24;
  r2 = t1 & 0x00ffffffc0f;

  // store r in un-used space of st->P[1]
  p = &st->P[1];
  p->R20.d[1] = (uint32_t)(r0);
  p->R20.d[3] = (uint32_t)(r0 >> 32);
  p->R21.d[1] = (uint32_t)(r1);
  p->R21.d[3] = (uint32_t)(r1 >> 32);
  p->R22.d[1] = (uint32_t)(r2);
  p->R22.d[3] = (uint32_t)(r2 >> 32);

  // store pad
  p->R23.d[1] = CRYPTO_load_u32_le(key + 16);
  p->R23.d[3] = CRYPTO_load_u32_le(key + 20);
  p->R24.d[1] = CRYPTO_load_u32_le(key + 24);
  p->R24.d[3] = CRYPTO_load_u32_le(key + 28);

  // H = 0
  st->u.H[0] = _mm_setzero_si128();
  st->u.H[1] = _mm_setzero_si128();
  st->u.H[2] = _mm_setzero_si128();
  st->u.H[3] = _mm_setzero_si128();
  st->u.H[4] = _mm_setzero_si128();

  st->started = 0;
  st->leftover = 0;
}

static void poly1305_first_block(poly1305_state_internal *st,
                                 const uint8_t *m) {
  const xmmi MMASK = _mm_load_si128((const xmmi *)poly1305_x64_sse2_message_mask);
  const xmmi FIVE = _mm_load_si128((const xmmi *)poly1305_x64_sse2_5);
  const xmmi HIBIT = _mm_load_si128((const xmmi *)poly1305_x64_sse2_1shl128);
  xmmi T5, T6;
  poly1305_power *p;
  uint128_t d[3];
  uint64_t r0, r1, r2;
  uint64_t r20, r21, r22, s22;
  uint64_t pad0, pad1;
  uint64_t c;
  uint64_t i;

  // pull out stored info
  p = &st->P[1];

  r0 = ((uint64_t)p->R20.d[3] << 32) | (uint64_t)p->R20.d[1];
  r1 = ((uint64_t)p->R21.d[3] << 32) | (uint64_t)p->R21.d[1];
  r2 = ((uint64_t)p->R22.d[3] << 32) | (uint64_t)p->R22.d[1];
  pad0 = ((uint64_t)p->R23.d[3] << 32) | (uint64_t)p->R23.d[1];
  pad1 = ((uint64_t)p->R24.d[3] << 32) | (uint64_t)p->R24.d[1];

  // compute powers r^2,r^4
  r20 = r0;
  r21 = r1;
  r22 = r2;
  for (i = 0; i < 2; i++) {
    s22 = r22 * (5 << 2);

    d[0] = add128(mul64x64_128(r20, r20), mul64x64_128(r21 * 2, s22));
    d[1] = add128(mul64x64_128(r22, s22), mul64x64_128(r20 * 2, r21));
    d[2] = add128(mul64x64_128(r21, r21), mul64x64_128(r22 * 2, r20));

    r20 = lo128(d[0]) & 0xfffffffffff;
    c = shr128(d[0], 44);
    d[1] = add128_64(d[1], c);
    r21 = lo128(d[1]) & 0xfffffffffff;
    c = shr128(d[1], 44);
    d[2] = add128_64(d[2], c);
    r22 = lo128(d[2]) & 0x3ffffffffff;
    c = shr128(d[2], 42);
    r20 += c * 5;
    c = (r20 >> 44);
    r20 = r20 & 0xfffffffffff;
    r21 += c;

    p->R20.v = _mm_shuffle_epi32(_mm_cvtsi32_si128((uint32_t)(r20)&0x3ffffff),
                                 _MM_SHUFFLE(1, 0, 1, 0));
    p->R21.v = _mm_shuffle_epi32(
        _mm_cvtsi32_si128((uint32_t)((r20 >> 26) | (r21 << 18)) & 0x3ffffff),
        _MM_SHUFFLE(1, 0, 1, 0));
    p->R22.v =
        _mm_shuffle_epi32(_mm_cvtsi32_si128((uint32_t)((r21 >> 8)) & 0x3ffffff),
                          _MM_SHUFFLE(1, 0, 1, 0));
    p->R23.v = _mm_shuffle_epi32(
        _mm_cvtsi32_si128((uint32_t)((r21 >> 34) | (r22 << 10)) & 0x3ffffff),
        _MM_SHUFFLE(1, 0, 1, 0));
    p->R24.v = _mm_shuffle_epi32(_mm_cvtsi32_si128((uint32_t)((r22 >> 16))),
                                 _MM_SHUFFLE(1, 0, 1, 0));
    p->S21.v = _mm_mul_epu32(p->R21.v, FIVE);
    p->S22.v = _mm_mul_epu32(p->R22.v, FIVE);
    p->S23.v = _mm_mul_epu32(p->R23.v, FIVE);
    p->S24.v = _mm_mul_epu32(p->R24.v, FIVE);
    p--;
  }

  // put saved info back
  p = &st->P[1];
  p->R20.d[1] = (uint32_t)(r0);
  p->R20.d[3] = (uint32_t)(r0 >> 32);
  p->R21.d[1] = (uint32_t)(r1);
  p->R21.d[3] = (uint32_t)(r1 >> 32);
  p->R22.d[1] = (uint32_t)(r2);
  p->R22.d[3] = (uint32_t)(r2 >> 32);
  p->R23.d[1] = (uint32_t)(pad0);
  p->R23.d[3] = (uint32_t)(pad0 >> 32);
  p->R24.d[1] = (uint32_t)(pad1);
  p->R24.d[3] = (uint32_t)(pad1 >> 32);

  // H = [Mx,My]
  T5 = _mm_unpacklo_epi64(_mm_loadl_epi64((const xmmi *)(m + 0)),
                          _mm_loadl_epi64((const xmmi *)(m + 16)));
  T6 = _mm_unpacklo_epi64(_mm_loadl_epi64((const xmmi *)(m + 8)),
                          _mm_loadl_epi64((const xmmi *)(m + 24)));
  st->u.H[0] = _mm_and_si128(MMASK, T5);
  st->u.H[1] = _mm_and_si128(MMASK, _mm_srli_epi64(T5, 26));
  T5 = _mm_or_si128(_mm_srli_epi64(T5, 52), _mm_slli_epi64(T6, 12));
  st->u.H[2] = _mm_and_si128(MMASK, T5);
  st->u.H[3] = _mm_and_si128(MMASK, _mm_srli_epi64(T5, 26));
  st->u.H[4] = _mm_or_si128(_mm_srli_epi64(T6, 40), HIBIT);
}

static void poly1305_blocks(poly1305_state_internal *st, const uint8_t *m,
                            size_t bytes) {
  const xmmi MMASK = _mm_load_si128((const xmmi *)poly1305_x64_sse2_message_mask);
  const xmmi FIVE = _mm_load_si128((const xmmi *)poly1305_x64_sse2_5);
  const xmmi HIBIT = _mm_load_si128((const xmmi *)poly1305_x64_sse2_1shl128);

  poly1305_power *p;
  xmmi H0, H1, H2, H3, H4;
  xmmi T0, T1, T2, T3, T4, T5, T6;
  xmmi M0, M1, M2, M3, M4;
  xmmi C1, C2;

  H0 = st->u.H[0];
  H1 = st->u.H[1];
  H2 = st->u.H[2];
  H3 = st->u.H[3];
  H4 = st->u.H[4];

  while (bytes >= 64) {
    // H *= [r^4,r^4]
    p = &st->P[0];
    T0 = _mm_mul_epu32(H0, p->R20.v);
    T1 = _mm_mul_epu32(H0, p->R21.v);
    T2 = _mm_mul_epu32(H0, p->R22.v);
    T3 = _mm_mul_epu32(H0, p->R23.v);
    T4 = _mm_mul_epu32(H0, p->R24.v);
    T5 = _mm_mul_epu32(H1, p->S24.v);
    T6 = _mm_mul_epu32(H1, p->R20.v);
    T0 = _mm_add_epi64(T0, T5);
    T1 = _mm_add_epi64(T1, T6);
    T5 = _mm_mul_epu32(H2, p->S23.v);
    T6 = _mm_mul_epu32(H2, p->S24.v);
    T0 = _mm_add_epi64(T0, T5);
    T1 = _mm_add_epi64(T1, T6);
    T5 = _mm_mul_epu32(H3, p->S22.v);
    T6 = _mm_mul_epu32(H3, p->S23.v);
    T0 = _mm_add_epi64(T0, T5);
    T1 = _mm_add_epi64(T1, T6);
    T5 = _mm_mul_epu32(H4, p->S21.v);
    T6 = _mm_mul_epu32(H4, p->S22.v);
    T0 = _mm_add_epi64(T0, T5);
    T1 = _mm_add_epi64(T1, T6);
    T5 = _mm_mul_epu32(H1, p->R21.v);
    T6 = _mm_mul_epu32(H1, p->R22.v);
    T2 = _mm_add_epi64(T2, T5);
    T3 = _mm_add_epi64(T3, T6);
    T5 = _mm_mul_epu32(H2, p->R20.v);
    T6 = _mm_mul_epu32(H2, p->R21.v);
    T2 = _mm_add_epi64(T2, T5);
    T3 = _mm_add_epi64(T3, T6);
    T5 = _mm_mul_epu32(H3, p->S24.v);
    T6 = _mm_mul_epu32(H3, p->R20.v);
    T2 = _mm_add_epi64(T2, T5);
    T3 = _mm_add_epi64(T3, T6);
    T5 = _mm_mul_epu32(H4, p->S23.v);
    T6 = _mm_mul_epu32(H4, p->S24.v);
    T2 = _mm_add_epi64(T2, T5);
    T3 = _mm_add_epi64(T3, T6);
    T5 = _mm_mul_epu32(H1, p->R23.v);
    T4 = _mm_add_epi64(T4, T5);
    T5 = _mm_mul_epu32(H2, p->R22.v);
    T4 = _mm_add_epi64(T4, T5);
    T5 = _mm_mul_epu32(H3, p->R21.v);
    T4 = _mm_add_epi64(T4, T5);
    T5 = _mm_mul_epu32(H4, p->R20.v);
    T4 = _mm_add_epi64(T4, T5);

    // H += [Mx,My]*[r^2,r^2]
    T5 = _mm_unpacklo_epi64(_mm_loadl_epi64((const xmmi *)(m + 0)),
                            _mm_loadl_epi64((const xmmi *)(m + 16)));
    T6 = _mm_unpacklo_epi64(_mm_loadl_epi64((const xmmi *)(m + 8)),
                            _mm_loadl_epi64((const xmmi *)(m + 24)));
    M0 = _mm_and_si128(MMASK, T5);
    M1 = _mm_and_si128(MMASK, _mm_srli_epi64(T5, 26));
    T5 = _mm_or_si128(_mm_srli_epi64(T5, 52), _mm_slli_epi64(T6, 12));
    M2 = _mm_and_si128(MMASK, T5);
    M3 = _mm_and_si128(MMASK, _mm_srli_epi64(T5, 26));
    M4 = _mm_or_si128(_mm_srli_epi64(T6, 40), HIBIT);

    p = &st->P[1];
    T5 = _mm_mul_epu32(M0, p->R20.v);
    T6 = _mm_mul_epu32(M0, p->R21.v);
    T0 = _mm_add_epi64(T0, T5);
    T1 = _mm_add_epi64(T1, T6);
    T5 = _mm_mul_epu32(M1, p->S24.v);
    T6 = _mm_mul_epu32(M1, p->R20.v);
    T0 = _mm_add_epi64(T0, T5);
    T1 = _mm_add_epi64(T1, T6);
    T5 = _mm_mul_epu32(M2, p->S23.v);
    T6 = _mm_mul_epu32(M2, p->S24.v);
    T0 = _mm_add_epi64(T0, T5);
    T1 = _mm_add_epi64(T1, T6);
    T5 = _mm_mul_epu32(M3, p->S22.v);
    T6 = _mm_mul_epu32(M3, p->S23.v);
    T0 = _mm_add_epi64(T0, T5);
    T1 = _mm_add_epi64(T1, T6);
    T5 = _mm_mul_epu32(M4, p->S21.v);
    T6 = _mm_mul_epu32(M4, p->S22.v);
    T0 = _mm_add_epi64(T0, T5);
    T1 = _mm_add_epi64(T1, T6);
    T5 = _mm_mul_epu32(M0, p->R22.v);
    T6 = _mm_mul_epu32(M0, p->R23.v);
    T2 = _mm_add_epi64(T2, T5);
    T3 = _mm_add_epi64(T3, T6);
    T5 = _mm_mul_epu32(M1, p->R21.v);
    T6 = _mm_mul_epu32(M1, p->R22.v);
    T2 = _mm_add_epi64(T2, T5);
    T3 = _mm_add_epi64(T3, T6);
    T5 = _mm_mul_epu32(M2, p->R20.v);
    T6 = _mm_mul_epu32(M2, p->R21.v);
    T2 = _mm_add_epi64(T2, T5);
    T3 = _mm_add_epi64(T3, T6);
    T5 = _mm_mul_epu32(M3, p->S24.v);
    T6 = _mm_mul_epu32(M3, p->R20.v);
    T2 = _mm_add_epi64(T2, T5);
    T3 = _mm_add_epi64(T3, T6);
    T5 = _mm_mul_epu32(M4, p->S23.v);
    T6 = _mm_mul_epu32(M4, p->S24.v);
    T2 = _mm_add_epi64(T2, T5);
    T3 = _mm_add_epi64(T3, T6);
    T5 = _mm_mul_epu32(M0, p->R24.v);
    T4 = _mm_add_epi64(T4, T5);
    T5 = _mm_mul_epu32(M1, p->R23.v);
    T4 = _mm_add_epi64(T4, T5);
    T5 = _mm_mul_epu32(M2, p->R22.v);
    T4 = _mm_add_epi64(T4, T5);
    T5 = _mm_mul_epu32(M3, p->R21.v);
    T4 = _mm_add_epi64(T4, T5);
    T5 = _mm_mul_epu32(M4, p->R20.v);
    T4 = _mm_add_epi64(T4, T5);

    // H += [Mx,My]
    T5 = _mm_unpacklo_epi64(_mm_loadl_epi64((const xmmi *)(m + 32)),
                            _mm_loadl_epi64((const xmmi *)(m + 48)));
    T6 = _mm_unpacklo_epi64(_mm_loadl_epi64((const xmmi *)(m + 40)),
                            _mm_loadl_epi64((const xmmi *)(m + 56)));
    M0 = _mm_and_si128(MMASK, T5);
    M1 = _mm_and_si128(MMASK, _mm_srli_epi64(T5, 26));
    T5 = _mm_or_si128(_mm_srli_epi64(T5, 52), _mm_slli_epi64(T6, 12));
    M2 = _mm_and_si128(MMASK, T5);
    M3 = _mm_and_si128(MMASK, _mm_srli_epi64(T5, 26));
    M4 = _mm_or_si128(_mm_srli_epi64(T6, 40), HIBIT);

    T0 = _mm_add_epi64(T0, M0);
    T1 = _mm_add_epi64(T1, M1);
    T2 = _mm_add_epi64(T2, M2);
    T3 = _mm_add_epi64(T3, M3);
    T4 = _mm_add_epi64(T4, M4);

    // reduce
    C1 = _mm_srli_epi64(T0, 26);
    C2 = _mm_srli_epi64(T3, 26);
    T0 = _mm_and_si128(T0, MMASK);
    T3 = _mm_and_si128(T3, MMASK);
    T1 = _mm_add_epi64(T1, C1);
    T4 = _mm_add_epi64(T4, C2);
    C1 = _mm_srli_epi64(T1, 26);
    C2 = _mm_srli_epi64(T4, 26);
    T1 = _mm_and_si128(T1, MMASK);
    T4 = _mm_and_si128(T4, MMASK);
    T2 = _mm_add_epi64(T2, C1);
    T0 = _mm_add_epi64(T0, _mm_mul_epu32(C2, FIVE));
    C1 = _mm_srli_epi64(T2, 26);
    C2 = _mm_srli_epi64(T0, 26);
    T2 = _mm_and_si128(T2, MMASK);
    T0 = _mm_and_si128(T0, MMASK);
    T3 = _mm_add_epi64(T3, C1);
    T1 = _mm_add_epi64(T1, C2);
    C1 = _mm_srli_epi64(T3, 26);
    T3 = _mm_and_si128(T3, MMASK);
    T4 = _mm_add_epi64(T4, C1);

    // H = (H*[r^4,r^4] + [Mx,My]*[r^2,r^2] + [Mx,My])
    H0 = T0;
    H1 = T1;
    H2 = T2;
    H3 = T3;
    H4 = T4;

    m += 64;
    bytes -= 64;
  }

  st->u.H[0] = H0;
  st->u.H[1] = H1;
  st->u.H[2] = H2;
  st->u.H[3] = H3;
  st->u.H[4] = H4;
}

static size_t poly1305_combine(poly1305_state_internal *st, const uint8_t *m,
                               size_t bytes) {
  const xmmi MMASK = _mm_load_si128((const xmmi *)poly1305_x64_sse2_message_mask);
  const xmmi HIBIT = _mm_load_si128((const xmmi *)poly1305_x64_sse2_1shl128);
  const xmmi FIVE = _mm_load_si128((const xmmi *)poly1305_x64_sse2_5);

  poly1305_power *p;
  xmmi H0, H1, H2, H3, H4;
  xmmi M0, M1, M2, M3, M4;
  xmmi T0, T1, T2, T3, T4, T5, T6;
  xmmi C1, C2;

  uint64_t r0, r1, r2;
  uint64_t t0, t1, t2, t3, t4;
  uint64_t c;
  size_t consumed = 0;

  H0 = st->u.H[0];
  H1 = st->u.H[1];
  H2 = st->u.H[2];
  H3 = st->u.H[3];
  H4 = st->u.H[4];

  // p = [r^2,r^2]
  p = &st->P[1];

  if (bytes >= 32) {
    // H *= [r^2,r^2]
    T0 = _mm_mul_epu32(H0, p->R20.v);
    T1 = _mm_mul_epu32(H0, p->R21.v);
    T2 = _mm_mul_epu32(H0, p->R22.v);
    T3 = _mm_mul_epu32(H0, p->R23.v);
    T4 = _mm_mul_epu32(H0, p->R24.v);
    T5 = _mm_mul_epu32(H1, p->S24.v);
    T6 = _mm_mul_epu32(H1, p->R20.v);
    T0 = _mm_add_epi64(T0, T5);
    T1 = _mm_add_epi64(T1, T6);
    T5 = _mm_mul_epu32(H2, p->S23.v);
    T6 = _mm_mul_epu32(H2, p->S24.v);
    T0 = _mm_add_epi64(T0, T5);
    T1 = _mm_add_epi64(T1, T6);
    T5 = _mm_mul_epu32(H3, p->S22.v);
    T6 = _mm_mul_epu32(H3, p->S23.v);
    T0 = _mm_add_epi64(T0, T5);
    T1 = _mm_add_epi64(T1, T6);
    T5 = _mm_mul_epu32(H4, p->S21.v);
    T6 = _mm_mul_epu32(H4, p->S22.v);
    T0 = _mm_add_epi64(T0, T5);
    T1 = _mm_add_epi64(T1, T6);
    T5 = _mm_mul_epu32(H1, p->R21.v);
    T6 = _mm_mul_epu32(H1, p->R22.v);
    T2 = _mm_add_epi64(T2, T5);
    T3 = _mm_add_epi64(T3, T6);
    T5 = _mm_mul_epu32(H2, p->R20.v);
    T6 = _mm_mul_epu32(H2, p->R21.v);
    T2 = _mm_add_epi64(T2, T5);
    T3 = _mm_add_epi64(T3, T6);
    T5 = _mm_mul_epu32(H3, p->S24.v);
    T6 = _mm_mul_epu32(H3, p->R20.v);
    T2 = _mm_add_epi64(T2, T5);
    T3 = _mm_add_epi64(T3, T6);
    T5 = _mm_mul_epu32(H4, p->S23.v);
    T6 = _mm_mul_epu32(H4, p->S24.v);
    T2 = _mm_add_epi64(T2, T5);
    T3 = _mm_add_epi64(T3, T6);
    T5 = _mm_mul_epu32(H1, p->R23.v);
    T4 = _mm_add_epi64(T4, T5);
    T5 = _mm_mul_epu32(H2, p->R22.v);
    T4 = _mm_add_epi64(T4, T5);
    T5 = _mm_mul_epu32(H3, p->R21.v);
    T4 = _mm_add_epi64(T4, T5);
    T5 = _mm_mul_epu32(H4, p->R20.v);
    T4 = _mm_add_epi64(T4, T5);

    // H += [Mx,My]
    T5 = _mm_unpacklo_epi64(_mm_loadl_epi64((const xmmi *)(m + 0)),
                            _mm_loadl_epi64((const xmmi *)(m + 16)));
    T6 = _mm_unpacklo_epi64(_mm_loadl_epi64((const xmmi *)(m + 8)),
                            _mm_loadl_epi64((const xmmi *)(m + 24)));
    M0 = _mm_and_si128(MMASK, T5);
    M1 = _mm_and_si128(MMASK, _mm_srli_epi64(T5, 26));
    T5 = _mm_or_si128(_mm_srli_epi64(T5, 52), _mm_slli_epi64(T6, 12));
    M2 = _mm_and_si128(MMASK, T5);
    M3 = _mm_and_si128(MMASK, _mm_srli_epi64(T5, 26));
    M4 = _mm_or_si128(_mm_srli_epi64(T6, 40), HIBIT);

    T0 = _mm_add_epi64(T0, M0);
    T1 = _mm_add_epi64(T1, M1);
    T2 = _mm_add_epi64(T2, M2);
    T3 = _mm_add_epi64(T3, M3);
    T4 = _mm_add_epi64(T4, M4);

    // reduce
    C1 = _mm_srli_epi64(T0, 26);
    C2 = _mm_srli_epi64(T3, 26);
    T0 = _mm_and_si128(T0, MMASK);
    T3 = _mm_and_si128(T3, MMASK);
    T1 = _mm_add_epi64(T1, C1);
    T4 = _mm_add_epi64(T4, C2);
    C1 = _mm_srli_epi64(T1, 26);
    C2 = _mm_srli_epi64(T4, 26);
    T1 = _mm_and_si128(T1, MMASK);
    T4 = _mm_and_si128(T4, MMASK);
    T2 = _mm_add_epi64(T2, C1);
    T0 = _mm_add_epi64(T0, _mm_mul_epu32(C2, FIVE));
    C1 = _mm_srli_epi64(T2, 26);
    C2 = _mm_srli_epi64(T0, 26);
    T2 = _mm_and_si128(T2, MMASK);
    T0 = _mm_and_si128(T0, MMASK);
    T3 = _mm_add_epi64(T3, C1);
    T1 = _mm_add_epi64(T1, C2);
    C1 = _mm_srli_epi64(T3, 26);
    T3 = _mm_and_si128(T3, MMASK);
    T4 = _mm_add_epi64(T4, C1);

    // H = (H*[r^2,r^2] + [Mx,My])
    H0 = T0;
    H1 = T1;
    H2 = T2;
    H3 = T3;
    H4 = T4;

    consumed = 32;
  }

  // finalize, H *= [r^2,r]
  r0 = ((uint64_t)p->R20.d[3] << 32) | (uint64_t)p->R20.d[1];
  r1 = ((uint64_t)p->R21.d[3] << 32) | (uint64_t)p->R21.d[1];
  r2 = ((uint64_t)p->R22.d[3] << 32) | (uint64_t)p->R22.d[1];

  p->R20.d[2] = (uint32_t)(r0)&0x3ffffff;
  p->R21.d[2] = (uint32_t)((r0 >> 26) | (r1 << 18)) & 0x3ffffff;
  p->R22.d[2] = (uint32_t)((r1 >> 8)) & 0x3ffffff;
  p->R23.d[2] = (uint32_t)((r1 >> 34) | (r2 << 10)) & 0x3ffffff;
  p->R24.d[2] = (uint32_t)((r2 >> 16));
  p->S21.d[2] = p->R21.d[2] * 5;
  p->S22.d[2] = p->R22.d[2] * 5;
  p->S23.d[2] = p->R23.d[2] * 5;
  p->S24.d[2] = p->R24.d[2] * 5;

  // H *= [r^2,r]
  T0 = _mm_mul_epu32(H0, p->R20.v);
  T1 = _mm_mul_epu32(H0, p->R21.v);
  T2 = _mm_mul_epu32(H0, p->R22.v);
  T3 = _mm_mul_epu32(H0, p->R23.v);
  T4 = _mm_mul_epu32(H0, p->R24.v);
  T5 = _mm_mul_epu32(H1, p->S24.v);
  T6 = _mm_mul_epu32(H1, p->R20.v);
  T0 = _mm_add_epi64(T0, T5);
  T1 = _mm_add_epi64(T1, T6);
  T5 = _mm_mul_epu32(H2, p->S23.v);
  T6 = _mm_mul_epu32(H2, p->S24.v);
  T0 = _mm_add_epi64(T0, T5);
  T1 = _mm_add_epi64(T1, T6);
  T5 = _mm_mul_epu32(H3, p->S22.v);
  T6 = _mm_mul_epu32(H3, p->S23.v);
  T0 = _mm_add_epi64(T0, T5);
  T1 = _mm_add_epi64(T1, T6);
  T5 = _mm_mul_epu32(H4, p->S21.v);
  T6 = _mm_mul_epu32(H4, p->S22.v);
  T0 = _mm_add_epi64(T0, T5);
  T1 = _mm_add_epi64(T1, T6);
  T5 = _mm_mul_epu32(H1, p->R21.v);
  T6 = _mm_mul_epu32(H1, p->R22.v);
  T2 = _mm_add_epi64(T2, T5);
  T3 = _mm_add_epi64(T3, T6);
  T5 = _mm_mul_epu32(H2, p->R20.v);
  T6 = _mm_mul_epu32(H2, p->R21.v);
  T2 = _mm_add_epi64(T2, T5);
  T3 = _mm_add_epi64(T3, T6);
  T5 = _mm_mul_epu32(H3, p->S24.v);
  T6 = _mm_mul_epu32(H3, p->R20.v);
  T2 = _mm_add_epi64(T2, T5);
  T3 = _mm_add_epi64(T3, T6);
  T5 = _mm_mul_epu32(H4, p->S23.v);
  T6 = _mm_mul_epu32(H4, p->S24.v);
  T2 = _mm_add_epi64(T2, T5);
  T3 = _mm_add_epi64(T3, T6);
  T5 = _mm_mul_epu32(H1, p->R23.v);
  T4 = _mm_add_epi64(T4, T5);
  T5 = _mm_mul_epu32(H2, p->R22.v);
  T4 = _mm_add_epi64(T4, T5);
  T5 = _mm_mul_epu32(H3, p->R21.v);
  T4 = _mm_add_epi64(T4, T5);
  T5 = _mm_mul_epu32(H4, p->R20.v);
  T4 = _mm_add_epi64(T4, T5);

  C1 = _mm_srli_epi64(T0, 26);
  C2 = _mm_srli_epi64(T3, 26);
  T0 = _mm_and_si128(T0, MMASK);
  T3 = _mm_and_si128(T3, MMASK);
  T1 = _mm_add_epi64(T1, C1);
  T4 = _mm_add_epi64(T4, C2);
  C1 = _mm_srli_epi64(T1, 26);
  C2 = _mm_srli_epi64(T4, 26);
  T1 = _mm_and_si128(T1, MMASK);
  T4 = _mm_and_si128(T4, MMASK);
  T2 = _mm_add_epi64(T2, C1);
  T0 = _mm_add_epi64(T0, _mm_mul_epu32(C2, FIVE));
  C1 = _mm_srli_epi64(T2, 26);
  C2 = _mm_srli_epi64(T0, 26);
  T2 = _mm_and_si128(T2, MMASK);
  T0 = _mm_and_si128(T0, MMASK);
  T3 = _mm_add_epi64(T3, C1);
  T1 = _mm_add_epi64(T1, C2);
  C1 = _mm_srli_epi64(T3, 26);
  T3 = _mm_and_si128(T3, MMASK);
  T4 = _mm_add_epi64(T4, C1);

  // H = H[0]+H[1]
  H0 = _mm_add_epi64(T0, _mm_srli_si128(T0, 8));
  H1 = _mm_add_epi64(T1, _mm_srli_si128(T1, 8));
  H2 = _mm_add_epi64(T2, _mm_srli_si128(T2, 8));
  H3 = _mm_add_epi64(T3, _mm_srli_si128(T3, 8));
  H4 = _mm_add_epi64(T4, _mm_srli_si128(T4, 8));

  t0 = _mm_cvtsi128_si32(H0);
  c = (t0 >> 26);
  t0 &= 0x3ffffff;
  t1 = _mm_cvtsi128_si32(H1) + c;
  c = (t1 >> 26);
  t1 &= 0x3ffffff;
  t2 = _mm_cvtsi128_si32(H2) + c;
  c = (t2 >> 26);
  t2 &= 0x3ffffff;
  t3 = _mm_cvtsi128_si32(H3) + c;
  c = (t3 >> 26);
  t3 &= 0x3ffffff;
  t4 = _mm_cvtsi128_si32(H4) + c;
  c = (t4 >> 26);
  t4 &= 0x3ffffff;
  t0 = t0 + (c * 5);
  c = (t0 >> 26);
  t0 &= 0x3ffffff;
  t1 = t1 + c;

  st->u.HH[0] = ((t0) | (t1 << 26)) & UINT64_C(0xfffffffffff);
  st->u.HH[1] = ((t1 >> 18) | (t2 << 8) | (t3 << 34)) & UINT64_C(0xfffffffffff);
  st->u.HH[2] = ((t3 >> 10) | (t4 << 16)) & UINT64_C(0x3ffffffffff);

  return consumed;
}

void CRYPTO_poly1305_update(poly1305_state *state, const uint8_t *m,
                            size_t bytes) {
  poly1305_state_internal *st = poly1305_aligned_state(state);
  size_t want;

  // Work around a C language bug. See https://crbug.com/1019588.
  if (bytes == 0) {
    return;
  }

  // need at least 32 initial bytes to start the accelerated branch
  if (!st->started) {
    if ((st->leftover == 0) && (bytes > 32)) {
      poly1305_first_block(st, m);
      m += 32;
      bytes -= 32;
    } else {
      want = poly1305_min(32 - st->leftover, bytes);
      OPENSSL_memcpy(st->buffer + st->leftover, m, want);
      bytes -= want;
      m += want;
      st->leftover += want;
      if ((st->leftover < 32) || (bytes == 0)) {
        return;
      }
      poly1305_first_block(st, st->buffer);
      st->leftover = 0;
    }
    st->started = 1;
  }

  // handle leftover
  if (st->leftover) {
    want = poly1305_min(64 - st->leftover, bytes);
    OPENSSL_memcpy(st->buffer + st->leftover, m, want);
    bytes -= want;
    m += want;
    st->leftover += want;
    if (st->leftover < 64) {
      return;
    }
    poly1305_blocks(st, st->buffer, 64);
    st->leftover = 0;
  }

  // process 64 byte blocks
  if (bytes >= 64) {
    want = (bytes & ~63);
    poly1305_blocks(st, m, want);
    m += want;
    bytes -= want;
  }

  if (bytes) {
    OPENSSL_memcpy(st->buffer + st->leftover, m, bytes);
    st->leftover += bytes;
  }
}

void CRYPTO_poly1305_finish(poly1305_state *state, uint8_t mac[16]) {
  poly1305_state_internal *st = poly1305_aligned_state(state);
  size_t leftover = st->leftover;
  uint8_t *m = st->buffer;
  uint128_t d[3];
  uint64_t h0, h1, h2;
  uint64_t t0, t1;
  uint64_t g0, g1, g2, c, nc;
  uint64_t r0, r1, r2, s1, s2;
  poly1305_power *p;

  if (st->started) {
    size_t consumed = poly1305_combine(st, m, leftover);
    leftover -= consumed;
    m += consumed;
  }

  // st->HH will either be 0 or have the combined result
  h0 = st->u.HH[0];
  h1 = st->u.HH[1];
  h2 = st->u.HH[2];

  p = &st->P[1];
  r0 = ((uint64_t)p->R20.d[3] << 32) | (uint64_t)p->R20.d[1];
  r1 = ((uint64_t)p->R21.d[3] << 32) | (uint64_t)p->R21.d[1];
  r2 = ((uint64_t)p->R22.d[3] << 32) | (uint64_t)p->R22.d[1];
  s1 = r1 * (5 << 2);
  s2 = r2 * (5 << 2);

  if (leftover < 16) {
    goto poly1305_donna_atmost15bytes;
  }

poly1305_donna_atleast16bytes:
  t0 = CRYPTO_load_u64_le(m + 0);
  t1 = CRYPTO_load_u64_le(m + 8);
  h0 += t0 & 0xfffffffffff;
  t0 = shr128_pair(t1, t0, 44);
  h1 += t0 & 0xfffffffffff;
  h2 += (t1 >> 24) | ((uint64_t)1 << 40);

poly1305_donna_mul:
  d[0] = add128(add128(mul64x64_128(h0, r0), mul64x64_128(h1, s2)),
                mul64x64_128(h2, s1));
  d[1] = add128(add128(mul64x64_128(h0, r1), mul64x64_128(h1, r0)),
                mul64x64_128(h2, s2));
  d[2] = add128(add128(mul64x64_128(h0, r2), mul64x64_128(h1, r1)),
                mul64x64_128(h2, r0));
  h0 = lo128(d[0]) & 0xfffffffffff;
  c = shr128(d[0], 44);
  d[1] = add128_64(d[1], c);
  h1 = lo128(d[1]) & 0xfffffffffff;
  c = shr128(d[1], 44);
  d[2] = add128_64(d[2], c);
  h2 = lo128(d[2]) & 0x3ffffffffff;
  c = shr128(d[2], 42);
  h0 += c * 5;

  m += 16;
  leftover -= 16;
  if (leftover >= 16) {
    goto poly1305_donna_atleast16bytes;
  }

// final bytes
poly1305_donna_atmost15bytes:
  if (!leftover) {
    goto poly1305_donna_finish;
  }

  m[leftover++] = 1;
  OPENSSL_memset(m + leftover, 0, 16 - leftover);
  leftover = 16;

  t0 = CRYPTO_load_u64_le(m + 0);
  t1 = CRYPTO_load_u64_le(m + 8);
  h0 += t0 & 0xfffffffffff;
  t0 = shr128_pair(t1, t0, 44);
  h1 += t0 & 0xfffffffffff;
  h2 += (t1 >> 24);

  goto poly1305_donna_mul;

poly1305_donna_finish:
  c = (h0 >> 44);
  h0 &= 0xfffffffffff;
  h1 += c;
  c = (h1 >> 44);
  h1 &= 0xfffffffffff;
  h2 += c;
  c = (h2 >> 42);
  h2 &= 0x3ffffffffff;
  h0 += c * 5;

  g0 = h0 + 5;
  c = (g0 >> 44);
  g0 &= 0xfffffffffff;
  g1 = h1 + c;
  c = (g1 >> 44);
  g1 &= 0xfffffffffff;
  g2 = h2 + c - ((uint64_t)1 << 42);

  c = (g2 >> 63) - 1;
  nc = ~c;
  h0 = (h0 & nc) | (g0 & c);
  h1 = (h1 & nc) | (g1 & c);
  h2 = (h2 & nc) | (g2 & c);

  // pad
  t0 = ((uint64_t)p->R23.d[3] << 32) | (uint64_t)p->R23.d[1];
  t1 = ((uint64_t)p->R24.d[3] << 32) | (uint64_t)p->R24.d[1];
  h0 += (t0 & 0xfffffffffff);
  c = (h0 >> 44);
  h0 &= 0xfffffffffff;
  t0 = shr128_pair(t1, t0, 44);
  h1 += (t0 & 0xfffffffffff) + c;
  c = (h1 >> 44);
  h1 &= 0xfffffffffff;
  t1 = (t1 >> 24);
  h2 += (t1)+c;

  CRYPTO_store_u64_le(mac + 0, ((h0) | (h1 << 44)));
  CRYPTO_store_u64_le(mac + 8, ((h1 >> 20) | (h2 << 24)));
}

#endif  // BORINGSSL_HAS_UINT128 && OPENSSL_X86_64
