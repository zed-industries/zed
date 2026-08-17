// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

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

BIGNUM *BN_le2bn(const uint8_t *in, size_t len, BIGNUM *ret) {
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
  ret->neg = 0;

  bn_little_endian_to_words(ret->d, ret->width, in, len);

  return ret;
}

void bn_little_endian_to_words(BN_ULONG *out, size_t out_len, const uint8_t *in, const size_t in_len) {
  assert(out_len > 0);
#ifdef OPENSSL_BIG_ENDIAN
  size_t in_index = 0;
  for (size_t i = 0; i < out_len; i++) {
    if ((in_len-in_index) < sizeof(BN_ULONG)) {
      // Load the last partial word.
      BN_ULONG word = 0;
      // size_t is unsigned, so j >= 0 is always true.
      for (size_t j = in_len-1; j >= in_index && j < in_len; j--) {
        word = (word << 8) | in[j];
      }
      in_index = in_len;
      out[i] = word;

      // Fill the remainder with zeros.
      OPENSSL_memset(out + i + 1, 0, (out_len - i - 1) * sizeof(BN_ULONG));
      break;
    }

    out[i] = CRYPTO_load_word_le(in + in_index);
    in_index += sizeof(BN_ULONG);
  }

  // The caller should have sized the output to avoid truncation.
  assert(in_index == in_len);
#else
  OPENSSL_memcpy(out, in, in_len);
  // Fill the remainder with zeros.
  OPENSSL_memset( ((uint8_t*)out) + in_len, 0, sizeof(BN_ULONG)*out_len - in_len);
#endif
}

// fits_in_bytes returns one if the |num_words| words in |words| can be
// represented in |num_bytes| bytes.
static int fits_in_bytes(const BN_ULONG *words, size_t num_words,
                         size_t num_bytes) {
  uint8_t mask = 0;
#ifdef OPENSSL_BIG_ENDIAN
  for (size_t i = num_bytes / BN_BYTES; i < num_words; i++) {
    BN_ULONG word = words[i];
    for (size_t j = 0; j < BN_BYTES; j++) {
      if ((i * BN_BYTES) + j < num_bytes) {
        // For the first word we don't need to check any bytes shorter than len
        continue ;
      } else {
        mask |= (word >> (j * 8)) & 0xff;
      }
    }
  }
#else
  const uint8_t *bytes = (const uint8_t *)words;
  size_t tot_bytes = num_words * sizeof(BN_ULONG);
  for (size_t i = num_bytes; i < tot_bytes; i++) {
    mask |= bytes[i];
  }
#endif
  return mask == 0;
}

// Asserts that the BIGNUM can be represented within |num| bytes.
// The logic is consistent with `fits_in_bytes` but assertions will fail when false.
void bn_assert_fits_in_bytes(const BIGNUM *bn, size_t num) {
  const uint8_t *bytes = (const uint8_t *)bn->d;
  size_t tot_bytes = bn->width * sizeof(BN_ULONG);
  if (tot_bytes > num) {
    CONSTTIME_DECLASSIFY(bytes + num, tot_bytes - num);
// Avoids compiler error: unused variable 'byte' or 'word'
// The assert statements below are only effective in DEBUG builds
#ifndef NDEBUG
#ifdef OPENSSL_BIG_ENDIAN
    for (int i = num / BN_BYTES; i < bn->width; i++) {
      BN_ULONG word = bn->d[i];
      for (size_t j = 0; j < BN_BYTES; j++) {
        if ((i * BN_BYTES) + j < num) {
          // For the first word we don't need to check any bytes shorter than len
          continue;
        } else {
          uint8_t byte = (word >> (j * 8)) & 0xff;
          assert(byte == 0);
        }
      }
    }
#else
    for (size_t i = num; i < tot_bytes; i++) {
      assert(bytes[i] == 0);
    }
#endif
#endif
    (void)bytes;
  }
}

void bn_words_to_big_endian(uint8_t *out, size_t out_len, const BN_ULONG *in,
                            size_t in_len) {
  // The caller should have selected an output length without truncation.
  declassify_assert(fits_in_bytes(in, in_len, out_len));
  size_t num_bytes = in_len * sizeof(BN_ULONG);
  if (out_len < num_bytes) {
    num_bytes = out_len;
  }

  // The redundant `i < out_len` guard below silences a GCC 14+
  // `-Wstringop-overflow` false positive under LTO (see aws-lc#3166).
#ifdef OPENSSL_BIG_ENDIAN
  for (size_t i = 0; i < num_bytes && i < out_len; i++) {
    BN_ULONG l = in[i / BN_BYTES];
    out[out_len - i - 1] = (uint8_t)(l >> (8 * (i % BN_BYTES))) & 0xff;
  }
#else
  const uint8_t *bytes = (const uint8_t *)in;
  for (size_t i = 0; i < num_bytes && i < out_len; i++) {
    out[out_len - i - 1] = bytes[i];
  }
#endif
  // Pad out the rest of the buffer with zeroes.
  OPENSSL_memset(out, 0, out_len - num_bytes);
}

size_t BN_bn2bin(const BIGNUM *in, uint8_t *out) {
  size_t n = BN_num_bytes(in);
  bn_words_to_big_endian(out, n, in->d, in->width);
  return n;
}

void bn_words_to_little_endian(uint8_t *out, size_t out_len, const BN_ULONG *in, const size_t in_len) {
  // The caller should have selected an output length without truncation.
  assert(fits_in_bytes(in, in_len, out_len));
  size_t num_bytes = in_len * sizeof(BN_ULONG);
  if (out_len < num_bytes) {
    num_bytes = out_len;
  }
#ifdef OPENSSL_BIG_ENDIAN
  size_t byte_idx = 0;
  for (size_t word_idx = 0; word_idx < in_len; word_idx++) {
    BN_ULONG l = in[word_idx];
    for(size_t j = 0; j < BN_BYTES && byte_idx < num_bytes; j++) {
      out[byte_idx] = (uint8_t)(l & 0xff);
      l >>= 8;
      byte_idx++;
    }
  }
#else
  const uint8_t *bytes = (const uint8_t *)in;
  OPENSSL_memcpy(out, bytes, num_bytes);
#endif
  // Fill the remainder with zeros.
  OPENSSL_memset(out + num_bytes, 0, out_len - num_bytes);
}

int BN_bn2le_padded(uint8_t *out, size_t len, const BIGNUM *in) {
  if (!fits_in_bytes(in->d, in->width, len)) {
    return 0;
  }
  bn_words_to_little_endian(out, len, in->d, in->width);
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
