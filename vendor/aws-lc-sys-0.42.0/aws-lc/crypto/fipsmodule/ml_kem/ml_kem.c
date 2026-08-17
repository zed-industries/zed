// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// mlkem-native source code

// Include level-independent code
#define MLK_CONFIG_FILE "../mlkem_native_config.h"
#define MLK_CONFIG_MONOBUILD_KEEP_SHARED_HEADERS

// MLKEM-512
#define MLK_CONFIG_PARAMETER_SET 512
#define MLK_CONFIG_MULTILEVEL_WITH_SHARED // Include level-independent code
#include "mlkem/mlkem_native_bcm.c"
// MLKEM-768
#undef MLK_CONFIG_PARAMETER_SET
#define MLK_CONFIG_PARAMETER_SET 768
#define MLK_CONFIG_MULTILEVEL_NO_SHARED // Exclude level-inpendent code
#include "mlkem/mlkem_native_bcm.c"
// MLKEM-1024
#undef MLK_CONFIG_MONOBUILD_KEEP_SHARED_HEADERS
#undef MLK_CONFIG_PARAMETER_SET
#define MLK_CONFIG_PARAMETER_SET 1024
#include "mlkem/mlkem_native_bcm.c"

// End of mlkem-native source code

#include "./ml_kem.h"

typedef struct {
  uint8_t *buffer;
  size_t *length;
  const size_t expected_length;
} output_buffer;

// Ensure buffer is long enough
static int check_buffer(const output_buffer data) {
  if (data.buffer == NULL || *data.length < data.expected_length) {
    return 0;
  }
  return 1;
}

// EVP layer assumes the length parameter passed in will be set to the number of bytes written if call is successful
static void set_written_len_on_success(const int result, output_buffer data) {
  if (result == 0) {
    *data.length = data.expected_length;
  }
}

int ml_kem_common_keypair(int (*keypair)(uint8_t * public_key, uint8_t *secret_key),
                          output_buffer public_key,
                          output_buffer secret_key);

int ml_kem_common_encapsulate_deterministic(int (*encapsulate)(uint8_t *ciphertext, uint8_t *shared_secret, const uint8_t *public_key, const uint8_t *seed),
                                            output_buffer ciphertext,
                                            output_buffer shared_secret,
                                            const uint8_t *public_key,
                                            const uint8_t *seed);

int ml_kem_common_encapsulate(int (*encapsulate)(uint8_t *ciphertext, uint8_t *shared_secret, const uint8_t *public_key),
                              output_buffer ciphertext,
                              output_buffer shared_secret,
                              const uint8_t *public_key);

int ml_kem_common_decapsulate(int (*decapsulate)(uint8_t *shared_secret, const uint8_t *ciphertext, const uint8_t *secret_key),
                              output_buffer shared_secret,
                              const uint8_t *ciphertext,
                              const uint8_t *secret_key);

// Note: These methods currently default to using the reference code for ML_KEM.
// In a future where AWS-LC has optimized options available, those can be
// conditionally (or based on compile-time flags) called here, depending on
// platform support.

int ml_kem_512_keypair_deterministic(uint8_t *public_key /* OUT */,
                                     size_t *public_len /* IN_OUT */,
                                     uint8_t *secret_key /* OUT */,
                                     size_t *secret_len /* IN_OUT */,
                                     const uint8_t *seed /* IN */) {
  boringssl_ensure_ml_kem_self_test();
  return ml_kem_512_keypair_deterministic_no_self_test(
      public_key, public_len, secret_key, secret_len, seed);
}

int ml_kem_512_keypair_deterministic_no_self_test(uint8_t *public_key  /* OUT */,
                                                  size_t *public_len  /* IN_OUT */,
                                                  uint8_t *secret_key  /* OUT */,
                                                  size_t *secret_len  /* IN_OUT */,
                                                  const uint8_t *seed  /* IN */) {
  output_buffer pkey = {public_key, public_len, MLKEM512_PUBLIC_KEY_BYTES};
  output_buffer skey = {secret_key, secret_len, MLKEM512_SECRET_KEY_BYTES};
  if (!check_buffer(pkey) || !check_buffer(skey)) {
    return 1;
  }
  const int res = mlkem512_keypair_derand(pkey.buffer, skey.buffer, seed);
#if defined(AWSLC_FIPS)
  /* PCT failure is the only failure condition for key generation. */
  if (res != 0) {
    AWS_LC_FIPS_failure("ML-KEM keygen PCT failed");
  }
#endif
  set_written_len_on_success(res, pkey);
  set_written_len_on_success(res, skey);
  return res;
}

int ml_kem_512_keypair(uint8_t *public_key /* OUT */,
                       size_t *public_len  /* IN_OUT */,
                       uint8_t *secret_key /* OUT */,
                       size_t *secret_len  /* IN_OUT */) {
  output_buffer pkey = {public_key, public_len, MLKEM512_PUBLIC_KEY_BYTES};
  output_buffer skey = {secret_key, secret_len, MLKEM512_SECRET_KEY_BYTES};
  return ml_kem_common_keypair(mlkem512_keypair, pkey, skey);
}

int ml_kem_512_encapsulate_deterministic(uint8_t *ciphertext       /* OUT */,
                                         size_t *ciphertext_len    /* IN_OUT */,
                                         uint8_t *shared_secret    /* OUT */,
                                         size_t *shared_secret_len /* IN_OUT */,
                                         const uint8_t *public_key /* IN  */,
                                         const uint8_t *seed       /* IN */) {
  boringssl_ensure_ml_kem_self_test();
  return ml_kem_512_encapsulate_deterministic_no_self_test(ciphertext, ciphertext_len, shared_secret, shared_secret_len, public_key, seed);
}

int ml_kem_512_encapsulate_deterministic_no_self_test(uint8_t *ciphertext       /* OUT */,
                                                      size_t *ciphertext_len    /* IN_OUT */,
                                                      uint8_t *shared_secret    /* OUT */,
                                                      size_t *shared_secret_len /* IN_OUT */,
                                                      const uint8_t *public_key /* IN */,
                                                      const uint8_t *seed       /* IN */) {
  output_buffer ctext = {ciphertext, ciphertext_len, MLKEM512_CIPHERTEXT_BYTES};
  output_buffer ss = {shared_secret, shared_secret_len, MLKEM512_SHARED_SECRET_LEN};
  return ml_kem_common_encapsulate_deterministic(mlkem512_enc_derand, ctext, ss, public_key, seed);
}

int ml_kem_512_encapsulate(uint8_t *ciphertext       /* OUT */,
                           size_t *ciphertext_len    /* IN_OUT */,
                           uint8_t *shared_secret    /* OUT */,
                           size_t *shared_secret_len /* IN_OUT */,
                           const uint8_t *public_key /* IN */) {
  output_buffer ctext = {ciphertext, ciphertext_len, MLKEM512_CIPHERTEXT_BYTES};
  output_buffer ss = {shared_secret, shared_secret_len, MLKEM512_SHARED_SECRET_LEN};
  return ml_kem_common_encapsulate(mlkem512_enc, ctext, ss, public_key);
}

int ml_kem_512_decapsulate(uint8_t *shared_secret    /* OUT */,
                           size_t *shared_secret_len /* IN_OUT */,
                           const uint8_t *ciphertext /* IN */,
                           const uint8_t *secret_key /* IN */) {
  boringssl_ensure_ml_kem_self_test();
  return ml_kem_512_decapsulate_no_self_test(shared_secret, shared_secret_len, ciphertext, secret_key);
}

int ml_kem_512_decapsulate_no_self_test(uint8_t *shared_secret    /* OUT */,
                                        size_t *shared_secret_len /* IN_OUT */,
                                        const uint8_t *ciphertext /* IN  */,
                                        const uint8_t *secret_key /* IN  */) {
  output_buffer ss = {shared_secret, shared_secret_len, MLKEM512_SHARED_SECRET_LEN};
  return ml_kem_common_decapsulate(mlkem512_dec, ss, ciphertext, secret_key);
}


int ml_kem_768_keypair_deterministic(uint8_t *public_key /* OUT */,
                                     size_t *public_len  /* IN_OUT */,
                                     uint8_t *secret_key /* OUT */,
                                     size_t *secret_len  /* IN_OUT */,
                                     const uint8_t *seed /* IN */) {
  boringssl_ensure_ml_kem_self_test();
  return ml_kem_768_keypair_deterministic_no_self_test(public_key, public_len, secret_key, secret_len, seed);
}


int ml_kem_768_keypair_deterministic_no_self_test(uint8_t *public_key /* OUT */,
                                                  size_t *public_len  /* IN_OUT */,
                                                  uint8_t *secret_key /* OUT */,
                                                  size_t *secret_len  /* IN_OUT */,
                                                  const uint8_t *seed /* IN */) {
  output_buffer pkey = {public_key, public_len, MLKEM768_PUBLIC_KEY_BYTES};
  output_buffer skey = {secret_key, secret_len, MLKEM768_SECRET_KEY_BYTES};
  if (!check_buffer(pkey) || !check_buffer(skey)) {
    return 1;
  }
  const int res = mlkem768_keypair_derand(pkey.buffer, skey.buffer, seed);
#if defined(AWSLC_FIPS)
  /* PCT failure is the only failure condition for key generation. */
  if (res != 0) {
    AWS_LC_FIPS_failure("ML-KEM keygen PCT failed");
  }
#endif
  set_written_len_on_success(res, pkey);
  set_written_len_on_success(res, skey);
  return res;
}

int ml_kem_768_keypair(uint8_t *public_key /* OUT */,
                       size_t *public_len  /* IN_OUT */,
                       uint8_t *secret_key /* OUT */,
                       size_t *secret_len  /* IN_OUT */) {
  output_buffer pkey = {public_key, public_len, MLKEM768_PUBLIC_KEY_BYTES};
  output_buffer skey = {secret_key, secret_len, MLKEM768_SECRET_KEY_BYTES};
  return ml_kem_common_keypair(mlkem768_keypair, pkey, skey);
}

int ml_kem_768_encapsulate_deterministic(uint8_t *ciphertext       /* OUT */,
                                         size_t *ciphertext_len    /* IN_OUT */,
                                         uint8_t *shared_secret    /* OUT */,
                                         size_t *shared_secret_len /* IN_OUT */,
                                         const uint8_t *public_key /* IN */,
                                         const uint8_t *seed       /* IN */) {
  boringssl_ensure_ml_kem_self_test();
  return ml_kem_768_encapsulate_deterministic_no_self_test(ciphertext, ciphertext_len, shared_secret, shared_secret_len, public_key, seed);
}

int ml_kem_768_encapsulate_deterministic_no_self_test(uint8_t *ciphertext       /* OUT */,
                                                      size_t *ciphertext_len    /* IN_OUT */,
                                                      uint8_t *shared_secret    /* OUT */,
                                                      size_t *shared_secret_len /* IN_OUT */,
                                                      const uint8_t *public_key /* IN */,
                                                      const uint8_t *seed       /* IN */) {
  output_buffer ctext = {ciphertext, ciphertext_len, MLKEM768_CIPHERTEXT_BYTES};
  output_buffer ss = {shared_secret, shared_secret_len, MLKEM768_SHARED_SECRET_LEN};
  return ml_kem_common_encapsulate_deterministic(mlkem768_enc_derand, ctext, ss, public_key, seed);
}

int ml_kem_768_encapsulate(uint8_t *ciphertext       /* OUT */,
                           size_t *ciphertext_len    /* IN_OUT */,
                           uint8_t *shared_secret    /* OUT */,
                           size_t *shared_secret_len /* IN_OUT */,
                           const uint8_t *public_key /* IN */) {
  output_buffer ctext = {ciphertext, ciphertext_len, MLKEM768_CIPHERTEXT_BYTES};
  output_buffer ss = {shared_secret, shared_secret_len, MLKEM768_SHARED_SECRET_LEN};
  return ml_kem_common_encapsulate(mlkem768_enc, ctext, ss, public_key);
}

int ml_kem_768_decapsulate(uint8_t *shared_secret    /* OUT */,
                           size_t *shared_secret_len /* IN_OUT */,
                           const uint8_t *ciphertext /* IN */,
                           const uint8_t *secret_key /* IN */) {
  boringssl_ensure_ml_kem_self_test();
  return ml_kem_768_decapsulate_no_self_test(shared_secret, shared_secret_len, ciphertext, secret_key);
}

int ml_kem_768_decapsulate_no_self_test(uint8_t *shared_secret    /* OUT */,
                                        size_t *shared_secret_len /* IN_OUT */,
                                        const uint8_t *ciphertext /* IN */,
                                        const uint8_t *secret_key /* IN */) {
  output_buffer ss = {shared_secret, shared_secret_len, MLKEM768_SHARED_SECRET_LEN};
  return ml_kem_common_decapsulate(mlkem768_dec, ss, ciphertext, secret_key);
}

int ml_kem_1024_keypair_deterministic(uint8_t *public_key /* OUT */,
                                      size_t *public_len  /* IN_OUT */,
                                      uint8_t *secret_key /* OUT */,
                                      size_t *secret_len  /* IN_OUT */,
                                      const uint8_t *seed /* IN */) {
  boringssl_ensure_ml_kem_self_test();
  return ml_kem_1024_keypair_deterministic_no_self_test(public_key, public_len, secret_key, secret_len, seed);
}

int ml_kem_1024_keypair_deterministic_no_self_test(uint8_t *public_key /* OUT */,
                                                   size_t *public_len  /* IN_OUT */,
                                                   uint8_t *secret_key /* OUT */,
                                                   size_t *secret_len  /* IN_OUT */,
                                                   const uint8_t *seed /* IN */) {
  output_buffer pkey = {public_key, public_len, MLKEM1024_PUBLIC_KEY_BYTES};
  output_buffer skey = {secret_key, secret_len, MLKEM1024_SECRET_KEY_BYTES};
  if (!check_buffer(pkey) || !check_buffer(skey)) {
    return 1;
  }
  const int res = mlkem1024_keypair_derand(pkey.buffer, skey.buffer, seed);
#if defined(AWSLC_FIPS)
  /* PCT failure is the only failure condition for key generation. */
  if (res != 0) {
    AWS_LC_FIPS_failure("ML-KEM keygen PCT failed");
  }
#endif
  set_written_len_on_success(res, pkey);
  set_written_len_on_success(res, skey);
  return res;
}

int ml_kem_1024_keypair(uint8_t *public_key /* OUT */,
                        size_t *public_len  /* IN_OUT */,
                        uint8_t *secret_key /* OUT */,
                        size_t *secret_len  /* IN_OUT */) {
  output_buffer pkey = {public_key, public_len, MLKEM1024_PUBLIC_KEY_BYTES};
  output_buffer skey = {secret_key, secret_len, MLKEM1024_SECRET_KEY_BYTES};
  return ml_kem_common_keypair(mlkem1024_keypair, pkey, skey);
}

int ml_kem_1024_encapsulate_deterministic(uint8_t *ciphertext       /* OUT */,
                                          size_t *ciphertext_len    /* IN_OUT */,
                                          uint8_t *shared_secret    /* OUT */,
                                          size_t *shared_secret_len /* IN_OUT */,
                                          const uint8_t *public_key /* IN */,
                                          const uint8_t *seed       /* IN */) {
  boringssl_ensure_ml_kem_self_test();
  return ml_kem_1024_encapsulate_deterministic_no_self_test(ciphertext, ciphertext_len, shared_secret, shared_secret_len, public_key, seed);
}

int ml_kem_1024_encapsulate_deterministic_no_self_test(uint8_t *ciphertext       /* OUT */,
                                                       size_t *ciphertext_len    /* IN_OUT */,
                                                       uint8_t *shared_secret    /* OUT */,
                                                       size_t *shared_secret_len /* IN_OUT */,
                                                       const uint8_t *public_key /* IN */,
                                                       const uint8_t *seed       /* IN */) {
  output_buffer ctext = {ciphertext, ciphertext_len, MLKEM1024_CIPHERTEXT_BYTES};
  output_buffer ss = {shared_secret, shared_secret_len, MLKEM1024_SHARED_SECRET_LEN};
  return ml_kem_common_encapsulate_deterministic(mlkem1024_enc_derand, ctext, ss, public_key, seed);
}

int ml_kem_1024_encapsulate(uint8_t *ciphertext       /* OUT */,
                            size_t *ciphertext_len    /* IN_OUT */,
                            uint8_t *shared_secret    /* OUT */,
                            size_t *shared_secret_len /* IN_OUT */,
                            const uint8_t *public_key /* IN */) {
  output_buffer ctext = {ciphertext, ciphertext_len, MLKEM1024_CIPHERTEXT_BYTES};
  output_buffer ss = {shared_secret, shared_secret_len, MLKEM1024_SHARED_SECRET_LEN};
  return ml_kem_common_encapsulate(mlkem1024_enc, ctext, ss, public_key);
}

int ml_kem_1024_decapsulate(uint8_t *shared_secret    /* OUT */,
                            size_t *shared_secret_len /* IN_OUT */,
                            const uint8_t *ciphertext /* IN */,
                            const uint8_t *secret_key /* IN */) {
  boringssl_ensure_ml_kem_self_test();
  return ml_kem_1024_decapsulate_no_self_test(shared_secret, shared_secret_len, ciphertext, secret_key);
}

int ml_kem_1024_decapsulate_no_self_test(uint8_t *shared_secret    /* OUT */,
                                         size_t *shared_secret_len /* IN_OUT */,
                                         const uint8_t *ciphertext /* IN */,
                                         const uint8_t *secret_key /* IN */) {
  output_buffer ss = {shared_secret, shared_secret_len, MLKEM1024_SHARED_SECRET_LEN};
  return ml_kem_common_decapsulate(mlkem1024_dec, ss, ciphertext, secret_key);
}

int ml_kem_common_keypair(int (*keypair)(uint8_t * public_key, uint8_t *secret_key),
                          output_buffer public_key,
                          output_buffer secret_key) {
  boringssl_ensure_ml_kem_self_test();
  if (!check_buffer(public_key) || !check_buffer(secret_key)) {
    return 1;
  }
  const int res = keypair(public_key.buffer, secret_key.buffer);
#if defined(AWSLC_FIPS)
  /* PCT failure is the only failure condition for key generation. */
  if (res != 0) {
    AWS_LC_FIPS_failure("ML-KEM keygen PCT failed");
  }
#endif
  set_written_len_on_success(res, public_key);
  set_written_len_on_success(res, secret_key);
  return res;
}

int ml_kem_common_encapsulate_deterministic(int (*encapsulate)(uint8_t *ciphertext, uint8_t *shared_secret, const uint8_t *public_key, const uint8_t *seed),
                                            output_buffer ciphertext,
                                            output_buffer shared_secret,
                                            const uint8_t *public_key,
                                            const uint8_t *seed) {
  if (!check_buffer(ciphertext) || !check_buffer(shared_secret)) {
    return 1;
  }
  const int res = encapsulate(ciphertext.buffer, shared_secret.buffer, public_key, seed);
  set_written_len_on_success(res, ciphertext);
  set_written_len_on_success(res, shared_secret);
  return res;
}

int ml_kem_common_encapsulate(int (*encapsulate)(uint8_t *ciphertext, uint8_t *shared_secret, const uint8_t *public_key),
                              output_buffer ciphertext,
                              output_buffer shared_secret,
                              const uint8_t *public_key) {
  boringssl_ensure_ml_kem_self_test();
  if (!check_buffer(ciphertext) || !check_buffer(shared_secret)) {
    return 1;
  }
  const int res = encapsulate(ciphertext.buffer, shared_secret.buffer, public_key);
  set_written_len_on_success(res, ciphertext);
  set_written_len_on_success(res, shared_secret);
  return res;
}

int ml_kem_common_decapsulate(int (*decapsulate)(uint8_t *shared_secret, const uint8_t *ciphertext, const uint8_t *secret_key),
                              output_buffer shared_secret,
                              const uint8_t *ciphertext,
                              const uint8_t *secret_key) {
  if (!check_buffer(shared_secret)) {
    return 1;
  }
  const int res = decapsulate(shared_secret.buffer, ciphertext, secret_key);
  set_written_len_on_success(res, shared_secret);
  return res;
}

int ml_kem_512_check_pk(const uint8_t *public_key, size_t public_key_len) {
  if (public_key == NULL || public_key_len != MLKEM512_PUBLIC_KEY_BYTES) {
    return -1;
  }
  return mlkem512_check_pk(public_key);
}

int ml_kem_512_check_sk(const uint8_t *secret_key, size_t secret_key_len) {
  if (secret_key == NULL || secret_key_len != MLKEM512_SECRET_KEY_BYTES) {
    return -1;
  }
  return mlkem512_check_sk(secret_key);
}

int ml_kem_768_check_pk(const uint8_t *public_key, size_t public_key_len) {
  if (public_key == NULL || public_key_len != MLKEM768_PUBLIC_KEY_BYTES) {
    return -1;
  }
  return mlkem768_check_pk(public_key);
}

int ml_kem_768_check_sk(const uint8_t *secret_key, size_t secret_key_len) {
  if (secret_key == NULL || secret_key_len != MLKEM768_SECRET_KEY_BYTES) {
    return -1;
  }
  return mlkem768_check_sk(secret_key);
}

int ml_kem_1024_check_pk(const uint8_t *public_key, size_t public_key_len) {
  if (public_key == NULL || public_key_len != MLKEM1024_PUBLIC_KEY_BYTES) {
    return -1;
  }
  return mlkem1024_check_pk(public_key);
}

int ml_kem_1024_check_sk(const uint8_t *secret_key, size_t secret_key_len) {
  if (secret_key == NULL || secret_key_len != MLKEM1024_SECRET_KEY_BYTES) {
    return -1;
  }
  return mlkem1024_check_sk(secret_key);
}
