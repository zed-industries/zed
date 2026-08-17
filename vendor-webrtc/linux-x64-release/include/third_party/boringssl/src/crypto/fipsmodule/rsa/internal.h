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

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_RSA_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_RSA_INTERNAL_H

#include <openssl/base.h>

#include <openssl/bn.h>
#include <openssl/rsa.h>

#include "../../internal.h"

#if defined(__cplusplus)
extern "C" {
#endif


typedef struct bn_blinding_st BN_BLINDING;

struct rsa_st {
  RSA_METHOD *meth;

  BIGNUM *n;
  BIGNUM *e;
  BIGNUM *d;
  BIGNUM *p;
  BIGNUM *q;
  BIGNUM *dmp1;
  BIGNUM *dmq1;
  BIGNUM *iqmp;

  // be careful using this if the RSA structure is shared
  CRYPTO_EX_DATA ex_data;
  CRYPTO_refcount_t references;
  int flags;

  CRYPTO_MUTEX lock;

  // Used to cache montgomery values. The creation of these values is protected
  // by |lock|.
  BN_MONT_CTX *mont_n;
  BN_MONT_CTX *mont_p;
  BN_MONT_CTX *mont_q;

  // The following fields are copies of |d|, |dmp1|, and |dmq1|, respectively,
  // but with the correct widths to prevent side channels. These must use
  // separate copies due to threading concerns caused by OpenSSL's API
  // mistakes. See https://github.com/openssl/openssl/issues/5158 and
  // the |freeze_private_key| implementation.
  BIGNUM *d_fixed, *dmp1_fixed, *dmq1_fixed;

  // iqmp_mont is q^-1 mod p in Montgomery form, using |mont_p|.
  BIGNUM *iqmp_mont;

  // num_blindings contains the size of the |blindings| and |blindings_inuse|
  // arrays. This member and the |blindings_inuse| array are protected by
  // |lock|.
  size_t num_blindings;
  // blindings is an array of BN_BLINDING structures that can be reserved by a
  // thread by locking |lock| and changing the corresponding element in
  // |blindings_inuse| from 0 to 1.
  BN_BLINDING **blindings;
  unsigned char *blindings_inuse;
  uint64_t blinding_fork_generation;

  // private_key_frozen is one if the key has been used for a private key
  // operation and may no longer be mutated.
  unsigned private_key_frozen:1;
};


#define RSA_PKCS1_PADDING_SIZE 11

// Default implementations of RSA operations.

const RSA_METHOD *RSA_default_method(void);

int rsa_default_sign_raw(RSA *rsa, size_t *out_len, uint8_t *out,
                         size_t max_out, const uint8_t *in, size_t in_len,
                         int padding);
int rsa_default_private_transform(RSA *rsa, uint8_t *out, const uint8_t *in,
                                  size_t len);


BN_BLINDING *BN_BLINDING_new(void);
void BN_BLINDING_free(BN_BLINDING *b);
void BN_BLINDING_invalidate(BN_BLINDING *b);
int BN_BLINDING_convert(BIGNUM *n, BN_BLINDING *b, const BIGNUM *e,
                        const BN_MONT_CTX *mont_ctx, BN_CTX *ctx);
int BN_BLINDING_invert(BIGNUM *n, const BN_BLINDING *b, BN_MONT_CTX *mont_ctx,
                       BN_CTX *ctx);


int PKCS1_MGF1(uint8_t *out, size_t len, const uint8_t *seed, size_t seed_len,
               const EVP_MD *md);
int RSA_padding_add_PKCS1_type_1(uint8_t *to, size_t to_len,
                                 const uint8_t *from, size_t from_len);
int RSA_padding_check_PKCS1_type_1(uint8_t *out, size_t *out_len,
                                   size_t max_out, const uint8_t *from,
                                   size_t from_len);
int RSA_padding_add_none(uint8_t *to, size_t to_len, const uint8_t *from,
                         size_t from_len);

// rsa_check_public_key checks that |rsa|'s public modulus and exponent are
// within DoS bounds.
int rsa_check_public_key(const RSA *rsa);

// rsa_private_transform_no_self_test calls either the method-specific
// |private_transform| function (if given) or the generic one. See the comment
// for |private_transform| in |rsa_meth_st|.
int rsa_private_transform_no_self_test(RSA *rsa, uint8_t *out,
                                       const uint8_t *in, size_t len);

// rsa_private_transform acts the same as |rsa_private_transform_no_self_test|
// but, in FIPS mode, performs an RSA self test before calling the default RSA
// implementation.
int rsa_private_transform(RSA *rsa, uint8_t *out, const uint8_t *in,
                          size_t len);

// rsa_invalidate_key is called after |rsa| has been mutated, to invalidate
// fields derived from the original structure. This function assumes exclusive
// access to |rsa|. In particular, no other thread may be concurrently signing,
// etc., with |rsa|.
void rsa_invalidate_key(RSA *rsa);


// This constant is exported for test purposes.
extern const BN_ULONG kBoringSSLRSASqrtTwo[];
extern const size_t kBoringSSLRSASqrtTwoLen;


// Functions that avoid self-tests.
//
// Self-tests need to call functions that don't try and ensure that the
// self-tests have passed. These functions, in turn, need to limit themselves
// to such functions too.
//
// These functions are the same as their public versions, but skip the self-test
// check.

int rsa_verify_no_self_test(int hash_nid, const uint8_t *digest,
                            size_t digest_len, const uint8_t *sig,
                            size_t sig_len, RSA *rsa);

int rsa_verify_raw_no_self_test(RSA *rsa, size_t *out_len, uint8_t *out,
                                size_t max_out, const uint8_t *in,
                                size_t in_len, int padding);

int rsa_sign_no_self_test(int hash_nid, const uint8_t *digest,
                          size_t digest_len, uint8_t *out, unsigned *out_len,
                          RSA *rsa);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_RSA_INTERNAL_H
