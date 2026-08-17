// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef AWSLC_HEADER_KEM_INTERNAL_H
#define AWSLC_HEADER_KEM_INTERNAL_H

#include <openssl/base.h>


#if defined(__cplusplus)
extern "C" {
#endif

// KEM_METHOD structure and helper functions.
typedef struct {
  int (*keygen_deterministic)(uint8_t *ctx,
                              size_t *ctx_len,
                              uint8_t *pkey,
                              size_t *pkey_len,
                              const uint8_t *seed);

  int (*encaps_deterministic)(uint8_t *ciphertext,
                              size_t *ciphertext_len,
                              uint8_t *shared_secret,
                              size_t *shared_secret_len,
                              const uint8_t *public_key,
                              const uint8_t *seed);

  int (*encaps)(uint8_t *ciphertext,
                size_t *ciphertext_len,
                uint8_t *shared_secret,
                size_t *shared_secret_len,
                const uint8_t *public_key);

  int (*decaps)(uint8_t *shared_secret,
                size_t *shared_secret_len,
                const uint8_t *ciphertext,
                const uint8_t *secret_key);
} KEM_METHOD;

// KEM structure and helper functions.
typedef struct {
  int nid;
  const uint8_t *oid;
  uint8_t oid_len;
  const char *comment;
  size_t public_key_len;
  size_t secret_key_len;
  size_t ciphertext_len;
  size_t shared_secret_len;
  size_t keygen_seed_len;
  size_t encaps_seed_len;
  const KEM_METHOD *method;
} KEM;

// KEM_KEY structure and helper functions.
struct kem_key_st {
  const KEM *kem;
  uint8_t *public_key;
  uint8_t *secret_key;
  uint8_t *seed;
};

const KEM *KEM_find_kem_by_nid(int nid);
const EVP_PKEY_ASN1_METHOD *KEM_find_asn1_by_nid(int nid);
int EVP_PKEY_kem_set_params(EVP_PKEY *pkey, int nid);

KEM_KEY *KEM_KEY_new(void);
int KEM_KEY_init(KEM_KEY *key, const KEM *kem);
void KEM_KEY_free(KEM_KEY *key);
const KEM *KEM_KEY_get0_kem(KEM_KEY* key);

// KEM_KEY_set_raw_public_key function allocates the public key buffer
// within the given |key| and copies the contents of |in| to it.
//
// NOTE: No checks are done in this function, the caller has to ensure
//       that the pointers are valid and |in| has the correct size.
int KEM_KEY_set_raw_public_key(KEM_KEY *key, const uint8_t *in);

// KEM_KEY_set_raw_secret_key function allocates the secret key buffer
// within the given |key| and copies the contents of |in| to it.
//
// NOTE: No checks are done in this function, the caller has to ensure
//       that the pointers are valid and |in| has the correct size.
int KEM_KEY_set_raw_secret_key(KEM_KEY *key, const uint8_t *in);

// KEM_KEY_set_raw_key function allocates the public and secret key buffers
// within the given |key| and copies the contents of |in_public| and
// |in_secret| to them.
//
// NOTE: No checks are done in this function, the caller has to ensure
//       that the pointers are valid and |in_public| and |in_secret|
//       have the correct size.
int KEM_KEY_set_raw_key(KEM_KEY *key, const uint8_t *in_public,
                                      const uint8_t *in_secret);

// KEM_KEY_set_raw_keypair_from_seed function generates a keypair from the
// given seed using the appropriate key generation function based on the
// KEM variant, then allocates and sets both public and secret key buffers
// within the given |key|.
//
// NOTE: The seed must be exactly 64 bytes for all ML-KEM variants.
//       The caller must ensure the seed CBS contains valid data.
//       |key->kem| must be initialized and |key->public_key| and 
//       |key->secret_key| must both be NULL.
int KEM_KEY_set_raw_keypair_from_seed(KEM_KEY *key, const CBS *seed);

#if defined(__cplusplus)
}  // extern C
#endif

#endif // AWSLC_HEADER_KEM_TEST_INTERNAL_H
