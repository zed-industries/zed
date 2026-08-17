// Copyright (c) 2022, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_DH_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_DH_INTERNAL_H

#include <openssl/base.h>

#include <openssl/thread.h>

#if defined(__cplusplus)
extern "C" {
#endif


#define OPENSSL_DH_MAX_MODULUS_BITS 10000

struct dh_st {
  BIGNUM *p;
  BIGNUM *g;
  BIGNUM *q;
  BIGNUM *pub_key;   // g^x mod p
  BIGNUM *priv_key;  // x

  // priv_length contains the length, in bits, of the private value. If zero,
  // the private value will be the same length as |p|.
  unsigned priv_length;

  CRYPTO_MUTEX method_mont_p_lock;
  BN_MONT_CTX *method_mont_p;

  int flags;
  CRYPTO_refcount_t references;
};

// dh_check_params_fast checks basic invariants on |dh|'s domain parameters. It
// does not check that |dh| forms a valid group, only that the sizes are within
// DoS bounds.
int dh_check_params_fast(const DH *dh);

// dh_compute_key_padded_no_self_test does the same as |DH_compute_key_padded|,
// but doesn't try to run the self-test first. This is for use in the self tests
// themselves, to prevent an infinite loop.
int dh_compute_key_padded_no_self_test(unsigned char *out,
                                       const BIGNUM *peers_key, DH *dh);

// DH_get_rfc7919_3072 returns the group `ffdhe3072` from
// https://tools.ietf.org/html/rfc7919#appendix-A.2. It returns NULL if out
// of memory.
OPENSSL_EXPORT DH *DH_get_rfc7919_3072(void);

// DH_get_rfc7919_8192 returns the group `ffdhe8192` from
// https://tools.ietf.org/html/rfc7919#appendix-A.4. It returns NULL if out
// of memory.
OPENSSL_EXPORT DH *DH_get_rfc7919_8192(void);

#if defined(__cplusplus)
}
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_DH_INTERNAL_H
