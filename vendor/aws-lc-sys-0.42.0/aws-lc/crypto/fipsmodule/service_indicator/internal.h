// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef AWSLC_HEADER_SERVICE_INDICATOR_INTERNAL_H
#define AWSLC_HEADER_SERVICE_INDICATOR_INTERNAL_H

#include <openssl/aead.h>
#include <openssl/service_indicator.h>

#if defined(AWSLC_FIPS)

// Only to be used internally, it is not intended for the user to update the
// state. This function is used to update the service indicator state, if the
// service is deemed to be approved.
void FIPS_service_indicator_update_state(void);

// Only to be used internally. Certain approved algorithms call upon other
// approved algorithms, and some services provide one-shot functions that call
// upon multiple functions that are approved themselves.
// These functions lock/unlock the counter state, so that nested calls of
// |FIPS_service_indicator_update_state| within don't update the counter
// unintentionally. The lock is implemented as a counter, as one-shot functions
// may call upon approved nested functions which have approved nested algorithms
// within them as well. The counter state can only be updated when the
// |lock_state| has a value of |STATE_UNLOCKED|.
// For the approval checks to work correctly, whenever
// |FIPS_service_indicator_lock_state| is called,
// |FIPS_service_indicator_unlock_state| must be called before exiting the
// function. This ensures that the counter is only updated when the most
// high level function that initially locked the state first, unlocks the
// |lock_state| back to |STATE_UNLOCKED|.
void FIPS_service_indicator_lock_state(void);
void FIPS_service_indicator_unlock_state(void);

// The following functions may call |FIPS_service_indicator_update_state| if
// their parameter specifies an approved operation.

void AEAD_GCM_verify_service_indicator(const EVP_AEAD_CTX *ctx);
void AEAD_CCM_verify_service_indicator(const EVP_AEAD_CTX *ctx);
void AES_CMAC_verify_service_indicator(const CMAC_CTX *ctx);
void EC_KEY_keygen_verify_service_indicator(const EC_KEY *eckey);
void ECDH_verify_service_indicator(const EC_KEY *ec_key);
void EVP_Cipher_verify_service_indicator(const EVP_CIPHER_CTX *ctx);
void EVP_DigestSign_verify_service_indicator(const EVP_MD_CTX *ctx);
void EVP_DigestVerify_verify_service_indicator(const EVP_MD_CTX *ctx);
void EVP_PKEY_keygen_verify_service_indicator(const EVP_PKEY *pkey);
void HMAC_verify_service_indicator(const EVP_MD *evp_md);
void HKDF_verify_service_indicator(const EVP_MD *evp_md, const uint8_t *salt,
    size_t salt_len, size_t info_len);
void HKDFExpand_verify_service_indicator(const EVP_MD *evp_md);
void PBKDF2_verify_service_indicator(const EVP_MD *evp_md, size_t password_len,
                                     size_t salt_len, unsigned iterations);
void SSHKDF_verify_service_indicator(const EVP_MD *evp_md);
void TLSKDF_verify_service_indicator(const EVP_MD *dgst, const char *label,
                                     size_t label_len);
void TLS13_KDF_verify_service_indicator(const EVP_MD *dgst);
void SSKDF_digest_verify_service_indicator(const EVP_MD *dgst);
void SSKDF_hmac_verify_service_indicator(const EVP_MD *dgst);
void KBKDF_ctr_hmac_verify_service_indicator(const EVP_MD *dgst, size_t secret_len);
void EVP_PKEY_encapsulate_verify_service_indicator(const EVP_PKEY_CTX* ctx);
void EVP_PKEY_decapsulate_verify_service_indicator(const EVP_PKEY_CTX* ctx);

#else

// Service indicator functions are no-ops in non-FIPS builds.

OPENSSL_INLINE void FIPS_service_indicator_update_state(void) { }
OPENSSL_INLINE void FIPS_service_indicator_lock_state(void) { }
OPENSSL_INLINE void FIPS_service_indicator_unlock_state(void) { }

// Service indicator check functions listed below are optimized to not do extra
// checks, when not in FIPS mode. Arguments are cast with |OPENSSL_UNUSED| in an
// attempt to avoid unused warnings.
OPENSSL_INLINE void AEAD_GCM_verify_service_indicator(
    OPENSSL_UNUSED const EVP_AEAD_CTX *ctx) {}

OPENSSL_INLINE void AEAD_CCM_verify_service_indicator(
    OPENSSL_UNUSED const EVP_AEAD_CTX *ctx) {}

OPENSSL_INLINE void AES_CMAC_verify_service_indicator(
    OPENSSL_UNUSED const CMAC_CTX *ctx) {}

OPENSSL_INLINE void EC_KEY_keygen_verify_service_indicator(
    OPENSSL_UNUSED const EC_KEY *eckey) {}

OPENSSL_INLINE void ECDH_verify_service_indicator(
    OPENSSL_UNUSED const EC_KEY *ec_key) {}

OPENSSL_INLINE void EVP_Cipher_verify_service_indicator(
    OPENSSL_UNUSED const EVP_CIPHER_CTX *ctx) {}

OPENSSL_INLINE void EVP_DigestSign_verify_service_indicator(
    OPENSSL_UNUSED const EVP_MD_CTX *ctx) {}

OPENSSL_INLINE void EVP_DigestVerify_verify_service_indicator(
    OPENSSL_UNUSED const EVP_MD_CTX *ctx) {}

OPENSSL_INLINE void EVP_PKEY_keygen_verify_service_indicator(
    OPENSSL_UNUSED const EVP_PKEY *pkey) {}

OPENSSL_INLINE void HMAC_verify_service_indicator(
    OPENSSL_UNUSED const EVP_MD *evp_md) {}

OPENSSL_INLINE void HKDF_verify_service_indicator(
    OPENSSL_UNUSED const EVP_MD *evp_md,
    OPENSSL_UNUSED const uint8_t *salt,
    OPENSSL_UNUSED size_t salt_len,
    OPENSSL_UNUSED size_t info_len) {}

OPENSSL_INLINE void HKDFExpand_verify_service_indicator(
    OPENSSL_UNUSED const EVP_MD *evp_md) {}

OPENSSL_INLINE void PBKDF2_verify_service_indicator(
    OPENSSL_UNUSED const EVP_MD *evp_md, OPENSSL_UNUSED size_t password_len,
    OPENSSL_UNUSED size_t salt_len, OPENSSL_UNUSED unsigned iterations) {}

OPENSSL_INLINE void SSHKDF_verify_service_indicator(
    OPENSSL_UNUSED const EVP_MD *evp_md) {}

OPENSSL_INLINE void TLSKDF_verify_service_indicator(
    OPENSSL_UNUSED const EVP_MD *dgst,
    OPENSSL_UNUSED const char *label,
    OPENSSL_UNUSED size_t label_len) {}

OPENSSL_INLINE void TLS13_KDF_verify_service_indicator(
    OPENSSL_UNUSED const EVP_MD *dgst) {}

OPENSSL_INLINE void SSKDF_digest_verify_service_indicator(
    OPENSSL_UNUSED const EVP_MD *dgst) {}

OPENSSL_INLINE void SSKDF_hmac_verify_service_indicator(
    OPENSSL_UNUSED const EVP_MD *dgst) {}

OPENSSL_INLINE void KBKDF_ctr_hmac_verify_service_indicator(OPENSSL_UNUSED const EVP_MD *dgst, size_t secret_len) {}

OPENSSL_INLINE void EVP_PKEY_encapsulate_verify_service_indicator(OPENSSL_UNUSED const EVP_PKEY_CTX* ctx) {}

OPENSSL_INLINE void EVP_PKEY_decapsulate_verify_service_indicator(OPENSSL_UNUSED const EVP_PKEY_CTX* ctx) {}

#endif // AWSLC_FIPS

// is_fips_build is similar to |FIPS_mode| but returns 1 including in the case
// of #if defined(OPENSSL_ASAN) or #if defined(OPENSSL_MSAN)
int is_fips_build(void);

#endif  // AWSLC_HEADER_SERVICE_INDICATOR_INTERNAL_H
