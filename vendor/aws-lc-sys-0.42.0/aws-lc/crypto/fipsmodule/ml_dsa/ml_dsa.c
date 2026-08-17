// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// mldsa-native source code

// Include level-independent code
#define MLD_CONFIG_FILE "../mldsa_native_config.h"
#define MLD_CONFIG_MONOBUILD_KEEP_SHARED_HEADERS

// MLDSA-44
#define MLD_CONFIG_PARAMETER_SET 44
#define MLD_CONFIG_MULTILEVEL_WITH_SHARED  // Include level-independent code
#include "mldsa/mldsa_native_bcm.c"
// MLDSA-65
#undef MLD_CONFIG_PARAMETER_SET
#define MLD_CONFIG_PARAMETER_SET 65
#define MLD_CONFIG_MULTILEVEL_NO_SHARED  // Exclude level-independent code
#include "mldsa/mldsa_native_bcm.c"
// MLDSA-87
#undef MLD_CONFIG_MONOBUILD_KEEP_SHARED_HEADERS
#undef MLD_CONFIG_PARAMETER_SET
#define MLD_CONFIG_PARAMETER_SET 87
#include "mldsa/mldsa_native_bcm.c"

// End of mldsa-native source code

#include "./ml_dsa.h"
#include "../../evp_extra/internal.h"
#include "../evp/internal.h"
#include "../service_indicator/internal.h"

// Note: These methods provide AWS-LC-specific wrappers around mldsa-native.

int ml_dsa_44_keypair_internal(uint8_t *public_key   /* OUT */,
                               uint8_t *private_key  /* OUT */,
                               const uint8_t *seed   /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  return ml_dsa_44_keypair_internal_no_self_test(public_key, private_key, seed);
}

int ml_dsa_44_keypair_internal_no_self_test(uint8_t *public_key   /* OUT */,
                                            uint8_t *private_key  /* OUT */,
                                            const uint8_t *seed   /* IN */) {
  int ret = mldsa44_keypair_internal(public_key, private_key, seed);
#if defined(AWSLC_FIPS)
  /* PCT failure is the only failure condition for key generation. */
  if (ret != 0) {
    AWS_LC_FIPS_failure("ML-DSA keygen PCT failed");
  }
#endif
  return (ret == 0) ? 1 : 0;  // Convert: mldsa 0=success -> AWS-LC 1=success
}

int ml_dsa_44_keypair(uint8_t *public_key   /* OUT */,
                      uint8_t *private_key  /* OUT */,
                      uint8_t *seed         /* OUT */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();

  // Generate seed
  AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(seed, MLDSA44_KEYGEN_SEED_BYTES));

  int ret = mldsa44_keypair_internal(public_key, private_key, seed);
#if defined(AWSLC_FIPS)
  /* PCT failure is the only failure condition for key generation. */
  if (ret != 0) {
    AWS_LC_FIPS_failure("ML-DSA keygen PCT failed");
  }
#endif

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_44_pack_pk_from_sk(uint8_t *public_key          /* OUT */,
                              const uint8_t *private_key   /* IN  */) {
  int ret = mldsa44_pk_from_sk(public_key, private_key);
  return (ret == 0) ? 1 : 0;  // Convert: mldsa 0=success, -1=failure -> AWS-LC 1=success, 0=failure
}

int ml_dsa_44_sign(const uint8_t *private_key /* IN */,
                   uint8_t *sig               /* OUT */,
                   size_t *sig_len            /* OUT */,
                   const uint8_t *message     /* IN */,
                   size_t message_len         /* IN */,
                   const uint8_t *ctx_string  /* IN */,
                   size_t ctx_string_len      /* IN */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();

  int ret = mldsa44_signature(sig, sig_len, message, message_len,
                               ctx_string, ctx_string_len, private_key);

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_extmu_44_sign(const uint8_t *private_key /* IN */,
                         uint8_t *sig               /* OUT */,
                         size_t *sig_len            /* OUT */,
                         const uint8_t *mu          /* IN */,
                         size_t mu_len              /* IN */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();

  // mu_len is ignored - extmu always uses MLDSA_CRHBYTES (64 bytes)
  (void)mu_len;
  int ret = mldsa44_signature_extmu(sig, sig_len, mu, private_key);

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_44_sign_internal(const uint8_t *private_key  /* IN */,
                            uint8_t *sig                /* OUT */,
                            size_t *sig_len             /* OUT */,
                            const uint8_t *message      /* IN */,
                            size_t message_len          /* IN */,
                            const uint8_t *pre          /* IN */,
                            size_t pre_len              /* IN */,
                            const uint8_t *rnd          /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  return ml_dsa_44_sign_internal_no_self_test(private_key, sig, sig_len, message,
                                              message_len, pre, pre_len, rnd);
}

int ml_dsa_44_sign_internal_no_self_test(const uint8_t *private_key  /* IN */,
                                         uint8_t *sig                /* OUT */,
                                         size_t *sig_len             /* OUT */,
                                         const uint8_t *message      /* IN */,
                                         size_t message_len          /* IN */,
                                         const uint8_t *pre          /* IN */,
                                         size_t pre_len              /* IN */,
                                         const uint8_t *rnd          /* IN */) {
  int ret = mldsa44_signature_internal(sig, sig_len, message, message_len,
                                        pre, pre_len, rnd, private_key, 0);
  return (ret == 0) ? 1 : 0;
}

int ml_dsa_extmu_44_sign_internal(const uint8_t *private_key  /* IN */,
                                  uint8_t *sig                /* OUT */,
                                  size_t *sig_len             /* OUT */,
                                  const uint8_t *mu           /* IN */,
                                  size_t mu_len               /* IN */,
                                  const uint8_t *pre          /* IN */,
                                  size_t pre_len              /* IN */,
                                  const uint8_t *rnd          /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  int ret = mldsa44_signature_internal(sig, sig_len, mu, mu_len,
                                        pre, pre_len, rnd, private_key, 1);
  return (ret == 0) ? 1 : 0;
}

int ml_dsa_44_verify(const uint8_t *public_key /* IN */,
                     const uint8_t *sig        /* IN */,
                     size_t sig_len            /* IN */,
                     const uint8_t *message    /* IN */,
                     size_t message_len        /* IN */,
                     const uint8_t *ctx_string /* IN */,
                     size_t ctx_string_len     /* IN */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();

  int ret = mldsa44_verify(sig, sig_len, message, message_len,
                            ctx_string, ctx_string_len, public_key);

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_extmu_44_verify(const uint8_t *public_key /* IN */,
                           const uint8_t *sig        /* IN */,
                           size_t sig_len            /* IN */,
                           const uint8_t *mu         /* IN */,
                           size_t mu_len             /* IN */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();

  // mu_len is ignored - extmu always uses MLDSA_CRHBYTES (64 bytes)
  (void)mu_len;
  int ret = mldsa44_verify_extmu(sig, sig_len, mu, public_key);

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_44_verify_internal(const uint8_t *public_key /* IN */,
                              const uint8_t *sig        /* IN */,
                              size_t sig_len            /* IN */,
                              const uint8_t *message    /* IN */,
                              size_t message_len        /* IN */,
                              const uint8_t *pre        /* IN */,
                              size_t pre_len            /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  return ml_dsa_44_verify_internal_no_self_test(public_key, sig, sig_len, message,
                                                message_len, pre, pre_len);
}

int ml_dsa_44_verify_internal_no_self_test(const uint8_t *public_key /* IN */,
                                           const uint8_t *sig        /* IN */,
                                           size_t sig_len            /* IN */,
                                           const uint8_t *message    /* IN */,
                                           size_t message_len        /* IN */,
                                           const uint8_t *pre        /* IN */,
                                           size_t pre_len            /* IN */) {
  int ret = mldsa44_verify_internal(sig, sig_len, message, message_len,
                                     pre, pre_len, public_key, 0);
  return (ret == 0) ? 1 : 0;
}

int ml_dsa_extmu_44_verify_internal(const uint8_t *public_key /* IN */,
                                    const uint8_t *sig        /* IN */,
                                    size_t sig_len            /* IN */,
                                    const uint8_t *mu           /* IN */,
                                    size_t mu_len               /* IN */,
                                    const uint8_t *pre        /* IN */,
                                    size_t pre_len            /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  int ret = mldsa44_verify_internal(sig, sig_len, mu, mu_len,
                                     pre, pre_len, public_key, 1);
  return (ret == 0) ? 1 : 0;
}

// ML-DSA-65 implementations
int ml_dsa_65_keypair(uint8_t *public_key   /* OUT */,
                      uint8_t *private_key  /* OUT */,
                      uint8_t *seed         /* OUT */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();

  AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(seed, MLDSA65_KEYGEN_SEED_BYTES));

  int ret = mldsa65_keypair_internal(public_key, private_key, seed);
#if defined(AWSLC_FIPS)
  /* PCT failure is the only failure condition for key generation. */
  if (ret != 0) {
    AWS_LC_FIPS_failure("ML-DSA keygen PCT failed");
  }
#endif

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_65_pack_pk_from_sk(uint8_t *public_key          /* OUT */,
                              const uint8_t *private_key   /* IN  */) {
  int ret = mldsa65_pk_from_sk(public_key, private_key);
  return (ret == 0) ? 1 : 0;  // Convert: mldsa 0=success, -1=failure -> AWS-LC 1=success, 0=failure
}

int ml_dsa_65_keypair_internal(uint8_t *public_key   /* OUT */,
                               uint8_t *private_key  /* OUT */,
                               const uint8_t *seed   /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  int ret = mldsa65_keypair_internal(public_key, private_key, seed);
#if defined(AWSLC_FIPS)
  /* PCT failure is the only failure condition for key generation. */
  if (ret != 0) {
    AWS_LC_FIPS_failure("ML-DSA keygen PCT failed");
  }
#endif
  return (ret == 0) ? 1 : 0;  // Convert: mldsa 0=success -> AWS-LC 1=success
}

int ml_dsa_65_sign(const uint8_t *private_key /* IN */,
                   uint8_t *sig               /* OUT */,
                   size_t *sig_len            /* OUT */,
                   const uint8_t *message     /* IN */,
                   size_t message_len         /* IN */,
                   const uint8_t *ctx_string  /* IN */,
                   size_t ctx_string_len      /* IN */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();

  int ret = mldsa65_signature(sig, sig_len, message, message_len,
                               ctx_string, ctx_string_len, private_key);

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_extmu_65_sign(const uint8_t *private_key /* IN */,
                         uint8_t *sig               /* OUT */,
                         size_t *sig_len            /* OUT */,
                         const uint8_t *mu          /* IN */,
                         size_t mu_len              /* IN */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();

  // mu_len is ignored - extmu always uses MLDSA_CRHBYTES (64 bytes)
  (void)mu_len;
  int ret = mldsa65_signature_extmu(sig, sig_len, mu, private_key);

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_65_sign_internal(const uint8_t *private_key  /* IN */,
                            uint8_t *sig                /* OUT */,
                            size_t *sig_len             /* OUT */,
                            const uint8_t *message      /* IN */,
                            size_t message_len          /* IN */,
                            const uint8_t *pre          /* IN */,
                            size_t pre_len              /* IN */,
                            const uint8_t *rnd          /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  int ret = mldsa65_signature_internal(sig, sig_len, message, message_len,
                                        pre, pre_len, rnd, private_key, 0);
  return (ret == 0) ? 1 : 0;
}

int ml_dsa_extmu_65_sign_internal(const uint8_t *private_key  /* IN */,
                                  uint8_t *sig                /* OUT */,
                                  size_t *sig_len             /* OUT */,
                                  const uint8_t *mu           /* IN */,
                                  size_t mu_len               /* IN */,
                                  const uint8_t *pre          /* IN */,
                                  size_t pre_len              /* IN */,
                                  const uint8_t *rnd          /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  int ret = mldsa65_signature_internal(sig, sig_len, mu, mu_len,
                                        pre, pre_len, rnd, private_key, 1);
  return (ret == 0) ? 1 : 0;
}

int ml_dsa_65_verify(const uint8_t *public_key /* IN */,
                     const uint8_t *sig        /* IN */,
                     size_t sig_len            /* IN */,
                     const uint8_t *message    /* IN */,
                     size_t message_len        /* IN */,
                     const uint8_t *ctx_string /* IN */,
                     size_t ctx_string_len     /* IN */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();

  int ret = mldsa65_verify(sig, sig_len, message, message_len,
                            ctx_string, ctx_string_len, public_key);

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_extmu_65_verify(const uint8_t *public_key /* IN */,
                           const uint8_t *sig        /* IN */,
                           size_t sig_len            /* IN */,
                           const uint8_t *mu         /* IN */,
                           size_t mu_len             /* IN */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();

  // mu_len is ignored - extmu always uses MLDSA_CRHBYTES (64 bytes)
  (void)mu_len;
  int ret = mldsa65_verify_extmu(sig, sig_len, mu, public_key);

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_65_verify_internal(const uint8_t *public_key /* IN */,
                              const uint8_t *sig        /* IN */,
                              size_t sig_len            /* IN */,
                              const uint8_t *message    /* IN */,
                              size_t message_len        /* IN */,
                              const uint8_t *pre        /* IN */,
                              size_t pre_len            /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  int ret = mldsa65_verify_internal(sig, sig_len, message, message_len,
                                     pre, pre_len, public_key, 0);
  return (ret == 0) ? 1 : 0;
}

int ml_dsa_extmu_65_verify_internal(const uint8_t *public_key /* IN */,
                                    const uint8_t *sig        /* IN */,
                                    size_t sig_len            /* IN */,
                                    const uint8_t *mu         /* IN */,
                                    size_t mu_len             /* IN */,
                                    const uint8_t *pre        /* IN */,
                                    size_t pre_len            /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  int ret = mldsa65_verify_internal(sig, sig_len, mu, mu_len,
                                     pre, pre_len, public_key, 1);
  return (ret == 0) ? 1 : 0;
}

// ML-DSA-87 implementations
int ml_dsa_87_keypair(uint8_t *public_key   /* OUT */,
                      uint8_t *private_key  /* OUT */,
                      uint8_t *seed         /* OUT */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();
  AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(seed, MLDSA87_KEYGEN_SEED_BYTES));

  int ret = mldsa87_keypair_internal(public_key, private_key, seed);
#if defined(AWSLC_FIPS)
  /* PCT failure is the only failure condition for key generation. */
  if (ret != 0) {
    AWS_LC_FIPS_failure("ML-DSA keygen PCT failed");
  }
#endif

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_87_pack_pk_from_sk(uint8_t *public_key          /* OUT */,
                              const uint8_t *private_key   /* IN  */) {
  int ret = mldsa87_pk_from_sk(public_key, private_key);
  return (ret == 0) ? 1 : 0;  // Convert: mldsa 0=success, -1=failure -> AWS-LC 1=success, 0=failure
}

int ml_dsa_87_keypair_internal(uint8_t *public_key   /* OUT */,
                               uint8_t *private_key  /* OUT */,
                               const uint8_t *seed   /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  int ret = mldsa87_keypair_internal(public_key, private_key, seed);
#if defined(AWSLC_FIPS)
  /* PCT failure is the only failure condition for key generation. */
  if (ret != 0) {
    AWS_LC_FIPS_failure("ML-DSA keygen PCT failed");
  }
#endif
  return (ret == 0) ? 1 : 0;  // Convert: mldsa 0=success -> AWS-LC 1=success
}

int ml_dsa_87_sign(const uint8_t *private_key /* IN */,
                   uint8_t *sig               /* OUT */,
                   size_t *sig_len            /* OUT */,
                   const uint8_t *message     /* IN */,
                   size_t message_len         /* IN */,
                   const uint8_t *ctx_string  /* IN */,
                   size_t ctx_string_len      /* IN */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();

  int ret = mldsa87_signature(sig, sig_len, message, message_len,
                               ctx_string, ctx_string_len, private_key);

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_extmu_87_sign(const uint8_t *private_key /* IN */,
                         uint8_t *sig               /* OUT */,
                         size_t *sig_len            /* OUT */,
                         const uint8_t *mu          /* IN */,
                         size_t mu_len              /* IN */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();

  // mu_len is ignored - extmu always uses MLDSA_CRHBYTES (64 bytes)
  (void)mu_len;
  int ret = mldsa87_signature_extmu(sig, sig_len, mu, private_key);

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_87_sign_internal(const uint8_t *private_key  /* IN */,
                            uint8_t *sig                /* OUT */,
                            size_t *sig_len             /* OUT */,
                            const uint8_t *message      /* IN */,
                            size_t message_len          /* IN */,
                            const uint8_t *pre          /* IN */,
                            size_t pre_len              /* IN */,
                            const uint8_t *rnd          /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  int ret = mldsa87_signature_internal(sig, sig_len, message, message_len,
                                        pre, pre_len, rnd, private_key, 0);
  return (ret == 0) ? 1 : 0;
}

int ml_dsa_extmu_87_sign_internal(const uint8_t *private_key  /* IN */,
                                  uint8_t *sig                /* OUT */,
                                  size_t *sig_len             /* OUT */,
                                  const uint8_t *mu           /* IN */,
                                  size_t mu_len               /* IN */,
                                  const uint8_t *pre          /* IN */,
                                  size_t pre_len              /* IN */,
                                  const uint8_t *rnd          /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  int ret = mldsa87_signature_internal(sig, sig_len, mu, mu_len,
                                        pre, pre_len, rnd, private_key, 1);
  return (ret == 0) ? 1 : 0;
}

int ml_dsa_87_verify(const uint8_t *public_key /* IN */,
                     const uint8_t *sig        /* IN */,
                     size_t sig_len            /* IN */,
                     const uint8_t *message    /* IN */,
                     size_t message_len        /* IN */,
                     const uint8_t *ctx_string /* IN */,
                     size_t ctx_string_len     /* IN */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();

  int ret = mldsa87_verify(sig, sig_len, message, message_len,
                            ctx_string, ctx_string_len, public_key);

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_extmu_87_verify(const uint8_t *public_key /* IN */,
                           const uint8_t *sig        /* IN */,
                           size_t sig_len            /* IN */,
                           const uint8_t *mu         /* IN */,
                           size_t mu_len             /* IN */) {
  FIPS_service_indicator_lock_state();
  boringssl_ensure_ml_dsa_self_test();

  // mu_len is ignored - extmu always uses MLDSA_CRHBYTES (64 bytes)
  (void)mu_len;
  int ret = mldsa87_verify_extmu(sig, sig_len, mu, public_key);

  FIPS_service_indicator_unlock_state();
  if (ret == 0) {
    FIPS_service_indicator_update_state();
    return 1;
  }
  return 0;
}

int ml_dsa_87_verify_internal(const uint8_t *public_key /* IN */,
                              const uint8_t *sig        /* IN */,
                              size_t sig_len            /* IN */,
                              const uint8_t *message    /* IN */,
                              size_t message_len        /* IN */,
                              const uint8_t *pre        /* IN */,
                              size_t pre_len            /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  int ret = mldsa87_verify_internal(sig, sig_len, message, message_len,
                                     pre, pre_len, public_key, 0);
  return (ret == 0) ? 1 : 0;
}

int ml_dsa_extmu_87_verify_internal(const uint8_t *public_key /* IN */,
                                    const uint8_t *sig        /* IN */,
                                    size_t sig_len            /* IN */,
                                    const uint8_t *mu         /* IN */,
                                    size_t mu_len             /* IN */,
                                    const uint8_t *pre        /* IN */,
                                    size_t pre_len            /* IN */) {
  boringssl_ensure_ml_dsa_self_test();
  int ret = mldsa87_verify_internal(sig, sig_len, mu, mu_len,
                                     pre, pre_len, public_key, 1);
  return (ret == 0) ? 1 : 0;
}
