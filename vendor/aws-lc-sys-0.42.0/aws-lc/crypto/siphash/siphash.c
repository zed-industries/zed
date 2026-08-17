// Copyright (c) 2019, Google Inc.
// SPDX-License-Identifier: ISC

#include <stdint.h>
#include <string.h>

#include <openssl/siphash.h>

#include "../internal.h"


static void siphash_round(uint64_t v[4]) {
  v[0] += v[1];
  v[2] += v[3];
  v[1] = CRYPTO_rotl_u64(v[1], 13);
  v[3] = CRYPTO_rotl_u64(v[3], 16);
  v[1] ^= v[0];
  v[3] ^= v[2];
  v[0] = CRYPTO_rotl_u64(v[0], 32);
  v[2] += v[1];
  v[0] += v[3];
  v[1] = CRYPTO_rotl_u64(v[1], 17);
  v[3] = CRYPTO_rotl_u64(v[3], 21);
  v[1] ^= v[2];
  v[3] ^= v[0];
  v[2] = CRYPTO_rotl_u64(v[2], 32);
}

uint64_t SIPHASH_24(const uint64_t key[2], const uint8_t *input,
                    size_t input_len) {
  const size_t orig_input_len = input_len;

  uint64_t v[4], k0, k1;
#ifdef OPENSSL_BIG_ENDIAN
  k0 = CRYPTO_bswap8(key[0]);
  k1 = CRYPTO_bswap8(key[1]);
#else
  k0 = key[0];
  k1 = key[1];
#endif
  v[0] = k0 ^ UINT64_C(0x736f6d6570736575);
  v[1] = k1 ^ UINT64_C(0x646f72616e646f6d);
  v[2] = k0 ^ UINT64_C(0x6c7967656e657261);
  v[3] = k1 ^ UINT64_C(0x7465646279746573);

  while (input_len >= sizeof(uint64_t)) {
    uint64_t m = CRYPTO_load_u64_le(input);
    v[3] ^= m;
    siphash_round(v);
    siphash_round(v);
    v[0] ^= m;

    input += sizeof(uint64_t);
    input_len -= sizeof(uint64_t);
  }

  uint8_t last_block[8];
  OPENSSL_memset(last_block, 0, sizeof(last_block));
  OPENSSL_memcpy(last_block, input, input_len);
  last_block[7] = orig_input_len & 0xff;

  uint64_t last_block_word = CRYPTO_load_u64_le(last_block);
  v[3] ^= last_block_word;
  siphash_round(v);
  siphash_round(v);
  v[0] ^= last_block_word;

  v[2] ^= 0xff;
  siphash_round(v);
  siphash_round(v);
  siphash_round(v);
  siphash_round(v);

  return v[0] ^ v[1] ^ v[2] ^ v[3];
}
