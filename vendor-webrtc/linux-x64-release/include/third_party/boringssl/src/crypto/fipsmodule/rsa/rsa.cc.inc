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

#include <openssl/rsa.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/bn.h>
#include <openssl/digest.h>
#include <openssl/engine.h>
#include <openssl/err.h>
#include <openssl/ex_data.h>
#include <openssl/md5.h>
#include <openssl/mem.h>
#include <openssl/nid.h>
#include <openssl/thread.h>

#include "../../internal.h"
#include "../bcm_interface.h"
#include "../bn/internal.h"
#include "../delocate.h"
#include "internal.h"


// RSA_R_BLOCK_TYPE_IS_NOT_02 is part of the legacy SSLv23 padding scheme.
// Cryptography.io depends on this error code.
OPENSSL_DECLARE_ERROR_REASON(RSA, BLOCK_TYPE_IS_NOT_02)

DEFINE_STATIC_EX_DATA_CLASS(g_rsa_ex_data_class)

static int bn_dup_into(BIGNUM **dst, const BIGNUM *src) {
  if (src == NULL) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  BN_free(*dst);
  *dst = BN_dup(src);
  return *dst != NULL;
}

RSA *RSA_new_public_key(const BIGNUM *n, const BIGNUM *e) {
  RSA *rsa = RSA_new();
  if (rsa == NULL ||               //
      !bn_dup_into(&rsa->n, n) ||  //
      !bn_dup_into(&rsa->e, e) ||  //
      !RSA_check_key(rsa)) {
    RSA_free(rsa);
    return NULL;
  }

  return rsa;
}

RSA *RSA_new_private_key(const BIGNUM *n, const BIGNUM *e, const BIGNUM *d,
                         const BIGNUM *p, const BIGNUM *q, const BIGNUM *dmp1,
                         const BIGNUM *dmq1, const BIGNUM *iqmp) {
  RSA *rsa = RSA_new();
  if (rsa == NULL ||                     //
      !bn_dup_into(&rsa->n, n) ||        //
      !bn_dup_into(&rsa->e, e) ||        //
      !bn_dup_into(&rsa->d, d) ||        //
      !bn_dup_into(&rsa->p, p) ||        //
      !bn_dup_into(&rsa->q, q) ||        //
      !bn_dup_into(&rsa->dmp1, dmp1) ||  //
      !bn_dup_into(&rsa->dmq1, dmq1) ||  //
      !bn_dup_into(&rsa->iqmp, iqmp) ||  //
      !RSA_check_key(rsa)) {
    RSA_free(rsa);
    return NULL;
  }

  return rsa;
}

RSA *RSA_new_private_key_no_crt(const BIGNUM *n, const BIGNUM *e,
                                const BIGNUM *d) {
  RSA *rsa = RSA_new();
  if (rsa == NULL ||               //
      !bn_dup_into(&rsa->n, n) ||  //
      !bn_dup_into(&rsa->e, e) ||  //
      !bn_dup_into(&rsa->d, d) ||  //
      !RSA_check_key(rsa)) {
    RSA_free(rsa);
    return NULL;
  }

  return rsa;
}

RSA *RSA_new_private_key_no_e(const BIGNUM *n, const BIGNUM *d) {
  RSA *rsa = RSA_new();
  if (rsa == NULL) {
    return NULL;
  }

  rsa->flags |= RSA_FLAG_NO_PUBLIC_EXPONENT;
  if (!bn_dup_into(&rsa->n, n) ||  //
      !bn_dup_into(&rsa->d, d) ||  //
      !RSA_check_key(rsa)) {
    RSA_free(rsa);
    return NULL;
  }

  return rsa;
}

RSA *RSA_new_public_key_large_e(const BIGNUM *n, const BIGNUM *e) {
  RSA *rsa = RSA_new();
  if (rsa == NULL) {
    return NULL;
  }

  rsa->flags |= RSA_FLAG_LARGE_PUBLIC_EXPONENT;
  if (!bn_dup_into(&rsa->n, n) ||  //
      !bn_dup_into(&rsa->e, e) ||  //
      !RSA_check_key(rsa)) {
    RSA_free(rsa);
    return NULL;
  }

  return rsa;
}

RSA *RSA_new_private_key_large_e(const BIGNUM *n, const BIGNUM *e,
                                 const BIGNUM *d, const BIGNUM *p,
                                 const BIGNUM *q, const BIGNUM *dmp1,
                                 const BIGNUM *dmq1, const BIGNUM *iqmp) {
  RSA *rsa = RSA_new();
  if (rsa == NULL) {
    return NULL;
  }

  rsa->flags |= RSA_FLAG_LARGE_PUBLIC_EXPONENT;
  if (!bn_dup_into(&rsa->n, n) ||        //
      !bn_dup_into(&rsa->e, e) ||        //
      !bn_dup_into(&rsa->d, d) ||        //
      !bn_dup_into(&rsa->p, p) ||        //
      !bn_dup_into(&rsa->q, q) ||        //
      !bn_dup_into(&rsa->dmp1, dmp1) ||  //
      !bn_dup_into(&rsa->dmq1, dmq1) ||  //
      !bn_dup_into(&rsa->iqmp, iqmp) ||  //
      !RSA_check_key(rsa)) {
    RSA_free(rsa);
    return NULL;
  }

  return rsa;
}

RSA *RSA_new(void) { return RSA_new_method(NULL); }

RSA *RSA_new_method(const ENGINE *engine) {
  RSA *rsa = reinterpret_cast<RSA *>(OPENSSL_zalloc(sizeof(RSA)));
  if (rsa == NULL) {
    return NULL;
  }

  if (engine) {
    rsa->meth = ENGINE_get_RSA_method(engine);
  }

  if (rsa->meth == NULL) {
    rsa->meth = (RSA_METHOD *)RSA_default_method();
  }
  METHOD_ref(rsa->meth);

  rsa->references = 1;
  rsa->flags = rsa->meth->flags;
  CRYPTO_MUTEX_init(&rsa->lock);
  CRYPTO_new_ex_data(&rsa->ex_data);

  if (rsa->meth->init && !rsa->meth->init(rsa)) {
    CRYPTO_free_ex_data(g_rsa_ex_data_class_bss_get(), rsa, &rsa->ex_data);
    CRYPTO_MUTEX_cleanup(&rsa->lock);
    METHOD_unref(rsa->meth);
    OPENSSL_free(rsa);
    return NULL;
  }

  return rsa;
}

RSA *RSA_new_method_no_e(const ENGINE *engine, const BIGNUM *n) {
  RSA *rsa = RSA_new_method(engine);
  if (rsa == NULL || !bn_dup_into(&rsa->n, n)) {
    RSA_free(rsa);
    return NULL;
  }
  rsa->flags |= RSA_FLAG_NO_PUBLIC_EXPONENT;
  return rsa;
}

void RSA_free(RSA *rsa) {
  if (rsa == NULL) {
    return;
  }

  if (!CRYPTO_refcount_dec_and_test_zero(&rsa->references)) {
    return;
  }

  if (rsa->meth->finish) {
    rsa->meth->finish(rsa);
  }
  METHOD_unref(rsa->meth);

  CRYPTO_free_ex_data(g_rsa_ex_data_class_bss_get(), rsa, &rsa->ex_data);

  BN_free(rsa->n);
  BN_free(rsa->e);
  BN_free(rsa->d);
  BN_free(rsa->p);
  BN_free(rsa->q);
  BN_free(rsa->dmp1);
  BN_free(rsa->dmq1);
  BN_free(rsa->iqmp);
  rsa_invalidate_key(rsa);
  CRYPTO_MUTEX_cleanup(&rsa->lock);
  OPENSSL_free(rsa);
}

int RSA_up_ref(RSA *rsa) {
  CRYPTO_refcount_inc(&rsa->references);
  return 1;
}

unsigned RSA_bits(const RSA *rsa) { return BN_num_bits(rsa->n); }

const BIGNUM *RSA_get0_n(const RSA *rsa) { return rsa->n; }

const BIGNUM *RSA_get0_e(const RSA *rsa) { return rsa->e; }

const BIGNUM *RSA_get0_d(const RSA *rsa) { return rsa->d; }

const BIGNUM *RSA_get0_p(const RSA *rsa) { return rsa->p; }

const BIGNUM *RSA_get0_q(const RSA *rsa) { return rsa->q; }

const BIGNUM *RSA_get0_dmp1(const RSA *rsa) { return rsa->dmp1; }

const BIGNUM *RSA_get0_dmq1(const RSA *rsa) { return rsa->dmq1; }

const BIGNUM *RSA_get0_iqmp(const RSA *rsa) { return rsa->iqmp; }

void RSA_get0_key(const RSA *rsa, const BIGNUM **out_n, const BIGNUM **out_e,
                  const BIGNUM **out_d) {
  if (out_n != NULL) {
    *out_n = rsa->n;
  }
  if (out_e != NULL) {
    *out_e = rsa->e;
  }
  if (out_d != NULL) {
    *out_d = rsa->d;
  }
}

void RSA_get0_factors(const RSA *rsa, const BIGNUM **out_p,
                      const BIGNUM **out_q) {
  if (out_p != NULL) {
    *out_p = rsa->p;
  }
  if (out_q != NULL) {
    *out_q = rsa->q;
  }
}

const RSA_PSS_PARAMS *RSA_get0_pss_params(const RSA *rsa) {
  // We do not support the id-RSASSA-PSS key encoding. If we add support later,
  // the |maskHash| field should be filled in for OpenSSL compatibility.
  return NULL;
}

void RSA_get0_crt_params(const RSA *rsa, const BIGNUM **out_dmp1,
                         const BIGNUM **out_dmq1, const BIGNUM **out_iqmp) {
  if (out_dmp1 != NULL) {
    *out_dmp1 = rsa->dmp1;
  }
  if (out_dmq1 != NULL) {
    *out_dmq1 = rsa->dmq1;
  }
  if (out_iqmp != NULL) {
    *out_iqmp = rsa->iqmp;
  }
}

int RSA_set0_key(RSA *rsa, BIGNUM *n, BIGNUM *e, BIGNUM *d) {
  if ((rsa->n == NULL && n == NULL) || (rsa->e == NULL && e == NULL)) {
    return 0;
  }

  if (n != NULL) {
    BN_free(rsa->n);
    rsa->n = n;
  }
  if (e != NULL) {
    BN_free(rsa->e);
    rsa->e = e;
  }
  if (d != NULL) {
    BN_free(rsa->d);
    rsa->d = d;
  }

  rsa_invalidate_key(rsa);
  return 1;
}

int RSA_set0_factors(RSA *rsa, BIGNUM *p, BIGNUM *q) {
  if ((rsa->p == NULL && p == NULL) || (rsa->q == NULL && q == NULL)) {
    return 0;
  }

  if (p != NULL) {
    BN_free(rsa->p);
    rsa->p = p;
  }
  if (q != NULL) {
    BN_free(rsa->q);
    rsa->q = q;
  }

  rsa_invalidate_key(rsa);
  return 1;
}

int RSA_set0_crt_params(RSA *rsa, BIGNUM *dmp1, BIGNUM *dmq1, BIGNUM *iqmp) {
  if ((rsa->dmp1 == NULL && dmp1 == NULL) ||
      (rsa->dmq1 == NULL && dmq1 == NULL) ||
      (rsa->iqmp == NULL && iqmp == NULL)) {
    return 0;
  }

  if (dmp1 != NULL) {
    BN_free(rsa->dmp1);
    rsa->dmp1 = dmp1;
  }
  if (dmq1 != NULL) {
    BN_free(rsa->dmq1);
    rsa->dmq1 = dmq1;
  }
  if (iqmp != NULL) {
    BN_free(rsa->iqmp);
    rsa->iqmp = iqmp;
  }

  rsa_invalidate_key(rsa);
  return 1;
}

static int rsa_sign_raw_no_self_test(RSA *rsa, size_t *out_len, uint8_t *out,
                                     size_t max_out, const uint8_t *in,
                                     size_t in_len, int padding) {
  if (rsa->meth->sign_raw) {
    return rsa->meth->sign_raw(rsa, out_len, out, max_out, in, in_len, padding);
  }

  return rsa_default_sign_raw(rsa, out_len, out, max_out, in, in_len, padding);
}

int RSA_sign_raw(RSA *rsa, size_t *out_len, uint8_t *out, size_t max_out,
                 const uint8_t *in, size_t in_len, int padding) {
  boringssl_ensure_rsa_self_test();
  return rsa_sign_raw_no_self_test(rsa, out_len, out, max_out, in, in_len,
                                   padding);
}

unsigned RSA_size(const RSA *rsa) { return BN_num_bytes(rsa->n); }

int RSA_is_opaque(const RSA *rsa) {
  return rsa->meth && (rsa->meth->flags & RSA_FLAG_OPAQUE);
}

int RSA_get_ex_new_index(long argl, void *argp, CRYPTO_EX_unused *unused,
                         CRYPTO_EX_dup *dup_unused, CRYPTO_EX_free *free_func) {
  return CRYPTO_get_ex_new_index_ex(g_rsa_ex_data_class_bss_get(), argl, argp,
                                    free_func);
}

int RSA_set_ex_data(RSA *rsa, int idx, void *arg) {
  return CRYPTO_set_ex_data(&rsa->ex_data, idx, arg);
}

void *RSA_get_ex_data(const RSA *rsa, int idx) {
  return CRYPTO_get_ex_data(&rsa->ex_data, idx);
}

// SSL_SIG_LENGTH is the size of an SSL/TLS (prior to TLS 1.2) signature: it's
// the length of an MD5 and SHA1 hash.
static const unsigned SSL_SIG_LENGTH = 36;

// pkcs1_sig_prefix contains the ASN.1, DER encoded prefix for a hash that is
// to be signed with PKCS#1.
struct pkcs1_sig_prefix {
  // nid identifies the hash function.
  int nid;
  // hash_len is the expected length of the hash function.
  uint8_t hash_len;
  // len is the number of bytes of |bytes| which are valid.
  uint8_t len;
  // bytes contains the DER bytes.
  uint8_t bytes[19];
};

// kPKCS1SigPrefixes contains the ASN.1 prefixes for PKCS#1 signatures with
// different hash functions.
static const struct pkcs1_sig_prefix kPKCS1SigPrefixes[] = {
    {
        NID_md5,
        MD5_DIGEST_LENGTH,
        18,
        {0x30, 0x20, 0x30, 0x0c, 0x06, 0x08, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d,
         0x02, 0x05, 0x05, 0x00, 0x04, 0x10},
    },
    {
        NID_sha1,
        BCM_SHA_DIGEST_LENGTH,
        15,
        {0x30, 0x21, 0x30, 0x09, 0x06, 0x05, 0x2b, 0x0e, 0x03, 0x02, 0x1a, 0x05,
         0x00, 0x04, 0x14},
    },
    {
        NID_sha224,
        BCM_SHA224_DIGEST_LENGTH,
        19,
        {0x30, 0x2d, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
         0x04, 0x02, 0x04, 0x05, 0x00, 0x04, 0x1c},
    },
    {
        NID_sha256,
        BCM_SHA256_DIGEST_LENGTH,
        19,
        {0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
         0x04, 0x02, 0x01, 0x05, 0x00, 0x04, 0x20},
    },
    {
        NID_sha384,
        BCM_SHA384_DIGEST_LENGTH,
        19,
        {0x30, 0x41, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
         0x04, 0x02, 0x02, 0x05, 0x00, 0x04, 0x30},
    },
    {
        NID_sha512,
        BCM_SHA512_DIGEST_LENGTH,
        19,
        {0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
         0x04, 0x02, 0x03, 0x05, 0x00, 0x04, 0x40},
    },
    {
        NID_undef,
        0,
        0,
        {0},
    },
};

static int rsa_check_digest_size(int hash_nid, size_t digest_len) {
  if (hash_nid == NID_md5_sha1) {
    if (digest_len != SSL_SIG_LENGTH) {
      OPENSSL_PUT_ERROR(RSA, RSA_R_INVALID_MESSAGE_LENGTH);
      return 0;
    }
    return 1;
  }

  for (size_t i = 0; kPKCS1SigPrefixes[i].nid != NID_undef; i++) {
    const struct pkcs1_sig_prefix *sig_prefix = &kPKCS1SigPrefixes[i];
    if (sig_prefix->nid == hash_nid) {
      if (digest_len != sig_prefix->hash_len) {
        OPENSSL_PUT_ERROR(RSA, RSA_R_INVALID_MESSAGE_LENGTH);
        return 0;
      }
      return 1;
    }
  }

  OPENSSL_PUT_ERROR(RSA, RSA_R_UNKNOWN_ALGORITHM_TYPE);
  return 0;
}

int RSA_add_pkcs1_prefix(uint8_t **out_msg, size_t *out_msg_len,
                         int *is_alloced, int hash_nid, const uint8_t *digest,
                         size_t digest_len) {
  if (!rsa_check_digest_size(hash_nid, digest_len)) {
    return 0;
  }

  if (hash_nid == NID_md5_sha1) {
    // The length should already have been checked.
    assert(digest_len == SSL_SIG_LENGTH);
    *out_msg = (uint8_t *)digest;
    *out_msg_len = digest_len;
    *is_alloced = 0;
    return 1;
  }

  for (size_t i = 0; kPKCS1SigPrefixes[i].nid != NID_undef; i++) {
    const struct pkcs1_sig_prefix *sig_prefix = &kPKCS1SigPrefixes[i];
    if (sig_prefix->nid != hash_nid) {
      continue;
    }

    // The length should already have been checked.
    assert(digest_len == sig_prefix->hash_len);
    const uint8_t *prefix = sig_prefix->bytes;
    size_t prefix_len = sig_prefix->len;
    size_t signed_msg_len = prefix_len + digest_len;
    if (signed_msg_len < prefix_len) {
      OPENSSL_PUT_ERROR(RSA, RSA_R_TOO_LONG);
      return 0;
    }

    uint8_t *signed_msg =
        reinterpret_cast<uint8_t *>(OPENSSL_malloc(signed_msg_len));
    if (!signed_msg) {
      return 0;
    }

    OPENSSL_memcpy(signed_msg, prefix, prefix_len);
    OPENSSL_memcpy(signed_msg + prefix_len, digest, digest_len);

    *out_msg = signed_msg;
    *out_msg_len = signed_msg_len;
    *is_alloced = 1;

    return 1;
  }

  OPENSSL_PUT_ERROR(RSA, RSA_R_UNKNOWN_ALGORITHM_TYPE);
  return 0;
}

int rsa_sign_no_self_test(int hash_nid, const uint8_t *digest,
                          size_t digest_len, uint8_t *out, unsigned *out_len,
                          RSA *rsa) {
  if (rsa->meth->sign) {
    if (!rsa_check_digest_size(hash_nid, digest_len)) {
      return 0;
    }
    // All supported digest lengths fit in |unsigned|.
    assert(digest_len <= EVP_MAX_MD_SIZE);
    static_assert(EVP_MAX_MD_SIZE <= UINT_MAX, "digest too long");
    return rsa->meth->sign(hash_nid, digest, (unsigned)digest_len, out, out_len,
                           rsa);
  }

  const unsigned rsa_size = RSA_size(rsa);
  int ret = 0;
  uint8_t *signed_msg = NULL;
  size_t signed_msg_len = 0;
  int signed_msg_is_alloced = 0;
  size_t size_t_out_len;
  if (!RSA_add_pkcs1_prefix(&signed_msg, &signed_msg_len,
                            &signed_msg_is_alloced, hash_nid, digest,
                            digest_len) ||
      !rsa_sign_raw_no_self_test(rsa, &size_t_out_len, out, rsa_size,
                                 signed_msg, signed_msg_len,
                                 RSA_PKCS1_PADDING)) {
    goto err;
  }

  if (size_t_out_len > UINT_MAX) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_OVERFLOW);
    goto err;
  }

  *out_len = (unsigned)size_t_out_len;
  ret = 1;

err:
  if (signed_msg_is_alloced) {
    OPENSSL_free(signed_msg);
  }
  return ret;
}

int RSA_sign(int hash_nid, const uint8_t *digest, size_t digest_len,
             uint8_t *out, unsigned *out_len, RSA *rsa) {
  boringssl_ensure_rsa_self_test();

  return rsa_sign_no_self_test(hash_nid, digest, digest_len, out, out_len, rsa);
}

int RSA_sign_pss_mgf1(RSA *rsa, size_t *out_len, uint8_t *out, size_t max_out,
                      const uint8_t *digest, size_t digest_len,
                      const EVP_MD *md, const EVP_MD *mgf1_md, int salt_len) {
  if (digest_len != EVP_MD_size(md)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_INVALID_MESSAGE_LENGTH);
    return 0;
  }

  size_t padded_len = RSA_size(rsa);
  uint8_t *padded = reinterpret_cast<uint8_t *>(OPENSSL_malloc(padded_len));
  if (padded == NULL) {
    return 0;
  }

  int ret = RSA_padding_add_PKCS1_PSS_mgf1(rsa, padded, digest, md, mgf1_md,
                                           salt_len) &&
            RSA_sign_raw(rsa, out_len, out, max_out, padded, padded_len,
                         RSA_NO_PADDING);
  OPENSSL_free(padded);
  return ret;
}

int rsa_verify_no_self_test(int hash_nid, const uint8_t *digest,
                            size_t digest_len, const uint8_t *sig,
                            size_t sig_len, RSA *rsa) {
  if (rsa->n == NULL || rsa->e == NULL) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_VALUE_MISSING);
    return 0;
  }

  const size_t rsa_size = RSA_size(rsa);
  uint8_t *buf = NULL;
  int ret = 0;
  uint8_t *signed_msg = NULL;
  size_t signed_msg_len = 0, len;
  int signed_msg_is_alloced = 0;

  if (hash_nid == NID_md5_sha1 && digest_len != SSL_SIG_LENGTH) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_INVALID_MESSAGE_LENGTH);
    return 0;
  }

  buf = reinterpret_cast<uint8_t *>(OPENSSL_malloc(rsa_size));
  if (!buf) {
    return 0;
  }

  if (!rsa_verify_raw_no_self_test(rsa, &len, buf, rsa_size, sig, sig_len,
                                   RSA_PKCS1_PADDING) ||
      !RSA_add_pkcs1_prefix(&signed_msg, &signed_msg_len,
                            &signed_msg_is_alloced, hash_nid, digest,
                            digest_len)) {
    goto out;
  }

  // Check that no other information follows the hash value (FIPS 186-4 Section
  // 5.5) and it matches the expected hash.
  if (len != signed_msg_len || OPENSSL_memcmp(buf, signed_msg, len) != 0) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_SIGNATURE);
    goto out;
  }

  ret = 1;

out:
  OPENSSL_free(buf);
  if (signed_msg_is_alloced) {
    OPENSSL_free(signed_msg);
  }
  return ret;
}

int RSA_verify(int hash_nid, const uint8_t *digest, size_t digest_len,
               const uint8_t *sig, size_t sig_len, RSA *rsa) {
  boringssl_ensure_rsa_self_test();
  return rsa_verify_no_self_test(hash_nid, digest, digest_len, sig, sig_len,
                                 rsa);
}

int RSA_verify_pss_mgf1(RSA *rsa, const uint8_t *digest, size_t digest_len,
                        const EVP_MD *md, const EVP_MD *mgf1_md, int salt_len,
                        const uint8_t *sig, size_t sig_len) {
  if (digest_len != EVP_MD_size(md)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_INVALID_MESSAGE_LENGTH);
    return 0;
  }

  size_t em_len = RSA_size(rsa);
  uint8_t *em = reinterpret_cast<uint8_t *>(OPENSSL_malloc(em_len));
  if (em == NULL) {
    return 0;
  }

  int ret = 0;
  if (!RSA_verify_raw(rsa, &em_len, em, em_len, sig, sig_len, RSA_NO_PADDING)) {
    goto err;
  }

  if (em_len != RSA_size(rsa)) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
    goto err;
  }

  ret = RSA_verify_PKCS1_PSS_mgf1(rsa, digest, md, mgf1_md, em, salt_len);

err:
  OPENSSL_free(em);
  return ret;
}

static int check_mod_inverse(int *out_ok, const BIGNUM *a, const BIGNUM *ainv,
                             const BIGNUM *m, unsigned m_min_bits,
                             BN_CTX *ctx) {
  if (BN_is_negative(ainv) ||
      constant_time_declassify_int(BN_cmp(ainv, m) >= 0)) {
    *out_ok = 0;
    return 1;
  }

  // Note |bn_mul_consttime| and |bn_div_consttime| do not scale linearly, but
  // checking |ainv| is in range bounds the running time, assuming |m|'s bounds
  // were checked by the caller.
  bssl::BN_CTXScope scope(ctx);
  BIGNUM *tmp = BN_CTX_get(ctx);
  if (tmp == nullptr ||  //
      !bn_mul_consttime(tmp, a, ainv, ctx) ||
      !bn_div_consttime(NULL, tmp, tmp, m, m_min_bits, ctx)) {
    return 0;
  }
  *out_ok = constant_time_declassify_int(BN_is_one(tmp));
  return 1;
}

int RSA_check_key(const RSA *key) {
  // TODO(davidben): RSA key initialization is spread across
  // |rsa_check_public_key|, |RSA_check_key|, |freeze_private_key|, and
  // |BN_MONT_CTX_set_locked| as a result of API issues. See
  // https://crbug.com/boringssl/316. As a result, we inconsistently check RSA
  // invariants. We should fix this and integrate that logic.

  if (!rsa_check_public_key(key)) {
    return 0;
  }

  if ((key->p != NULL) != (key->q != NULL)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_ONLY_ONE_OF_P_Q_GIVEN);
    return 0;
  }

  // |key->d| must be bounded by |key->n|. This ensures bounds on |RSA_bits|
  // translate to bounds on the running time of private key operations.
  if (key->d != NULL &&
      (BN_is_negative(key->d) || BN_cmp(key->d, key->n) >= 0)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_D_OUT_OF_RANGE);
    return 0;
  }

  if (key->d == NULL || key->p == NULL) {
    // For a public key, or without p and q, there's nothing that can be
    // checked.
    return 1;
  }

  BN_CTX *ctx = BN_CTX_new();
  if (ctx == NULL) {
    return 0;
  }

  BIGNUM tmp, de, pm1, qm1, dmp1, dmq1;
  int ok = 0, has_crt_values;
  unsigned pm1_bits, qm1_bits;
  BN_init(&tmp);
  BN_init(&de);
  BN_init(&pm1);
  BN_init(&qm1);
  BN_init(&dmp1);
  BN_init(&dmq1);

  // Check that p * q == n. Before we multiply, we check that p and q are in
  // bounds, to avoid a DoS vector in |bn_mul_consttime| below. Note that
  // n was bound by |rsa_check_public_key|. This also implicitly checks p and q
  // are odd, which is a necessary condition for Montgomery reduction.
  if (BN_is_negative(key->p) ||
      constant_time_declassify_int(BN_cmp(key->p, key->n) >= 0) ||
      BN_is_negative(key->q) ||
      constant_time_declassify_int(BN_cmp(key->q, key->n) >= 0)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_N_NOT_EQUAL_P_Q);
    goto out;
  }
  if (!bn_mul_consttime(&tmp, key->p, key->q, ctx)) {
    OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
    goto out;
  }
  if (BN_cmp(&tmp, key->n) != 0) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_N_NOT_EQUAL_P_Q);
    goto out;
  }

  // d must be an inverse of e mod the Carmichael totient, lcm(p-1, q-1), but it
  // may be unreduced because other implementations use the Euler totient. We
  // simply check that d * e is one mod p-1 and mod q-1. Note d and e were bound
  // by earlier checks in this function.
  if (!bn_usub_consttime(&pm1, key->p, BN_value_one()) ||
      !bn_usub_consttime(&qm1, key->q, BN_value_one())) {
    OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
    goto out;
  }
  pm1_bits = BN_num_bits(&pm1);
  qm1_bits = BN_num_bits(&qm1);
  if (!bn_mul_consttime(&de, key->d, key->e, ctx) ||
      !bn_div_consttime(NULL, &tmp, &de, &pm1, pm1_bits, ctx) ||
      !bn_div_consttime(NULL, &de, &de, &qm1, qm1_bits, ctx)) {
    OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
    goto out;
  }

  if (constant_time_declassify_int(!BN_is_one(&tmp)) ||
      constant_time_declassify_int(!BN_is_one(&de))) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_D_E_NOT_CONGRUENT_TO_1);
    goto out;
  }

  has_crt_values = key->dmp1 != NULL;
  if (has_crt_values != (key->dmq1 != NULL) ||
      has_crt_values != (key->iqmp != NULL)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_INCONSISTENT_SET_OF_CRT_VALUES);
    goto out;
  }

  if (has_crt_values) {
    int dmp1_ok, dmq1_ok, iqmp_ok;
    if (!check_mod_inverse(&dmp1_ok, key->e, key->dmp1, &pm1, pm1_bits, ctx) ||
        !check_mod_inverse(&dmq1_ok, key->e, key->dmq1, &qm1, qm1_bits, ctx) ||
        // |p| is odd, so |pm1| and |p| have the same bit width. If they didn't,
        // we only need a lower bound anyway.
        !check_mod_inverse(&iqmp_ok, key->q, key->iqmp, key->p, pm1_bits,
                           ctx)) {
      OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
      goto out;
    }

    if (!dmp1_ok || !dmq1_ok || !iqmp_ok) {
      OPENSSL_PUT_ERROR(RSA, RSA_R_CRT_VALUES_INCORRECT);
      goto out;
    }
  }

  ok = 1;

out:
  BN_free(&tmp);
  BN_free(&de);
  BN_free(&pm1);
  BN_free(&qm1);
  BN_free(&dmp1);
  BN_free(&dmq1);
  BN_CTX_free(ctx);

  return ok;
}


// This is the product of the 132 smallest odd primes, from 3 to 751.
static const BN_ULONG kSmallFactorsLimbs[] = {TOBN(0xc4309333, 0x3ef4e3e1),
                                              TOBN(0x71161eb6, 0xcd2d655f),
                                              TOBN(0x95e2238c, 0x0bf94862),
                                              TOBN(0x3eb233d3, 0x24f7912b),
                                              TOBN(0x6b55514b, 0xbf26c483),
                                              TOBN(0x0a84d817, 0x5a144871),
                                              TOBN(0x77d12fee, 0x9b82210a),
                                              TOBN(0xdb5b93c2, 0x97f050b3),
                                              TOBN(0x4acad6b9, 0x4d6c026b),
                                              TOBN(0xeb7751f3, 0x54aec893),
                                              TOBN(0xdba53368, 0x36bc85c4),
                                              TOBN(0xd85a1b28, 0x7f5ec78e),
                                              TOBN(0x2eb072d8, 0x6b322244),
                                              TOBN(0xbba51112, 0x5e2b3aea),
                                              TOBN(0x36ed1a6c, 0x0e2486bf),
                                              TOBN(0x5f270460, 0xec0c5727),
                                              0x000017b1};

DEFINE_LOCAL_DATA(BIGNUM, g_small_factors) {
  out->d = (BN_ULONG *)kSmallFactorsLimbs;
  out->width = OPENSSL_ARRAY_SIZE(kSmallFactorsLimbs);
  out->dmax = out->width;
  out->neg = 0;
  out->flags = BN_FLG_STATIC_DATA;
}

int RSA_check_fips(RSA *key) {
  if (!RSA_check_key(key)) {
    return 0;
  }

  BN_CTX *ctx = BN_CTX_new();
  if (ctx == NULL) {
    return 0;
  }

  BIGNUM small_gcd;
  BN_init(&small_gcd);

  int ret = 1;

  // Perform partial public key validation of RSA keys (SP 800-89 5.3.3).
  // Although this is not for primality testing, SP 800-89 cites an RSA
  // primality testing algorithm, so we use |BN_prime_checks_for_generation| to
  // match. This is only a plausibility test and we expect the value to be
  // composite, so too few iterations will cause us to reject the key, not use
  // an implausible one.
  //
  // |key->e| may be nullptr if created with |RSA_new_private_key_no_e|.
  enum bn_primality_result_t primality_result;
  if (key->e == nullptr ||          //
      BN_num_bits(key->e) <= 16 ||  //
      BN_num_bits(key->e) > 256 ||  //
      !BN_is_odd(key->n) ||         //
      !BN_is_odd(key->e) ||
      !BN_gcd(&small_gcd, key->n, g_small_factors(), ctx) ||
      !BN_is_one(&small_gcd) ||
      !BN_enhanced_miller_rabin_primality_test(&primality_result, key->n,
                                               BN_prime_checks_for_generation,
                                               ctx, NULL) ||
      primality_result != bn_non_prime_power_composite) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_PUBLIC_KEY_VALIDATION_FAILED);
    ret = 0;
  }

  BN_free(&small_gcd);
  BN_CTX_free(ctx);

  if (!ret || key->d == NULL || key->p == NULL) {
    // On a failure or on only a public key, there's nothing else can be
    // checked.
    return ret;
  }

  // FIPS pairwise consistency test (FIPS 140-2 4.9.2). Per FIPS 140-2 IG,
  // section 9.9, it is not known whether |rsa| will be used for signing or
  // encryption, so either pair-wise consistency self-test is acceptable. We
  // perform a signing test.
  uint8_t data[32] = {0};
  unsigned sig_len = RSA_size(key);
  uint8_t *sig = reinterpret_cast<uint8_t *>(OPENSSL_malloc(sig_len));
  if (sig == NULL) {
    return 0;
  }

  if (!RSA_sign(NID_sha256, data, sizeof(data), sig, &sig_len, key)) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
    ret = 0;
    goto cleanup;
  }
  if (boringssl_fips_break_test("RSA_PWCT")) {
    data[0] = ~data[0];
  }
  if (!RSA_verify(NID_sha256, data, sizeof(data), sig, sig_len, key)) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
    ret = 0;
  }

cleanup:
  OPENSSL_free(sig);

  return ret;
}

int rsa_private_transform_no_self_test(RSA *rsa, uint8_t *out,
                                       const uint8_t *in, size_t len) {
  if (rsa->meth->private_transform) {
    return rsa->meth->private_transform(rsa, out, in, len);
  }

  return rsa_default_private_transform(rsa, out, in, len);
}

int rsa_private_transform(RSA *rsa, uint8_t *out, const uint8_t *in,
                          size_t len) {
  boringssl_ensure_rsa_self_test();
  return rsa_private_transform_no_self_test(rsa, out, in, len);
}

int RSA_flags(const RSA *rsa) { return rsa->flags; }

int RSA_test_flags(const RSA *rsa, int flags) { return rsa->flags & flags; }
