// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/base.h>

#include <limits.h>

#include <openssl/err.h>
#include <openssl/rsa.h>
#include <openssl/bn.h>
#include <openssl/rand.h>
#include <openssl/mem.h>
#include <openssl/evp.h>

#include "../fipsmodule/bn/internal.h"
#include "../fipsmodule/rsa/internal.h"
#include "../internal.h"
#include "internal.h"


static void rand_nonzero(uint8_t *out, size_t len) {
  AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(out, len));

  for (size_t i = 0; i < len; i++) {
    // Zero values are replaced, and the distribution of zero and non-zero bytes
    // is public, so leaking this is safe.
    while (constant_time_declassify_int(out[i] == 0)) {
      AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(out + i, 1));
    }
  }
}

int RSA_padding_add_PKCS1_OAEP_mgf1(uint8_t *to, size_t to_len,
                                    const uint8_t *from, size_t from_len,
                                    const uint8_t *param, size_t param_len,
                                    const EVP_MD *md, const EVP_MD *mgf1md) {
  if (md == NULL) {
    md = EVP_sha1();
  }
  if (mgf1md == NULL) {
    mgf1md = md;
  }

  size_t mdlen = EVP_MD_size(md);

  if (to_len < 2 * mdlen + 2) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_KEY_SIZE_TOO_SMALL);
    return 0;
  }

  size_t emlen = to_len - 1;
  if (from_len > emlen - 2 * mdlen - 1) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_TOO_LARGE_FOR_KEY_SIZE);
    return 0;
  }

  if (emlen < 2 * mdlen + 1) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_KEY_SIZE_TOO_SMALL);
    return 0;
  }

  to[0] = 0;
  uint8_t *seed = to + 1;
  uint8_t *db = to + mdlen + 1;

  uint8_t *dbmask = NULL;
  int ret = 0;
  if (!EVP_Digest(param, param_len, db, NULL, md, NULL)) {
    goto out;
  }
  OPENSSL_memset(db + mdlen, 0, emlen - from_len - 2 * mdlen - 1);
  db[emlen - from_len - mdlen - 1] = 0x01;
  OPENSSL_memcpy(db + emlen - from_len - mdlen, from, from_len);
  AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(seed, mdlen));

  dbmask = OPENSSL_malloc(emlen - mdlen);
  if (dbmask == NULL) {
    goto out;
  }

  OPENSSL_BEGIN_ALLOW_DEPRECATED
  if (!PKCS1_MGF1(dbmask, emlen - mdlen, seed, mdlen, mgf1md)) {
    goto out;
  }
  for (size_t i = 0; i < emlen - mdlen; i++) {
    db[i] ^= dbmask[i];
  }

  uint8_t seedmask[EVP_MAX_MD_SIZE];
  if (!PKCS1_MGF1(seedmask, mdlen, db, emlen - mdlen, mgf1md)) {
    goto out;
  }
  OPENSSL_END_ALLOW_DEPRECATED

  for (size_t i = 0; i < mdlen; i++) {
    seed[i] ^= seedmask[i];
  }
  ret = 1;

out:
  OPENSSL_free(dbmask);
  return ret;
}

int RSA_padding_check_PKCS1_OAEP_mgf1(uint8_t *out, size_t *out_len,
                                      size_t max_out, const uint8_t *from,
                                      size_t from_len, const uint8_t *param,
                                      size_t param_len, const EVP_MD *md,
                                      const EVP_MD *mgf1md) {
  uint8_t *db = NULL;

  if (md == NULL) {
    md = EVP_sha1();
  }
  if (mgf1md == NULL) {
    mgf1md = md;
  }

  size_t mdlen = EVP_MD_size(md);

  // The encoded message is one byte smaller than the modulus to ensure that it
  // doesn't end up greater than the modulus. Thus there's an extra "+1" here
  // compared to https://tools.ietf.org/html/rfc2437#section-9.1.1.2.
  if (from_len < 1 + 2 * mdlen + 1) {
    // 'from_len' is the length of the modulus, i.e. does not depend on the
    // particular ciphertext.
    goto decoding_err;
  }

  size_t dblen = from_len - mdlen - 1;
  db = OPENSSL_malloc(dblen);
  if (db == NULL) {
    goto err;
  }

  const uint8_t *maskedseed = from + 1;
  const uint8_t *maskeddb = from + 1 + mdlen;

  uint8_t seed[EVP_MAX_MD_SIZE];

  OPENSSL_BEGIN_ALLOW_DEPRECATED
  if (!PKCS1_MGF1(seed, mdlen, maskeddb, dblen, mgf1md)) {
    goto err;
  }
  for (size_t i = 0; i < mdlen; i++) {
    seed[i] ^= maskedseed[i];
  }

  if (!PKCS1_MGF1(db, dblen, seed, mdlen, mgf1md)) {
    goto err;
  }
  OPENSSL_END_ALLOW_DEPRECATED

  for (size_t i = 0; i < dblen; i++) {
    db[i] ^= maskeddb[i];
  }

  uint8_t phash[EVP_MAX_MD_SIZE];
  if (!EVP_Digest(param, param_len, phash, NULL, md, NULL)) {
    goto err;
  }

  crypto_word_t bad = ~constant_time_is_zero_w(CRYPTO_memcmp(db, phash, mdlen));
  bad |= ~constant_time_is_zero_w(from[0]);

  crypto_word_t looking_for_one_byte = CONSTTIME_TRUE_W;
  size_t one_index = 0;
  for (size_t i = mdlen; i < dblen; i++) {
    crypto_word_t equals1 = constant_time_eq_w(db[i], 1);
    crypto_word_t equals0 = constant_time_eq_w(db[i], 0);
    one_index =
        constant_time_select_w(looking_for_one_byte & equals1, i, one_index);
    looking_for_one_byte =
        constant_time_select_w(equals1, 0, looking_for_one_byte);
    bad |= looking_for_one_byte & ~equals0;
  }

  bad |= looking_for_one_byte;

  // Whether the overall padding was valid or not in OAEP is public.
  if (constant_time_declassify_w(bad)) {
    goto decoding_err;
  }

  // Once the padding is known to be valid, the output length is also public.
  OPENSSL_STATIC_ASSERT(sizeof(size_t) <= sizeof(crypto_word_t),
                        size_t_does_not_fit_in_crypto_word_t);
  one_index = constant_time_declassify_w(one_index);

  one_index++;
  size_t mlen = dblen - one_index;
  if (max_out < mlen) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_TOO_LARGE);
    goto err;
  }

  OPENSSL_memcpy(out, db + one_index, mlen);
  *out_len = mlen;
  OPENSSL_free(db);
  return 1;

decoding_err:
  // To avoid chosen ciphertext attacks, the error message should not reveal
  // which kind of decoding error happened.
  OPENSSL_PUT_ERROR(RSA, RSA_R_OAEP_DECODING_ERROR);
err:
  OPENSSL_free(db);
  return 0;
}

static int rsa_padding_add_PKCS1_type_2(uint8_t *to, size_t to_len,
                                        const uint8_t *from, size_t from_len) {
  // See RFC 8017, section 7.2.1.
  if (to_len < RSA_PKCS1_PADDING_SIZE) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_KEY_SIZE_TOO_SMALL);
    return 0;
  }

  if (from_len > to_len - RSA_PKCS1_PADDING_SIZE) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_TOO_LARGE_FOR_KEY_SIZE);
    return 0;
  }

  to[0] = 0;
  to[1] = 2;

  size_t padding_len = to_len - 3 - from_len;
  rand_nonzero(to + 2, padding_len);
  to[2 + padding_len] = 0;
  OPENSSL_memcpy(to + to_len - from_len, from, from_len);
  return 1;
}

static int rsa_padding_check_PKCS1_type_2(uint8_t *out, size_t *out_len,
                                          size_t max_out, const uint8_t *from,
                                          size_t from_len) {
  if (from_len == 0) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_EMPTY_PUBLIC_KEY);
    return 0;
  }

  // PKCS#1 v1.5 decryption. See "PKCS #1 v2.2: RSA Cryptography
  // Standard", section 7.2.2.
  if (from_len < RSA_PKCS1_PADDING_SIZE) {
    // |from| is zero-padded to the size of the RSA modulus, a public value, so
    // this can be rejected in non-constant time.
    OPENSSL_PUT_ERROR(RSA, RSA_R_KEY_SIZE_TOO_SMALL);
    return 0;
  }

  crypto_word_t first_byte_is_zero = constant_time_eq_w(from[0], 0);
  crypto_word_t second_byte_is_two = constant_time_eq_w(from[1], 2);

  crypto_word_t zero_index = 0, looking_for_index = CONSTTIME_TRUE_W;
  for (size_t i = 2; i < from_len; i++) {
    crypto_word_t equals0 = constant_time_is_zero_w(from[i]);
    zero_index =
        constant_time_select_w(looking_for_index & equals0, i, zero_index);
    looking_for_index = constant_time_select_w(equals0, 0, looking_for_index);
  }

  // The input must begin with 00 02.
  crypto_word_t valid_index = first_byte_is_zero;
  valid_index &= second_byte_is_two;

  // We must have found the end of PS.
  valid_index &= ~looking_for_index;

  // PS must be at least 8 bytes long, and it starts two bytes into |from|.
  valid_index &= constant_time_ge_w(zero_index, 2 + 8);

  // Skip the zero byte.
  zero_index++;

  // NOTE: Although this logic attempts to be constant time, the API contracts
  // of this function and |RSA_decrypt| with |RSA_PKCS1_PADDING| make it
  // impossible to completely avoid Bleichenbacher's attack. Consumers should
  // use |RSA_PADDING_NONE| and perform the padding check in constant-time
  // combined with a swap to a random session key or other mitigation.
  CONSTTIME_DECLASSIFY(&valid_index, sizeof(valid_index));
  CONSTTIME_DECLASSIFY(&zero_index, sizeof(zero_index));

  if (!valid_index) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_PKCS_DECODING_ERROR);
    return 0;
  }

  const size_t msg_len = from_len - zero_index;
  if (msg_len > max_out) {
    // This shouldn't happen because this function is always called with
    // |max_out| as the key size and |from_len| is bounded by the key size.
    OPENSSL_PUT_ERROR(RSA, RSA_R_PKCS_DECODING_ERROR);
    return 0;
  }

  OPENSSL_memcpy(out, &from[zero_index], msg_len);
  *out_len = msg_len;
  return 1;
}

int RSA_public_encrypt(size_t flen, const uint8_t *from, uint8_t *to, RSA *rsa,
                       int padding) {
  size_t out_len;

  if (!RSA_encrypt(rsa, &out_len, to, RSA_size(rsa), from, flen, padding)) {
    return -1;
  }

  if (out_len > INT_MAX) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_OVERFLOW);
    return -1;
  }
  return (int)out_len;
}

int RSA_private_encrypt(size_t flen, const uint8_t *from, uint8_t *to, RSA *rsa,
                        int padding) {
  size_t out_len;

  if (!RSA_sign_raw(rsa, &out_len, to, RSA_size(rsa), from, flen, padding)) {
    return -1;
  }

  if (out_len > INT_MAX) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_OVERFLOW);
    return -1;
  }
  return (int)out_len;
}

int RSA_encrypt(RSA *rsa, size_t *out_len, uint8_t *out, size_t max_out,
                const uint8_t *in, size_t in_len, int padding) {
  if(rsa->meth && rsa->meth->encrypt) {
    // In OpenSSL, the RSA_METHOD |encrypt| or |pub_enc| operation does not
    // directly take and initialize an |out_len| parameter. Instead, it returns
    // the number of bytes written to |out| or a negative number for error.
    // Our wrapping functions like |RSA_encrypt| diverge from this paradigm and
    // expect an |out_len| parameter. To remain compatible with this new
    // paradigm and OpenSSL, we initialize |out_len| based on the return value
    // here.
    if (in_len > INT_MAX) {
      OPENSSL_PUT_ERROR(RSA, ERR_R_OVERFLOW);
      *out_len = 0;
      return 0;
    }
    int ret = rsa->meth->encrypt((int)in_len, in, out, rsa, padding);
    if(ret < 0) {
      *out_len = 0;
      return 0;
    }
    *out_len = ret;
    return 1;
  }

  if (rsa->n == NULL || rsa->e == NULL) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_VALUE_MISSING);
    return 0;
  }

  if (!is_public_component_of_rsa_key_good(rsa)) {
    return 0;
  }

  const unsigned rsa_size = RSA_size(rsa);
  BIGNUM *f, *result;
  uint8_t *buf = NULL;
  BN_CTX *ctx = NULL;
  int i, ret = 0;

  if (max_out < rsa_size) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_OUTPUT_BUFFER_TOO_SMALL);
    return 0;
  }

  ctx = BN_CTX_new();
  if (ctx == NULL) {
    goto err;
  }

  BN_CTX_start(ctx);
  f = BN_CTX_get(ctx);
  result = BN_CTX_get(ctx);
  buf = OPENSSL_malloc(rsa_size);
  if (!f || !result || !buf) {
    goto err;
  }

  switch (padding) {
    case RSA_PKCS1_PADDING:
      i = rsa_padding_add_PKCS1_type_2(buf, rsa_size, in, in_len);
      break;
    case RSA_PKCS1_OAEP_PADDING:
      // Use the default parameters: SHA-1 for both hashes and no label.
      i = RSA_padding_add_PKCS1_OAEP_mgf1(buf, rsa_size, in, in_len, NULL, 0,
                                          NULL, NULL);
      break;
    case RSA_NO_PADDING:
      i = RSA_padding_add_none(buf, rsa_size, in, in_len);
      break;
    default:
      OPENSSL_PUT_ERROR(RSA, RSA_R_UNKNOWN_PADDING_TYPE);
      goto err;
  }

  if (i <= 0) {
    goto err;
  }

  if (BN_bin2bn(buf, rsa_size, f) == NULL) {
    goto err;
  }

  if (BN_ucmp(f, rsa->n) >= 0) {
    // usually the padding functions would catch this
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_TOO_LARGE_FOR_MODULUS);
    goto err;
  }

  if (!BN_MONT_CTX_set_locked(&rsa->mont_n, &rsa->lock, rsa->n, ctx) ||
      !BN_mod_exp_mont(result, f, rsa->e, &rsa->mont_n->N, ctx, rsa->mont_n)) {
    goto err;
  }

  // put in leading 0 bytes if the number is less than the length of the
  // modulus
  if (!BN_bn2bin_padded(out, rsa_size, result)) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
    goto err;
  }

  *out_len = rsa_size;
  ret = 1;

err:
  if (ctx != NULL) {
    BN_CTX_end(ctx);
    BN_CTX_free(ctx);
  }
  OPENSSL_free(buf);

  return ret;
}

static int rsa_default_decrypt(RSA *rsa, size_t *out_len, uint8_t *out,
                               size_t max_out, const uint8_t *in, size_t in_len,
                               int padding) {
  const unsigned rsa_size = RSA_size(rsa);
  uint8_t *buf = NULL;
  int ret = 0;

  if (max_out < rsa_size) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_OUTPUT_BUFFER_TOO_SMALL);
    return 0;
  }

  if (padding == RSA_NO_PADDING) {
    buf = out;
  } else {
    // Allocate a temporary buffer to hold the padded plaintext.
    buf = OPENSSL_malloc(rsa_size);
    if (buf == NULL) {
      goto err;
    }
  }

  if (in_len != rsa_size) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_LEN_NOT_EQUAL_TO_MOD_LEN);
    goto err;
  }

  if (!rsa_private_transform(rsa, buf, in, rsa_size)) {
    goto err;
  }

  switch (padding) {
    case RSA_PKCS1_PADDING:
      ret =
          rsa_padding_check_PKCS1_type_2(out, out_len, rsa_size, buf, rsa_size);
      break;
    case RSA_PKCS1_OAEP_PADDING:
      // Use the default parameters: SHA-1 for both hashes and no label.
      ret = RSA_padding_check_PKCS1_OAEP_mgf1(out, out_len, rsa_size, buf,
                                              rsa_size, NULL, 0, NULL, NULL);
      break;
    case RSA_NO_PADDING:
      *out_len = rsa_size;
      ret = 1;
      break;
    default:
      OPENSSL_PUT_ERROR(RSA, RSA_R_UNKNOWN_PADDING_TYPE);
      goto err;
  }

  CONSTTIME_DECLASSIFY(&ret, sizeof(ret));
  if (!ret) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_PADDING_CHECK_FAILED);
  } else {
    CONSTTIME_DECLASSIFY(out, *out_len);
  }

err:
  if (padding != RSA_NO_PADDING) {
    OPENSSL_free(buf);
  }

  return ret;
}

int RSA_decrypt(RSA *rsa, size_t *out_len, uint8_t *out, size_t max_out,
                const uint8_t *in, size_t in_len, int padding) {
  if (rsa->meth && rsa->meth->decrypt) {
    // In OpenSSL, the RSA_METHOD |decrypt| operation does not directly take
    // and initialize an |out_len| parameter. Instead, it returns the number
    // of bytes written to |out| or a negative number for error. Our wrapping
    // functions like |RSA_decrypt| diverge from this paradigm and expect
    // an |out_len| parameter. To remain compatible with this new paradigm and
    // OpenSSL, we initialize |out_len| based on the return value here.
    if (in_len > INT_MAX) {
      OPENSSL_PUT_ERROR(RSA, ERR_R_OVERFLOW);
      *out_len = 0;
      return 0;
    }
    int ret = rsa->meth->decrypt((int)in_len, in, out, rsa, padding);
    if(ret < 0) {
      *out_len = 0;
      return 0;
    }
    *out_len = ret;
    return 1;
  }

  return rsa_default_decrypt(rsa, out_len, out, max_out, in, in_len, padding);
}

int RSA_private_decrypt(size_t flen, const uint8_t *from, uint8_t *to, RSA *rsa,
                        int padding) {
  size_t out_len;
  if (!RSA_decrypt(rsa, &out_len, to, RSA_size(rsa), from, flen, padding)) {
    return -1;
  }

  if (out_len > INT_MAX) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_OVERFLOW);
    return -1;
  }
  return (int)out_len;
}

int RSA_public_decrypt(size_t flen, const uint8_t *from, uint8_t *to, RSA *rsa,
                       int padding) {
  size_t out_len;
  if (!RSA_verify_raw(rsa, &out_len, to, RSA_size(rsa), from, flen, padding)) {
    return -1;
  }

  if (out_len > INT_MAX) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_OVERFLOW);
    return -1;
  }
  return (int)out_len;
}
