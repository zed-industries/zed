// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

// Adapted from the public domain, estream code by D. Bernstein.

#include <openssl/chacha.h>

#include <assert.h>
#include <string.h>

#include "../internal.h"
#include "internal.h"


// sigma contains the ChaCha constants, which happen to be an ASCII string.
// "expand 32-byte k"
static const uint32_t sigma_words[4] = {
        0x61707865, // 'e' 0x65, 'x' 0x78, 'p' 0x70, 'a' 0x61,
        0x3320646e, // 'n' 0x6E, 'd' 0x64, ' ' 0x20, '3' 0x33,
        0x79622d32, // '2' 0x32, '-' 0x2D, 'b' 0x62, 'y' 0x79,
        0x6b206574  // 't' 0x74, 'e' 0x65, ' ' 0x20, 'k' 0x6B
};

// QUARTERROUND updates a, b, c, d with a ChaCha "quarter" round.
#define QUARTERROUND(a, b, c, d)           \
  x[a] += x[b];                            \
  x[d] = CRYPTO_rotl_u32(x[d] ^ x[a], 16); \
  x[c] += x[d];                            \
  x[b] = CRYPTO_rotl_u32(x[b] ^ x[c], 12); \
  x[a] += x[b];                            \
  x[d] = CRYPTO_rotl_u32(x[d] ^ x[a], 8);  \
  x[c] += x[d];                            \
  x[b] = CRYPTO_rotl_u32(x[b] ^ x[c], 7);

void CRYPTO_hchacha20(uint8_t out[32], const uint8_t key[32],
                      const uint8_t nonce[16]) {
  uint32_t x[16];
  OPENSSL_memcpy(x, sigma_words, sizeof(sigma_words));
#ifdef OPENSSL_BIG_ENDIAN
  for(size_t i = 4; i < 12; i++) {
    x[i] = CRYPTO_load_u32_le(key + (i-4) * sizeof(uint32_t));
  }
  for(size_t i = 12; i < 16; i++) {
    x[i] = CRYPTO_load_u32_le(nonce + (i-12) * sizeof(uint32_t));
  }
#else
  OPENSSL_memcpy(&x[4], key, 32);
  OPENSSL_memcpy(&x[12], nonce, 16);
#endif

  for (size_t i = 0; i < 20; i += 2) {
    QUARTERROUND(0, 4, 8, 12)
    QUARTERROUND(1, 5, 9, 13)
    QUARTERROUND(2, 6, 10, 14)
    QUARTERROUND(3, 7, 11, 15)
    QUARTERROUND(0, 5, 10, 15)
    QUARTERROUND(1, 6, 11, 12)
    QUARTERROUND(2, 7, 8, 13)
    QUARTERROUND(3, 4, 9, 14)
  }

#ifdef OPENSSL_BIG_ENDIAN
  for(size_t i = 0; i < 4; i++) {
    CRYPTO_store_u32_le(out + i * sizeof(uint32_t), x[i]);
  }
  for(size_t i = 12; i < 16; i++) {
    CRYPTO_store_u32_le(out + (i-8) * sizeof(uint32_t), x[i]);
  }
#else
  OPENSSL_memcpy(out, &x[0], sizeof(uint32_t) * 4);
  OPENSSL_memcpy(&out[16], &x[12], sizeof(uint32_t) * 4);
#endif
}

#if defined(CHACHA20_ASM_NOHW)
static void ChaCha20_ctr32(uint8_t *out, const uint8_t *in, size_t in_len,
                           const uint32_t key[8], const uint32_t counter[4]) {
#if defined(CHACHA20_ASM_NEON)
  if (ChaCha20_ctr32_neon_capable(in_len)) {
    ChaCha20_ctr32_neon(out, in, in_len, key, counter);
    return;
  }
#endif
#if defined(CHACHA20_ASM_AVX2) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)
  if (ChaCha20_ctr32_avx2_capable(in_len)) {
    ChaCha20_ctr32_avx2(out, in, in_len, key, counter);
    return;
  }
#endif
#if defined(CHACHA20_ASM_SSSE3_4X)
  if (ChaCha20_ctr32_ssse3_4x_capable(in_len)) {
    ChaCha20_ctr32_ssse3_4x(out, in, in_len, key, counter);
    return;
  }
#endif
#if defined(CHACHA20_ASM_SSSE3)
  if (ChaCha20_ctr32_ssse3_capable(in_len)) {
    ChaCha20_ctr32_ssse3(out, in, in_len, key, counter);
    return;
  }
#endif
  if (in_len > 0) {
    ChaCha20_ctr32_nohw(out, in, in_len, key, counter);
  }
}
#endif

#if defined(CHACHA20_ASM_NOHW)

void CRYPTO_chacha_20(uint8_t *out, const uint8_t *in, size_t in_len,
                      const uint8_t key[32], const uint8_t nonce[12],
                      uint32_t counter) {
  assert(!buffers_alias(out, in_len, in, in_len) || in == out);

  uint32_t counter_nonce[4];
  counter_nonce[0] = counter;
  counter_nonce[1] = CRYPTO_load_u32_le(nonce + 0);
  counter_nonce[2] = CRYPTO_load_u32_le(nonce + 4);
  counter_nonce[3] = CRYPTO_load_u32_le(nonce + 8);

  const uint32_t *key_ptr = (const uint32_t *)key;
#if !defined(OPENSSL_X86) && !defined(OPENSSL_X86_64)
  // The assembly expects the key to be four-byte aligned.
  uint32_t key_u32[8];
  if ((((uintptr_t)key) & 3) != 0) {
    key_u32[0] = CRYPTO_load_u32_le(key + 0);
    key_u32[1] = CRYPTO_load_u32_le(key + 4);
    key_u32[2] = CRYPTO_load_u32_le(key + 8);
    key_u32[3] = CRYPTO_load_u32_le(key + 12);
    key_u32[4] = CRYPTO_load_u32_le(key + 16);
    key_u32[5] = CRYPTO_load_u32_le(key + 20);
    key_u32[6] = CRYPTO_load_u32_le(key + 24);
    key_u32[7] = CRYPTO_load_u32_le(key + 28);

    key_ptr = key_u32;
  }
#endif

  while (in_len > 0) {
    // The assembly functions do not have defined overflow behavior. While
    // overflow is almost always a bug in the caller, we prefer our functions to
    // behave the same across platforms, so divide into multiple calls to avoid
    // this case.
    uint64_t todo = 64 * ((UINT64_C(1) << 32) - counter_nonce[0]);
    if (todo > in_len) {
      todo = in_len;
    }

    ChaCha20_ctr32(out, in, (size_t)todo, key_ptr, counter_nonce);
    in += todo;
    out += todo;
    in_len -= todo;

    // We're either done and will next break out of the loop, or we stopped at
    // the wraparound point and the counter should continue at zero.
    counter_nonce[0] = 0;
  }
}

#else

// chacha_core performs 20 rounds of ChaCha on the input words in
// |input| and writes the 64 output bytes to |output|.
static void chacha_core(uint8_t output[64], const uint32_t input[16]) {
  uint32_t x[16];
  int i;

  OPENSSL_memcpy(x, input, sizeof(uint32_t) * 16);
  for (i = 20; i > 0; i -= 2) {
    QUARTERROUND(0, 4, 8, 12)
    QUARTERROUND(1, 5, 9, 13)
    QUARTERROUND(2, 6, 10, 14)
    QUARTERROUND(3, 7, 11, 15)
    QUARTERROUND(0, 5, 10, 15)
    QUARTERROUND(1, 6, 11, 12)
    QUARTERROUND(2, 7, 8, 13)
    QUARTERROUND(3, 4, 9, 14)
  }

  for (i = 0; i < 16; ++i) {
    x[i] += input[i];
  }
  for (i = 0; i < 16; ++i) {
    CRYPTO_store_u32_le(output + 4 * i, x[i]);
  }
}

void CRYPTO_chacha_20(uint8_t *out, const uint8_t *in, size_t in_len,
                      const uint8_t key[32], const uint8_t nonce[12],
                      uint32_t counter) {
  assert(!buffers_alias(out, in_len, in, in_len) || in == out);

  uint32_t input[16];
  uint8_t buf[64];
  size_t todo, i;

  input[0] = sigma_words[0];
  input[1] = sigma_words[1];
  input[2] = sigma_words[2];
  input[3] = sigma_words[3];

  input[4] = CRYPTO_load_u32_le(key + 0);
  input[5] = CRYPTO_load_u32_le(key + 4);
  input[6] = CRYPTO_load_u32_le(key + 8);
  input[7] = CRYPTO_load_u32_le(key + 12);

  input[8] = CRYPTO_load_u32_le(key + 16);
  input[9] = CRYPTO_load_u32_le(key + 20);
  input[10] = CRYPTO_load_u32_le(key + 24);
  input[11] = CRYPTO_load_u32_le(key + 28);

  input[12] = counter;
  input[13] = CRYPTO_load_u32_le(nonce + 0);
  input[14] = CRYPTO_load_u32_le(nonce + 4);
  input[15] = CRYPTO_load_u32_le(nonce + 8);

  while (in_len > 0) {
    todo = sizeof(buf);
    if (in_len < todo) {
      todo = in_len;
    }

    chacha_core(buf, input);
    for (i = 0; i < todo; i++) {
      out[i] = in[i] ^ buf[i];
    }

    out += todo;
    in += todo;
    in_len -= todo;

    input[12]++;
  }
}

#endif
