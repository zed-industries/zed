// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef ML_DSA_H
#define ML_DSA_H

#include <stddef.h>
#include <stdint.h>
#include <openssl/base.h>
#include <openssl/evp.h>

#define MLDSA44_PUBLIC_KEY_BYTES 1312
#define MLDSA44_PRIVATE_KEY_BYTES 2560
#define MLDSA44_SIGNATURE_BYTES 2420
#define MLDSA44_KEYGEN_SEED_BYTES 32
#define MLDSA44_SIGNATURE_SEED_BYTES 32

#define MLDSA65_PUBLIC_KEY_BYTES 1952
#define MLDSA65_PRIVATE_KEY_BYTES 4032
#define MLDSA65_SIGNATURE_BYTES 3309
#define MLDSA65_KEYGEN_SEED_BYTES 32
#define MLDSA65_SIGNATURE_SEED_BYTES 32

#define MLDSA87_PUBLIC_KEY_BYTES 2592
#define MLDSA87_PRIVATE_KEY_BYTES 4896
#define MLDSA87_SIGNATURE_BYTES 4627
#define MLDSA87_KEYGEN_SEED_BYTES 32
#define MLDSA87_SIGNATURE_SEED_BYTES 32

#if defined(__cplusplus)
extern "C" {
#endif
OPENSSL_EXPORT int ml_dsa_44_keypair(uint8_t *public_key,
                                     uint8_t *secret_key,
                                     uint8_t *seed);

OPENSSL_EXPORT int ml_dsa_44_pack_pk_from_sk(uint8_t *public_key,
                                             const uint8_t *private_key);

int ml_dsa_44_keypair_internal_no_self_test(uint8_t *public_key,
                                            uint8_t *private_key,
                                            const uint8_t *seed);

OPENSSL_EXPORT int ml_dsa_44_keypair_internal(uint8_t *public_key,
                                              uint8_t *private_key,
                                              const uint8_t *seed);

OPENSSL_EXPORT int ml_dsa_44_sign(const uint8_t *private_key,
                                  uint8_t *sig, size_t *sig_len,
                                  const uint8_t *message, size_t message_len,
                                  const uint8_t *ctx_string, size_t ctx_string_len);

OPENSSL_EXPORT int ml_dsa_extmu_44_sign(const uint8_t *private_key,
                                        uint8_t *sig, size_t *sig_len,
                                        const uint8_t *mu, size_t mu_len);

OPENSSL_EXPORT int ml_dsa_44_sign_internal(const uint8_t *private_key,
                                           uint8_t *sig, size_t *sig_len,
                                           const uint8_t *message, size_t message_len,
                                           const uint8_t *pre, size_t pre_len,
                                           const uint8_t *rnd);

int ml_dsa_44_sign_internal_no_self_test(const uint8_t *private_key,
                                         uint8_t *sig, size_t *sig_len,
                                         const uint8_t *message, size_t message_len,
                                         const uint8_t *pre, size_t pre_len,
                                         const uint8_t *rnd);

OPENSSL_EXPORT int ml_dsa_extmu_44_sign_internal(const uint8_t *private_key,
                                                 uint8_t *sig, size_t *sig_len,
                                                 const uint8_t *mu, size_t mu_len,
                                                 const uint8_t *pre, size_t pre_len,
                                                 const uint8_t *rnd);

OPENSSL_EXPORT int ml_dsa_44_verify(const uint8_t *public_key,
                                    const uint8_t *sig, size_t sig_len,
                                    const uint8_t *message, size_t message_len,
                                    const uint8_t *ctx_string, size_t ctx_string_len);

OPENSSL_EXPORT int ml_dsa_extmu_44_verify(const uint8_t *public_key,
                                          const uint8_t *sig, size_t sig_len,
                                          const uint8_t *mu, size_t mu_len);

OPENSSL_EXPORT int ml_dsa_44_verify_internal(const uint8_t *public_key,
                                             const uint8_t *sig, size_t sig_len,
                                             const uint8_t *message, size_t message_len,
                                             const uint8_t *pre, size_t pre_len);

int ml_dsa_44_verify_internal_no_self_test(const uint8_t *public_key,
                                           const uint8_t *sig, size_t sig_len,
                                           const uint8_t *message, size_t message_len,
                                           const uint8_t *pre, size_t pre_len);

OPENSSL_EXPORT int ml_dsa_extmu_44_verify_internal(const uint8_t *public_key,
                                                   const uint8_t *sig, size_t sig_len,
                                                   const uint8_t *mu, size_t mu_len,
                                                   const uint8_t *pre, size_t pre_len);

OPENSSL_EXPORT int ml_dsa_65_keypair(uint8_t *public_key,
                                     uint8_t *secret_key,
                                     uint8_t *seed);

OPENSSL_EXPORT int ml_dsa_65_pack_pk_from_sk(uint8_t *public_key,
                                             const uint8_t *private_key);

OPENSSL_EXPORT int ml_dsa_65_keypair_internal(uint8_t *public_key,
                                              uint8_t *private_key,
                                              const uint8_t *seed);

OPENSSL_EXPORT int ml_dsa_65_sign(const uint8_t *private_key,
                                  uint8_t *sig, size_t *sig_len,
                                  const uint8_t *message, size_t message_len,
                                  const uint8_t *ctx_string, size_t ctx_string_len);

OPENSSL_EXPORT int ml_dsa_extmu_65_sign(const uint8_t *private_key,
                                        uint8_t *sig, size_t *sig_len,
                                        const uint8_t *mu, size_t mu_len);

OPENSSL_EXPORT int ml_dsa_65_sign_internal(const uint8_t *private_key,
                                           uint8_t *sig, size_t *sig_len,
                                           const uint8_t *message, size_t message_len,
                                           const uint8_t *pre, size_t pre_len,
                                           const uint8_t *rnd);

OPENSSL_EXPORT int ml_dsa_extmu_65_sign_internal(const uint8_t *private_key,
                                                 uint8_t *sig, size_t *sig_len,
                                                 const uint8_t *mu, size_t mu_len,
                                                 const uint8_t *pre, size_t pre_len,
                                                 const uint8_t *rnd);

OPENSSL_EXPORT int ml_dsa_65_verify(const uint8_t *public_key,
                                    const uint8_t *sig, size_t sig_len,
                                    const uint8_t *message, size_t message_len,
                                    const uint8_t *ctx_string, size_t ctx_string_len);

OPENSSL_EXPORT int ml_dsa_extmu_65_verify(const uint8_t *public_key,
                                          const uint8_t *sig, size_t sig_len,
                                          const uint8_t *mu, size_t mu_len);

OPENSSL_EXPORT int ml_dsa_65_verify_internal(const uint8_t *public_key,
                                             const uint8_t *sig, size_t sig_len,
                                             const uint8_t *message, size_t message_len,
                                             const uint8_t *pre, size_t pre_len);

OPENSSL_EXPORT int ml_dsa_extmu_65_verify_internal(const uint8_t *public_key,
                                                   const uint8_t *sig, size_t sig_len,
                                                   const uint8_t *mu, size_t mu_len,
                                                   const uint8_t *pre, size_t pre_len);

OPENSSL_EXPORT int ml_dsa_87_keypair(uint8_t *public_key,
                                     uint8_t *secret_key,
                                     uint8_t *seed);

OPENSSL_EXPORT int ml_dsa_87_pack_pk_from_sk(uint8_t *public_key,
                                             const uint8_t *private_key);

OPENSSL_EXPORT int ml_dsa_87_keypair_internal(uint8_t *public_key,
                                              uint8_t *private_key,
                                              const uint8_t *seed);

OPENSSL_EXPORT int ml_dsa_87_sign(const uint8_t *private_key,
                                  uint8_t *sig, size_t *sig_len,
                                  const uint8_t *message, size_t message_len,
                                  const uint8_t *ctx_string, size_t ctx_string_len);

OPENSSL_EXPORT int ml_dsa_extmu_87_sign(const uint8_t *private_key,
                                        uint8_t *sig, size_t *sig_len,
                                        const uint8_t *mu, size_t mu_len);

OPENSSL_EXPORT int ml_dsa_87_sign_internal(const uint8_t *private_key,
                                           uint8_t *sig, size_t *sig_len,
                                           const uint8_t *message, size_t message_len,
                                           const uint8_t *pre, size_t pre_len,
                                           const uint8_t *rnd);

OPENSSL_EXPORT int ml_dsa_extmu_87_sign_internal(const uint8_t *private_key,
                                                 uint8_t *sig, size_t *sig_len,
                                                 const uint8_t *mu, size_t mu_len,
                                                 const uint8_t *pre, size_t pre_len,
                                                 const uint8_t *rnd);

OPENSSL_EXPORT int ml_dsa_87_verify(const uint8_t *public_key,
                                    const uint8_t *sig, size_t sig_len,
                                    const uint8_t *message, size_t message_len,
                                    const uint8_t *ctx_string, size_t ctx_string_len);

OPENSSL_EXPORT int ml_dsa_extmu_87_verify(const uint8_t *public_key,
                                          const uint8_t *sig, size_t sig_len,
                                          const uint8_t *mu, size_t mu_len);

OPENSSL_EXPORT int ml_dsa_87_verify_internal(const uint8_t *public_key,
                                             const uint8_t *sig, size_t sig_len,
                                             const uint8_t *message, size_t message_len,
                                             const uint8_t *pre, size_t pre_len);

OPENSSL_EXPORT int ml_dsa_extmu_87_verify_internal(const uint8_t *public_key,
                                                   const uint8_t *sig, size_t sig_len,
                                                   const uint8_t *mu, size_t mu_len,
                                                   const uint8_t *pre, size_t pre_len);
#if defined(__cplusplus)
}
#endif

#endif
