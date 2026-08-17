// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/hmac.h>

#include <assert.h>
#include <string.h>

#include <openssl/digest.h>
#include <openssl/mem.h>

#include "internal.h"
#include "../../internal.h"
#include "../service_indicator/internal.h"

#include "../md5/internal.h"
#include "../sha/internal.h"

typedef int (*HashInit)(void *);
typedef int (*HashUpdate)(void *, const void *, size_t);
typedef int (*HashFinal)(uint8_t *, void *);
typedef int (*HashInitFromState)(void *, const uint8_t *, uint64_t);
typedef int (*HashGetState)(void *, uint8_t *, uint64_t *);

struct hmac_methods_st {
  const EVP_MD* evp_md;
  size_t chaining_length;  // chaining length in bytes
  HashInit init;
  HashUpdate update;
  HashFinal finalize;  // Not named final to avoid keywords
  HashInitFromState init_from_state;
  HashGetState get_state;
};

// We need trampolines from the generic void* methods we use to the properly typed underlying methods.
// Without these methods some control flow integrity checks will fail because the function pointer types
// do not exactly match the destination functions. (Namely function pointers use void* pointers for the contexts)
// while the destination functions have specific pointer types for the relevant contexts.
//
// This also includes hash-specific static assertions as they can be added.
#define MD_TRAMPOLINES_EXPLICIT(HASH_NAME, HASH_CTX, HASH_CBLOCK)             \
  static int AWS_LC_TRAMPOLINE_##HASH_NAME##_Init(void *);                    \
  static int AWS_LC_TRAMPOLINE_##HASH_NAME##_Update(void *, const void *,     \
                                                    size_t);                  \
  static int AWS_LC_TRAMPOLINE_##HASH_NAME##_Final(uint8_t *, void *);        \
  static int AWS_LC_TRAMPOLINE_##HASH_NAME##_Init(void *ctx) {                \
    return HASH_NAME##_Init((HASH_CTX *)ctx);                                 \
  }                                                                           \
  static int AWS_LC_TRAMPOLINE_##HASH_NAME##_Update(                          \
      void *ctx, const void *key, size_t key_len) {                           \
    return HASH_NAME##_Update((HASH_CTX *)ctx, key, key_len);                 \
  }                                                                           \
  static int AWS_LC_TRAMPOLINE_##HASH_NAME##_Final(uint8_t *out, void *ctx) { \
    return HASH_NAME##_Final(out, (HASH_CTX *)ctx);                           \
  }                                                                           \
  OPENSSL_STATIC_ASSERT(HASH_CBLOCK % 8 == 0,                                 \
                        HASH_NAME##_has_blocksize_not_divisible_by_eight_t)   \
  OPENSSL_STATIC_ASSERT(HASH_CBLOCK <= EVP_MAX_MD_BLOCK_SIZE,                 \
                        HASH_NAME##_has_overlarge_blocksize_t)                \
  OPENSSL_STATIC_ASSERT(sizeof(HASH_CTX) <= sizeof(union md_ctx_union),       \
                        HASH_NAME##_has_overlarge_context_t)

// For merkle-damgard constructions, we also define functions for importing and
// exporting hash state for precomputed keys. These are not applicable to
// Keccak/SHA3.
#define MD_TRAMPOLINES_EXPLICIT_PRECOMPUTED(HASH_NAME, HASH_CTX, HASH_CBLOCK) \
  MD_TRAMPOLINES_EXPLICIT(HASH_NAME, HASH_CTX, HASH_CBLOCK);                  \
  static int AWS_LC_TRAMPOLINE_##HASH_NAME##_Init_from_state(                 \
      void *ctx, const uint8_t *h, uint64_t n) {                              \
    return HASH_NAME##_Init_from_state((HASH_CTX *)ctx, h, n);                \
  }                                                                           \
  static int AWS_LC_TRAMPOLINE_##HASH_NAME##_get_state(                       \
      void *ctx, uint8_t *out_h, uint64_t *out_n) {                           \
    return HASH_NAME##_get_state((HASH_CTX *)ctx, out_h, out_n);              \
  }                                                                           \
  OPENSSL_STATIC_ASSERT(HMAC_##HASH_NAME##_PRECOMPUTED_KEY_SIZE ==            \
                            2 * HASH_NAME##_CHAINING_LENGTH,                  \
                        HASH_NAME##_has_incorrect_precomputed_key_size)       \
  OPENSSL_STATIC_ASSERT(HMAC_##HASH_NAME##_PRECOMPUTED_KEY_SIZE <=            \
                            HMAC_MAX_PRECOMPUTED_KEY_SIZE,                    \
                        HASH_NAME##_has_too_large_precomputed_key_size)       \

// The maximum number of HMAC implementations
#define HMAC_METHOD_MAX 12

MD_TRAMPOLINES_EXPLICIT_PRECOMPUTED(MD5, MD5_CTX, MD5_CBLOCK)
MD_TRAMPOLINES_EXPLICIT_PRECOMPUTED(SHA1, SHA_CTX, SHA_CBLOCK)
MD_TRAMPOLINES_EXPLICIT_PRECOMPUTED(SHA224, SHA256_CTX, SHA256_CBLOCK)
MD_TRAMPOLINES_EXPLICIT_PRECOMPUTED(SHA256, SHA256_CTX, SHA256_CBLOCK)
MD_TRAMPOLINES_EXPLICIT_PRECOMPUTED(SHA384, SHA512_CTX, SHA512_CBLOCK)
MD_TRAMPOLINES_EXPLICIT_PRECOMPUTED(SHA512, SHA512_CTX, SHA512_CBLOCK)
MD_TRAMPOLINES_EXPLICIT_PRECOMPUTED(SHA512_224, SHA512_CTX, SHA512_CBLOCK)
MD_TRAMPOLINES_EXPLICIT_PRECOMPUTED(SHA512_256, SHA512_CTX, SHA512_CBLOCK)
MD_TRAMPOLINES_EXPLICIT(SHA3_224, KECCAK1600_CTX, SHA3_224_CBLOCK)
MD_TRAMPOLINES_EXPLICIT(SHA3_256, KECCAK1600_CTX, SHA3_256_CBLOCK)
MD_TRAMPOLINES_EXPLICIT(SHA3_384, KECCAK1600_CTX, SHA3_384_CBLOCK)
MD_TRAMPOLINES_EXPLICIT(SHA3_512, KECCAK1600_CTX, SHA3_512_CBLOCK)

struct hmac_method_array_st {
  HmacMethods methods[HMAC_METHOD_MAX];
};

// This macro does not set any values for precomputed keys for portable state,
// and as such is suitable for use with Keccak/SHA3.
#define DEFINE_IN_PLACE_METHODS(EVP_MD, HASH_NAME)  {                        \
    out->methods[idx].evp_md = EVP_MD;                                       \
    out->methods[idx].init = AWS_LC_TRAMPOLINE_##HASH_NAME##_Init;           \
    out->methods[idx].update = AWS_LC_TRAMPOLINE_##HASH_NAME##_Update;       \
    out->methods[idx].finalize = AWS_LC_TRAMPOLINE_##HASH_NAME##_Final;      \
    out->methods[idx].chaining_length = 0UL;                                 \
    out->methods[idx].init_from_state = NULL;                                \
    out->methods[idx].get_state = NULL;                                      \
    idx++;                                                                   \
    assert(idx <= HMAC_METHOD_MAX);                                          \
  }

// Use |idx-1| because DEFINE_IN_PLACE_METHODS has already incremented it.
#define DEFINE_IN_PLACE_METHODS_PRECOMPUTED(EVP_MD, HASH_NAME)  {            \
    DEFINE_IN_PLACE_METHODS(EVP_MD, HASH_NAME);                              \
    assert(idx-1 >= 0);                                                      \
    out->methods[idx-1].chaining_length = HASH_NAME##_CHAINING_LENGTH;       \
    out->methods[idx-1].init_from_state =                                    \
        AWS_LC_TRAMPOLINE_##HASH_NAME##_Init_from_state;                     \
    out->methods[idx-1].get_state =                                          \
        AWS_LC_TRAMPOLINE_##HASH_NAME##_get_state;                           \
  }

DEFINE_LOCAL_DATA(struct hmac_method_array_st, AWSLC_hmac_in_place_methods) {
  OPENSSL_memset((void*) out->methods, 0, sizeof(out->methods));
  int idx = 0;
  // Since we search these linearly it helps (just a bit) to put the most common ones first.
  // This isn't based on hard metrics and will not make a significant different on performance.
  DEFINE_IN_PLACE_METHODS_PRECOMPUTED(EVP_sha256(), SHA256);
  DEFINE_IN_PLACE_METHODS_PRECOMPUTED(EVP_sha1(), SHA1);
  DEFINE_IN_PLACE_METHODS_PRECOMPUTED(EVP_sha384(), SHA384);
  DEFINE_IN_PLACE_METHODS_PRECOMPUTED(EVP_sha512(), SHA512);
  DEFINE_IN_PLACE_METHODS_PRECOMPUTED(EVP_md5(), MD5);
  DEFINE_IN_PLACE_METHODS_PRECOMPUTED(EVP_sha224(), SHA224);
  DEFINE_IN_PLACE_METHODS_PRECOMPUTED(EVP_sha512_224(), SHA512_224);
  DEFINE_IN_PLACE_METHODS_PRECOMPUTED(EVP_sha512_256(), SHA512_256);
  DEFINE_IN_PLACE_METHODS(EVP_sha3_224(), SHA3_224);
  DEFINE_IN_PLACE_METHODS(EVP_sha3_256(), SHA3_256);
  DEFINE_IN_PLACE_METHODS(EVP_sha3_384(), SHA3_384);
  DEFINE_IN_PLACE_METHODS(EVP_sha3_512(), SHA3_512);
}

static const HmacMethods *GetInPlaceMethods(const EVP_MD *evp_md) {
  const struct hmac_method_array_st *method_array = AWSLC_hmac_in_place_methods();
  const HmacMethods *methods = method_array->methods;
  for (size_t idx = 0; idx < sizeof(method_array->methods) / sizeof(struct hmac_methods_st); idx++) {
    if (methods[idx].evp_md == evp_md) {
      return &methods[idx];
    }
  }
  return NULL;
}

// ctx->state has the following possible states
// (Pre/Post conditions):
// HMAC_STATE_UNINITIALIZED: Uninitialized.
// HMAC_STATE_INIT_NO_DATA: Initialized with an md and key. No data processed.
//    This means that if init is called but nothing changes, we don't need to reset our state.
// HMAC_STATE_IN_PROGRESS: Initialized with an md and key. Data processed.
//    This means that if init is called we do need to reset state.
// HMAC_STATE_READY_NEEDS_INIT: Identical to state HMAC_STATE_INIT_NO_DATA but API contract requires that Init be called prior to use.
//    This is an optimization because we can leave the context in a state ready for use after completion.
// HMAC_STATE_PRECOMPUTED_KEY_EXPORT_READY: Identical to state HMAC_STATE_READY_NEEDS_INIT but marked to allow precompute key export
//    This state is treated as HMAC_STATE_READY_NEEDS_INIT by Init/Update/Final.
//    This state is the only state that in which a precompute key can be exported.
//    This state is set by HMAC_set_precomputed_key_export.
// other: Invalid state and likely a result of using unitialized memory. Treated the same as 0.
//
// While we are within HMAC methods we allow for the state value and actual state of the context to diverge.

// HMAC_STATE_UNINITIALIZED *MUST* remain `0` so that callers can do `HMAC_CTX ctx = {0};` to get a usable context.
#define HMAC_STATE_UNINITIALIZED 0
#define HMAC_STATE_INIT_NO_DATA 1
#define HMAC_STATE_IN_PROGRESS 2
#define HMAC_STATE_READY_NEEDS_INIT 3
#define HMAC_STATE_PRECOMPUTED_KEY_EXPORT_READY 4

// Static assertion to ensure that no one has changed the value of HMAC_STATE_UNINITIALIZED.
// This really must stay with a zero value.
OPENSSL_STATIC_ASSERT(HMAC_STATE_UNINITIALIZED == 0, HMAC_STATE_UNINITIALIZED_is_not_zero_t)

// Indicates that a context has the md and methods configured and is ready to use
#define hmac_ctx_is_initialized(ctx) ((HMAC_STATE_INIT_NO_DATA == (ctx)->state || HMAC_STATE_IN_PROGRESS == (ctx)->state))

uint8_t *HMAC(const EVP_MD *evp_md, const void *key, size_t key_len,
              const uint8_t *data, size_t data_len, uint8_t *out,
              unsigned int *out_len) {

  if (out == NULL) {
    // Prevent further work from being done
    return NULL;
  }

  HMAC_CTX ctx;
  OPENSSL_memset(&ctx, 0, sizeof(HMAC_CTX));
  int result;

  // We have to avoid the underlying SHA services updating the indicator
  // state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  result = HMAC_Init_ex(&ctx, key, key_len, evp_md, NULL) &&
           HMAC_Update(&ctx, data, data_len) &&
           HMAC_Final(&ctx, out, out_len);

  FIPS_service_indicator_unlock_state();

  // Regardless of our success we need to zeroize our working state.
  HMAC_CTX_cleanup(&ctx);
  if (result) {
    HMAC_verify_service_indicator(evp_md);
    return out;
  } else {
    OPENSSL_cleanse(out, EVP_MD_size(evp_md));
    return NULL;
  }
}

uint8_t *HMAC_with_precompute(const EVP_MD *evp_md, const void *key,
                              size_t key_len, const uint8_t *data,
                              size_t data_len, uint8_t *out,
                              unsigned int *out_len) {
  if (out == NULL) {
    // Prevent further work from being done
    return NULL;
  }

  HMAC_CTX ctx;
  OPENSSL_memset(&ctx, 0, sizeof(HMAC_CTX));
  int result;

  // We have to avoid the underlying SHA services updating the indicator
  // state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  uint8_t precomputed_key[HMAC_MAX_PRECOMPUTED_KEY_SIZE];
  size_t precomputed_key_len = HMAC_MAX_PRECOMPUTED_KEY_SIZE;

  result =
      HMAC_Init_ex(&ctx, key, key_len, evp_md, NULL) &&
      HMAC_set_precomputed_key_export(&ctx) &&
      HMAC_get_precomputed_key(&ctx, precomputed_key, &precomputed_key_len) &&
      HMAC_Init_from_precomputed_key(&ctx, precomputed_key, precomputed_key_len,
                                     evp_md) &&
      HMAC_Update(&ctx, data, data_len) &&
      HMAC_Final(&ctx, out, out_len);

  FIPS_service_indicator_unlock_state();

  // Regardless of our success we need to zeroize our working state.
  HMAC_CTX_cleanup(&ctx);
  OPENSSL_cleanse(precomputed_key, HMAC_MAX_PRECOMPUTED_KEY_SIZE);
  if (result) {
    // Contrary to what happens in the |HMAC| function, we do not update the
    // service indicator here (i.e., we do not call
    // |HMAC_verify_service_indicator|), because the function
    // |HMAC_with_precompute| is not FIPS-approved per se and is only used in
    // tests.
    return out;
  } else {
    OPENSSL_cleanse(out, EVP_MD_size(evp_md));
    return NULL;
  }
}

void HMAC_CTX_init(HMAC_CTX *ctx) {
  OPENSSL_memset(ctx, 0, sizeof(HMAC_CTX));
}

HMAC_CTX *HMAC_CTX_new(void) {
  HMAC_CTX *ctx = OPENSSL_zalloc(sizeof(HMAC_CTX));
  if (ctx != NULL) {
    // NO-OP: struct already zeroed
    //HMAC_CTX_init(ctx);
  }
  return ctx;
}

void HMAC_CTX_cleanup(HMAC_CTX *ctx) {
  // All of the contexts are flat and can simply be zeroed
  OPENSSL_cleanse(ctx, sizeof(HMAC_CTX));
}

void HMAC_CTX_cleanse(HMAC_CTX *ctx) {
  HMAC_CTX_cleanup(ctx);
}

void HMAC_CTX_free(HMAC_CTX *ctx) {
  if (ctx == NULL) {
    return;
  }

  HMAC_CTX_cleanup(ctx);
  OPENSSL_free(ctx);
}

// hmac_ctx_set_md_methods is used to set ctx->methods and ctx->md from md.
// It is called as part of the initialization of the ctx (HMAC_Init_*).
// It returns one on success, and zero otherwise.
static int hmac_ctx_set_md_methods(HMAC_CTX *ctx, const EVP_MD *md) {
  if (md && (HMAC_STATE_UNINITIALIZED == ctx->state || ctx->md != md)) {
    // The MD has changed
    ctx->methods = GetInPlaceMethods(md);
    if (ctx->methods == NULL) {
      // Unsupported md
      return 0;
    }
    ctx->md = md;
  } else if (!hmac_ctx_is_initialized(ctx)) {
    // We are not initialized but have not been provided with an md to
    // initialize ourselves with.
    return 0;
  }

  return 1;
}

int HMAC_Init_ex(HMAC_CTX *ctx, const void *key, size_t key_len,
                 const EVP_MD *md, ENGINE *impl) {
  assert(impl == NULL);

  GUARD_PTR(ctx);

  // HMAC does not support SHAKE (XOF) algorithms
  if (md && (EVP_MD_flags(md) & EVP_MD_FLAG_XOF)) {
    OPENSSL_PUT_ERROR(HMAC, HMAC_R_UNSUPPORTED_DIGEST);
    return 0;
  }

  if (HMAC_STATE_READY_NEEDS_INIT == ctx->state ||
      HMAC_STATE_PRECOMPUTED_KEY_EXPORT_READY == ctx->state) {
    ctx->state = HMAC_STATE_INIT_NO_DATA;  // Mark that init has been called
  }

  if (hmac_ctx_is_initialized(ctx)) {
    // TODO(davidben,eroman): Passing the previous |md| with a NULL |key| is
    // ambiguous between using the empty key and reusing the previous key. There
    // exist callers which intend the latter, but the former is an awkward edge
    // case. Fix to API to avoid this.
    if (key == NULL && (md == NULL || md == ctx->md)) {
      if(HMAC_STATE_IN_PROGRESS == ctx->state) {
        // Reinitialize |md_ctx| from |i_ctx| to start fresh with the same key. This
        // is the same behavior as Openssl.
        OPENSSL_memcpy(&ctx->md_ctx, &ctx->i_ctx, sizeof(ctx->i_ctx));
      }
      // If nothing is changing then we can return without doing any further work.
      ctx->state = HMAC_STATE_INIT_NO_DATA;
      return 1;
    }
  }

  // At this point we *know* we need to change things and rekey because either the key has changed
  // or the md and they key has changed.
  // (It is a misuse to just change the md so we also assume that the key changes when the md changes.)

  if (!hmac_ctx_set_md_methods(ctx, md)) {
    return 0;
  }

  // At this point we know we have valid methods and a context allocated.
  const HmacMethods *methods = ctx->methods;
  size_t block_size = EVP_MD_block_size(methods->evp_md);
  assert(block_size % 8 == 0);
  assert(block_size <= EVP_MAX_MD_BLOCK_SIZE);

  // We have to avoid the underlying SHA services updating the indicator
  // state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  int result = 0;

  uint64_t pad[EVP_MAX_MD_BLOCK_SIZE / sizeof(uint64_t)] = {0};
  uint64_t key_block[EVP_MAX_MD_BLOCK_SIZE / sizeof(uint64_t)] = {0};
  if (block_size < key_len) {
    // Long keys are hashed.
    if (!methods->init(&ctx->md_ctx) ||
        !methods->update(&ctx->md_ctx, key, key_len) ||
        !methods->finalize((uint8_t *) key_block, &ctx->md_ctx)) {
      goto end;
    }
  } else {
    assert(key_len <= sizeof(key_block));
    OPENSSL_memcpy(key_block, key, key_len);
  }
  for (size_t i = 0; i < block_size / 8; i++) {
    pad[i] = 0x3636363636363636 ^ key_block[i];
  }
  if (!methods->init(&ctx->i_ctx) ||
      !methods->update(&ctx->i_ctx, pad, block_size)) {
    goto end;
  }
  for (size_t i = 0; i < block_size / 8; i++) {
    pad[i] = 0x5c5c5c5c5c5c5c5c ^ key_block[i];
  }
  if (!methods->init(&ctx->o_ctx) ||
      !methods->update(&ctx->o_ctx, pad, block_size)) {
    goto end;
  }

  OPENSSL_memcpy(&ctx->md_ctx, &ctx->i_ctx, sizeof(ctx->i_ctx));
  ctx->state = HMAC_STATE_INIT_NO_DATA;

  result = 1;
end:
  OPENSSL_cleanse(pad, EVP_MAX_MD_BLOCK_SIZE);
  OPENSSL_cleanse(key_block, EVP_MAX_MD_BLOCK_SIZE);
  FIPS_service_indicator_unlock_state();
  if (result != 1) {
    // We're in some error state, so return our context to a known and well defined zero state.
    HMAC_CTX_cleanup(ctx);
  }
  return result;
}

int HMAC_Update(HMAC_CTX *ctx, const uint8_t *data, size_t data_len) {
  if (!hmac_ctx_is_initialized(ctx)) {
    return 0;
  }
  ctx->state = HMAC_STATE_IN_PROGRESS;
  return ctx->methods->update(&ctx->md_ctx, data, data_len);
}

int HMAC_Final(HMAC_CTX *ctx, uint8_t *out, unsigned int *out_len) {
  if (out == NULL) {
    return 0;
  }

  const HmacMethods *methods = ctx->methods;
  if (!hmac_ctx_is_initialized(ctx)) {
    return 0;
  }
  // We have to avoid the underlying SHA services updating the indicator
  // state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  int result = 0;
  const EVP_MD *evp_md = ctx->md;
  int hmac_len = EVP_MD_size(evp_md);
  uint8_t tmp[EVP_MAX_MD_SIZE];
  if (!methods->finalize(tmp, &ctx->md_ctx)) {
    goto end;
  }
  OPENSSL_memcpy(&ctx->md_ctx, &ctx->o_ctx, sizeof(ctx->o_ctx));
  if (!ctx->methods->update(&ctx->md_ctx, tmp, hmac_len)) {
    goto end;
  }
  result = methods->finalize(out, &ctx->md_ctx);
  // Wipe out working state by initializing for next use
  OPENSSL_memcpy(&ctx->md_ctx, &ctx->i_ctx, sizeof(ctx->i_ctx));
  ctx->state = HMAC_STATE_READY_NEEDS_INIT; // Mark that we are ready for use but still need HMAC_Init_ex called.
end:
  // Cleanse sensitive intermediate inner hash from the stack.
  OPENSSL_cleanse(tmp, sizeof(tmp));
  FIPS_service_indicator_unlock_state();
  if (result) {
    HMAC_verify_service_indicator(evp_md);
    if (out_len) {
      *out_len = hmac_len;
    }
    return 1;
  } else {
    // On error, return context to a known and well-defined zero state.
    HMAC_CTX_cleanup(ctx);
    if (out_len) {
      *out_len = 0;
    }
    return 0;
  }
}

size_t HMAC_size(const HMAC_CTX *ctx) { return EVP_MD_size(ctx->md); }

const EVP_MD *HMAC_CTX_get_md(const HMAC_CTX *ctx) { return ctx->md; }

int HMAC_CTX_copy_ex(HMAC_CTX *dest, const HMAC_CTX *src) {
  OPENSSL_memcpy(dest, src, sizeof(HMAC_CTX));
  return 1;
}

void HMAC_CTX_reset(HMAC_CTX *ctx) {
  HMAC_CTX_cleanup(ctx);
  // Cleanup intrinsicly inits to all zeros which is valid
}

int HMAC_set_precomputed_key_export(HMAC_CTX *ctx) {
  GUARD_PTR(ctx);
  if (ctx->methods != NULL && ctx->methods->get_state == NULL) {
    OPENSSL_PUT_ERROR(HMAC, HMAC_R_PRECOMPUTED_KEY_NOT_SUPPORTED_FOR_DIGEST);
    return 0;
  }
  if (HMAC_STATE_INIT_NO_DATA != ctx->state &&
      HMAC_STATE_PRECOMPUTED_KEY_EXPORT_READY != ctx->state) {
    // HMAC_set_precomputed_key_export can only be called after Hmac_Init_*
    OPENSSL_PUT_ERROR(HMAC, HMAC_R_NOT_CALLED_JUST_AFTER_INIT);
    return 0;
  }
  ctx->state = HMAC_STATE_PRECOMPUTED_KEY_EXPORT_READY;
  return 1;
}

int HMAC_get_precomputed_key(HMAC_CTX *ctx, uint8_t *out, size_t *out_len) {
  GUARD_PTR(ctx);
  GUARD_PTR(ctx->methods);
  if (ctx->methods->get_state == NULL) {
    OPENSSL_PUT_ERROR(HMAC, HMAC_R_PRECOMPUTED_KEY_NOT_SUPPORTED_FOR_DIGEST);
    return 0;
  }

  if (HMAC_STATE_PRECOMPUTED_KEY_EXPORT_READY != ctx->state) {
    OPENSSL_PUT_ERROR(EVP, HMAC_R_SET_PRECOMPUTED_KEY_EXPORT_NOT_CALLED);
    return 0;
  }

  if (NULL == out_len) {
    OPENSSL_PUT_ERROR(EVP, HMAC_R_MISSING_PARAMETERS);
    return 0;
  }

  const size_t chaining_length = ctx->methods->chaining_length;
  size_t actual_out_len = chaining_length * 2;
  assert(actual_out_len <= HMAC_MAX_PRECOMPUTED_KEY_SIZE);
  if (NULL == out) {
    // When out is NULL, we just set out_len.
    // We keep the state as HMAC_STATE_PRECOMPUTED_KEY_EXPORT_READY
    // to allow an actual export of the precomputed key immediately afterward.
    *out_len = actual_out_len;
    return 1;
  }
  // When out is not NULL, we need to check that *out_len is large enough
  if (*out_len < actual_out_len) {
    OPENSSL_PUT_ERROR(HMAC, HMAC_R_BUFFER_TOO_SMALL);
    return 0;
  }
  *out_len = actual_out_len;

  uint64_t i_ctx_n;
  // Initializing o_ctx_n to zero to remove warning from Windows ARM64 compiler
  // "error : variable 'o_ctx_n' is used uninitialized whenever '&&' condition
  // is false". Note this should not be necessary because get_state cannot fail.
  uint64_t o_ctx_n = 0;

  if (!ctx->methods->get_state(&ctx->i_ctx, out, &i_ctx_n) ||
      !ctx->methods->get_state(&ctx->o_ctx, out + chaining_length, &o_ctx_n)) {
    // get_state should always succeed since i_ctx/o_ctx have processed exactly
    // one block, but handle failure defensively.
    assert(0); // Should never happen
    OPENSSL_cleanse(out, actual_out_len);
    return 0;
  }

  // Sanity check: we must have processed a single block at this time
  size_t block_size = EVP_MD_block_size(ctx->md);
  if (8 * block_size != i_ctx_n || 8 * block_size != o_ctx_n) {
    assert(0); // Should never happen
    OPENSSL_cleanse(out, actual_out_len);
    return 0;
  }

  // The context is ready to be used to compute HMAC values at this point.
  ctx->state = HMAC_STATE_INIT_NO_DATA;

  return 1;
}

int HMAC_Init_from_precomputed_key(HMAC_CTX *ctx,
                                   const uint8_t *precomputed_key,
                                   size_t precomputed_key_len,
                                   const EVP_MD *md) {

  // HMAC does not support SHAKE (XOF) algorithms
  if (md && (EVP_MD_flags(md) & EVP_MD_FLAG_XOF)) {
    OPENSSL_PUT_ERROR(HMAC, HMAC_R_UNSUPPORTED_DIGEST);
    return 0;
  }

  if (HMAC_STATE_READY_NEEDS_INIT == ctx->state ||
      HMAC_STATE_PRECOMPUTED_KEY_EXPORT_READY == ctx->state) {
    ctx->state = HMAC_STATE_INIT_NO_DATA;  // Mark that init has been called
  }

  if (HMAC_STATE_INIT_NO_DATA == ctx->state) {
    if (precomputed_key == NULL && (md == NULL || md == ctx->md)) {
      // If nothing is changing then we can return without doing any further
      // work.
      return 1;
    }
  }

  // Now we assume that we need to re-initialize everything.
  // See HMAC_Init_ex

  if (!hmac_ctx_set_md_methods(ctx, md)) {
    return 0;
  }

  const HmacMethods *methods = ctx->methods;
  if (ctx->methods->init_from_state == NULL) {
    OPENSSL_PUT_ERROR(HMAC, HMAC_R_PRECOMPUTED_KEY_NOT_SUPPORTED_FOR_DIGEST);
    return 0;
  }

  const size_t chaining_length = methods->chaining_length;
  const size_t block_size = EVP_MD_block_size(methods->evp_md);
  assert(block_size <= EVP_MAX_MD_BLOCK_SIZE);
  assert(2 * chaining_length <= HMAC_MAX_PRECOMPUTED_KEY_SIZE);

  if (2 * chaining_length != precomputed_key_len) {
    return 0;
  }

  // We require precomputed_key to be non-NULL, since here md changed
  if (NULL == precomputed_key) {
    OPENSSL_PUT_ERROR(HMAC, HMAC_R_MISSING_PARAMETERS);
    return 0;
  }

  // We have to avoid the underlying SHA services updating the indicator
  // state, so we lock the state here. Technically this is not really needed,
  // because the functions we call should not update the indicator state.
  // But this is safer.
  FIPS_service_indicator_lock_state();
  int result = 0;

  // Initialize i_ctx from the state stored in the first part of precomputed_key
  // Recall that i_ctx is the state of the hash function after processing
  // one block (ipad xor keyOrHashedKey)
  if (!methods->init_from_state(&ctx->i_ctx, precomputed_key, block_size * 8)) {
    goto end;
  }

  // Same for o_ctx using the second part of precomputed_key
  if (!methods->init_from_state(&ctx->o_ctx, precomputed_key + chaining_length,
                                block_size * 8)) {
    goto end;
  }

  OPENSSL_memcpy(&ctx->md_ctx, &ctx->i_ctx, sizeof(ctx->i_ctx));
  ctx->state = HMAC_STATE_INIT_NO_DATA;

  result = 1;
end:
  FIPS_service_indicator_unlock_state();
  if (result != 1) {
    // We're in some error state, so return our context to a known and
    // well-defined zero state.
    HMAC_CTX_cleanup(ctx);
  }
  return result;
}

int HMAC_Init(HMAC_CTX *ctx, const void *key, int key_len, const EVP_MD *md) {
  if (key && key_len < 0) {
    return 0;
  }
  if (key && md) {
    HMAC_CTX_init(ctx);
  }
  return HMAC_Init_ex(ctx, key, (size_t)key_len, md, NULL);
}

int HMAC_CTX_copy(HMAC_CTX *dest, const HMAC_CTX *src) {
  HMAC_CTX_init(dest);
  return HMAC_CTX_copy_ex(dest, src);
}
