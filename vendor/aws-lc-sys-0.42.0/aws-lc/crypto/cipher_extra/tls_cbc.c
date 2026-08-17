// Copyright (c) 2012 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <assert.h>
#include <string.h>

#include <openssl/digest.h>
#include <openssl/nid.h>
#include <openssl/sha.h>

#include "../internal.h"
#include "internal.h"
#include "../fipsmodule/cipher/internal.h"

// The length of the additional data field in AES-CBC-HMAC based AEADs.
#define AEAD_TLS_AES_CBC_HMAC_AD_LENGTH (13)

int EVP_tls_cbc_remove_padding(crypto_word_t *out_padding_ok, size_t *out_len,
                               const uint8_t *in, size_t in_len,
                               size_t block_size, size_t mac_size) {
  const size_t overhead = 1 /* padding length byte */ + mac_size;

  // These lengths are all public so we can test them in non-constant time.
  if (overhead > in_len) {
    return 0;
  }

  size_t padding_length = in[in_len - 1];

  crypto_word_t good = constant_time_ge_w(in_len, overhead + padding_length);
  // The padding consists of a length byte at the end of the record and
  // then that many bytes of padding, all with the same value as the
  // length byte. Thus, with the length byte included, there are i+1
  // bytes of padding.
  //
  // We can't check just |padding_length+1| bytes because that leaks
  // decrypted information. Therefore we always have to check the maximum
  // amount of padding possible. (Again, the length of the record is
  // public information so we can use it.)
  size_t to_check = 256;  // maximum amount of padding, inc length byte.
  if (to_check > in_len) {
    to_check = in_len;
  }

  for (size_t i = 0; i < to_check; i++) {
    uint8_t mask = constant_time_ge_8(padding_length, i);
    uint8_t b = in[in_len - 1 - i];
    // The final |padding_length+1| bytes should all have the value
    // |padding_length|. Therefore the XOR should be zero.
    good &= ~(mask & (padding_length ^ b));
  }

  // If any of the final |padding_length+1| bytes had the wrong value,
  // one or more of the lower eight bits of |good| will be cleared.
  good = constant_time_eq_w(0xff, good & 0xff);

  // Always treat |padding_length| as zero on error. If, assuming block size of
  // 16, a padding of [<15 arbitrary bytes> 15] treated |padding_length| as 16
  // and returned -1, distinguishing good MAC and bad padding from bad MAC and
  // bad padding would give POODLE's padding oracle.
  padding_length = good & (padding_length + 1);
  *out_len = in_len - padding_length;
  *out_padding_ok = good;
  return 1;
}

void EVP_tls_cbc_copy_mac(uint8_t *out, size_t md_size, const uint8_t *in,
                          size_t in_len, size_t orig_len) {
  uint8_t rotated_mac1[EVP_MAX_MD_SIZE], rotated_mac2[EVP_MAX_MD_SIZE];
  uint8_t *rotated_mac = rotated_mac1;
  uint8_t *rotated_mac_tmp = rotated_mac2;

  // mac_end is the index of |in| just after the end of the MAC.
  size_t mac_end = in_len;
  size_t mac_start = mac_end - md_size;

  declassify_assert(orig_len >= in_len);
  declassify_assert(in_len >= md_size);
  assert(md_size <= EVP_MAX_MD_SIZE);
  assert(md_size > 0);

  // scan_start contains the number of bytes that we can ignore because
  // the MAC's position can only vary by 255 bytes.
  size_t scan_start = 0;
  // This information is public so it's safe to branch based on it.
  if (orig_len > md_size + 255 + 1) {
    scan_start = orig_len - (md_size + 255 + 1);
  }

  size_t rotate_offset = 0;
  uint8_t mac_started = 0;
  OPENSSL_memset(rotated_mac, 0, md_size);
  for (size_t i = scan_start, j = 0; i < orig_len; i++, j++) {
    if (j >= md_size) {
      j -= md_size;
    }
    crypto_word_t is_mac_start = constant_time_eq_w(i, mac_start);
    mac_started |= is_mac_start;
    uint8_t mac_ended = constant_time_ge_8(i, mac_end);
    rotated_mac[j] |= in[i] & mac_started & ~mac_ended;
    // Save the offset that |mac_start| is mapped to.
    rotate_offset |= j & is_mac_start;
  }

  // Now rotate the MAC. We rotate in log(md_size) steps, one for each bit
  // position.
  for (size_t offset = 1; offset < md_size; offset <<= 1, rotate_offset >>= 1) {
    // Rotate by |offset| iff the corresponding bit is set in
    // |rotate_offset|, placing the result in |rotated_mac_tmp|.
    const uint8_t skip_rotate = (rotate_offset & 1) - 1;
    for (size_t i = 0, j = offset; i < md_size; i++, j++) {
      if (j >= md_size) {
        j -= md_size;
      }
      rotated_mac_tmp[i] =
          constant_time_select_8(skip_rotate, rotated_mac[i], rotated_mac[j]);
    }

    // Swap pointers so |rotated_mac| contains the (possibly) rotated value.
    // Note the number of iterations and thus the identity of these pointers is
    // public information.
    uint8_t *tmp = rotated_mac;
    rotated_mac = rotated_mac_tmp;
    rotated_mac_tmp = tmp;
  }

  OPENSSL_memcpy(out, rotated_mac, md_size);
}

int EVP_final_with_secret_suffix_sha1(SHA_CTX *ctx,
                                      uint8_t out[SHA_DIGEST_LENGTH],
                                      const uint8_t *in, size_t len,
                                      size_t max_len) {
  // Bound the input length so |total_bits| below fits in four bytes. This is
  // redundant with TLS record size limits. This also ensures |input_idx| below
  // does not overflow.
  size_t max_len_bits = max_len << 3;
  if (ctx->Nh != 0 ||
      (max_len_bits >> 3) != max_len ||  // Overflow
      ctx->Nl + max_len_bits < max_len_bits ||
      ctx->Nl + max_len_bits > UINT32_MAX) {
    return 0;
  }

  // We need to hash the following into |ctx|:
  //
  // - ctx->data[:ctx->num]
  // - in[:len]
  // - A 0x80 byte
  // - However many zero bytes are needed to pad up to a block.
  // - Eight bytes of length.
  size_t num_blocks = (ctx->num + len + 1 + 8 + SHA_CBLOCK - 1) >> 6;
  size_t last_block = num_blocks - 1;
  size_t max_blocks = (ctx->num + max_len + 1 + 8 + SHA_CBLOCK - 1) >> 6;

  // The bounds above imply |total_bits| fits in four bytes.
  size_t total_bits = ctx->Nl + (len << 3);
  uint8_t length_bytes[4];
  length_bytes[0] = (uint8_t)(total_bits >> 24);
  length_bytes[1] = (uint8_t)(total_bits >> 16);
  length_bytes[2] = (uint8_t)(total_bits >> 8);
  length_bytes[3] = (uint8_t)total_bits;

  // We now construct and process each expected block in constant-time.
  uint8_t block[SHA_CBLOCK] = {0};
  uint32_t result[5] = {0}; // The size of SHA1 state = 160 bits = 5*32 bits.
  // input_idx is the index into |in| corresponding to the current block.
  // However, we allow this index to overflow beyond |max_len|, to simplify the
  // 0x80 byte.
  size_t input_idx = 0;
  for (size_t i = 0; i < max_blocks; i++) {
    // Fill |block| with data from the partial block in |ctx| and |in|. We copy
    // as if we were hashing up to |max_len| and then zero the excess later.
    size_t block_start = 0;
    if (i == 0) {
      OPENSSL_memcpy(block, ctx->data, ctx->num);
      block_start = ctx->num;
    }
    if (input_idx < max_len) {
      size_t to_copy = SHA_CBLOCK - block_start;
      if (to_copy > max_len - input_idx) {
        to_copy = max_len - input_idx;
      }
      OPENSSL_memcpy(block + block_start, in + input_idx, to_copy);
    }

    // Zero any bytes beyond |len| and add the 0x80 byte.
    for (size_t j = block_start; j < SHA_CBLOCK; j++) {
      // input[idx] corresponds to block[j].
      size_t idx = input_idx + j - block_start;
      // The barriers on |len| are not strictly necessary. However, without
      // them, GCC compiles this code by incorporating |len| into the loop
      // counter and subtracting it out later. This is still constant-time, but
      // it frustrates attempts to validate this.
      uint8_t is_in_bounds = constant_time_lt_8(idx, value_barrier_w(len));
      uint8_t is_padding_byte = constant_time_eq_8(idx, value_barrier_w(len));
      block[j] &= is_in_bounds;
      block[j] |= 0x80 & is_padding_byte;
    }

    input_idx += SHA_CBLOCK - block_start;

    // Fill in the length if this is the last block.
    crypto_word_t is_last_block = constant_time_eq_w(i, last_block);
    for (size_t j = 0; j < 4; j++) {
      block[SHA_CBLOCK - 4 + j] |= is_last_block & length_bytes[j];
    }

    // Process the block and save the hash state if it is the final value.
    SHA1_Transform(ctx, block);
    for (size_t j = 0; j < 5; j++) {
      result[j] |= is_last_block & ctx->h[j];
    }
  }

  // Write the output.
  for (size_t i = 0; i < 5; i++) {
    CRYPTO_store_u32_be(out + 4 * i, result[i]);
  }
  return 1;
}

static int EVP_tls_cbc_digest_record_sha1(
    uint8_t *md_out, size_t *md_out_size,
    const uint8_t header[AEAD_TLS_AES_CBC_HMAC_AD_LENGTH], const uint8_t *data,
    size_t data_size, size_t data_plus_mac_plus_padding_size,
    const uint8_t *mac_secret, unsigned mac_secret_length) {
  if (mac_secret_length > SHA_CBLOCK) {
    // HMAC pads small keys with zeros and hashes large keys down. This function
    // should never reach the large key case.
    assert(0);
    return 0;
  }

  // Compute the initial HMAC block.
  uint8_t hmac_pad[SHA_CBLOCK];
  OPENSSL_memset(hmac_pad, 0, sizeof(hmac_pad));
  OPENSSL_memcpy(hmac_pad, mac_secret, mac_secret_length);
  for (size_t i = 0; i < SHA_CBLOCK; i++) {
    hmac_pad[i] ^= 0x36;
  }

  SHA_CTX ctx;
  SHA1_Init(&ctx);
  SHA1_Update(&ctx, hmac_pad, SHA_CBLOCK);
  SHA1_Update(&ctx, header, AEAD_TLS_AES_CBC_HMAC_AD_LENGTH);

  // There are at most 256 bytes of padding, so we can compute the public
  // minimum length for |data_size|.
  size_t min_data_size = 0;
  if (data_plus_mac_plus_padding_size > SHA_DIGEST_LENGTH + 256) {
    min_data_size = data_plus_mac_plus_padding_size - SHA_DIGEST_LENGTH - 256;
  }

  // Hash the public minimum length directly. This reduces the number of blocks
  // that must be computed in constant-time.
  SHA1_Update(&ctx, data, min_data_size);

  // Hash the remaining data without leaking |data_size|.
  uint8_t mac_out[SHA_DIGEST_LENGTH];
  if (!EVP_final_with_secret_suffix_sha1(
          &ctx, mac_out, data + min_data_size, data_size - min_data_size,
          data_plus_mac_plus_padding_size - min_data_size)) {
    return 0;
  }

  // Complete the HMAC in the standard manner.
  SHA1_Init(&ctx);
  for (size_t i = 0; i < SHA_CBLOCK; i++) {
    hmac_pad[i] ^= 0x6a;
  }

  SHA1_Update(&ctx, hmac_pad, SHA_CBLOCK);
  SHA1_Update(&ctx, mac_out, SHA_DIGEST_LENGTH);
  SHA1_Final(md_out, &ctx);
  *md_out_size = SHA_DIGEST_LENGTH;
  return 1;
}

int EVP_final_with_secret_suffix_sha256(SHA256_CTX *ctx,
                                        uint8_t out[SHA256_DIGEST_LENGTH],
                                        const uint8_t *in, size_t len,
                                        size_t max_len) {
  // Bound the input length so |total_bits| below fits in four bytes. This is
  // redundant with TLS record size limits. This also ensures |input_idx| below
  // does not overflow.
  size_t max_len_bits = max_len << 3;
  if (ctx->Nh != 0 ||
      (max_len_bits >> 3) != max_len ||  // Overflow
      ctx->Nl + max_len_bits < max_len_bits ||
      ctx->Nl + max_len_bits > UINT32_MAX) {
    return 0;
  }

  // We need to hash the following into |ctx|:
  //
  // - ctx->data[:ctx->num]
  // - in[:len]
  // - A 0x80 byte
  // - However many zero bytes are needed to pad up to a block.
  // - Eight bytes of length.
  size_t num_blocks = (ctx->num + len + 1 + 8 + SHA256_CBLOCK - 1) >> 6;
  size_t last_block = num_blocks - 1;
  size_t max_blocks = (ctx->num + max_len + 1 + 8 + SHA256_CBLOCK - 1) >> 6;

  // The bounds above imply |total_bits| fits in four bytes.
  size_t total_bits = ctx->Nl + (len << 3);
  uint8_t length_bytes[4];
  length_bytes[0] = (uint8_t)(total_bits >> 24);
  length_bytes[1] = (uint8_t)(total_bits >> 16);
  length_bytes[2] = (uint8_t)(total_bits >> 8);
  length_bytes[3] = (uint8_t)total_bits;

  // We now construct and process each expected block in constant-time.
  uint8_t block[SHA256_CBLOCK] = {0};
  uint32_t result[8] = {0}; // The size of SHA256 state = 256 bits = 8*32 bits.
  // input_idx is the index into |in| corresponding to the current block.
  // However, we allow this index to overflow beyond |max_len|, to simplify the
  // 0x80 byte.
  size_t input_idx = 0;
  for (size_t i = 0; i < max_blocks; i++) {
    // Fill |block| with data from the partial block in |ctx| and |in|. We copy
    // as if we were hashing up to |max_len| and then zero the excess later.
    size_t block_start = 0;
    if (i == 0) {
      OPENSSL_memcpy(block, ctx->data, ctx->num);
      block_start = ctx->num;
    }
    if (input_idx < max_len) {
      size_t to_copy = SHA256_CBLOCK - block_start;
      if (to_copy > max_len - input_idx) {
        to_copy = max_len - input_idx;
      }
      OPENSSL_memcpy(block + block_start, in + input_idx, to_copy);
    }

    // Zero any bytes beyond |len| and add the 0x80 byte.
    for (size_t j = block_start; j < SHA256_CBLOCK; j++) {
      // input[idx] corresponds to block[j].
      size_t idx = input_idx + j - block_start;
      // The barriers on |len| are not strictly necessary. However, without
      // them, GCC compiles this code by incorporating |len| into the loop
      // counter and subtracting it out later. This is still constant-time, but
      // it frustrates attempts to validate this.
      uint8_t is_in_bounds = constant_time_lt_8(idx, value_barrier_w(len));
      uint8_t is_padding_byte = constant_time_eq_8(idx, value_barrier_w(len));
      block[j] &= is_in_bounds;
      block[j] |= 0x80 & is_padding_byte;
    }

    input_idx += SHA256_CBLOCK - block_start;

    // Fill in the length if this is the last block.
    crypto_word_t is_last_block = constant_time_eq_w(i, last_block);
    for (size_t j = 0; j < 4; j++) {
      block[SHA256_CBLOCK - 4 + j] |= is_last_block & length_bytes[j];
    }

    // Process the block and save the hash state if it is the final value.
    SHA256_Transform(ctx, block);
    for (size_t j = 0; j < 8; j++) {
      result[j] |= is_last_block & ctx->h[j];
    }
  }

  // Write the output.
  for (size_t i = 0; i < 8; i++) {
    CRYPTO_store_u32_be(out + 4 * i, result[i]);
  }
  return 1;
}

static int EVP_tls_cbc_digest_record_sha256(
    uint8_t *md_out, size_t *md_out_size,
    const uint8_t header[AEAD_TLS_AES_CBC_HMAC_AD_LENGTH], const uint8_t *data,
    size_t data_size, size_t data_plus_mac_plus_padding_size,
    const uint8_t *mac_secret, unsigned mac_secret_length) {
  if (mac_secret_length > SHA256_CBLOCK) {
    // HMAC pads small keys with zeros and hashes large keys down. This function
    // should never reach the large key case.
    assert(0);
    return 0;
  }

  // Compute the initial HMAC block.
  uint8_t hmac_pad[SHA256_CBLOCK];
  OPENSSL_memset(hmac_pad, 0, sizeof(hmac_pad));
  OPENSSL_memcpy(hmac_pad, mac_secret, mac_secret_length);
  for (size_t i = 0; i < SHA256_CBLOCK; i++) {
    hmac_pad[i] ^= 0x36;
  }

  SHA256_CTX ctx;
  SHA256_Init(&ctx);
  SHA256_Update(&ctx, hmac_pad, SHA256_CBLOCK);
  SHA256_Update(&ctx, header, AEAD_TLS_AES_CBC_HMAC_AD_LENGTH);

  // There are at most 256 bytes of padding, so we can compute the public
  // minimum length for |data_size|.
  size_t min_data_size = 0;
  if (data_plus_mac_plus_padding_size > SHA256_DIGEST_LENGTH + 256) {
    min_data_size = data_plus_mac_plus_padding_size - SHA256_DIGEST_LENGTH - 256;
  }

  // Hash the public minimum length directly. This reduces the number of blocks
  // that must be computed in constant-time.
  SHA256_Update(&ctx, data, min_data_size);

  // Hash the remaining data without leaking |data_size|.
  uint8_t mac_out[SHA256_DIGEST_LENGTH];
  if (!EVP_final_with_secret_suffix_sha256(
          &ctx, mac_out, data + min_data_size, data_size - min_data_size,
          data_plus_mac_plus_padding_size - min_data_size)) {
    return 0;
  }

  // Complete the HMAC in the standard manner.
  SHA256_Init(&ctx);
  for (size_t i = 0; i < SHA256_CBLOCK; i++) {
    hmac_pad[i] ^= 0x6a;
  }

  SHA256_Update(&ctx, hmac_pad, SHA256_CBLOCK);
  SHA256_Update(&ctx, mac_out, SHA256_DIGEST_LENGTH);
  SHA256_Final(md_out, &ctx);
  *md_out_size = SHA256_DIGEST_LENGTH;
  return 1;
}

// The size of SHA384 working state = 512 bits = 8 64-bit words.
#define SHA384_WORKING_VARIABLES 8

int EVP_final_with_secret_suffix_sha384(SHA512_CTX *ctx,
                                        uint8_t out[SHA384_DIGEST_LENGTH],
                                        const uint8_t *in, size_t len,
                                        size_t max_len) {
  // Bound the input length so |total_bits| below fits in four bytes. This is
  // redundant with TLS record size limits. This also ensures |input_idx| below
  // does not overflow.
  size_t max_len_bits = max_len << 3;
  if (ctx->Nh != 0 ||
      (max_len_bits >> 3) != max_len ||  // Overflow
      ctx->Nl + max_len_bits < max_len_bits ||
      ctx->Nl + max_len_bits > UINT32_MAX) {
    return 0;
  }

  // See FIPS 180-4 section 5.1.2 for an explanation on SHA-384 message padding
  // and preprocessing. Here are some constants of interest:
  // * 16 == 128 bits for the message length
  // * 1 byte to cover padding bit.
  // * SHA384_CBLOCK == 1024 bits the padded message length that we should have
  //   a multiple of. The padding added will be less then this value.
  // * 7 is the how much we shift right (divide) by 128 bytes (1024 bits) to
  //   get the total number of blocks.

  // We need to hash the following into |ctx|:
  //
  // - ctx->data[:ctx->num]
  // - in[:len]
  // - A 0x80 byte
  // - However many zero bytes are needed to pad up to a block.
  // - 16 bytes of length.
  size_t num_blocks = (ctx->num + len + 1 + 16 + SHA384_CBLOCK - 1) >> 7;
  size_t last_block = num_blocks - 1;
  size_t max_blocks = (ctx->num + max_len + 1 + 16 + SHA384_CBLOCK - 1) >> 7;

  // The bounds above imply |total_bits| fits in four bytes.
  size_t total_bits = ctx->Nl + (len << 3);
  uint8_t length_bytes[4];
  length_bytes[0] = (uint8_t)(total_bits >> 24);
  length_bytes[1] = (uint8_t)(total_bits >> 16);
  length_bytes[2] = (uint8_t)(total_bits >> 8);
  length_bytes[3] = (uint8_t)total_bits;

  // We now construct and process each expected block in constant-time.
  uint8_t block[SHA384_CBLOCK] = {0};
  uint64_t result[SHA384_WORKING_VARIABLES] = {0};
  // input_idx is the index into |in| corresponding to the current block.
  // However, we allow this index to overflow beyond |max_len|, to simplify the
  // 0x80 byte.
  size_t input_idx = 0;
  for (size_t i = 0; i < max_blocks; i++) {
    // Fill |block| with data from the partial block in |ctx| and |in|. We copy
    // as if we were hashing up to |max_len| and then zero the excess later.
    size_t block_start = 0;
    if (i == 0) {
      OPENSSL_memcpy(block, ctx->p, ctx->num);
      block_start = ctx->num;
    }
    if (input_idx < max_len) {
      size_t to_copy = SHA384_CBLOCK- block_start;
      if (to_copy > max_len - input_idx) {
        to_copy = max_len - input_idx;
      }
      OPENSSL_memcpy(block + block_start, in + input_idx, to_copy);
    }

    // Zero any bytes beyond |len| and add the 0x80 byte.
    for (size_t j = block_start; j < SHA384_CBLOCK; j++) {
      // input[idx] corresponds to block[j].
      size_t idx = input_idx + j - block_start;
      // The barriers on |len| are not strictly necessary. However, without
      // them, GCC compiles this code by incorporating |len| into the loop
      // counter and subtracting it out later. This is still constant-time, but
      // it frustrates attempts to validate this.
      uint8_t is_in_bounds = constant_time_lt_8(idx, value_barrier_w(len));
      uint8_t is_padding_byte = constant_time_eq_8(idx, value_barrier_w(len));
      block[j] &= is_in_bounds;
      block[j] |= 0x80 & is_padding_byte;
    }

    input_idx += SHA384_CBLOCK - block_start;

    // Fill in the length if this is the last block.
    crypto_word_t is_last_block = constant_time_eq_w(i, last_block);
    for (size_t j = 0; j < 4; j++) {
      block[SHA384_CBLOCK - 4 + j] |= is_last_block & length_bytes[j];
    }

    // Process the block and save the hash state if it is the final value.
    assert(SHA384_CBLOCK == SHA512_CBLOCK);
    SHA512_Transform(ctx, block);

#if defined(OPENSSL_64_BIT)
    uint64_t mask = is_last_block;
#elif defined(OPENSSL_32_BIT)
    uint64_t mask =
        ((uint64_t)is_last_block) | (((uint64_t)is_last_block) << 32);
#else
#error "Must define either OPENSSL_32_BIT or OPENSSL_64_BIT"
#endif
    for (size_t j = 0; j < 8; j++) {
      result[j] |= mask & ctx->h[j];
    }
  }

  // Write the output. For SHA384 the resulting hash is truncated to the left-most
  // 384-bits (6 64-bit words).
  for (size_t i = 0; i < 6; i++) {
    CRYPTO_store_u64_be(out + 8 * i, result[i]);
  }
  return 1;
}

static int EVP_tls_cbc_digest_record_sha384(
    uint8_t *md_out, size_t *md_out_size,
    const uint8_t header[AEAD_TLS_AES_CBC_HMAC_AD_LENGTH], const uint8_t *data,
    size_t data_size, size_t data_plus_mac_plus_padding_size,
    const uint8_t *mac_secret, unsigned mac_secret_length) {
  if (mac_secret_length > SHA384_CBLOCK) {
    // HMAC pads small keys with zeros and hashes large keys down. This function
    // should never reach the large key case.
    assert(0);
    return 0;
  }

  // Compute the initial HMAC block.
  uint8_t hmac_pad[SHA384_CBLOCK];
  OPENSSL_memset(hmac_pad, 0, sizeof(hmac_pad));
  OPENSSL_memcpy(hmac_pad, mac_secret, mac_secret_length);
  for (size_t i = 0; i < SHA384_CBLOCK; i++) {
    hmac_pad[i] ^= 0x36;
  }

  SHA512_CTX ctx;
  SHA384_Init(&ctx);
  SHA384_Update(&ctx, hmac_pad, SHA384_CBLOCK);
  SHA384_Update(&ctx, header, AEAD_TLS_AES_CBC_HMAC_AD_LENGTH);

  // There are at most 256 bytes of padding, so we can compute the public
  // minimum length for |data_size|.
  size_t min_data_size = 0;
  if (data_plus_mac_plus_padding_size > SHA384_DIGEST_LENGTH + 256) {
    min_data_size =
        data_plus_mac_plus_padding_size - SHA384_DIGEST_LENGTH - 256;
  }

  // Hash the public minimum length directly. This reduces the number of blocks
  // that must be computed in constant-time.
  SHA384_Update(&ctx, data, min_data_size);

  // Hash the remaining data without leaking |data_size|.
  uint8_t mac_out[SHA384_DIGEST_LENGTH];
  if (!EVP_final_with_secret_suffix_sha384(
          &ctx, mac_out, data + min_data_size, data_size - min_data_size,
          data_plus_mac_plus_padding_size - min_data_size)) {
    return 0;
  }

  // Complete the HMAC in the standard manner.
  SHA384_Init(&ctx);
  for (size_t i = 0; i < SHA384_CBLOCK; i++) {
    hmac_pad[i] ^= 0x6a;
  }

  SHA384_Update(&ctx, hmac_pad, SHA384_CBLOCK);
  SHA384_Update(&ctx, mac_out, SHA384_DIGEST_LENGTH);
  SHA384_Final(md_out, &ctx);
  *md_out_size = SHA384_DIGEST_LENGTH;
  return 1;
}

int EVP_tls_cbc_record_digest_supported(const EVP_MD *md) {
  return (EVP_MD_type(md) == NID_sha1) || (EVP_MD_type(md) == NID_sha256) ||
         (EVP_MD_type(md) == NID_sha384);
}

int EVP_tls_cbc_digest_record(const EVP_MD *md, uint8_t *md_out,
                              size_t *md_out_size,
                              const uint8_t header[AEAD_TLS_AES_CBC_HMAC_AD_LENGTH],
                              const uint8_t *data, size_t data_size,
                              size_t data_plus_mac_plus_padding_size,
                              const uint8_t *mac_secret,
                              unsigned mac_secret_length) {

  // The specific hash algorithm is public knowledge.
  if (EVP_MD_type(md) == NID_sha1) {
    return EVP_tls_cbc_digest_record_sha1(
        md_out, md_out_size, header, data, data_size,
        data_plus_mac_plus_padding_size, mac_secret, mac_secret_length);
  } else if (EVP_MD_type(md) == NID_sha256) {
    return EVP_tls_cbc_digest_record_sha256(
        md_out, md_out_size, header, data, data_size,
        data_plus_mac_plus_padding_size, mac_secret, mac_secret_length);
  } else if (EVP_MD_type(md) == NID_sha384) {
    return EVP_tls_cbc_digest_record_sha384(
        md_out, md_out_size, header, data, data_size,
        data_plus_mac_plus_padding_size, mac_secret, mac_secret_length);
  }

  // EVP_tls_cbc_record_digest_supported should have been called first to
  // check that the hash function is supported.
  assert(0);
  *md_out_size = 0;

  return 0;
}
