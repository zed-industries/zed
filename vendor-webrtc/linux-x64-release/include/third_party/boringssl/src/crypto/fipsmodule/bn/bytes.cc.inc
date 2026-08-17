// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#include <openssl/bn.h>

#include <assert.h>
#include <limits.h>

#include "internal.h"

void bn_big_endian_to_words(BN_ULONG *out, size_t out_len, const uint8_t *in,
                            size_t in_len) {
  // The caller should have sized |out| to fit |in| without truncating. This
  // condition ensures we do not overflow |out|, so use a runtime check.
  BSSL_CHECK(in_len <= out_len * sizeof(BN_ULONG));

  // Load whole words.
  while (in_len >= sizeof(BN_ULONG)) {
    in_len -= sizeof(BN_ULONG);
    out[0] = CRYPTO_load_word_be(in + in_len);
    out++;
    out_len--;
  }

  // Load the last partial word.
  if (in_len != 0) {
    BN_ULONG word = 0;
    for (size_t i = 0; i < in_len; i++) {
      word = (word << 8) | in[i];
    }
    out[0] = word;
    out++;
    out_len--;
  }

  // Fill the remainder with zeros.
  OPENSSL_memset(out, 0, out_len * sizeof(BN_ULONG));
}

BIGNUM *BN_bin2bn(const uint8_t *in, size_t len, BIGNUM *ret) {
  BIGNUM *bn = NULL;
  if (ret == NULL) {
    bn = BN_new();
    if (bn == NULL) {
      return NULL;
    }
    ret = bn;
  }

  if (len == 0) {
    ret->width = 0;
    return ret;
  }

  size_t num_words = ((len - 1) / BN_BYTES) + 1;
  if (!bn_wexpand(ret, num_words)) {
    BN_free(bn);
    return NULL;
  }

  // |bn_wexpand| must check bounds on |num_words| to write it into
  // |ret->dmax|.
  assert(num_words <= INT_MAX);
  ret->width = (int)num_words;
  ret->neg = 0;

  bn_big_endian_to_words(ret->d, ret->width, in, len);
  return ret;
}

BIGNUM *BN_lebin2bn(const uint8_t *in, size_t len, BIGNUM *ret) {
  BIGNUM *bn = NULL;
  if (ret == NULL) {
    bn = BN_new();
    if (bn == NULL) {
      return NULL;
    }
    ret = bn;
  }

  if (len == 0) {
    ret->width = 0;
    ret->neg = 0;
    return ret;
  }

  // Reserve enough space in |ret|.
  size_t num_words = ((len - 1) / BN_BYTES) + 1;
  if (!bn_wexpand(ret, num_words)) {
    BN_free(bn);
    return NULL;
  }
  ret->width = (int)num_words;

  // Make sure the top bytes will be zeroed.
  ret->d[num_words - 1] = 0;

  // We only support little-endian platforms, so we can simply memcpy the
  // internal representation.
  OPENSSL_memcpy(ret->d, in, len);
  return ret;
}

BIGNUM *BN_le2bn(const uint8_t *in, size_t len, BIGNUM *ret) {
  return BN_lebin2bn(in, len, ret);
}

// fits_in_bytes returns one if the |num_words| words in |words| can be
// represented in |num_bytes| bytes.
static int fits_in_bytes(const BN_ULONG *words, size_t num_words,
                         size_t num_bytes) {
  const uint8_t *bytes = (const uint8_t *)words;
  size_t tot_bytes = num_words * sizeof(BN_ULONG);
  uint8_t mask = 0;
  for (size_t i = num_bytes; i < tot_bytes; i++) {
    mask |= bytes[i];
  }
  return mask == 0;
}

void bn_assert_fits_in_bytes(const BIGNUM *bn, size_t num) {
  const uint8_t *bytes = (const uint8_t *)bn->d;
  size_t tot_bytes = bn->width * sizeof(BN_ULONG);
  if (tot_bytes > num) {
    CONSTTIME_DECLASSIFY(bytes + num, tot_bytes - num);
    for (size_t i = num; i < tot_bytes; i++) {
      assert(bytes[i] == 0);
    }
    (void)bytes;
  }
}

void bn_words_to_big_endian(uint8_t *out, size_t out_len, const BN_ULONG *in,
                            size_t in_len) {
  // The caller should have selected an output length without truncation.
  declassify_assert(fits_in_bytes(in, in_len, out_len));

  // We only support little-endian platforms, so the internal representation is
  // also little-endian as bytes. We can simply copy it in reverse.
  const uint8_t *bytes = (const uint8_t *)in;
  size_t num_bytes = in_len * sizeof(BN_ULONG);
  if (out_len < num_bytes) {
    num_bytes = out_len;
  }

  for (size_t i = 0; i < num_bytes; i++) {
    out[out_len - i - 1] = bytes[i];
  }
  // Pad out the rest of the buffer with zeroes.
  OPENSSL_memset(out, 0, out_len - num_bytes);
}

size_t BN_bn2bin(const BIGNUM *in, uint8_t *out) {
  size_t n = BN_num_bytes(in);
  bn_words_to_big_endian(out, n, in->d, in->width);
  return n;
}

int BN_bn2le_padded(uint8_t *out, size_t len, const BIGNUM *in) {
  if (!fits_in_bytes(in->d, in->width, len)) {
    return 0;
  }

  // We only support little-endian platforms, so we can simply memcpy into the
  // internal representation.
  const uint8_t *bytes = (const uint8_t *)in->d;
  size_t num_bytes = in->width * BN_BYTES;
  if (len < num_bytes) {
    num_bytes = len;
  }

  OPENSSL_memcpy(out, bytes, num_bytes);
  // Pad out the rest of the buffer with zeroes.
  OPENSSL_memset(out + num_bytes, 0, len - num_bytes);
  return 1;
}

int BN_bn2bin_padded(uint8_t *out, size_t len, const BIGNUM *in) {
  if (!fits_in_bytes(in->d, in->width, len)) {
    return 0;
  }

  bn_words_to_big_endian(out, len, in->d, in->width);
  return 1;
}

BN_ULONG BN_get_word(const BIGNUM *bn) {
  switch (bn_minimal_width(bn)) {
    case 0:
      return 0;
    case 1:
      return bn->d[0];
    default:
      return BN_MASK2;
  }
}

int BN_get_u64(const BIGNUM *bn, uint64_t *out) {
  switch (bn_minimal_width(bn)) {
    case 0:
      *out = 0;
      return 1;
    case 1:
      *out = bn->d[0];
      return 1;
#if defined(OPENSSL_32_BIT)
    case 2:
      *out = (uint64_t) bn->d[0] | (((uint64_t) bn->d[1]) << 32);
      return 1;
#endif
    default:
      return 0;
  }
}
