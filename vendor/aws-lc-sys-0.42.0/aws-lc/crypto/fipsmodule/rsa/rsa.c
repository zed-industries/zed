// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

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
#include <openssl/sha.h>
#include <openssl/thread.h>

#include "../../internal.h"
//#include "../../rsa_extra/internal.h"
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

  if (*dst == src) {
    return 1;
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
  SET_DIT_AUTO_RESET;
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
  SET_DIT_AUTO_RESET;
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
  SET_DIT_AUTO_RESET;
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
  SET_DIT_AUTO_RESET;
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
  RSA *rsa = OPENSSL_zalloc(sizeof(RSA));
  if (rsa == NULL) {
    return NULL;
  }

  if (engine) {
    rsa->meth = ENGINE_get_RSA(engine);
  }

  if (rsa->meth == NULL) {
    rsa->meth = (RSA_METHOD *) RSA_get_default_method();
  }

  rsa->references = 1;
  rsa->flags = rsa->meth->flags;
  CRYPTO_MUTEX_init(&rsa->lock);
  CRYPTO_new_ex_data(&rsa->ex_data);

  if (rsa->meth->init && !rsa->meth->init(rsa)) {
    CRYPTO_free_ex_data(g_rsa_ex_data_class_bss_get(), rsa, &rsa->ex_data);
    CRYPTO_MUTEX_cleanup(&rsa->lock);
    OPENSSL_free(rsa);
    return NULL;
  }

  return rsa;
}

RSA *RSA_new_method_no_e(const ENGINE *engine, const BIGNUM *n) {
  RSA *rsa = RSA_new_method(engine);
  if (rsa == NULL ||
      !bn_dup_into(&rsa->n, n)) {
    RSA_free(rsa);
    return NULL;
  }
  rsa->flags |= RSA_FLAG_NO_PUBLIC_EXPONENT;
  return rsa;
}

void RSA_free(RSA *rsa) {
  SET_DIT_AUTO_RESET;
  if (rsa == NULL) {
    return;
  }


  if (!CRYPTO_refcount_dec_and_test_zero(&rsa->references)) {
    return;
  }

  if (rsa->meth && rsa->meth->finish) {
    rsa->meth->finish(rsa);
  }

  CRYPTO_free_ex_data(g_rsa_ex_data_class_bss_get(), rsa, &rsa->ex_data);

  BN_free(rsa->n);
  BN_free(rsa->e);
  BN_free(rsa->d);
  BN_free(rsa->p);
  BN_free(rsa->q);
  BN_free(rsa->dmp1);
  BN_free(rsa->dmq1);
  BN_free(rsa->iqmp);
  RSASSA_PSS_PARAMS_free(rsa->pss);
  rsa_invalidate_key(rsa);
  CRYPTO_MUTEX_cleanup(&rsa->lock);
  OPENSSL_free(rsa);
}

int RSA_up_ref(RSA *rsa) {
  SET_DIT_AUTO_RESET;
  CRYPTO_refcount_inc(&rsa->references);
  return 1;
}

unsigned RSA_bits(const RSA *rsa) {
  SET_DIT_AUTO_RESET;
  return BN_num_bits(rsa->n);
}

const BIGNUM *RSA_get0_n(const RSA *rsa) {
  SET_DIT_AUTO_RESET;
  return rsa->n;
}

const BIGNUM *RSA_get0_e(const RSA *rsa) {
  SET_DIT_AUTO_RESET;
  return rsa->e;
}

const BIGNUM *RSA_get0_d(const RSA *rsa) {
  SET_DIT_AUTO_RESET;
  return rsa->d;
}

const BIGNUM *RSA_get0_p(const RSA *rsa) {
  SET_DIT_AUTO_RESET;
  return rsa->p;
}

const BIGNUM *RSA_get0_q(const RSA *rsa) {
  SET_DIT_AUTO_RESET;
  return rsa->q;
}

const BIGNUM *RSA_get0_dmp1(const RSA *rsa) {
  SET_DIT_AUTO_RESET;
  return rsa->dmp1;
}

const BIGNUM *RSA_get0_dmq1(const RSA *rsa) {
  SET_DIT_AUTO_RESET;
  return rsa->dmq1;
}

const BIGNUM *RSA_get0_iqmp(const RSA *rsa) {
  SET_DIT_AUTO_RESET;
  return rsa->iqmp;
}

void RSA_get0_key(const RSA *rsa, const BIGNUM **out_n, const BIGNUM **out_e,
                  const BIGNUM **out_d) {
  SET_DIT_AUTO_RESET;
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
  SET_DIT_AUTO_RESET;
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
  SET_DIT_AUTO_RESET;
  return NULL;
}

const RSASSA_PSS_PARAMS *RSA_get0_ssa_pss_params(const RSA *rsa) {
  SET_DIT_AUTO_RESET;
  return rsa->pss;
}

void RSA_get0_crt_params(const RSA *rsa, const BIGNUM **out_dmp1,
                         const BIGNUM **out_dmq1, const BIGNUM **out_iqmp) {
  SET_DIT_AUTO_RESET;
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
  SET_DIT_AUTO_RESET;
  if ((rsa->n == NULL && n == NULL) ||
      (rsa->e == NULL && e == NULL && rsa->d == NULL && d == NULL)) {
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
  SET_DIT_AUTO_RESET;
  if ((rsa->p == NULL && p == NULL) ||
      (rsa->q == NULL && q == NULL)) {
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
  SET_DIT_AUTO_RESET;
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

RSA_METHOD *RSA_meth_new(const char *name, int flags) {
  RSA_METHOD *meth = OPENSSL_zalloc(sizeof(*meth));

  if (meth == NULL) {
    return NULL;
  }

  if (flags == RSA_FLAG_OPAQUE) {
    meth->flags = flags;
  }
  return meth;
}

int RSA_set_method(RSA *rsa, const RSA_METHOD *meth) {
  if(rsa == NULL || meth == NULL) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  rsa->meth = meth;
  return 1;
}

const RSA_METHOD *RSA_get_method(const RSA *rsa) {
  if(rsa == NULL) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  return rsa->meth;
}

void RSA_meth_free(RSA_METHOD *meth)
{
  if (meth != NULL) {
    OPENSSL_free(meth);
  }
}

int RSA_meth_set_init(RSA_METHOD *meth, int (*init) (RSA *rsa)) {
  if(meth == NULL) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  meth->init = init;
  return 1;
}

int RSA_meth_set_finish(RSA_METHOD *meth, int (*finish) (RSA *rsa)) {
  if(meth == NULL) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  meth->finish = finish;
  return 1;
}

int RSA_meth_set_priv_dec(RSA_METHOD *meth,
                          int (*priv_dec) (int max_out, const uint8_t *from,
                                           uint8_t *to, RSA *rsa,
                                           int padding)) {
  if(meth == NULL) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  meth->decrypt = priv_dec;
  return 1;
}

int RSA_meth_set_priv_enc(RSA_METHOD *meth,
                          int (*priv_enc) (int max_out, const uint8_t *from,
                                           uint8_t *to, RSA *rsa,
                                           int padding)) {
  if(meth == NULL) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  meth->sign_raw = priv_enc;
  return 1;
}

int RSA_meth_set_pub_dec(RSA_METHOD *meth,
                         int (*pub_dec) (int max_out, const uint8_t *from,
                                         uint8_t *to, RSA *rsa,
                                         int padding)) {
  if(meth == NULL) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  meth->verify_raw = pub_dec;
  return 1;
}

int RSA_meth_set_pub_enc(RSA_METHOD *meth,
                         int (*pub_enc) (int max_out, const uint8_t *from,
                                         uint8_t *to, RSA *rsa,
                                         int padding)) {
  if(meth == NULL) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  meth->encrypt = pub_enc;
  return 1;
}

int RSA_meth_set0_app_data(RSA_METHOD *meth, void *app_data) {
  if(meth == NULL) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  meth->app_data = app_data;
  return 1;
}

int RSA_meth_set_sign(RSA_METHOD *meth, int (*sign) (int type,
        const unsigned char *m, unsigned int m_length, unsigned char *sigret,
        unsigned int *siglen, const RSA *rsa)) {
  if(meth == NULL) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  meth->sign = sign;
  return 1;
}

static int rsa_sign_raw_no_self_test(RSA *rsa, size_t *out_len, uint8_t *out,
                                     size_t max_out, const uint8_t *in,
                                     size_t in_len, int padding) {
  SET_DIT_AUTO_RESET;
  if (rsa->meth && rsa->meth->sign_raw) {
    // In OpenSSL, the RSA_METHOD |sign_raw| or |priv_enc| operation does
    // not directly take and initialize an |out_len| parameter. Instead, it
    // returns the size of the encrypted data or a negative number for error.
    // Our wrapping functions like |RSA_sign_raw| diverge from this paradigm
    // and expect an |out_len| parameter. To remain compatible with this new
    // paradigm and OpenSSL, we initialize |out_len| based on the return value
    // here.
    if (in_len > INT_MAX) {
      OPENSSL_PUT_ERROR(RSA, ERR_R_OVERFLOW);
      *out_len = 0;
      return 0;
    }
    int ret = rsa->meth->sign_raw((int)in_len, in, out, rsa, padding);
    if(ret < 0) {
      *out_len = 0;
      return 0;
    }
    *out_len = ret;
    return 1;
  }

  return rsa_default_sign_raw(rsa, out_len, out, max_out, in, in_len, padding);
}

int RSA_sign_raw(RSA *rsa, size_t *out_len, uint8_t *out, size_t max_out,
                 const uint8_t *in, size_t in_len, int padding) {
  boringssl_ensure_rsa_self_test();
  SET_DIT_AUTO_RESET;

  return rsa_sign_raw_no_self_test(rsa, out_len, out, max_out, in, in_len,
                                   padding);
}

unsigned RSA_size(const RSA *rsa) {
  SET_DIT_AUTO_RESET;
  size_t ret = (rsa->meth && rsa->meth->size) ?
          rsa->meth->size(rsa) : rsa_default_size(rsa);
  // RSA modulus sizes are bounded by |BIGNUM|, which must fit in |unsigned|.
  //
  // TODO(https://crbug.com/boringssl/516): Should we make this return |size_t|?
  assert(ret < UINT_MAX);
  return (unsigned)ret;
}

int RSA_is_opaque(const RSA *rsa) {
  SET_DIT_AUTO_RESET;
  return rsa->meth && (rsa->meth->flags & RSA_FLAG_OPAQUE);
}

int RSA_get_ex_new_index(long argl, void *argp, CRYPTO_EX_unused *unused,
                         CRYPTO_EX_dup *dup_unused, CRYPTO_EX_free *free_func) {
  SET_DIT_AUTO_RESET;
  int index;
  if (!CRYPTO_get_ex_new_index(g_rsa_ex_data_class_bss_get(), &index, argl,
                               argp, free_func)) {
    return -1;
  }
  return index;
}

int RSA_set_ex_data(RSA *rsa, int idx, void *arg) {
  SET_DIT_AUTO_RESET;
  return CRYPTO_set_ex_data(&rsa->ex_data, idx, arg);
}

void *RSA_get_ex_data(const RSA *rsa, int idx) {
  SET_DIT_AUTO_RESET;
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
// different hash functions. These are defined in RFC-8017 Section 9.2
// https://datatracker.ietf.org/doc/html/rfc8017#section-9.2
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
     SHA_DIGEST_LENGTH,
     15,
     {0x30, 0x21, 0x30, 0x09, 0x06, 0x05, 0x2b, 0x0e, 0x03, 0x02, 0x1a, 0x05,
      0x00, 0x04, 0x14},
    },
    {
     NID_sha224,
     SHA224_DIGEST_LENGTH,
     19,
     {0x30, 0x2d, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
      0x04, 0x02, 0x04, 0x05, 0x00, 0x04, 0x1c},
    },
    {
     NID_sha256,
     SHA256_DIGEST_LENGTH,
     19,
     {0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
      0x04, 0x02, 0x01, 0x05, 0x00, 0x04, 0x20},
    },
    {
     NID_sha384,
     SHA384_DIGEST_LENGTH,
     19,
     {0x30, 0x41, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
      0x04, 0x02, 0x02, 0x05, 0x00, 0x04, 0x30},
    },
    {
     NID_sha512,
     SHA512_DIGEST_LENGTH,
     19,
     {0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
      0x04, 0x02, 0x03, 0x05, 0x00, 0x04, 0x40},
    },
    {
     NID_sha512_224,
     SHA512_224_DIGEST_LENGTH,
     19,
     {0x30, 0x2d, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
      0x04, 0x02, 0x05, 0x05, 0x00, 0x04, 0x1c},
    },
    {
     NID_sha512_256,
     SHA512_256_DIGEST_LENGTH,
     19,
     {0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
      0x04, 0x02, 0x06, 0x05, 0x00, 0x04, 0x20},
    },
    {
      NID_sha3_224,
      28,
      19,
      {0x30, 0x2d, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
       0x04, 0x02, 0x07, 0x05, 0x00, 0x04, 0x1c},
    },
    {
      NID_sha3_256,
      32,
      19,
      {0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
       0x04, 0x02, 0x08, 0x05, 0x00, 0x04, 0x20},
    },
    {
      NID_sha3_384,
      48,
      19,
      {0x30, 0x41, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
       0x04, 0x02, 0x09, 0x05, 0x00, 0x04, 0x30},
    },
    {
      NID_sha3_512,
      64,
      19,
      {0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
       0x04, 0x02, 0x0a, 0x05, 0x00, 0x04, 0x40},
    },
    {
     NID_undef, 0, 0, {0},
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
    const uint8_t* prefix = sig_prefix->bytes;
    size_t prefix_len = sig_prefix->len;
    size_t signed_msg_len = prefix_len + digest_len;
    if (signed_msg_len < prefix_len) {
      OPENSSL_PUT_ERROR(RSA, RSA_R_TOO_LONG);
      return 0;
    }

    uint8_t *signed_msg = OPENSSL_malloc(signed_msg_len);
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
  if (rsa->meth && rsa->meth->sign) {
    if (!rsa_check_digest_size(hash_nid, digest_len)) {
      return 0;
    }
    // All supported digest lengths fit in |unsigned|.
    assert(digest_len <= EVP_MAX_MD_SIZE);
    OPENSSL_STATIC_ASSERT(EVP_MAX_MD_SIZE <= UINT_MAX, digest_too_long);
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
  SET_DIT_AUTO_RESET;

  return rsa_sign_no_self_test(hash_nid, digest, digest_len, out, out_len, rsa);
}

int RSA_sign_pss_mgf1(RSA *rsa, size_t *out_len, uint8_t *out, size_t max_out,
                      const uint8_t *digest, size_t digest_len,
                      const EVP_MD *md, const EVP_MD *mgf1_md, int salt_len) {
  SET_DIT_AUTO_RESET;
  if (digest_len != EVP_MD_size(md)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_INVALID_MESSAGE_LENGTH);
    return 0;
  }

  size_t padded_len = RSA_size(rsa);
  uint8_t *padded = OPENSSL_malloc(padded_len);
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

int rsa_digestsign_no_self_test(const EVP_MD *md, const uint8_t *input,
                                size_t in_len, uint8_t *out, unsigned *out_len,
                                RSA *rsa) {
  SET_DIT_AUTO_RESET;
  uint8_t digest[EVP_MAX_MD_SIZE];
  unsigned int digest_len = EVP_MAX_MD_SIZE;
  if (!EVP_Digest(input, in_len, digest, &digest_len, md, NULL)) {
    return 0;
  }

  return rsa_sign_no_self_test(EVP_MD_type(md), digest, digest_len, out,
                               out_len, rsa);
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

  buf = OPENSSL_malloc(rsa_size);
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

  // Check that no other information follows the hash value (FIPS 186-4 Section 5.5)
  if (len != signed_msg_len) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_SIGNATURE);
    goto out;
  }

  // Check that the computed hash matches the expected hash
  if (OPENSSL_memcmp(buf, signed_msg, len) != 0) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_MISMATCHED_SIGNATURE);
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

int rsa_digestverify_no_self_test(const EVP_MD *md, const uint8_t *input,
                                  size_t in_len, const uint8_t *sig,
                                  size_t sig_len, RSA *rsa) {
  uint8_t digest[EVP_MAX_MD_SIZE];
  unsigned int digest_len = EVP_MAX_MD_SIZE;
  if (!EVP_Digest(input, in_len, digest, &digest_len, md, NULL)) {
    return 0;
  }

  return rsa_verify_no_self_test(EVP_MD_type(md), digest, digest_len, sig,
                                 sig_len, rsa);
}

int RSA_verify(int hash_nid, const uint8_t *digest, size_t digest_len,
               const uint8_t *sig, size_t sig_len, RSA *rsa) {
  boringssl_ensure_rsa_self_test();
  SET_DIT_AUTO_RESET;
  return rsa_verify_no_self_test(hash_nid, digest, digest_len, sig, sig_len,
                                 rsa);
}

int RSA_verify_pss_mgf1(RSA *rsa, const uint8_t *digest, size_t digest_len,
                        const EVP_MD *md, const EVP_MD *mgf1_md, int salt_len,
                        const uint8_t *sig, size_t sig_len) {
  SET_DIT_AUTO_RESET;
  if (digest_len != EVP_MD_size(md)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_INVALID_MESSAGE_LENGTH);
    return 0;
  }

  size_t em_len = RSA_size(rsa);
  uint8_t *em = OPENSSL_malloc(em_len);
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

int rsa_private_transform_no_self_test(RSA *rsa, uint8_t *out,
                                       const uint8_t *in, size_t len) {
  if (rsa->meth && rsa->meth->private_transform) {
    return rsa->meth->private_transform(rsa, out, in, len);
  }

  return rsa_default_private_transform(rsa, out, in, len);
}

int rsa_private_transform(RSA *rsa, uint8_t *out, const uint8_t *in,
                          size_t len) {
  boringssl_ensure_rsa_self_test();
  SET_DIT_AUTO_RESET;
  return rsa_private_transform_no_self_test(rsa, out, in, len);
}

int RSA_flags(const RSA *rsa) {
  SET_DIT_AUTO_RESET;
  if (rsa == NULL) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  return rsa->flags;
}

void RSA_set_flags(RSA *rsa, int flags) {
  SET_DIT_AUTO_RESET;
  if (rsa == NULL) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_PASSED_NULL_PARAMETER);
    return;
  }

  rsa->flags |= flags;
}

int RSA_test_flags(const RSA *rsa, int flags) {
  SET_DIT_AUTO_RESET;
  if (rsa) {
    return rsa->flags & flags;
  }

  OPENSSL_PUT_ERROR(RSA, ERR_R_PASSED_NULL_PARAMETER);
  return 0;
}

int RSA_blinding_on(RSA *rsa, BN_CTX *ctx) {
  SET_DIT_AUTO_RESET;
  return (rsa != NULL && ((rsa->flags & RSA_FLAG_NO_BLINDING) == 0)) ? 1 : 0;
}

void RSA_blinding_off_temp_for_accp_compatibility(RSA *rsa) {
  SET_DIT_AUTO_RESET;
  if (rsa != NULL) {
    rsa->flags |= RSA_FLAG_NO_BLINDING;
  }
}

int RSA_pkey_ctx_ctrl(EVP_PKEY_CTX *ctx, int optype, int cmd, int p1, void *p2) {
  SET_DIT_AUTO_RESET;
  if (ctx != NULL && ctx->pmeth != NULL) {
    if (ctx->pmeth->pkey_id == EVP_PKEY_RSA ||
        ctx->pmeth->pkey_id == EVP_PKEY_RSA_PSS) {
      return EVP_PKEY_CTX_ctrl(ctx, -1, optype, cmd, p1, p2);
    }
    return -1;
  }
  return 0;
}

// ------------- KEY CHECKING FUNCTIONS ----------------
//
// Performs several checks on the public component of the given RSA key.
// The key must have at least the public modulus n, the public exponent e is
// optional (this is to support the special case of JCA stripped private keys
// that are missing e).
//
// The checks:
//   - n is positive, odd, and fits in 16k bits,
//   - e is positive and odd (if present),
//   - e is either <= 2^33 in default case,
//              or <= n when RSA_FLAG_LARGE_PUBLIC_EXPONENT is set.
//
int is_public_component_of_rsa_key_good(const RSA *key) {
  SET_DIT_AUTO_RESET;
  if (key->n == NULL) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_VALUE_MISSING);
    return 0;
  }

  unsigned int n_bits = BN_num_bits(key->n);
  if (n_bits > 16 * 1024) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_MODULUS_TOO_LARGE);
    return 0;
  }

  // RSA moduli n must be positive and odd because it is
  // a product of positive odd prime numbers.
  if (!BN_is_odd(key->n) || BN_is_negative(key->n)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_RSA_PARAMETERS);
    return 0;
  }

  // Stripped private keys do not have the public exponent e, so the remaining
  // checks in this function are not applicable. However, such keys should have
  // the RSA_FLAG_NO_PUBLIC_EXPONENT flag set.
  if (key->e == NULL) {
    if (!(key->flags & RSA_FLAG_NO_PUBLIC_EXPONENT)) {
      OPENSSL_PUT_ERROR(RSA, RSA_R_VALUE_MISSING);
      return 0;
    }
    return 1;
  }

  unsigned int e_bits = BN_num_bits(key->e);

  // RSA public exponent e must be odd because it is a multiplicative inverse
  // of the corresponding private exponent modulo phi(n). To be invertible
  // modulo phi(n), e has to be realtively prime to phi(n). Since
  // phi(n) = (p-1)(q-1) and p and q are odd prime numbers, it follows that
  // phi(n) is even. Therefore, for e to be relatively prime to phi(n) it is
  // necessary that e is odd. Additionally, reject e = 1 and negative e.
  if (!BN_is_odd(key->e) || BN_is_negative(key->e) || e_bits < 2) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_E_VALUE);
    return 0;
  }

  if (key->flags & RSA_FLAG_LARGE_PUBLIC_EXPONENT) {
    // The caller has requested disabling DoS protections.
    // Still, e must be less than n.
    if (BN_ucmp(key->n, key->e) <= 0) {
      OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_E_VALUE);
      return 0;
    }

  } else {
    // Mitigate DoS attacks by limiting the exponent size. 33 bits was chosen as
    // the limit based on the recommendations in:
    //   - https://www.imperialviolet.org/2012/03/16/rsae.html
    //   - https://www.imperialviolet.org/2012/03/17/rsados.html
    if (e_bits > 33) {
      OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_E_VALUE);
      return 0;
    }
  }

  return 1;
}

// The RSA key checking function works with five different types of keys:
//   - public:        (n, e),
//   - private_min:   (n, e, d),
//   - private:       (n, e, d, p, q),
//   - private_crt:   (n, e, d, p, q, dmp1, dmq1, iqmp),
//   - private_strip: (n, d).
enum rsa_key_type_for_checking {
    RSA_KEY_TYPE_FOR_CHECKING_PUBLIC,
    RSA_KEY_TYPE_FOR_CHECKING_PRIVATE_MIN,
    RSA_KEY_TYPE_FOR_CHECKING_PRIVATE,
    RSA_KEY_TYPE_FOR_CHECKING_PRIVATE_CRT,
    RSA_KEY_TYPE_FOR_CHECKING_PRIVATE_STRIP,
    RSA_KEY_TYPE_FOR_CHECKING_INVALID,
};

static enum rsa_key_type_for_checking determine_key_type_for_checking(const RSA *key) {
    // The key must have the modulus n.
    SET_DIT_AUTO_RESET;
    if (key->n == NULL) {
      return RSA_KEY_TYPE_FOR_CHECKING_INVALID;
    }

    // (n, e)
    if (key->e != NULL && key->d == NULL && key->p == NULL && key->q == NULL &&
        key->dmp1 == NULL && key->dmq1 == NULL && key->iqmp == NULL) {
      return RSA_KEY_TYPE_FOR_CHECKING_PUBLIC;
    }

    // (n, e, d)
    if (key->e != NULL && key->d != NULL && key->p == NULL && key->q == NULL &&
        key->dmp1 == NULL && key->dmq1 == NULL && key->iqmp == NULL) {
      return RSA_KEY_TYPE_FOR_CHECKING_PRIVATE_MIN;
    }

    // (n, e, d, p, q)
    if (key->e != NULL && key->d != NULL && key->p != NULL && key->q != NULL &&
        key->dmp1 == NULL && key->dmq1 == NULL && key->iqmp == NULL) {
      return RSA_KEY_TYPE_FOR_CHECKING_PRIVATE;
    }

    // (n, e, d, p, q, dmp1, dmq1, iqmp)
    if (key->e != NULL && key->d != NULL && key->p != NULL && key->q != NULL &&
        key->dmp1 != NULL && key->dmq1 != NULL && key->iqmp != NULL) {
      return RSA_KEY_TYPE_FOR_CHECKING_PRIVATE_CRT;
    }

    // (n, d)
    if (key->e == NULL && key->d != NULL && key->p == NULL && key->q == NULL &&
        key->dmp1 == NULL && key->dmq1 == NULL && key->iqmp == NULL) {
      return RSA_KEY_TYPE_FOR_CHECKING_PRIVATE_STRIP;
    }

    return RSA_KEY_TYPE_FOR_CHECKING_INVALID;
}

// Performs certain checks on the given RSA key. The key can be a key pair
// consisting of public and private component, but it can also be only the
// public component. The public component is
//     (n, e),
// the modulus n and the public exponent e. A private key contains at minimum
// the private exponent e in addition to the public part:
//     (n, e, d),
// while normally a private key would consist of
//     (n, e, d, p, q)
// where p and q are the prime factors of n. Some keys store additional
// precomputed private parameters
//     (dmp1, dmq1, iqmp).
// Additionally, we support checking stripped private keys that JCA supports
// that consist of (n, d).
//
// The function performs the following checks (when possible):
//   - n fits in 16k bits,
//   - 1 < log(e, 2) <= 33,
//   - n and e are odd,
//   - n > e,
//   - p * q = n,
//   - (d * e) mod (p - 1) = 1,
//   - (d * e) mod (q - 1) = 1,
//   - dmp1 = d mod (p - 1),
//   - dmq1 = d mod (q - 1),
//   - (q * iqmp) mod p = 1.
//
// Note: see the rsa_key_type_for_checking enum for details on types of keys
// the function can work with.
int RSA_check_key(const RSA *key) {
  SET_DIT_AUTO_RESET;
  GUARD_PTR(key);

  enum rsa_key_type_for_checking key_type = determine_key_type_for_checking(key);
  if (key_type == RSA_KEY_TYPE_FOR_CHECKING_INVALID) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_RSA_PARAMETERS);
    return 0;
  }

  // We check the public component for every key type.
  if (!is_public_component_of_rsa_key_good(key)) {
    return 0;
  }

  // Nothing else to check for public keys (n, e) and private keys in minimal
  // or stripped format, (n, e, d) and (n, d), resp.
  if (key_type == RSA_KEY_TYPE_FOR_CHECKING_PUBLIC ||
      key_type == RSA_KEY_TYPE_FOR_CHECKING_PRIVATE_MIN ||
      key_type == RSA_KEY_TYPE_FOR_CHECKING_PRIVATE_STRIP) {
    return 1;
  }

  // Keys that reach this point are either private keys (n, e, p, q, d),
  // or CRT keys with (dmp1, dmq1, iqmp) values precomputed.
  if (key_type != RSA_KEY_TYPE_FOR_CHECKING_PRIVATE &&
      key_type != RSA_KEY_TYPE_FOR_CHECKING_PRIVATE_CRT) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_RSA_PARAMETERS);
    return 0;
  }

  int ret = 0;

  BN_CTX *ctx = BN_CTX_new();
  if (ctx == NULL) {
    OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
    return 0;
  }

  BIGNUM tmp, de, pm1, qm1, dmp1, dmq1;
  BN_init(&tmp);
  BN_init(&de);
  BN_init(&pm1);
  BN_init(&qm1);
  BN_init(&dmp1);
  BN_init(&dmq1);

  // Check that p * q == n. Before we multiply, we check that p and q are in
  // bounds, to avoid a DoS vector in |bn_mul_consttime| below. Note that
  // n was bound by |is_public_component_of_rsa_key_good|. This also implicitly
  // checks p and q are odd, which is a necessary condition for Montgomery
  // reduction.
  if (BN_is_negative(key->p) ||
      constant_time_declassify_int(BN_cmp(key->p, key->n) >= 0) ||
      BN_is_negative(key->q) ||
      constant_time_declassify_int(BN_cmp(key->q, key->n) >= 0)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_RSA_PARAMETERS);
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
  const unsigned pm1_bits = BN_num_bits(&pm1);
  const unsigned qm1_bits = BN_num_bits(&qm1);
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

  // No more checks for a basic private key without CRT parameters.
  if (key_type == RSA_KEY_TYPE_FOR_CHECKING_PRIVATE) {
    ret = 1;
    goto out;
  }

  // Keys that reach this point are RSA_KEY_TYPE_FOR_CHECKING_PRIVATE_CRT,
  // so check that the CRT params are correct:
  //   - dmp1 == d mod (p - 1),
  //   - dmq1 == d mod (q - 1),
  //   - (iqmp * q) mod (p) == 1.

  if (!bn_div_consttime(NULL, &tmp, key->d, &pm1, pm1_bits, ctx) ||
      !bn_div_consttime(NULL, &de, key->d, &qm1, qm1_bits, ctx)) {
    OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
    goto out;
  }

  // dmp1 == d mod (p - 1) and dmq1 == d mod (q - 1).
  if (BN_cmp(&tmp, key->dmp1) != 0 || BN_cmp(&de, key->dmq1) != 0) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_CRT_VALUES_INCORRECT);
    goto out;
  }

  // Check that iqmp is fully reduced modulo p.
  if (BN_cmp(key->iqmp, key->p) >= 0) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_CRT_VALUES_INCORRECT);
    goto out;
  }

  if (!bn_mul_consttime(&tmp, key->q, key->iqmp, ctx) ||
      // p is odd, so pm1 and p have the same bit width.
      !bn_div_consttime(NULL, &tmp, &tmp, key->p, pm1_bits, ctx)) {
    OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
    goto out;
  }

  // (iqmp * q) mod p = 1.
  if (BN_cmp(&tmp, BN_value_one()) != 0) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_CRT_VALUES_INCORRECT);
    goto out;
  }

  ret = 1;

out:

  BN_free(&tmp);
  BN_free(&de);
  BN_free(&pm1);
  BN_free(&qm1);
  BN_free(&dmp1);
  BN_free(&dmq1);
  BN_CTX_free(ctx);

  return ret;
}

// Performs Pair-Wise Consistency Test (PWCT) with the given RSA key
// by signing and verifying a message. This is required for RSA_check_fips
// function further below. According to our FIPS lab we have to do the test
// with EVP_DigestSign/Verify API.
static int rsa_key_fips_pairwise_consistency_test_signing(RSA *key) {
  int ret = 0;

  uint8_t msg[1] = {0};
  size_t  msg_len = 1;
  uint8_t *sig_der = NULL;
  size_t  sig_len = 0;

  EVP_PKEY *evp_pkey = NULL;
  EVP_MD_CTX md_ctx;
  EVP_MD_CTX_init(&md_ctx);
  const EVP_MD *md = EVP_sha256();

  evp_pkey = EVP_PKEY_new();
  if (!evp_pkey || !EVP_PKEY_set1_RSA(evp_pkey, key)) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
    goto end;
  }

  // Grab the expected signature length.
  if (!EVP_DigestSignInit(&md_ctx, NULL, md, NULL, evp_pkey) ||
      !EVP_DigestSign(&md_ctx, NULL, &sig_len, msg, msg_len)) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
    goto end;
  }

  sig_der = OPENSSL_malloc(sig_len);
  if (!sig_der ||
      !EVP_DigestSign(&md_ctx, sig_der, &sig_len, msg, msg_len)) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
    goto end;
  }
  if (boringssl_fips_break_test("RSA_PWCT")) {
    msg[0] = ~msg[0];
  }
  if (!EVP_DigestVerifyInit(&md_ctx, NULL, md, NULL, evp_pkey) ||
      !EVP_DigestVerify(&md_ctx, sig_der, sig_len, msg, msg_len)) {
    goto end;
  }

  ret = 1;

end:
  EVP_PKEY_free(evp_pkey);
  EVP_MD_CTX_cleanse(&md_ctx);
  OPENSSL_free(sig_der);
  return ret;
}

// This is the product of the 132 smallest odd primes, from 3 to 751,
// as defined in SP 800-89 5.3.3.
static const BN_ULONG kSmallFactorsLimbs[] = {
    TOBN(0xc4309333, 0x3ef4e3e1), TOBN(0x71161eb6, 0xcd2d655f),
    TOBN(0x95e2238c, 0x0bf94862), TOBN(0x3eb233d3, 0x24f7912b),
    TOBN(0x6b55514b, 0xbf26c483), TOBN(0x0a84d817, 0x5a144871),
    TOBN(0x77d12fee, 0x9b82210a), TOBN(0xdb5b93c2, 0x97f050b3),
    TOBN(0x4acad6b9, 0x4d6c026b), TOBN(0xeb7751f3, 0x54aec893),
    TOBN(0xdba53368, 0x36bc85c4), TOBN(0xd85a1b28, 0x7f5ec78e),
    TOBN(0x2eb072d8, 0x6b322244), TOBN(0xbba51112, 0x5e2b3aea),
    TOBN(0x36ed1a6c, 0x0e2486bf), TOBN(0x5f270460, 0xec0c5727),
    0x000017b1
};

DEFINE_LOCAL_DATA(BIGNUM, g_small_factors) {
  out->d = (BN_ULONG *) kSmallFactorsLimbs;
  out->width = OPENSSL_ARRAY_SIZE(kSmallFactorsLimbs);
  out->dmax = out->width;
  out->neg = 0;
  out->flags = BN_FLG_STATIC_DATA;
}

// |RSA_check_fips| function:
//   - validates basic properties of the key (by calling RSA_check_key),
//   - performs partial public key validation (SP 800-89 5.3.3),
//   - performs a pair-wise consistency test, if possible.
// The reason this function offers only key checks that are relevant for
// RSA signatures (SP 800-89) and not RSA key establishment (SP 800-56B) is
// that the AWS-LC FIPS module offers only RSA signing and verification as
// approved FIPS services.
int RSA_check_fips(RSA *key) {
  SET_DIT_AUTO_RESET;
  GUARD_PTR(key);

  enum rsa_key_type_for_checking key_type = determine_key_type_for_checking(key);
  // In addition to invalid key type, stripped private keys can not be checked
  // with this function because they lack the public component which is
  // necessary for both FIPS checks performed here.
  if (key_type == RSA_KEY_TYPE_FOR_CHECKING_INVALID ||
      key_type == RSA_KEY_TYPE_FOR_CHECKING_PRIVATE_STRIP) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_RSA_PARAMETERS);
    return 0;
  }

  // Validate basic properties of the key.
  if (!RSA_check_key(key)) {
    return 0;
  }

  BN_CTX *ctx = BN_CTX_new();
  if (ctx == NULL) {
    return 0;
  }

  BIGNUM small_gcd;
  BN_init(&small_gcd);

  int ret = 0;

  // Perform partial public key validation of RSA keys (SP 800-89 5.3.3).
  // Although this is not for primality testing, SP 800-89 cites an RSA
  // primality testing algorithm, so we use |BN_prime_checks_for_generation| to
  // match. This is only a plausibility test and we expect the value to be
  // composite, so too few iterations will cause us to reject the key, not use
  // an implausible one.
  enum bn_primality_result_t primality_result;
  if (BN_num_bits(key->e) <= 16 ||
      BN_num_bits(key->e) > 256 ||
      !BN_is_odd(key->n) ||
      !BN_is_odd(key->e) ||
      !BN_gcd(&small_gcd, key->n, g_small_factors(), ctx) ||
      !BN_is_one(&small_gcd) ||
      !BN_enhanced_miller_rabin_primality_test(&primality_result, key->n,
                                               BN_prime_checks_for_generation,
                                               ctx, NULL) ||
      primality_result != bn_non_prime_power_composite) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_PUBLIC_KEY_VALIDATION_FAILED);
    goto end;
  }

  // For public keys the check is over because the public key validation is
  // the only thing we can test, we can't perform the pair-wise  consistency
  // test without the private key.
  if (key_type == RSA_KEY_TYPE_FOR_CHECKING_PUBLIC) {
    ret = 1;
    goto end;
  }

  // Only private keys (that contain the public component as well) can be
  // tested with a pair-wise consistency test.
  if (key_type != RSA_KEY_TYPE_FOR_CHECKING_PRIVATE_MIN &&
      key_type != RSA_KEY_TYPE_FOR_CHECKING_PRIVATE &&
      key_type != RSA_KEY_TYPE_FOR_CHECKING_PRIVATE_CRT) {
      goto end;
  }

  // FIPS pair-wise consistency test (FIPS 140-2 4.9.2). Per FIPS 140-2 IG,
  // section 9.9, it is not known whether |rsa| will be used for signing or
  // encryption, so either pair-wise consistency self-test is acceptable. We
  // perform a signing test. The same guidance can be found in FIPS 140-3 IG
  // in Section 7.10.3.3, sub-section Additional comments.
  if (!rsa_key_fips_pairwise_consistency_test_signing(key)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_PUBLIC_KEY_VALIDATION_FAILED);
    goto end;
  }

  ret = 1;

end:
  BN_free(&small_gcd);
  BN_CTX_free(ctx);

  return ret;
}
