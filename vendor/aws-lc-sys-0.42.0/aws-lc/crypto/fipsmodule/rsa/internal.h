// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_RSA_INTERNAL_H
#define OPENSSL_HEADER_RSA_INTERNAL_H

#include <openssl/base.h>

#include <openssl/bn.h>
#include <openssl/rsa.h>


#if defined(__cplusplus)
extern "C" {
#endif

typedef struct bn_blinding_st BN_BLINDING;

struct rsa_meth_st {
    void *app_data;

    int (*init)(RSA *rsa);
    int (*finish)(RSA *rsa);

    // size returns the size of the RSA modulus in bytes.
    size_t (*size)(const RSA *rsa);

    // Set via |RSA_meth_set_sign|. The default behavior for |sign| is
    // implemented in |RSA_sign|. If custom functionality is provided, |sign|
    // will be invoked within |RSA_sign|.
    int (*sign)(int type, const uint8_t *m, unsigned int m_length,
                uint8_t *sigret, unsigned int *siglen, const RSA *rsa);

    // Set via |RSA_meth_set_priv_enc|. |sign_raw| is equivalent to the
    // |priv_enc| field of OpenSSL's |RSA_METHOD| struct. The default behavior
    // for |sign_raw| is implemented in |RSA_sign_raw|. If custom
    // functionality is provided, |sign_raw| will be invoked within
    // |RSA_sign_raw|.
    int (*sign_raw)(int max_out, const uint8_t *in, uint8_t *out, RSA *rsa,
                    int padding);

    // Set via |RSA_meth_set_pub_dec|. |verify_raw| is equivalent to the
    // |pub_dec| field of OpenSSL's |RSA_METHOD| struct. The default behavior
    // for |verify_raw| is implemented in |RSA_verify_raw|. If custom
    // functionality is provided, |verify_raw| will be invoked within
    // |RSA_verify_raw|.
    int (*verify_raw)(int max_out, const uint8_t *in, uint8_t *out, RSA *rsa,
                      int padding);

    // Set via |RSA_meth_set_priv_dec|. |decrypt| is equivalent to the
    // |priv_dec| field of OpenSSL's |RSA_METHOD| struct. The default behavior
    // for |decrypt| is implemented in |RSA_decrypt|. If custom
    // functionality is provided, |decrypt| will be invoked within
    // |RSA_decrypt|.
    int (*decrypt)(int max_out, const uint8_t *in, uint8_t *out, RSA *rsa,
                   int padding);

    // Set via |RSA_meth_set_pub_enc|. |encrypt| is equivalent to the
    // |pub_enc| field of OpenSSL's |RSA_METHOD| struct. The default behavior
    // for |encrypt| is implemented in |RSA_encrypt|. If custom
    // functionality is provided, |encrypt| will be invoked within
    // |RSA_encrypt|.
    int (*encrypt)(int max_out, const uint8_t *in, uint8_t *out, RSA *rsa,
                   int padding);

    // private_transform takes a big-endian integer from |in|, calculates the
    // d'th power of it, modulo the RSA modulus and writes the result as a
    // big-endian integer to |out|. Both |in| and |out| are |len| bytes long and
    // |len| is always equal to |RSA_size(rsa)|. If the result of the transform
    // can be represented in fewer than |len| bytes, then |out| must be zero
    // padded on the left.
    //
    // It returns one on success and zero otherwise.
    //
    // RSA decrypt and sign operations will call this, thus an ENGINE might wish
    // to override it in order to avoid having to implement the padding
    // functionality demanded by those, higher level, operations.
    int (*private_transform)(RSA *rsa, uint8_t *out, const uint8_t *in,
                             size_t len);

    int flags;
};

struct rsa_st {
  const RSA_METHOD *meth;

  BIGNUM *n;
  BIGNUM *e;
  BIGNUM *d;
  BIGNUM *p;
  BIGNUM *q;
  BIGNUM *dmp1;
  BIGNUM *dmq1;
  BIGNUM *iqmp;

  // If a PSS only key this contains the parameter restrictions.
  RSASSA_PSS_PARAMS *pss;

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

size_t rsa_default_size(const RSA *rsa);
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

int RSA_padding_add_PKCS1_type_1(uint8_t *to, size_t to_len,
                                 const uint8_t *from, size_t from_len);
int RSA_padding_check_PKCS1_type_1(uint8_t *out, size_t *out_len,
                                   size_t max_out, const uint8_t *from,
                                   size_t from_len);
int RSA_padding_add_none(uint8_t *to, size_t to_len, const uint8_t *from,
                         size_t from_len);

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

// rsa_digestsign_no_self_test calculates the digest and calls
// |rsa_sign_no_self_test|, which doesn't try to run the self-test first. This
// is for use in the self tests themselves, to prevent an infinite loop.
int rsa_digestsign_no_self_test(const EVP_MD *md, const uint8_t *input,
                                size_t in_len, uint8_t *out, unsigned *out_len,
                                RSA *rsa);

// rsa_digestverify_no_self_test calculates the digest and calls
// |rsa_verify_no_self_test|, which doesn't try to run the self-test first. This
// is for use in the self tests themselves, to prevent an infinite loop.
int rsa_digestverify_no_self_test(const EVP_MD *md, const uint8_t *input,
                                  size_t in_len, const uint8_t *sig,
                                  size_t sig_len, RSA *rsa);

// Performs several checks on the public component of the given RSA key.
// See the implemetation in |rsa.c| for details.
int is_public_component_of_rsa_key_good(const RSA *key);

OPENSSL_EXPORT const RSASSA_PSS_PARAMS *RSA_get0_ssa_pss_params(const RSA *rsa);

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_RSA_INTERNAL_H
