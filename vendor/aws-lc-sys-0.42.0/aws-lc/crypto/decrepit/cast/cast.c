// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// SPDX-License-Identifier: Apache-2.0

#include <openssl/cipher.h>
#include <openssl/obj.h>

#if defined(OPENSSL_WINDOWS)
OPENSSL_MSVC_PRAGMA(warning(push, 3))
#include <intrin.h>
OPENSSL_MSVC_PRAGMA(warning(pop))
#endif

#include "../../../crypto/internal.h"
#include "../../fipsmodule/cipher/internal.h"
#include "../macros.h"
#include "internal.h"


void CAST_ecb_encrypt(const uint8_t *in, uint8_t *out, const CAST_KEY *ks,
                      int enc) {
  uint32_t d[2];

  n2l(in, d[0]);
  n2l(in, d[1]);
  if (enc) {
    CAST_encrypt(d, ks);
  } else {
    CAST_decrypt(d, ks);
  }
  l2n(d[0], out);
  l2n(d[1], out);
}

#if defined(OPENSSL_WINDOWS) && defined(_MSC_VER)
#define ROTL(a, n) (_lrotl(a, n))
#else
#define ROTL(a, n) ((((a) << (n)) | ((a) >> ((-(n))&31))) & 0xffffffffL)
#endif

#define E_CAST(n, key, L, R, OP1, OP2, OP3)                                   \
  {                                                                           \
    uint32_t a, b, c, d;                                                      \
    t = (key[n * 2] OP1 R) & 0xffffffff;                                      \
    t = ROTL(t, (key[n * 2 + 1]));                                            \
    a = CAST_S_table0[(t >> 8) & 0xff];                                       \
    b = CAST_S_table1[(t)&0xff];                                              \
    c = CAST_S_table2[(t >> 24) & 0xff];                                      \
    d = CAST_S_table3[(t >> 16) & 0xff];                                      \
    L ^= (((((a OP2 b)&0xffffffffL)OP3 c) & 0xffffffffL)OP1 d) & 0xffffffffL; \
  }

void CAST_encrypt(uint32_t *data, const CAST_KEY *key) {
  uint32_t l, r, t;
  const uint32_t *k;

  k = &key->data[0];
  l = data[0];
  r = data[1];

  E_CAST(0, k, l, r, +, ^, -);
  E_CAST(1, k, r, l, ^, -, +);
  E_CAST(2, k, l, r, -, +, ^);
  E_CAST(3, k, r, l, +, ^, -);
  E_CAST(4, k, l, r, ^, -, +);
  E_CAST(5, k, r, l, -, +, ^);
  E_CAST(6, k, l, r, +, ^, -);
  E_CAST(7, k, r, l, ^, -, +);
  E_CAST(8, k, l, r, -, +, ^);
  E_CAST(9, k, r, l, +, ^, -);
  E_CAST(10, k, l, r, ^, -, +);
  E_CAST(11, k, r, l, -, +, ^);

  if (!key->short_key) {
    E_CAST(12, k, l, r, +, ^, -);
    E_CAST(13, k, r, l, ^, -, +);
    E_CAST(14, k, l, r, -, +, ^);
    E_CAST(15, k, r, l, +, ^, -);
  }

  data[1] = l & 0xffffffffL;
  data[0] = r & 0xffffffffL;
}

void CAST_decrypt(uint32_t *data, const CAST_KEY *key) {
  uint32_t l, r, t;
  const uint32_t *k;

  k = &key->data[0];
  l = data[0];
  r = data[1];

  if (!key->short_key) {
    E_CAST(15, k, l, r, +, ^, -);
    E_CAST(14, k, r, l, -, +, ^);
    E_CAST(13, k, l, r, ^, -, +);
    E_CAST(12, k, r, l, +, ^, -);
  }

  E_CAST(11, k, l, r, -, +, ^);
  E_CAST(10, k, r, l, ^, -, +);
  E_CAST(9, k, l, r, +, ^, -);
  E_CAST(8, k, r, l, -, +, ^);
  E_CAST(7, k, l, r, ^, -, +);
  E_CAST(6, k, r, l, +, ^, -);
  E_CAST(5, k, l, r, -, +, ^);
  E_CAST(4, k, r, l, ^, -, +);
  E_CAST(3, k, l, r, +, ^, -);
  E_CAST(2, k, r, l, -, +, ^);
  E_CAST(1, k, l, r, ^, -, +);
  E_CAST(0, k, r, l, +, ^, -);

  data[1] = l & 0xffffffffL;
  data[0] = r & 0xffffffffL;
}

void CAST_cbc_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                      const CAST_KEY *ks, uint8_t *iv, int enc) {
  uint32_t tin0, tin1;
  uint32_t tout0, tout1, xor0, xor1;
  size_t l = length;
  uint32_t tin[2];

  if (enc) {
    n2l(iv, tout0);
    n2l(iv, tout1);
    iv -= 8;
    while (l >= 8) {
      n2l(in, tin0);
      n2l(in, tin1);
      tin0 ^= tout0;
      tin1 ^= tout1;
      tin[0] = tin0;
      tin[1] = tin1;
      CAST_encrypt(tin, ks);
      tout0 = tin[0];
      tout1 = tin[1];
      l2n(tout0, out);
      l2n(tout1, out);
      l -= 8;
    }
    if (l != 0) {
      n2ln(in, tin0, tin1, l);
      tin0 ^= tout0;
      tin1 ^= tout1;
      tin[0] = tin0;
      tin[1] = tin1;
      CAST_encrypt(tin, ks);
      tout0 = tin[0];
      tout1 = tin[1];
      l2n(tout0, out);
      l2n(tout1, out);
    }
    l2n(tout0, iv);
    l2n(tout1, iv);
  } else {
    n2l(iv, xor0);
    n2l(iv, xor1);
    iv -= 8;
    while (l >= 8) {
      n2l(in, tin0);
      n2l(in, tin1);
      tin[0] = tin0;
      tin[1] = tin1;
      CAST_decrypt(tin, ks);
      tout0 = tin[0] ^ xor0;
      tout1 = tin[1] ^ xor1;
      l2n(tout0, out);
      l2n(tout1, out);
      xor0 = tin0;
      xor1 = tin1;
      l -= 8;
    }
    if (l != 0) {
      n2l(in, tin0);
      n2l(in, tin1);
      tin[0] = tin0;
      tin[1] = tin1;
      CAST_decrypt(tin, ks);
      tout0 = tin[0] ^ xor0;
      tout1 = tin[1] ^ xor1;
      l2nn(tout0, tout1, out, l);
      xor0 = tin0;
      xor1 = tin1;
    }
    l2n(xor0, iv);
    l2n(xor1, iv);
  }
  OPENSSL_cleanse(&tin0, sizeof(tin0));
  OPENSSL_cleanse(&tin1, sizeof(tin1));
  OPENSSL_cleanse(&tout0, sizeof(tout0));
  OPENSSL_cleanse(&tout1, sizeof(tout1));
  OPENSSL_cleanse(&xor0, sizeof(xor0));
  OPENSSL_cleanse(&xor1, sizeof(xor1));
  OPENSSL_cleanse(&tin, sizeof(tin));
}

#define CAST_exp(l, A, a, n)   \
  A[n / 4] = l;                \
  a[n + 3] = (l)&0xff;         \
  a[n + 2] = (l >> 8) & 0xff;  \
  a[n + 1] = (l >> 16) & 0xff; \
  a[n + 0] = (l >> 24) & 0xff;
#define S4 CAST_S_table4
#define S5 CAST_S_table5
#define S6 CAST_S_table6
#define S7 CAST_S_table7

void CAST_set_key(CAST_KEY *key, size_t len, const uint8_t *data) {
  uint32_t x[16];
  uint32_t z[16];
  uint32_t k[32];
  uint32_t X[4], Z[4];
  uint32_t l, *K;
  size_t i;

  for (i = 0; i < 16; i++) {
    x[i] = 0;
  }

  if (len > 16) {
    len = 16;
  }

  for (i = 0; i < len; i++) {
    x[i] = data[i];
  }

  if (len <= 10) {
    key->short_key = 1;
  } else {
    key->short_key = 0;
  }

  K = &k[0];
  X[0] = ((x[0] << 24) | (x[1] << 16) | (x[2] << 8) | x[3]) & 0xffffffffL;
  X[1] = ((x[4] << 24) | (x[5] << 16) | (x[6] << 8) | x[7]) & 0xffffffffL;
  X[2] = ((x[8] << 24) | (x[9] << 16) | (x[10] << 8) | x[11]) & 0xffffffffL;
  X[3] = ((x[12] << 24) | (x[13] << 16) | (x[14] << 8) | x[15]) & 0xffffffffL;

  for (;;) {
    l = X[0] ^ S4[x[13]] ^ S5[x[15]] ^ S6[x[12]] ^ S7[x[14]] ^ S6[x[8]];
    CAST_exp(l, Z, z, 0);
    l = X[2] ^ S4[z[0]] ^ S5[z[2]] ^ S6[z[1]] ^ S7[z[3]] ^ S7[x[10]];
    CAST_exp(l, Z, z, 4);
    l = X[3] ^ S4[z[7]] ^ S5[z[6]] ^ S6[z[5]] ^ S7[z[4]] ^ S4[x[9]];
    CAST_exp(l, Z, z, 8);
    l = X[1] ^ S4[z[10]] ^ S5[z[9]] ^ S6[z[11]] ^ S7[z[8]] ^ S5[x[11]];
    CAST_exp(l, Z, z, 12);

    K[0] = S4[z[8]] ^ S5[z[9]] ^ S6[z[7]] ^ S7[z[6]] ^ S4[z[2]];
    K[1] = S4[z[10]] ^ S5[z[11]] ^ S6[z[5]] ^ S7[z[4]] ^ S5[z[6]];
    K[2] = S4[z[12]] ^ S5[z[13]] ^ S6[z[3]] ^ S7[z[2]] ^ S6[z[9]];
    K[3] = S4[z[14]] ^ S5[z[15]] ^ S6[z[1]] ^ S7[z[0]] ^ S7[z[12]];

    l = Z[2] ^ S4[z[5]] ^ S5[z[7]] ^ S6[z[4]] ^ S7[z[6]] ^ S6[z[0]];
    CAST_exp(l, X, x, 0);
    l = Z[0] ^ S4[x[0]] ^ S5[x[2]] ^ S6[x[1]] ^ S7[x[3]] ^ S7[z[2]];
    CAST_exp(l, X, x, 4);
    l = Z[1] ^ S4[x[7]] ^ S5[x[6]] ^ S6[x[5]] ^ S7[x[4]] ^ S4[z[1]];
    CAST_exp(l, X, x, 8);
    l = Z[3] ^ S4[x[10]] ^ S5[x[9]] ^ S6[x[11]] ^ S7[x[8]] ^ S5[z[3]];
    CAST_exp(l, X, x, 12);

    K[4] = S4[x[3]] ^ S5[x[2]] ^ S6[x[12]] ^ S7[x[13]] ^ S4[x[8]];
    K[5] = S4[x[1]] ^ S5[x[0]] ^ S6[x[14]] ^ S7[x[15]] ^ S5[x[13]];
    K[6] = S4[x[7]] ^ S5[x[6]] ^ S6[x[8]] ^ S7[x[9]] ^ S6[x[3]];
    K[7] = S4[x[5]] ^ S5[x[4]] ^ S6[x[10]] ^ S7[x[11]] ^ S7[x[7]];

    l = X[0] ^ S4[x[13]] ^ S5[x[15]] ^ S6[x[12]] ^ S7[x[14]] ^ S6[x[8]];
    CAST_exp(l, Z, z, 0);
    l = X[2] ^ S4[z[0]] ^ S5[z[2]] ^ S6[z[1]] ^ S7[z[3]] ^ S7[x[10]];
    CAST_exp(l, Z, z, 4);
    l = X[3] ^ S4[z[7]] ^ S5[z[6]] ^ S6[z[5]] ^ S7[z[4]] ^ S4[x[9]];
    CAST_exp(l, Z, z, 8);
    l = X[1] ^ S4[z[10]] ^ S5[z[9]] ^ S6[z[11]] ^ S7[z[8]] ^ S5[x[11]];
    CAST_exp(l, Z, z, 12);

    K[8] = S4[z[3]] ^ S5[z[2]] ^ S6[z[12]] ^ S7[z[13]] ^ S4[z[9]];
    K[9] = S4[z[1]] ^ S5[z[0]] ^ S6[z[14]] ^ S7[z[15]] ^ S5[z[12]];
    K[10] = S4[z[7]] ^ S5[z[6]] ^ S6[z[8]] ^ S7[z[9]] ^ S6[z[2]];
    K[11] = S4[z[5]] ^ S5[z[4]] ^ S6[z[10]] ^ S7[z[11]] ^ S7[z[6]];

    l = Z[2] ^ S4[z[5]] ^ S5[z[7]] ^ S6[z[4]] ^ S7[z[6]] ^ S6[z[0]];
    CAST_exp(l, X, x, 0);
    l = Z[0] ^ S4[x[0]] ^ S5[x[2]] ^ S6[x[1]] ^ S7[x[3]] ^ S7[z[2]];
    CAST_exp(l, X, x, 4);
    l = Z[1] ^ S4[x[7]] ^ S5[x[6]] ^ S6[x[5]] ^ S7[x[4]] ^ S4[z[1]];
    CAST_exp(l, X, x, 8);
    l = Z[3] ^ S4[x[10]] ^ S5[x[9]] ^ S6[x[11]] ^ S7[x[8]] ^ S5[z[3]];
    CAST_exp(l, X, x, 12);

    K[12] = S4[x[8]] ^ S5[x[9]] ^ S6[x[7]] ^ S7[x[6]] ^ S4[x[3]];
    K[13] = S4[x[10]] ^ S5[x[11]] ^ S6[x[5]] ^ S7[x[4]] ^ S5[x[7]];
    K[14] = S4[x[12]] ^ S5[x[13]] ^ S6[x[3]] ^ S7[x[2]] ^ S6[x[8]];
    K[15] = S4[x[14]] ^ S5[x[15]] ^ S6[x[1]] ^ S7[x[0]] ^ S7[x[13]];
    if (K != k) {
      break;
    }
    K += 16;
  }

  for (i = 0; i < 16; i++) {
    key->data[i * 2] = k[i];
    key->data[i * 2 + 1] = ((k[i + 16]) + 16) & 0x1f;
  }
}

static int cast_init_key(EVP_CIPHER_CTX *ctx, const uint8_t *key,
                         const uint8_t *iv, int enc) {
  CAST_KEY *cast_key = ctx->cipher_data;
  CAST_set_key(cast_key, ctx->key_len, key);
  return 1;
}

static int cast_ecb_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out, const uint8_t *in,
                           size_t len) {
  CAST_KEY *cast_key = ctx->cipher_data;
  assert(len % CAST_BLOCK == 0);

  while (len >= CAST_BLOCK) {
    CAST_ecb_encrypt(in, out, cast_key, ctx->encrypt);
    in += CAST_BLOCK;
    out += CAST_BLOCK;
    len -= CAST_BLOCK;
  }
  assert(len == 0);

  return 1;
}

static int cast_cbc_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out, const uint8_t *in,
                           size_t len) {
  CAST_KEY *cast_key = ctx->cipher_data;
  CAST_cbc_encrypt(in, out, len, cast_key, ctx->iv, ctx->encrypt);
  return 1;
}

static const EVP_CIPHER cast5_ecb = {
    .nid = NID_cast5_ecb,
    .block_size = CAST_BLOCK,
    .key_len = CAST_KEY_LENGTH,
    .iv_len = CAST_BLOCK,
    .ctx_size = sizeof(CAST_KEY),
    .flags = EVP_CIPH_ECB_MODE | EVP_CIPH_VARIABLE_LENGTH,
    .init = cast_init_key,
    .cipher = cast_ecb_cipher,
};

static const EVP_CIPHER cast5_cbc = {
    .nid = NID_cast5_cbc,
    .block_size = CAST_BLOCK,
    .key_len = CAST_KEY_LENGTH,
    .iv_len = CAST_BLOCK,
    .ctx_size = sizeof(CAST_KEY),
    .flags = EVP_CIPH_CBC_MODE | EVP_CIPH_VARIABLE_LENGTH,
    .init = cast_init_key,
    .cipher = cast_cbc_cipher,
};

const EVP_CIPHER *EVP_cast5_ecb(void) { return &cast5_ecb; }

const EVP_CIPHER *EVP_cast5_cbc(void) { return &cast5_cbc; }
