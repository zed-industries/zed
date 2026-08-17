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

#include <openssl/crypto.h>
#include <openssl/ec.h>
#include <openssl/ec_key.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/service_indicator.h>

#include "../../evp/internal.h"
#include "../../internal.h"
#include "internal.h"

#if defined(BORINGSSL_FIPS)

#define STATE_UNLOCKED 0

// fips_service_indicator_state is a thread-local structure that stores the
// state of the FIPS service indicator.
struct fips_service_indicator_state {
  // lock_state records the number of times the indicator has been locked.
  // When it is zero (i.e. |STATE_UNLOCKED|) then the indicator can be updated.
  uint64_t lock_state;
  // counter is the indicator state. It is incremented when an approved service
  // completes.
  uint64_t counter;
};

// service_indicator_get returns a pointer to the |fips_service_indicator_state|
// for the current thread. It returns NULL on error.
//
// FIPS 140-3 requires that the module should provide the service indicator
// for approved services irrespective of whether the user queries it or not.
// Hence, it is lazily initialized in any call to an approved service.
static struct fips_service_indicator_state *service_indicator_get(void) {
  struct fips_service_indicator_state *indicator =
      reinterpret_cast<fips_service_indicator_state *>(CRYPTO_get_thread_local(
          OPENSSL_THREAD_LOCAL_FIPS_SERVICE_INDICATOR_STATE));

  if (indicator == NULL) {
    indicator = reinterpret_cast<fips_service_indicator_state *>(
        OPENSSL_malloc(sizeof(struct fips_service_indicator_state)));
    if (indicator == NULL) {
      return NULL;
    }

    indicator->lock_state = STATE_UNLOCKED;
    indicator->counter = 0;

    if (!CRYPTO_set_thread_local(
            OPENSSL_THREAD_LOCAL_FIPS_SERVICE_INDICATOR_STATE, indicator,
            OPENSSL_free)) {
      OPENSSL_PUT_ERROR(CRYPTO, ERR_R_INTERNAL_ERROR);
      return NULL;
    }
  }

  return indicator;
}

static uint64_t service_indicator_get_counter(void) {
  struct fips_service_indicator_state *indicator = service_indicator_get();
  if (indicator == NULL) {
    return 0;
  }
  return indicator->counter;
}

uint64_t FIPS_service_indicator_before_call(void) {
  return service_indicator_get_counter();
}

uint64_t FIPS_service_indicator_after_call(void) {
  return service_indicator_get_counter();
}

void FIPS_service_indicator_update_state(void) {
  struct fips_service_indicator_state *indicator = service_indicator_get();
  if (indicator && indicator->lock_state == STATE_UNLOCKED) {
    indicator->counter++;
  }
}

void FIPS_service_indicator_lock_state(void) {
  struct fips_service_indicator_state *indicator = service_indicator_get();
  if (indicator == NULL) {
    return;
  }

  // |FIPS_service_indicator_lock_state| and
  // |FIPS_service_indicator_unlock_state| should not under/overflow in normal
  // operation. They are still checked and errors added to facilitate testing in
  // service_indicator_test.cc. This should only happen if lock/unlock are
  // called in an incorrect order or multiple times in the same function.
  const uint64_t new_state = indicator->lock_state + 1;
  if (new_state < indicator->lock_state) {
    // Overflow. This would imply that our call stack length has exceeded a
    // |uint64_t| which impossible on a 64-bit system.
    abort();
  }

  indicator->lock_state = new_state;
}

void FIPS_service_indicator_unlock_state(void) {
  struct fips_service_indicator_state *indicator = service_indicator_get();
  if (indicator == NULL) {
    return;
  }

  if (indicator->lock_state == 0) {
    abort();
  }

  indicator->lock_state--;
}

void AEAD_GCM_verify_service_indicator(const EVP_AEAD_CTX *ctx) {
  const size_t key_len = EVP_AEAD_key_length(ctx->aead);
  if (key_len == 16 || key_len == 32) {
    FIPS_service_indicator_update_state();
  }
}

void AEAD_CCM_verify_service_indicator(const EVP_AEAD_CTX *ctx) {
  if (EVP_AEAD_key_length(ctx->aead) == 16 && ctx->tag_len == 4) {
    FIPS_service_indicator_update_state();
  }
}

// is_ec_fips_approved returns one if the curve corresponding to the given NID
// is FIPS approved, and zero otherwise.
static int is_ec_fips_approved(int curve_nid) {
  switch (curve_nid) {
    case NID_secp224r1:
    case NID_X9_62_prime256v1:
    case NID_secp384r1:
    case NID_secp521r1:
      return 1;
    default:
      return 0;
  }
}

// is_md_fips_approved_for_signing returns one if the given message digest type
// is FIPS approved for signing, and zero otherwise.
static int is_md_fips_approved_for_signing(int md_type) {
  switch (md_type) {
    case NID_sha224:
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
    case NID_sha512_256:
      return 1;
    default:
      return 0;
  }
}

// is_md_fips_approved_for_verifying returns one if the given message digest
// type is FIPS approved for verifying, and zero otherwise.
static int is_md_fips_approved_for_verifying(int md_type) {
  switch (md_type) {
    case NID_sha224:
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
    case NID_sha512_256:
      return 1;
    default:
      return 0;
  }
}

static void evp_md_ctx_verify_service_indicator(const EVP_MD_CTX *ctx,
                                                int (*md_ok)(int md_type)) {
  if (EVP_MD_CTX_get0_md(ctx) == NULL) {
    // Signature schemes without a prehash are currently never FIPS approved.
    return;
  }

  EVP_PKEY_CTX *const pctx = ctx->pctx;
  const EVP_PKEY *const pkey = EVP_PKEY_CTX_get0_pkey(pctx);
  const int pkey_type = EVP_PKEY_id(pkey);
  const int md_type = EVP_MD_CTX_type(ctx);

  // EVP_PKEY_RSA_PSS SPKIs aren't supported.
  if (pkey_type == EVP_PKEY_RSA) {
    // Message digest used in the private key should be of the same type
    // as the given one, so we extract the MD type from the |EVP_PKEY|
    // and compare it with the type in |ctx|.
    const EVP_MD *pctx_md;
    if (!EVP_PKEY_CTX_get_signature_md(pctx, &pctx_md)) {
      goto err;
    }
    if (EVP_MD_type(pctx_md) != md_type) {
      goto err;
    }

    int padding;
    if (!EVP_PKEY_CTX_get_rsa_padding(pctx, &padding)) {
      goto err;
    }
    if (padding == RSA_PKCS1_PSS_PADDING) {
      int salt_len;
      const EVP_MD *mgf1_md;
      if (!EVP_PKEY_CTX_get_rsa_pss_saltlen(pctx, &salt_len) ||
          !EVP_PKEY_CTX_get_rsa_mgf1_md(pctx, &mgf1_md) ||
          (salt_len != -1 && salt_len != (int)EVP_MD_size(pctx_md)) ||
          EVP_MD_type(mgf1_md) != md_type) {
        // Only PSS where saltLen == hashLen is tested with ACVP. Cases with
        // non-standard padding functions are also excluded.
        goto err;
      }
    }

    // The approved RSA key sizes for signing are 2048, 3072 and 4096 bits.
    // Note: |EVP_PKEY_size| returns the size in bytes.
    size_t pkey_size = EVP_PKEY_size(ctx->pctx->pkey);

    // Check if the MD type and the RSA key size are approved.
    if (md_ok(md_type) &&
        (pkey_size == 256 || pkey_size == 384 || pkey_size == 512)) {
      FIPS_service_indicator_update_state();
    }
  } else if (pkey_type == EVP_PKEY_EC) {
    // Check if the MD type and the elliptic curve are approved.
    if (md_ok(md_type) &&
        is_ec_fips_approved(EC_GROUP_get_curve_name(
            EC_KEY_get0_group(EVP_PKEY_get0_EC_KEY(ctx->pctx->pkey))))) {
      FIPS_service_indicator_update_state();
    }
  }

err:
  // Ensure that junk errors aren't left on the queue.
  ERR_clear_error();
}

void EC_KEY_keygen_verify_service_indicator(const EC_KEY *eckey) {
  if (is_ec_fips_approved(EC_GROUP_get_curve_name(EC_KEY_get0_group(eckey)))) {
    FIPS_service_indicator_update_state();
  }
}

void ECDH_verify_service_indicator(const EC_KEY *ec_key) {
  if (is_ec_fips_approved(EC_GROUP_get_curve_name(EC_KEY_get0_group(ec_key)))) {
    FIPS_service_indicator_update_state();
  }
}

void EVP_Cipher_verify_service_indicator(const EVP_CIPHER_CTX *ctx) {
  switch (EVP_CIPHER_CTX_nid(ctx)) {
    case NID_aes_128_ecb:
    case NID_aes_192_ecb:
    case NID_aes_256_ecb:

    case NID_aes_128_cbc:
    case NID_aes_192_cbc:
    case NID_aes_256_cbc:

    case NID_aes_128_ctr:
    case NID_aes_192_ctr:
    case NID_aes_256_ctr:
      FIPS_service_indicator_update_state();
  }
}

void EVP_DigestVerify_verify_service_indicator(const EVP_MD_CTX *ctx) {
  return evp_md_ctx_verify_service_indicator(ctx,
                                             is_md_fips_approved_for_verifying);
}

void EVP_DigestSign_verify_service_indicator(const EVP_MD_CTX *ctx) {
  return evp_md_ctx_verify_service_indicator(ctx,
                                             is_md_fips_approved_for_signing);
}

void HMAC_verify_service_indicator(const EVP_MD *evp_md) {
  switch (EVP_MD_type(evp_md)) {
    case NID_sha1:
    case NID_sha224:
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
    case NID_sha512_256:
      FIPS_service_indicator_update_state();
      break;
  }
}

void TLSKDF_verify_service_indicator(const EVP_MD *md) {
  // HMAC-SHA{256, 384, 512} are approved for use in the KDF in TLS 1.2. These
  // Key Derivation functions are to be used in the context of the TLS protocol.
  switch (EVP_MD_type(md)) {
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
      FIPS_service_indicator_update_state();
      break;
  }
}

#else

uint64_t FIPS_service_indicator_before_call(void) { return 0; }

uint64_t FIPS_service_indicator_after_call(void) {
  // One is returned so that the return value is always greater than zero, the
  // return value of |FIPS_service_indicator_before_call|. This makes everything
  // report as "approved" in non-FIPS builds.
  return 1;
}

#endif  // BORINGSSL_FIPS
