// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// Ensure we can't call OPENSSL_malloc.
#define _BORINGSSL_PROHIBIT_OPENSSL_MALLOC
#include <openssl/crypto.h>
#include <openssl/service_indicator.h>
#include "internal.h"
#include "../pqdsa/internal.h"

const char *awslc_version_string(void) { return AWSLC_VERSION_STRING; }

int is_fips_build(void) {
#if defined(AWSLC_FIPS)
  return 1;
#else
  return 0;
#endif
}

#if defined(AWSLC_FIPS)

// Trampoline function to avoid ARM64 ADR range issues in large FIPS module.
// This function is intentionally not inlined to ensure the __FILE__ string
// literal reference stays close to the call site, avoiding the ±1MB PC-relative
// addressing limit of the ARM64 ADR instruction.
static OPENSSL_NOINLINE void put_set_thread_local_error(void) {
  OPENSSL_PUT_ERROR(CRYPTO, ERR_R_INTERNAL_ERROR);
}

#define STATE_UNLOCKED 0
#define TLS_MD_EXTENDED_MASTER_SECRET_CONST "extended master secret"
#define TLS_MD_EXTENDED_MASTER_SECRET_CONST_SIZE 22

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
      CRYPTO_get_thread_local(AWSLC_THREAD_LOCAL_FIPS_SERVICE_INDICATOR_STATE);

  if (indicator == NULL) {
    indicator = malloc(sizeof(struct fips_service_indicator_state));
    if (indicator == NULL) {
      return NULL;
    }

    indicator->lock_state = STATE_UNLOCKED;
    indicator->counter = 0;

    if (!CRYPTO_set_thread_local(
            AWSLC_THREAD_LOCAL_FIPS_SERVICE_INDICATOR_STATE, indicator, free)) {
      put_set_thread_local_error();
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

// |FIPS_service_indicator_lock_state| and |FIPS_service_indicator_unlock_state|
// should not under/overflow in normal operation. They are still checked and
// errors added to facilitate testing in service_indicator_test.cc This should
// only happen if lock/unlock are called in an incorrect order or multiple times
// in the same function.
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
  // We only have support for 128 bit and 256 bit keys for AES-GCM. AES-GCM is
  // approved only with an internal IV, see SP 800-38D Sec 8.2.2.
  // Note: |EVP_AEAD_key_length| returns the length in bytes.
  const size_t key_len = EVP_AEAD_key_length(ctx->aead);
  if (key_len == 16 || key_len == 32) {
    FIPS_service_indicator_update_state();
  }
}

void AEAD_CCM_verify_service_indicator(const EVP_AEAD_CTX *ctx) {
  // Only 128 bit keys with 32 bit tag lengths are approved for AES-CCM.
  // Note: |EVP_AEAD_key_length| returns the length in bytes.
  if (EVP_AEAD_key_length(ctx->aead) == 16 && ctx->tag_len == 4) {
    FIPS_service_indicator_update_state();
  }
}

void AES_CMAC_verify_service_indicator(const CMAC_CTX *ctx) {
  // Only 128 and 256 bit keys are approved for AES-CMAC.
  // Note: |key_len| is the length in bytes.
  const size_t key_len = ctx->cipher_ctx.key_len;
  if (key_len == 16 || key_len == 32) {
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
static int is_md_fips_approved_for_signing(int md_type, int pkey_type) {
  switch (md_type) {
    case NID_sha224:
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
    case NID_sha512_224:
    case NID_sha512_256:
    case NID_sha3_224:
    case NID_sha3_256:
    case NID_sha3_384:
    case NID_sha3_512:
      return 1;

      // [TODO] SHAKE is only approved for signing with RSA PSS
      // if (pkey_type == EVP_PKEY_RSA_PSS) // This will be needed when SHAKE is added
      //  return 1;
      //}
    default:
      return 0;
  }
}

// is_md_fips_approved_for_verifying returns one if the given message digest
// type is FIPS approved for verifying, and zero otherwise.
static int is_md_fips_approved_for_verifying(int md_type, int pkey_type) {
  switch (md_type) {
    case NID_sha1:
    case NID_sha224:
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
    case NID_sha512_224:
    case NID_sha512_256:
    case NID_sha3_224:
    case NID_sha3_256:
    case NID_sha3_384:
    case NID_sha3_512:
      return 1;

      // [TODO] SHAKE is only approved for signing with RSA PSS
      // if (pkey_type == EVP_PKEY_RSA_PSS) // This will be needed when SHAKE is added
      //  return 1;
      //}
    default:
      return 0;
  }
}

// mldsa_verify_service_indicator checks if the given PQDSA key uses an approved
// ML-DSA variant and updates the service indicator if so.
static void mldsa_verify_service_indicator(const EVP_PKEY *pkey) {
  if (pkey->type != EVP_PKEY_PQDSA) {
    return;
  }

  const PQDSA *pqdsa = PQDSA_KEY_get0_dsa(pkey->pkey.pqdsa_key);
  if (pqdsa == NULL) {
    return;
  }

  switch (pqdsa->nid) {
    case NID_MLDSA44:
    case NID_MLDSA65:
    case NID_MLDSA87:
      FIPS_service_indicator_update_state();
      break;
    default:
      break;
  }
}

// custom_meth_invoked checks whether custom crypto was invoked in the |meth|
// or |eckey_method| fields for a given |RSA| or |EC_KEY| respectively. For
// |RSA| keys, custom verify and sign functionality is supported. For |EC_KEY|
// keys, only custom sign functionality is supported.
// Returns one if custom crypto was invoked and zero otherwise.
static int custom_meth_invoked(const EVP_PKEY_CTX *ctx) {
  const int pkey_type = EVP_PKEY_id(ctx->pkey);
  switch (pkey_type) {
    case EVP_PKEY_RSA:
    case EVP_PKEY_RSA_PSS: {
      const RSA_METHOD *meth = ctx->pkey->pkey.rsa->meth;
      // Must be either |EVP_PKEY_OP_VERIFY| or |EVP_PKEY_OP_SIGN|
      if (ctx->operation == EVP_PKEY_OP_VERIFY) {
        return meth->verify_raw ? 1 : 0;
      }
      if(ctx->operation == EVP_PKEY_OP_SIGN) {
        // There are cases where custom |sign| functionality may be set but
        // not |sign_raw|. This check is more conservative and fails if
        // custom functionality is provided for either function pointer.
        return (meth->sign || meth->sign_raw) ? 1 : 0;
      }
      return 0;  // custom crypto can't be invoked for unsupported ops
    }

    case EVP_PKEY_EC: {
      if (ctx->operation == EVP_PKEY_OP_SIGN) {
        const EC_KEY_METHOD *meth = ctx->pkey->pkey.ec->eckey_method;
        return (meth->sign || meth->sign_sig) ? 1 : 0;
      }
      return 0;
    }

    default:
      // custom crypto can't be invoked for unsupported key types
      return 0;
  }
}

static void evp_md_ctx_verify_service_indicator(const EVP_MD_CTX *ctx,
                                                int rsa_1024_ok,
                                                int (*md_ok)(int md_type,
                                                             int pkey_type)) {
  if (EVP_MD_CTX_md(ctx) == NULL) {
    if(ctx->pctx->pkey->type == EVP_PKEY_ED25519) {
      // FIPS 186-5:
      //. 7.6 EdDSA Signature Generation
      //  7.7 EdDSA Signature Verification
      FIPS_service_indicator_update_state();
      return;
    }
    if(ctx->pctx->pkey->type == EVP_PKEY_PQDSA) {
      // FIPS 204: ML-DSA Signature Generation/Verification
      mldsa_verify_service_indicator(ctx->pctx->pkey);
      return;
    }
    // All other signature schemes without a prehash are currently never FIPS approved.
    goto err;
  }

  EVP_PKEY_CTX *const pctx = ctx->pctx;
  const EVP_PKEY *const pkey = EVP_PKEY_CTX_get0_pkey(pctx);
  const int pkey_type = EVP_PKEY_id(pkey);
  const int md_type = EVP_MD_CTX_type(ctx);

  if (pkey_type == EVP_PKEY_RSA || pkey_type == EVP_PKEY_RSA_PSS) {
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

    // The approved RSA key sizes for signing are key sizes >= 2048 bits and bits % 2 == 0.
    size_t n_bits = RSA_bits(ctx->pctx->pkey->pkey.rsa);

    // Check if the MD type and the RSA key size are approved. Also checking if
    // custom operations from |pkey.rsa->meth| were invoked.
    if (md_ok(md_type, pkey_type) &&
        ((rsa_1024_ok && n_bits == 1024) || (n_bits >= 2048 && n_bits % 2 == 0))
        && !custom_meth_invoked(pctx)) {
      FIPS_service_indicator_update_state();
    }
  } else if (pkey_type == EVP_PKEY_EC) {
    // Check if the MD type and the elliptic curve are approved. Also checking
    // if custom operations from |pkey.ec->eckey_method| were invoked.
    int curve_nid = EC_GROUP_get_curve_name(pkey->pkey.ec->group);
    if (md_ok(md_type, pkey_type) && is_ec_fips_approved(curve_nid) &&
        !custom_meth_invoked(pctx)) {
      FIPS_service_indicator_update_state();
    }
  }

err:
  // Ensure that junk errors aren't left on the queue.
  ERR_clear_error();
}

// Updates the service indicator if the elliptic curve contained in |eckey|
// is FIPS approved.
void EC_KEY_keygen_verify_service_indicator(const EC_KEY *eckey) {
  if (is_ec_fips_approved(EC_GROUP_get_curve_name(eckey->group))) {
    FIPS_service_indicator_update_state();
  }
}

void ECDH_verify_service_indicator(const EC_KEY *ec_key) {
  if (is_ec_fips_approved(EC_GROUP_get_curve_name(EC_KEY_get0_group(ec_key)))) {
    FIPS_service_indicator_update_state();
  }
}

void EVP_PKEY_keygen_verify_service_indicator(const EVP_PKEY *pkey) {
  if (pkey->type == EVP_PKEY_RSA || pkey->type == EVP_PKEY_RSA_PSS) {
    // The approved RSA key sizes for signing are key sizes >= 2048 bits and
    // bits % 2 == 0, though we check bits % 128 == 0 for consistency with
    // our RSA key generation.
    size_t n_bits = RSA_bits(pkey->pkey.rsa);
    if (n_bits >= 2048 && n_bits % 128 == 0) {
      FIPS_service_indicator_update_state();
    }
  } else if (pkey->type == EVP_PKEY_EC) {
    // Note: even though the function is called |EC_GROUP_get_curve_name|
    // it actually returns the NID of the curve rather than the name.
    int curve_nid = EC_GROUP_get_curve_name(pkey->pkey.ec->group);
    if (is_ec_fips_approved(curve_nid)) {
      FIPS_service_indicator_update_state();
    }
  } else if (pkey->type == EVP_PKEY_KEM) {
    const KEM *kem = KEM_KEY_get0_kem(pkey->pkey.kem_key);
    switch (kem->nid) {
      case NID_MLKEM512:
      case NID_MLKEM768:
      case NID_MLKEM1024:
        FIPS_service_indicator_update_state();
        break;
      default:
        break;
    }
  } else if (pkey->type == EVP_PKEY_ED25519) {
    FIPS_service_indicator_update_state();
  } else if (pkey->type == EVP_PKEY_PQDSA) {
    // FIPS 204: ML-DSA Key Generation
    mldsa_verify_service_indicator(pkey);
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

    case NID_aes_256_xts:
      FIPS_service_indicator_update_state();
  }
}

void EVP_DigestVerify_verify_service_indicator(const EVP_MD_CTX *ctx) {
  evp_md_ctx_verify_service_indicator(ctx, /*rsa_1024_ok=*/1,
                                      is_md_fips_approved_for_verifying);
}

void EVP_DigestSign_verify_service_indicator(const EVP_MD_CTX *ctx) {
  evp_md_ctx_verify_service_indicator(ctx, /*rsa_1024_ok=*/0,
                                      is_md_fips_approved_for_signing);
}

void HMAC_verify_service_indicator(const EVP_MD *evp_md) {
  switch (evp_md->type) {
    case NID_sha1:
    case NID_sha224:
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
    case NID_sha512_224:
    case NID_sha512_256:
    case NID_sha3_224:
    case NID_sha3_256:
    case NID_sha3_384:
    case NID_sha3_512:
      FIPS_service_indicator_update_state();
      break;
    default:
      break;
  }
}

void HKDF_verify_service_indicator(const EVP_MD *evp_md, const uint8_t *salt,
                                   size_t salt_len, size_t info_len) {
  // HKDF with SHA1, SHA224, SHA256, SHA384, and SHA512 are approved.
  //
  // FIPS 140 parameter requirements, per NIST SP 800-108 Rev. 1:
  //
  // It is recommended that the length of a KDK used by a KDF be at least as
  // large as the targeted security strength (in bits) of any application that
  // will be supported by the use of the derived keying material.
  //
  // We can't test for that, as we don't know what the HKDF output will be
  // used for.
  //
  // Per SP800-56Crev2, the salt cannot be empty in FIPS mode; a NULL salt
  // is replaced in the HMAC with a buffer filled with NUL bytes (0x00),
  // when |evp_md| is not NULL, so that's OK. Note that we reach this check
  // if |HMAC| succeeds which cannot be the case if |evp_md| == NULL; it would
  // have failed in |HMAC_Init_ex|.
  if ((salt != NULL && salt_len == 0) || (info_len == 0)) {
    return;
  }

  switch (evp_md->type) {
    case NID_sha1:
    case NID_sha224:
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
    case NID_sha512_224:
    case NID_sha512_256:
      FIPS_service_indicator_update_state();
      break;
    default:
      break;
  }
}

void HKDFExpand_verify_service_indicator(const EVP_MD *evp_md) {
  // HKDF_expand is a "KBKDF in Feedback Mode" per NIST SP800-108r1 with SHA 1,
  // SHA224, SHA256, SHA384, and SHA512 is approved.
  switch (evp_md->type) {
    case NID_sha1:
    case NID_sha224:
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
    case NID_sha512_224:
    case NID_sha512_256:
      FIPS_service_indicator_update_state();
      break;
    default:
      break;
  }
}
void PBKDF2_verify_service_indicator(const EVP_MD *evp_md, size_t password_len,
                                     size_t salt_len, unsigned iterations) {
  // PBKDF with SHA1, SHA224, SHA256, SHA384, and SHA512 are approved.
  //
  // FIPS 140 parameter requirements, per NIST SP800-132:
  //
  // * password_len >= 14 bytes (112 bits)
  // * salt_len >= 16 bytes (128 bits), assuming its randomly generated
  // * iterations "as large as possible, as long as the time required to
  //   generate the key using the entered password is acceptable for the users."
  //   (clearly we can't test for "as large as possible");
  //   NIST SP800-132 suggests >= 1000. For real-world implementations the
  //   actual iteration count should be much higher (at least hundreds of
  //   thousands), but as a general-purpose cryptographic library, AWS-LC
  //   can't make this decision.
  switch (evp_md->type) {
    case NID_sha1:
    case NID_sha224:
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
    case NID_sha512_224:
    case NID_sha512_256:
      if (password_len >= 14 && salt_len >= 16 && iterations >= 1000) {
        FIPS_service_indicator_update_state();
      }
      break;
    default:
      break;
  }
}

void SSHKDF_verify_service_indicator(const EVP_MD *evp_md) {
  // FIPS 180-4 allows SHA1, and SHA2.
  //
  // This KDF should only be called from SSH client/server code; it's not a
  // general-purpose KDF and is only Approved for FIPS 140-3 use specifically
  // in SSH. There's no way to test this requirement from here.
  switch (evp_md->type) {
    case NID_sha1:
    case NID_sha224:
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
      FIPS_service_indicator_update_state();
      break;
    default:
      break;
  }
}

void TLSKDF_verify_service_indicator(const EVP_MD *dgst, const char *label,
                                     size_t label_len) {
  // HMAC-MD5/HMAC-SHA1 (both used concurrently) is approved for use in the KDF
  // in TLS 1.0/1.1.
  if (dgst->type == NID_md5_sha1) {
    FIPS_service_indicator_update_state();
    return;
  }
  // HMAC-SHA{256, 384, 512} are approved for use in the KDF in TLS 1.2.
  // These Key Derivation functions are to be used in the context of the TLS
  // protocol. Only the label "extended master secret" is allowed because
  // it implies that the PRF is being used within a TLS 1.2 context.
  switch (dgst->type) {
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
      if (label_len >= TLS_MD_EXTENDED_MASTER_SECRET_CONST_SIZE &&
          memcmp(label, TLS_MD_EXTENDED_MASTER_SECRET_CONST,
                 TLS_MD_EXTENDED_MASTER_SECRET_CONST_SIZE) == 0) {
        FIPS_service_indicator_update_state();
      }
      break;
    default:
      break;
  }
}

void TLS13_KDF_verify_service_indicator(const EVP_MD *dgst) {
  // Per RFC 8446 and the corresponding FIPS validation, the TLS 1.3 KDF
  // (HKDF-Expand-Label) is approved with SHA2-256 and SHA2-384 as the
  // underlying HMAC hash.
  switch (dgst->type) {
    case NID_sha256:
    case NID_sha384:
      FIPS_service_indicator_update_state();
      break;
    default:
      break;
  }
}

// "Whenever a hash function is employed (including as the primitive used by HMAC), an
// approved hash function shall be used. FIPS 180 and FIPS 202 specify approved hash
// functions"
//
// * FIPS 180 covers the SHA-1 and SHA-2* family of algorithms
// * FIPS 202 covers the SHA3-* family of algorithms
//
// Sourced from NIST.SP.800-56Cr2 Section 7: Selecting Hash Functions and MAC Algorithms
// https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-56Cr2.pdf
void SSKDF_digest_verify_service_indicator(const EVP_MD *dgst) {
  switch (dgst->type) {
    case NID_sha1:
    case NID_sha224:
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
    case NID_sha512_224:
    case NID_sha512_256:
    case NID_sha3_224:
    case NID_sha3_256:
    case NID_sha3_384:
    case NID_sha3_512:
      FIPS_service_indicator_update_state();
      break;
    default:
      break;
  }
}

// "Whenever a hash function is employed (including as the primitive used by HMAC), an
// approved hash function shall be used. FIPS 180 and FIPS 202 specify approved hash
// functions"
//
// * FIPS 180 covers the SHA-1 and SHA-2* family of algorithms
// * FIPS 202 covers the SHA3-* family of algorithms (Note: AWS-LC does not currently support SHA-3 with HMAC)
//
// Sourced from NIST.SP.800-56Cr2 Section 7: Selecting Hash Functions and MAC Algorithms
// https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-56Cr2.pdf
void SSKDF_hmac_verify_service_indicator(const EVP_MD *dgst) {
  switch (dgst->type) {
    case NID_sha1:
    case NID_sha224:
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
    case NID_sha512_224:
    case NID_sha512_256:
      FIPS_service_indicator_update_state();
      break;
    default:
      break;
  }
}

// "For key derivation, this Recommendation approves the use of the keyed-Hash Message
// Authentication Code (HMAC) specified in FIPS 198-1".

// * FIPS 198-1 references FIPS 180-3 which covers the SHA-1 and SHA-2* family of algorithms
// * NIST also provides ACVP vectors for SHA3-* family of algorithms but our HMAC does not support this
//
// Sourced from NIST SP 800-108r1-upd1 Section 3:  Pseudorandom Function (PRF)
// https://doi.org/10.6028/NIST.SP.800-108r1-upd1
void KBKDF_ctr_hmac_verify_service_indicator(const EVP_MD *dgst, size_t secret_len) {
  switch (dgst->type) {
    case NID_sha1:
    case NID_sha224:
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
    case NID_sha512_224:
    case NID_sha512_256:
      // SP 800-131Ar1, Section 8: "The length of the key-derivation key shall be at least 112 bits.” 
      if (secret_len >= 14) {
        FIPS_service_indicator_update_state();
      }
      break;
    default:
      break;
  }
}

void EVP_PKEY_encapsulate_verify_service_indicator(const EVP_PKEY_CTX* ctx) {
  if (ctx->pkey->type == EVP_PKEY_KEM) {
    const KEM *kem = KEM_KEY_get0_kem(ctx->pkey->pkey.kem_key);
    switch (kem->nid) {
      case NID_MLKEM512:
      case NID_MLKEM768:
      case NID_MLKEM1024:
        FIPS_service_indicator_update_state();
        break;
      default:
        break;
    }
  }
}

void EVP_PKEY_decapsulate_verify_service_indicator(const EVP_PKEY_CTX* ctx) {
  if (ctx->pkey->type == EVP_PKEY_KEM) {
    const KEM *kem = KEM_KEY_get0_kem(ctx->pkey->pkey.kem_key);
    switch (kem->nid) {
      case NID_MLKEM512:
      case NID_MLKEM768:
      case NID_MLKEM1024:
        FIPS_service_indicator_update_state();
        break;
      default:
        break;
    }
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

#endif  // AWSLC_FIPS
