// Copyright (c) 2020, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_DSA_INTERNAL_H
#define OPENSSL_HEADER_DSA_INTERNAL_H

#include <openssl/dsa.h>

#include <openssl/thread.h>

#if defined(__cplusplus)
extern "C" {
#endif


struct dsa_st {
  BIGNUM *p;
  BIGNUM *q;
  BIGNUM *g;

  BIGNUM *pub_key;
  BIGNUM *priv_key;

  // Normally used to cache montgomery values
  CRYPTO_MUTEX method_mont_lock;
  BN_MONT_CTX *method_mont_p;
  BN_MONT_CTX *method_mont_q;
  CRYPTO_refcount_t references;
  CRYPTO_EX_DATA ex_data;
};

// dsa_check_key performs cheap self-checks on |dsa|, and ensures it is within
// DoS bounds. It returns one on success and zero on error.
int dsa_check_key(const DSA *dsa);

int dsa_internal_paramgen(DSA *dsa, size_t bits, const EVP_MD *evpmd,
                          const unsigned char *seed_in, size_t seed_len,
                          int *out_counter, unsigned long *out_h, BN_GENCB *cb);

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_DSA_INTERNAL_H
