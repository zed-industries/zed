// Copyright 2011-2016 The OpenSSL Project Authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/opensslconf.h>

#include <stdio.h>
#include <string.h>


#include <openssl/aes.h>
#include <openssl/cipher.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/objects.h>
#include <openssl/rand.h>
#include <openssl/sha.h>
#include "../fipsmodule/aes/internal.h"
#include "../fipsmodule/cipher/internal.h"
#include "internal.h"

#if defined(AES_CBC_HMAC_SHA_STITCH)

typedef struct {
  AES_KEY ks;
  // Used to compute(init, update and final) HMAC-SHA1.
  // head stores the initialised inner hash state.
  // tail stores the outer hash state.
  // These storage are for using in subsequent invocations with the same MAC key.
  SHA_CTX head, tail, md;
  // In encrypt case, it's eiv_len + plaintext_len. eiv is explicit iv(required
  // TLS 1.1+). In decrypt case, it's |EVP_AEAD_TLS1_AAD_LEN(13)|.
  size_t payload_length;
  union {
    uint16_t tls_ver;
    // In encrypt case, it's not set.
    // In decrypt case, it stores |additional_data|.
    // additional_data = seq_num + content_type + protocol_version +
    // payload_eiv_len seq_num: 8 octets long. content_type: 1 octets long.
    // protocol_version: 2 octets long.
    // payload_eiv_len: 2 octets long. eiv is explicit iv required by TLS 1.1+.
    //
    // TLS 1.0: https://www.rfc-editor.org/rfc/rfc2246.html#section-6.2.3.2
    // TLS 1.1: https://www.ietf.org/rfc/rfc5246.html#section-6.2.3.2
    // TLS 1.2: https://www.ietf.org/rfc/rfc5246.html#section-6.2.3.2
    uint8_t tls_aad[EVP_AEAD_TLS1_AAD_LEN];
  } aux;
  // Used to store the key computed in EVP_CTRL_AEAD_SET_MAC_KEY operation.
  uint8_t hmac_key[HMAC_KEY_SIZE];
} EVP_AES_HMAC_SHA1;

void aesni_cbc_sha1_enc(const void *inp, void *out, size_t blocks,
                        const AES_KEY *key, uint8_t iv[AES_BLOCK_SIZE], SHA_CTX *ctx,
                        const void *in0);

static int aesni_cbc_hmac_sha1_init_key(EVP_CIPHER_CTX *ctx,
                                        const uint8_t *inkey,
                                        const uint8_t *iv, int enc) {
  EVP_AES_HMAC_SHA1 *key = (EVP_AES_HMAC_SHA1 *)(ctx->cipher_data);
  int ret;

  int key_bits = EVP_CIPHER_CTX_key_length(ctx) * 8;
  if (enc) {
    ret = aes_hw_set_encrypt_key(inkey, key_bits, &key->ks);
  } else {
    ret = aes_hw_set_decrypt_key(inkey, key_bits, &key->ks);
  }
  if (ret < 0) {
    return 0;
  }

  SHA1_Init(&key->head);
  key->tail = key->head;
  key->md = key->head;

  key->payload_length = NO_PAYLOAD_LENGTH;

  return 1;
}

// aesni_cbc_hmac_sha1_cipher implements TLS-specific CBC-mode+HMAC-SHA1 cipher suite based encryption and decryption.
//
// For encryption in TLS version 1.0
// |in|: payload/fragment
// |len|: (|payload| + SHA_DIGEST_LENGTH + AES_BLOCK_SIZE) & -AES_BLOCK_SIZE
// |out|: Must point to allocated memory of at least (|payload| + SHA_DIGEST_LENGTH + AES_BLOCK_SIZE) & -AES_BLOCK_SIZE bytes
// If the function returns successfully |out| will contain AES-CBC(aes_key, IV, payload || hmac-sha1(mac_key, aad || payload) || padding || padding_length)

// For encryption in TLS version 1.1 and 1.2
// |in|: payload/fragment
// |len|: (|IV| + |payload| + SHA_DIGEST_LENGTH + AES_BLOCK_SIZE) & -AES_BLOCK_SIZE
// |out|: Must point to allocated memory of at least (|IV| + |payload| + SHA_DIGEST_LENGTH + AES_BLOCK_SIZE) & -AES_BLOCK_SIZE bytes
// If the function returns successfully |out| will contain AES-CBC(aes_key, mask, IV || payload || hmac-sha1(mac_key, aad || payload) || padding || padding_length)
// |len|: should be (eiv_len + plaintext_len + SHA_DIGEST_LENGTH + AES_BLOCK_SIZE) & -AES_BLOCK_SIZE).
// The mask and IV are according to method 2.b from https://datatracker.ietf.org/doc/html/rfc2246#section-6.2.3.2
//
// WARNING: Do not set explicit |IV| = |mask|. It will result in aes(aes_key, 0) being used at the effective IV for all records.
//
// In decryption, this function performs decrytion, removing padding, and verifying mac value.
static int aesni_cbc_hmac_sha1_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out,
                                      const uint8_t *in, size_t len) {
  EVP_AES_HMAC_SHA1 *key = (EVP_AES_HMAC_SHA1 *)(ctx->cipher_data);
  size_t plen = key->payload_length, iv_len = 0;

  key->payload_length = NO_PAYLOAD_LENGTH;

  if (len % AES_BLOCK_SIZE) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_DATA_NOT_MULTIPLE_OF_BLOCK_LENGTH);
    return 0;
  }

  if (EVP_CIPHER_CTX_encrypting(ctx)) {
    // NOTE: Difference between openssl and aws-lc:
    // In encrypt case, |plen| is set in the call |EVP_CIPHER_CTX_ctrl| with
    // |EVP_CTRL_AEAD_TLS1_AAD| operation.
    // When |plen == NO_PAYLOAD_LENGTH|, it means the call did not happen.
    // In this case, aws-lc returns error(0) but openssl supports that with
    // below explanation.
    // https://mta.openssl.org/pipermail/openssl-users/2019-November/011458.html
    // -- These stitched ciphers are specifically targeted at use by libssl
    //    and are designed for use in SSL/TLS only.
    if (plen == NO_PAYLOAD_LENGTH) {
      // |EVP_CIPHER_CTX_ctrl| with |EVP_CTRL_AEAD_TLS1_AAD| operation is not
      // performed.
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_CTRL_OPERATION_NOT_PERFORMED);
      return 0;
    }
    if (len !=
        ((plen + SHA_DIGEST_LENGTH + AES_BLOCK_SIZE) & -AES_BLOCK_SIZE)) {
      // The input should have space for plen(eiv + plaintext) + SHA_DIGEST_LENGTH + padding.
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_UNSUPPORTED_INPUT_SIZE);
      return 0;
    } else if (key->aux.tls_ver >= TLS1_1_VERSION) {
      iv_len = AES_BLOCK_SIZE;
    }

    size_t aes_off = 0;
    size_t sha_off = SHA_CBLOCK - key->md.num;
    size_t blocks;

    // Use stitch code |aesni_cbc_sha1_enc| when there are multiple of SHA_CBLOCK
    // so |aesni_cbc_sha1_enc| can use AES and SHA on the same data block.
    //
    // Assembly stitch handles AVX-capable processors, but its
    // performance is not optimal on AMD Jaguar, ~40% worse, for
    // unknown reasons. Incidentally processor in question supports
    // AVX, but not AMD-specific XOP extension, which can be used
    // to identify it and avoid stitch invocation. So that after we
    // establish that current CPU supports AVX, we even see if it's
    // either even XOP-capable Bulldozer-based or GenuineIntel one.
    // But SHAEXT-capable go ahead...
    if ((CRYPTO_is_SHAEXT_capable() ||
        (CRYPTO_is_AVX_capable() &&
        (CRYPTO_is_AMD_XOP_support() | CRYPTO_is_intel_cpu()))) &&
        plen > (sha_off + iv_len) &&
        (blocks = (plen - (sha_off + iv_len)) / SHA_CBLOCK)) {
      // Before calling |aesni_cbc_sha1_enc|, |key->md| should not
      // include not hashed data(partial data).
      SHA1_Update(&key->md, in + iv_len, sha_off);

      aesni_cbc_sha1_enc(in, out, blocks, &key->ks,
                         ctx->iv, &key->md,
                         in + iv_len + sha_off);
      // Update the offset to record and skip the part processed
      // (encrypted and hashed) by |aesni_cbc_sha1_enc|.
      blocks *= SHA_CBLOCK;
      aes_off += blocks;
      sha_off += blocks;
      key->md.Nh += blocks >> 29;
      key->md.Nl += blocks <<= 3;
      if (key->md.Nl < (unsigned int)blocks) {
        key->md.Nh++;
      }
    } else {
      sha_off = 0;
    }
    sha_off += iv_len;
    SHA1_Update(&key->md, in + sha_off, plen - sha_off);

    if (in != out) {
      OPENSSL_memcpy(out + aes_off, in + aes_off, plen - aes_off);
    }

    // calculate HMAC and append it to payload.
    SHA1_Final(out + plen, &key->md);
    key->md = key->tail;
    SHA1_Update(&key->md, out + plen, SHA_DIGEST_LENGTH);
    SHA1_Final(out + plen, &key->md);

    // pad the payload|hmac.
    plen += SHA_DIGEST_LENGTH;
    for (unsigned int l = len - plen - 1; plen < len; plen++) {
      out[plen] = l;
    }
    // encrypt HMAC|padding at once.
    aes_hw_cbc_encrypt(out + aes_off, out + aes_off, len - aes_off, &key->ks,
                       ctx->iv, 1);
    return 1;
  } else {
    if (plen != EVP_AEAD_TLS1_AAD_LEN) {
      // |EVP_CIPHER_CTX_ctrl| with |EVP_CTRL_AEAD_TLS1_AAD| operation is not
      // performed.
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_CTRL_OPERATION_NOT_PERFORMED);
      return 0;
    }

    if ((key->aux.tls_aad[plen - 4] << 8 | key->aux.tls_aad[plen - 3]) >=
        TLS1_1_VERSION) {
      if (len < (AES_BLOCK_SIZE + SHA_DIGEST_LENGTH + 1)) {
        OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_UNSUPPORTED_INPUT_SIZE);
        return 0;
      }

      // omit explicit iv.
      OPENSSL_memcpy(ctx->iv, in, AES_BLOCK_SIZE);
      in += AES_BLOCK_SIZE;
      out += AES_BLOCK_SIZE;
      len -= AES_BLOCK_SIZE;
    } else if (len < (SHA_DIGEST_LENGTH + 1)) {
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_UNSUPPORTED_INPUT_SIZE);
      return 0;
    }
    // decrypt HMAC|padding at once.
    aes_hw_cbc_encrypt(in, out, len, &key->ks, ctx->iv, 0);

    CONSTTIME_SECRET(out, len);

    // Remove CBC padding. Code from here on is timing-sensitive with respect to
    // |padding_ok| and |data_plus_mac_len| for CBC ciphers.
    size_t data_plus_mac_len;
    crypto_word_t padding_ok;
    if (!EVP_tls_cbc_remove_padding(&padding_ok, &data_plus_mac_len, out, len,
                                    AES_BLOCK_SIZE, SHA_DIGEST_LENGTH)) {
      // Publicly invalid. This can be rejected in non-constant time.
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
      return 0;
    }

    size_t data_len = data_plus_mac_len - SHA_DIGEST_LENGTH;

    key->aux.tls_aad[11] = (uint8_t)(data_len >> 8);
    key->aux.tls_aad[12] = (uint8_t)(data_len);

    // Compute the MAC and extract the one in the record.
    uint8_t mac[EVP_MAX_MD_SIZE];
    size_t mac_len;
    uint8_t record_mac_tmp[EVP_MAX_MD_SIZE];
    uint8_t *record_mac;
    if (!EVP_tls_cbc_digest_record(EVP_sha1(), mac, &mac_len, key->aux.tls_aad,
                                   out, data_len, len, key->hmac_key, 64)) {
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
      return 0;
    }
    assert(mac_len == SHA_DIGEST_LENGTH);

    record_mac = record_mac_tmp;
    EVP_tls_cbc_copy_mac(record_mac, mac_len, out, data_plus_mac_len, len);

    // Perform the MAC check and the padding check in constant-time. It should
    // be safe to simply perform the padding check first, but it would not be
    // under a different choice of MAC location on padding failure. See
    // |EVP_tls_cbc_remove_padding|.
    crypto_word_t good =
        constant_time_eq_int(CRYPTO_memcmp(record_mac, mac, mac_len), 0);
    good &= padding_ok;
    CONSTTIME_DECLASSIFY(&good, sizeof(good));
    if (!good) {
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
      return 0;
    }

    CONSTTIME_DECLASSIFY(&data_len, sizeof(data_len));
    CONSTTIME_DECLASSIFY(out, data_len);

    // End of timing-sensitive code.
    return 1;
  }
}

static int aesni_cbc_hmac_sha1_ctrl(EVP_CIPHER_CTX *ctx, int type, int arg,
                                    void *ptr) {
  EVP_AES_HMAC_SHA1 *key = (EVP_AES_HMAC_SHA1 *)(ctx->cipher_data);

  switch (type) {
    case EVP_CTRL_AEAD_SET_MAC_KEY: {
      if (arg < 0) {
        return 0;
      }
      // This CTRL operation is to perform |HMAC_Init_ex| with SHA1 on |ptr|.
      uint8_t hmac_key[HMAC_KEY_SIZE];
      OPENSSL_memset(hmac_key, 0, sizeof(hmac_key));
      size_t u_arg = (size_t)arg;
      if (u_arg > sizeof(hmac_key)) {
        SHA1_Init(&key->head);
        SHA1_Update(&key->head, ptr, arg);
        SHA1_Final(hmac_key, &key->head);
      } else {
        OPENSSL_memcpy(hmac_key, ptr, arg);
      }
      OPENSSL_memcpy(&key->hmac_key, hmac_key, 64);

      for (size_t i = 0; i < sizeof(hmac_key); i++) {
        hmac_key[i] ^= 0x36; /* ipad */
      }
      SHA1_Init(&key->head);
      SHA1_Update(&key->head, hmac_key, sizeof(hmac_key));

      for (size_t i = 0; i < sizeof(hmac_key); i++) {
        hmac_key[i] ^= 0x36 ^ 0x5c; /* opad */
      }
      SHA1_Init(&key->tail);
      SHA1_Update(&key->tail, hmac_key, sizeof(hmac_key));

      OPENSSL_cleanse(hmac_key, sizeof(hmac_key));

      return 1;
    }
    case EVP_CTRL_AEAD_TLS1_AAD: {
      // p is
      // additional_data = |seq_num + content_type + protocol_version + payload_eiv_len|.
      // seq_num: 8 octets long.
      // content_type: 1 octets long.
      // protocol_version: 2 octets long.
      // payload_eiv_len: 2 octets long. eiv is explicit iv required by TLS 1.1+.
      uint8_t *p = ptr;
      if (arg != EVP_AEAD_TLS1_AAD_LEN) {
        OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_AD_SIZE);
        return 0;
      }

      if (EVP_CIPHER_CTX_encrypting(ctx)) {
        uint16_t len = p[arg - 2] << 8 | p[arg - 1];
        key->payload_length = len;
        if ((key->aux.tls_ver = p[arg - 4] << 8 | p[arg - 3]) >=
            TLS1_1_VERSION) {
          if (len < AES_BLOCK_SIZE) {
            OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_AD_SIZE);
            return 0;
          }
          len -= AES_BLOCK_SIZE;
          p[arg - 2] = len >> 8;
          p[arg - 1] = len;
        }
        key->md = key->head;
        SHA1_Update(&key->md, p, arg);

        return (int)(((len + SHA_DIGEST_LENGTH + AES_BLOCK_SIZE) &
                      -AES_BLOCK_SIZE) -
                     len);
      } else {
        OPENSSL_memcpy(key->aux.tls_aad, ptr, arg);
        key->payload_length = arg;

        return SHA_DIGEST_LENGTH;
      }
    }
    default:
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_CTRL_NOT_IMPLEMENTED);
      return 0;
  }
}

static const EVP_CIPHER aesni_128_cbc_hmac_sha1_cipher = {
    NID_aes_128_cbc_hmac_sha1 /* nid */,
    AES_BLOCK_SIZE /* block size */,
    16 /* key len */,
    AES_BLOCK_SIZE /* iv len */,
    sizeof(EVP_AES_HMAC_SHA1) /* ctx_size */,
    EVP_CIPH_CBC_MODE | EVP_CIPH_FLAG_AEAD_CIPHER /* flags */,
    aesni_cbc_hmac_sha1_init_key,
    aesni_cbc_hmac_sha1_cipher,
    NULL /* cleanup */,
    aesni_cbc_hmac_sha1_ctrl};

static const EVP_CIPHER aesni_256_cbc_hmac_sha1_cipher = {
    NID_aes_256_cbc_hmac_sha1 /* nid */,
    AES_BLOCK_SIZE /* block size */,
    32 /* key len */,
    AES_BLOCK_SIZE /* iv len */,
    sizeof(EVP_AES_HMAC_SHA1) /* ctx_size */,
    EVP_CIPH_CBC_MODE | EVP_CIPH_FLAG_AEAD_CIPHER /* flags */,
    aesni_cbc_hmac_sha1_init_key,
    aesni_cbc_hmac_sha1_cipher,
    NULL,
    aesni_cbc_hmac_sha1_ctrl};

const EVP_CIPHER *EVP_aes_128_cbc_hmac_sha1(void) {
  return (hwaes_capable() ? &aesni_128_cbc_hmac_sha1_cipher : NULL);
}

const EVP_CIPHER *EVP_aes_256_cbc_hmac_sha1(void) {
  return (hwaes_capable() ? &aesni_256_cbc_hmac_sha1_cipher : NULL);
}
#else
const EVP_CIPHER *EVP_aes_128_cbc_hmac_sha1(void) { return NULL; }

const EVP_CIPHER *EVP_aes_256_cbc_hmac_sha1(void) { return NULL; }
#endif
