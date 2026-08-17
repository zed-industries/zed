// Copyright (c) 2021, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_ECDSA_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_ECDSA_INTERNAL_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif


// ecdsa_sign_with_nonce_for_known_answer_test behaves like |ECDSA_do_sign| but
// takes a fixed nonce. This function is used as part of known-answer tests in
// the FIPS module.
ECDSA_SIG *ecdsa_sign_with_nonce_for_known_answer_test(const uint8_t *digest,
                                                       size_t digest_len,
                                                       const EC_KEY *eckey,
                                                       const uint8_t *nonce,
                                                       size_t nonce_len);

// ecdsa_do_verify_no_self_test does the same as |ECDSA_do_verify|, but doesn't
// try to run the self-test first. This is for use in the self tests themselves,
// to prevent an infinite loop.
int ecdsa_do_verify_no_self_test(const uint8_t *digest, size_t digest_len,
                                 const ECDSA_SIG *sig, const EC_KEY *eckey);

// ecdsa_digestsign_no_self_test calculates the digest and calls
// |ecdsa_sign_with_nonce_for_known_answer_test|, which doesn't try to run the
// self-test first. This is for use in the self tests themselves, to prevent
// an infinite loop.
ECDSA_SIG *ecdsa_digestsign_no_self_test(const EVP_MD *md, const uint8_t *input,
                                         size_t in_len, const EC_KEY *eckey,
                                         const uint8_t *nonce,
                                         size_t nonce_len);

// ecdsa_digestverify_no_self_test calculates the digest and calls
// |ecdsa_do_verify_no_self_test|, which doesn't try to run the self-test
// first. This is for use in the self tests themselves, to prevent an infinite
// loop.
int ecdsa_digestverify_no_self_test(const EVP_MD *md, const uint8_t *input,
                                    size_t in_len, const ECDSA_SIG *sig,
                                    const EC_KEY *eckey);


#if defined(__cplusplus)
}
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_ECDSA_INTERNAL_H
