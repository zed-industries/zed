// Copyright 2004-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#include <string.h>

#include <openssl/mem.h>

#include "../../internal.h"
#include "../bcm_interface.h"
#include "../digest/md32_common.h"
#include "../service_indicator/internal.h"
#include "internal.h"


bcm_infallible BCM_sha224_init(SHA256_CTX *sha) {
  OPENSSL_memset(sha, 0, sizeof(SHA256_CTX));
  sha->h[0] = 0xc1059ed8UL;
  sha->h[1] = 0x367cd507UL;
  sha->h[2] = 0x3070dd17UL;
  sha->h[3] = 0xf70e5939UL;
  sha->h[4] = 0xffc00b31UL;
  sha->h[5] = 0x68581511UL;
  sha->h[6] = 0x64f98fa7UL;
  sha->h[7] = 0xbefa4fa4UL;
  sha->md_len = BCM_SHA224_DIGEST_LENGTH;
  return bcm_infallible::approved;
}

bcm_infallible BCM_sha256_init(SHA256_CTX *sha) {
  OPENSSL_memset(sha, 0, sizeof(SHA256_CTX));
  sha->h[0] = 0x6a09e667UL;
  sha->h[1] = 0xbb67ae85UL;
  sha->h[2] = 0x3c6ef372UL;
  sha->h[3] = 0xa54ff53aUL;
  sha->h[4] = 0x510e527fUL;
  sha->h[5] = 0x9b05688cUL;
  sha->h[6] = 0x1f83d9abUL;
  sha->h[7] = 0x5be0cd19UL;
  sha->md_len = BCM_SHA256_DIGEST_LENGTH;
  return bcm_infallible::approved;
}

#if !defined(SHA256_ASM)
static void sha256_block_data_order(uint32_t state[8], const uint8_t *in,
                                    size_t num);
#endif

bcm_infallible BCM_sha256_transform(SHA256_CTX *c,
                                    const uint8_t data[BCM_SHA256_CBLOCK]) {
  sha256_block_data_order(c->h, data, 1);
  return bcm_infallible::approved;
}

bcm_infallible BCM_sha256_update(SHA256_CTX *c, const void *data, size_t len) {
  crypto_md32_update(&sha256_block_data_order, c->h, c->data, BCM_SHA256_CBLOCK,
                     &c->num, &c->Nh, &c->Nl,
                     reinterpret_cast<const uint8_t *>(data), len);
  return bcm_infallible::approved;
}

bcm_infallible BCM_sha224_update(SHA256_CTX *ctx, const void *data,
                                 size_t len) {
  return BCM_sha256_update(ctx, data, len);
}

static void sha256_final_impl(uint8_t *out, size_t md_len, SHA256_CTX *c) {
  crypto_md32_final(&sha256_block_data_order, c->h, c->data, BCM_SHA256_CBLOCK,
                    &c->num, c->Nh, c->Nl, /*is_big_endian=*/1);

  BSSL_CHECK(md_len <= BCM_SHA256_DIGEST_LENGTH);

  assert(md_len % 4 == 0);
  const size_t out_words = md_len / 4;
  for (size_t i = 0; i < out_words; i++) {
    CRYPTO_store_u32_be(out, c->h[i]);
    out += 4;
  }

  FIPS_service_indicator_update_state();
}

bcm_infallible BCM_sha256_final(uint8_t out[BCM_SHA256_DIGEST_LENGTH],
                                SHA256_CTX *c) {
  // Ideally we would assert |sha->md_len| is |BCM_SHA256_DIGEST_LENGTH| to
  // match the size hint, but calling code often pairs |SHA224_Init| with
  // |SHA256_Final| and expects |sha->md_len| to carry the size over.
  //
  // TODO(davidben): Add an assert and fix code to match them up.
  sha256_final_impl(out, c->md_len, c);
  return bcm_infallible::approved;
}

bcm_infallible BCM_sha224_final(uint8_t out[BCM_SHA224_DIGEST_LENGTH],
                                SHA256_CTX *ctx) {
  // This function must be paired with |SHA224_Init|, which sets |ctx->md_len|
  // to |BCM_SHA224_DIGEST_LENGTH|.
  assert(ctx->md_len == BCM_SHA224_DIGEST_LENGTH);
  sha256_final_impl(out, BCM_SHA224_DIGEST_LENGTH, ctx);
  return bcm_infallible::approved;
}

#if !defined(SHA256_ASM)

#if !defined(SHA256_ASM_NOHW)
static const uint32_t K256[64] = {
    0x428a2f98UL, 0x71374491UL, 0xb5c0fbcfUL, 0xe9b5dba5UL, 0x3956c25bUL,
    0x59f111f1UL, 0x923f82a4UL, 0xab1c5ed5UL, 0xd807aa98UL, 0x12835b01UL,
    0x243185beUL, 0x550c7dc3UL, 0x72be5d74UL, 0x80deb1feUL, 0x9bdc06a7UL,
    0xc19bf174UL, 0xe49b69c1UL, 0xefbe4786UL, 0x0fc19dc6UL, 0x240ca1ccUL,
    0x2de92c6fUL, 0x4a7484aaUL, 0x5cb0a9dcUL, 0x76f988daUL, 0x983e5152UL,
    0xa831c66dUL, 0xb00327c8UL, 0xbf597fc7UL, 0xc6e00bf3UL, 0xd5a79147UL,
    0x06ca6351UL, 0x14292967UL, 0x27b70a85UL, 0x2e1b2138UL, 0x4d2c6dfcUL,
    0x53380d13UL, 0x650a7354UL, 0x766a0abbUL, 0x81c2c92eUL, 0x92722c85UL,
    0xa2bfe8a1UL, 0xa81a664bUL, 0xc24b8b70UL, 0xc76c51a3UL, 0xd192e819UL,
    0xd6990624UL, 0xf40e3585UL, 0x106aa070UL, 0x19a4c116UL, 0x1e376c08UL,
    0x2748774cUL, 0x34b0bcb5UL, 0x391c0cb3UL, 0x4ed8aa4aUL, 0x5b9cca4fUL,
    0x682e6ff3UL, 0x748f82eeUL, 0x78a5636fUL, 0x84c87814UL, 0x8cc70208UL,
    0x90befffaUL, 0xa4506cebUL, 0xbef9a3f7UL, 0xc67178f2UL};

// See FIPS 180-4, section 4.1.2.
#define Sigma0(x)                                       \
  (CRYPTO_rotr_u32((x), 2) ^ CRYPTO_rotr_u32((x), 13) ^ \
   CRYPTO_rotr_u32((x), 22))
#define Sigma1(x)                                       \
  (CRYPTO_rotr_u32((x), 6) ^ CRYPTO_rotr_u32((x), 11) ^ \
   CRYPTO_rotr_u32((x), 25))
#define sigma0(x) \
  (CRYPTO_rotr_u32((x), 7) ^ CRYPTO_rotr_u32((x), 18) ^ ((x) >> 3))
#define sigma1(x) \
  (CRYPTO_rotr_u32((x), 17) ^ CRYPTO_rotr_u32((x), 19) ^ ((x) >> 10))

#define Ch(x, y, z) (((x) & (y)) ^ ((~(x)) & (z)))
#define Maj(x, y, z) (((x) & (y)) ^ ((x) & (z)) ^ ((y) & (z)))

#define ROUND_00_15(i, a, b, c, d, e, f, g, h)   \
  do {                                           \
    T1 += h + Sigma1(e) + Ch(e, f, g) + K256[i]; \
    h = Sigma0(a) + Maj(a, b, c);                \
    d += T1;                                     \
    h += T1;                                     \
  } while (0)

#define ROUND_16_63(i, a, b, c, d, e, f, g, h, X)      \
  do {                                                 \
    s0 = X[(i + 1) & 0x0f];                            \
    s0 = sigma0(s0);                                   \
    s1 = X[(i + 14) & 0x0f];                           \
    s1 = sigma1(s1);                                   \
    T1 = X[(i) & 0x0f] += s0 + s1 + X[(i + 9) & 0x0f]; \
    ROUND_00_15(i, a, b, c, d, e, f, g, h);            \
  } while (0)

static void sha256_block_data_order_nohw(uint32_t state[8], const uint8_t *data,
                                         size_t num) {
  uint32_t a, b, c, d, e, f, g, h, s0, s1, T1;
  uint32_t X[16];
  int i;

  while (num--) {
    a = state[0];
    b = state[1];
    c = state[2];
    d = state[3];
    e = state[4];
    f = state[5];
    g = state[6];
    h = state[7];

    T1 = X[0] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(0, a, b, c, d, e, f, g, h);
    T1 = X[1] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(1, h, a, b, c, d, e, f, g);
    T1 = X[2] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(2, g, h, a, b, c, d, e, f);
    T1 = X[3] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(3, f, g, h, a, b, c, d, e);
    T1 = X[4] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(4, e, f, g, h, a, b, c, d);
    T1 = X[5] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(5, d, e, f, g, h, a, b, c);
    T1 = X[6] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(6, c, d, e, f, g, h, a, b);
    T1 = X[7] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(7, b, c, d, e, f, g, h, a);
    T1 = X[8] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(8, a, b, c, d, e, f, g, h);
    T1 = X[9] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(9, h, a, b, c, d, e, f, g);
    T1 = X[10] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(10, g, h, a, b, c, d, e, f);
    T1 = X[11] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(11, f, g, h, a, b, c, d, e);
    T1 = X[12] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(12, e, f, g, h, a, b, c, d);
    T1 = X[13] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(13, d, e, f, g, h, a, b, c);
    T1 = X[14] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(14, c, d, e, f, g, h, a, b);
    T1 = X[15] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(15, b, c, d, e, f, g, h, a);

    for (i = 16; i < 64; i += 8) {
      ROUND_16_63(i + 0, a, b, c, d, e, f, g, h, X);
      ROUND_16_63(i + 1, h, a, b, c, d, e, f, g, X);
      ROUND_16_63(i + 2, g, h, a, b, c, d, e, f, X);
      ROUND_16_63(i + 3, f, g, h, a, b, c, d, e, X);
      ROUND_16_63(i + 4, e, f, g, h, a, b, c, d, X);
      ROUND_16_63(i + 5, d, e, f, g, h, a, b, c, X);
      ROUND_16_63(i + 6, c, d, e, f, g, h, a, b, X);
      ROUND_16_63(i + 7, b, c, d, e, f, g, h, a, X);
    }

    state[0] += a;
    state[1] += b;
    state[2] += c;
    state[3] += d;
    state[4] += e;
    state[5] += f;
    state[6] += g;
    state[7] += h;
  }
}

#endif  // !defined(SHA256_ASM_NOHW)

static void sha256_block_data_order(uint32_t state[8], const uint8_t *data,
                                    size_t num) {
#if defined(SHA256_ASM_HW)
  if (sha256_hw_capable()) {
    sha256_block_data_order_hw(state, data, num);
    return;
  }
#endif
#if defined(SHA256_ASM_AVX)
  if (sha256_avx_capable()) {
    sha256_block_data_order_avx(state, data, num);
    return;
  }
#endif
#if defined(SHA256_ASM_SSSE3)
  if (sha256_ssse3_capable()) {
    sha256_block_data_order_ssse3(state, data, num);
    return;
  }
#endif
#if defined(SHA256_ASM_NEON)
  if (CRYPTO_is_NEON_capable()) {
    sha256_block_data_order_neon(state, data, num);
    return;
  }
#endif
  sha256_block_data_order_nohw(state, data, num);
}

#endif  // !defined(SHA256_ASM)


bcm_infallible BCM_sha256_transform_blocks(uint32_t state[8],
                                           const uint8_t *data,
                                           size_t num_blocks) {
  sha256_block_data_order(state, data, num_blocks);
  return bcm_infallible::approved;
}

#undef Sigma0
#undef Sigma1
#undef sigma0
#undef sigma1
#undef Ch
#undef Maj
#undef ROUND_00_15
#undef ROUND_16_63
