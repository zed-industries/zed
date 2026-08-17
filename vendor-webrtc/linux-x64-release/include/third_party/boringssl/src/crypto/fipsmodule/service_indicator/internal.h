// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_SERVICE_INDICATOR_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_SERVICE_INDICATOR_INTERNAL_H

#include <openssl/base.h>
#include <openssl/service_indicator.h>

#if defined(BORINGSSL_FIPS)

// FIPS_service_indicator_update_state records that an approved service has been
// invoked.
void FIPS_service_indicator_update_state(void);

// FIPS_service_indicator_lock_state and |FIPS_service_indicator_unlock_state|
// stop |FIPS_service_indicator_update_state| from actually updating the service
// indicator. This is used when a primitive calls a potentially approved
// primitive to avoid false positives. For example, just because a key
// generation calls |BCM_rand_bytes| (and thus the approved DRBG) doesn't mean
// that the key generation operation itself is approved.
//
// This lock nests: i.e. locking twice is fine so long as each lock is paired
// with an unlock. If the (64-bit) counter overflows, the process aborts.
void FIPS_service_indicator_lock_state(void);
void FIPS_service_indicator_unlock_state(void);

// The following functions may call |FIPS_service_indicator_update_state| if
// their parameter specifies an approved operation.

void AEAD_GCM_verify_service_indicator(const EVP_AEAD_CTX *ctx);
void AEAD_CCM_verify_service_indicator(const EVP_AEAD_CTX *ctx);
void EC_KEY_keygen_verify_service_indicator(const EC_KEY *eckey);
void ECDH_verify_service_indicator(const EC_KEY *ec_key);
void EVP_Cipher_verify_service_indicator(const EVP_CIPHER_CTX *ctx);
void EVP_DigestSign_verify_service_indicator(const EVP_MD_CTX *ctx);
void EVP_DigestVerify_verify_service_indicator(const EVP_MD_CTX *ctx);
void HMAC_verify_service_indicator(const EVP_MD *evp_md);
void TLSKDF_verify_service_indicator(const EVP_MD *dgst);

#else

// Service indicator functions are no-ops in non-FIPS builds.

inline void FIPS_service_indicator_update_state(void) {}
inline void FIPS_service_indicator_lock_state(void) {}
inline void FIPS_service_indicator_unlock_state(void) {}

inline void AEAD_GCM_verify_service_indicator(
    [[maybe_unused]] const EVP_AEAD_CTX *ctx) {}

inline void AEAD_CCM_verify_service_indicator(
    [[maybe_unused]] const EVP_AEAD_CTX *ctx) {}

inline void EC_KEY_keygen_verify_service_indicator(
    [[maybe_unused]] const EC_KEY *eckey) {}

inline void ECDH_verify_service_indicator(
    [[maybe_unused]] const EC_KEY *ec_key) {}

inline void EVP_Cipher_verify_service_indicator(
    [[maybe_unused]] const EVP_CIPHER_CTX *ctx) {}

inline void EVP_DigestSign_verify_service_indicator(
    [[maybe_unused]] const EVP_MD_CTX *ctx) {}

inline void EVP_DigestVerify_verify_service_indicator(
    [[maybe_unused]] const EVP_MD_CTX *ctx) {}

inline void HMAC_verify_service_indicator(
    [[maybe_unused]] const EVP_MD *evp_md) {}

inline void TLSKDF_verify_service_indicator(
    [[maybe_unused]] const EVP_MD *dgst) {}

#endif  // BORINGSSL_FIPS

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_SERVICE_INDICATOR_INTERNAL_H
