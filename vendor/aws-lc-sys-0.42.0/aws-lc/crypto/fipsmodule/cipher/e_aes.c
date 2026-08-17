// Copyright (c) 2001-2011 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/aead.h>
#include <openssl/aes.h>
#include <openssl/cipher.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/nid.h>
#include <openssl/rand.h>

#include <openssl/bytestring.h>
#include "../../internal.h"
#include "../aes/internal.h"
#include "../cpucap/internal.h"
#include "../delocate.h"
#include "../modes/internal.h"
#include "internal.h"


OPENSSL_MSVC_PRAGMA(warning(push))
OPENSSL_MSVC_PRAGMA(warning(disable : 4702))  // Unreachable code.

#define AES_GCM_NONCE_LENGTH 12

#if defined(BSAES)
static void vpaes_ctr32_encrypt_blocks_with_bsaes(const uint8_t *in,
                                                  uint8_t *out, size_t blocks,
                                                  const AES_KEY *key,
                                                  const uint8_t ivec[16]) {
  // |bsaes_ctr32_encrypt_blocks| is faster than |vpaes_ctr32_encrypt_blocks|,
  // but it takes at least one full 8-block batch to amortize the conversion.
  if (blocks < 8) {
    vpaes_ctr32_encrypt_blocks(in, out, blocks, key, ivec);
    return;
  }

  size_t bsaes_blocks = blocks;
  if (bsaes_blocks % 8 < 6) {
    // |bsaes_ctr32_encrypt_blocks| internally works in 8-block batches. If the
    // final batch is too small (under six blocks), it is faster to loop over
    // |vpaes_encrypt|. Round |bsaes_blocks| down to a multiple of 8.
    bsaes_blocks -= bsaes_blocks % 8;
  }

  AES_KEY bsaes;
  vpaes_encrypt_key_to_bsaes(&bsaes, key);
  bsaes_ctr32_encrypt_blocks(in, out, bsaes_blocks, &bsaes, ivec);
  OPENSSL_cleanse(&bsaes, sizeof(bsaes));

  in += 16 * bsaes_blocks;
  out += 16 * bsaes_blocks;
  blocks -= bsaes_blocks;

  uint8_t new_ivec[16];
  memcpy(new_ivec, ivec, 12);
  uint32_t ctr = CRYPTO_load_u32_be(ivec + 12) + bsaes_blocks;
  CRYPTO_store_u32_be(new_ivec + 12, ctr);

  // Finish any remaining blocks with |vpaes_ctr32_encrypt_blocks|.
  vpaes_ctr32_encrypt_blocks(in, out, blocks, key, new_ivec);
}
#endif  // BSAES

typedef struct {
  union {
    double align;
    AES_KEY ks;
  } ks;
  block128_f block;
  union {
    cbc128_f cbc;
    ctr128_f ctr;
  } stream;
} EVP_AES_KEY;

typedef struct {
  GCM128_CONTEXT gcm;
  union {
    double align;
    AES_KEY ks;
  } ks;         // AES key schedule to use
  int key_set;  // Set if key initialised
  int iv_set;   // Set if an iv is set
  uint8_t *iv;  // Temporary IV store
  int ivlen;    // IV length
  int taglen;
  int iv_gen;  // It is OK to generate IVs
  ctr128_f ctr;
} EVP_AES_GCM_CTX;

typedef struct {
  union {
    double align;
    AES_KEY ks;
  } ks;
  const uint8_t *iv; // Indicates if an IV has been set.
} EVP_AES_WRAP_CTX;

static int aes_init_key(EVP_CIPHER_CTX *ctx, const uint8_t *key,
                        const uint8_t *iv, int enc) {
  int ret;
  EVP_AES_KEY *dat = (EVP_AES_KEY *)ctx->cipher_data;
  const int mode = ctx->cipher->flags & EVP_CIPH_MODE_MASK;

  if ((mode == EVP_CIPH_ECB_MODE || mode == EVP_CIPH_CBC_MODE) && !enc) {
    if (hwaes_capable()) {
      ret = aes_hw_set_decrypt_key(key, ctx->key_len * 8, &dat->ks.ks);
      dat->block = aes_hw_decrypt;
      dat->stream.cbc = NULL;
      if (mode == EVP_CIPH_CBC_MODE) {
        dat->stream.cbc = aes_hw_cbc_encrypt;
      }
    } else if (bsaes_capable() && mode == EVP_CIPH_CBC_MODE) {
      assert(vpaes_capable());
      ret = vpaes_set_decrypt_key(key, ctx->key_len * 8, &dat->ks.ks);
      if (ret == 0) {
        vpaes_decrypt_key_to_bsaes(&dat->ks.ks, &dat->ks.ks);
      }
      // If |dat->stream.cbc| is provided, |dat->block| is never used.
      dat->block = NULL;
      dat->stream.cbc = bsaes_cbc_encrypt;
    } else if (vpaes_capable()) {
      ret = vpaes_set_decrypt_key(key, ctx->key_len * 8, &dat->ks.ks);
      dat->block = vpaes_decrypt;
      dat->stream.cbc = NULL;
#if defined(VPAES_CBC)
      if (mode == EVP_CIPH_CBC_MODE) {
        dat->stream.cbc = vpaes_cbc_encrypt;
      }
#endif
    } else {
      ret = aes_nohw_set_decrypt_key(key, ctx->key_len * 8, &dat->ks.ks);
      dat->block = aes_nohw_decrypt;
      dat->stream.cbc = NULL;
      if (mode == EVP_CIPH_CBC_MODE) {
        dat->stream.cbc = aes_nohw_cbc_encrypt;
      }
    }
  } else if (hwaes_capable()) {
    ret = aes_hw_set_encrypt_key(key, ctx->key_len * 8, &dat->ks.ks);
    dat->block = aes_hw_encrypt;
    dat->stream.cbc = NULL;
    if (mode == EVP_CIPH_CBC_MODE) {
      dat->stream.cbc = aes_hw_cbc_encrypt;
    } else if (mode == EVP_CIPH_CTR_MODE) {
      dat->stream.ctr = aes_hw_ctr32_encrypt_blocks;
    }
  } else if (vpaes_capable()) {
    ret = vpaes_set_encrypt_key(key, ctx->key_len * 8, &dat->ks.ks);
    dat->block = vpaes_encrypt;
    dat->stream.cbc = NULL;
#if defined(VPAES_CBC)
    if (mode == EVP_CIPH_CBC_MODE) {
      dat->stream.cbc = vpaes_cbc_encrypt;
    }
#endif
    if (mode == EVP_CIPH_CTR_MODE) {
#if defined(BSAES)
      assert(bsaes_capable());
      dat->stream.ctr = vpaes_ctr32_encrypt_blocks_with_bsaes;
#elif defined(VPAES_CTR32)
      dat->stream.ctr = vpaes_ctr32_encrypt_blocks;
#endif
    }
  } else {
    ret = aes_nohw_set_encrypt_key(key, ctx->key_len * 8, &dat->ks.ks);
    dat->block = aes_nohw_encrypt;
    dat->stream.cbc = NULL;
    if (mode == EVP_CIPH_CBC_MODE) {
      dat->stream.cbc = aes_nohw_cbc_encrypt;
    }
  }

  if (ret < 0) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_AES_KEY_SETUP_FAILED);
    return 0;
  }

  return 1;
}

static int aes_cbc_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out, const uint8_t *in,
                          size_t len) {
  EVP_AES_KEY *dat = (EVP_AES_KEY *)ctx->cipher_data;

  if (dat->stream.cbc) {
    (*dat->stream.cbc)(in, out, len, &dat->ks.ks, ctx->iv, ctx->encrypt);
  } else if (ctx->encrypt) {
    CRYPTO_cbc128_encrypt(in, out, len, &dat->ks.ks, ctx->iv, dat->block);
  } else {
    CRYPTO_cbc128_decrypt(in, out, len, &dat->ks.ks, ctx->iv, dat->block);
  }

  return 1;
}

static int aes_ecb_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out, const uint8_t *in,
                          size_t len) {
  size_t bl = ctx->cipher->block_size;
  EVP_AES_KEY *dat = (EVP_AES_KEY *)ctx->cipher_data;

  if (len < bl) {
    return 1;
  }

  len -= bl;
  for (size_t i = 0; i <= len; i += bl) {
    (*dat->block)(in + i, out + i, &dat->ks.ks);
  }

  return 1;
}

static int aes_ctr_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out, const uint8_t *in,
                          size_t len) {
  EVP_AES_KEY *dat = (EVP_AES_KEY *)ctx->cipher_data;

  if (dat->stream.ctr) {
    CRYPTO_ctr128_encrypt_ctr32(in, out, len, &dat->ks.ks, ctx->iv, ctx->buf,
                                &ctx->num, dat->stream.ctr);
  } else {
    CRYPTO_ctr128_encrypt(in, out, len, &dat->ks.ks, ctx->iv, ctx->buf,
                          &ctx->num, dat->block);
  }
  return 1;
}

static int aes_ofb_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out, const uint8_t *in,
                          size_t len) {
  EVP_AES_KEY *dat = (EVP_AES_KEY *)ctx->cipher_data;

  CRYPTO_ofb128_encrypt(in, out, len, &dat->ks.ks, ctx->iv, &ctx->num,
                        dat->block);
  return 1;
}

ctr128_f aes_ctr_set_key(AES_KEY *aes_key, GCM128_KEY *gcm_key,
                         block128_f *out_block, const uint8_t *key,
                         size_t key_bytes) {
  // This function assumes the key length was previously validated.
  assert(key_bytes == 128 / 8 || key_bytes == 192 / 8 || key_bytes == 256 / 8);
  if (hwaes_capable()) {
    aes_hw_set_encrypt_key(key, (int)key_bytes * 8, aes_key);
    if (gcm_key != NULL) {
      CRYPTO_gcm128_init_key(gcm_key, aes_key, aes_hw_encrypt_wrapper, 1);
    }
    if (out_block) {
      *out_block = aes_hw_encrypt_wrapper;
    }
    return aes_hw_ctr32_encrypt_blocks_wrapper;
  }

  if (vpaes_capable()) {
    vpaes_set_encrypt_key(key, (int)key_bytes * 8, aes_key);
    if (out_block) {
      *out_block = vpaes_encrypt_wrapper;
    }
    if (gcm_key != NULL) {
      CRYPTO_gcm128_init_key(gcm_key, aes_key, vpaes_encrypt_wrapper, 0);
    }
#if defined(BSAES)
    assert(bsaes_capable());
    return vpaes_ctr32_encrypt_blocks_with_bsaes;
#elif defined(VPAES_CTR32)
    return vpaes_ctr32_encrypt_blocks_wrapper;
#else
    return NULL;
#endif
  }

  aes_nohw_set_encrypt_key(key, (int)key_bytes * 8, aes_key);
  if (gcm_key != NULL) {
    CRYPTO_gcm128_init_key(gcm_key, aes_key, aes_nohw_encrypt_wrapper, 0);
  }
  if (out_block) {
    *out_block = aes_nohw_encrypt_wrapper;
  }
  return aes_nohw_ctr32_encrypt_blocks_wrapper;
}

#if defined(OPENSSL_32_BIT)
#define EVP_AES_GCM_CTX_PADDING (4 + 8)
#else
#define EVP_AES_GCM_CTX_PADDING 8
#endif

static EVP_AES_GCM_CTX *aes_gcm_from_cipher_ctx(EVP_CIPHER_CTX *ctx) {
  OPENSSL_STATIC_ASSERT(
      alignof(EVP_AES_GCM_CTX) <= 16,
      EVP_AES_GCM_CTX_needs_more_alignment_than_this_function_provides)

  // |malloc| guarantees up to 4-byte alignment on 32-bit and 8-byte alignment
  // on 64-bit systems, so we need to adjust to reach 16-byte alignment.
  assert(ctx->cipher->ctx_size ==
         sizeof(EVP_AES_GCM_CTX) + EVP_AES_GCM_CTX_PADDING);

  char *ptr = ctx->cipher_data;
#if defined(OPENSSL_32_BIT)
  assert((uintptr_t)ptr % 4 == 0);
  ptr += (uintptr_t)ptr & 4;
#endif
  assert((uintptr_t)ptr % 8 == 0);
  ptr += (uintptr_t)ptr & 8;
  return (EVP_AES_GCM_CTX *)ptr;
}

static int aes_gcm_init_key(EVP_CIPHER_CTX *ctx, const uint8_t *key,
                            const uint8_t *iv, int enc) {
  EVP_AES_GCM_CTX *gctx = aes_gcm_from_cipher_ctx(ctx);
  if (!iv && !key) {
    return 1;
  }

  if (key) {
    OPENSSL_memset(&gctx->gcm, 0, sizeof(gctx->gcm));
    gctx->ctr = aes_ctr_set_key(&gctx->ks.ks, &gctx->gcm.gcm_key, NULL, key,
                                ctx->key_len);
    // If we have an iv can set it directly, otherwise use saved IV.
    if (iv == NULL && gctx->iv_set) {
      iv = gctx->iv;
    }
    if (iv) {
      CRYPTO_gcm128_setiv(&gctx->gcm, &gctx->ks.ks, iv, gctx->ivlen);
      gctx->iv_set = 1;
    }
    gctx->key_set = 1;
  } else {
    // If key set use IV, otherwise copy
    if (gctx->key_set) {
      CRYPTO_gcm128_setiv(&gctx->gcm, &gctx->ks.ks, iv, gctx->ivlen);
    } else {
      OPENSSL_memcpy(gctx->iv, iv, gctx->ivlen);
    }
    gctx->iv_set = 1;
    gctx->iv_gen = 0;
  }
  return 1;
}

static void aes_gcm_cleanup(EVP_CIPHER_CTX *c) {
  EVP_AES_GCM_CTX *gctx = aes_gcm_from_cipher_ctx(c);
  OPENSSL_cleanse(&gctx->gcm, sizeof(gctx->gcm));
  if (gctx->iv != c->iv) {
    OPENSSL_free(gctx->iv);
  }
}

static int aes_gcm_ctrl(EVP_CIPHER_CTX *c, int type, int arg, void *ptr) {
  EVP_AES_GCM_CTX *gctx = aes_gcm_from_cipher_ctx(c);
  switch (type) {
    case EVP_CTRL_INIT:
      gctx->key_set = 0;
      gctx->iv_set = 0;
      gctx->ivlen = c->cipher->iv_len;
      gctx->iv = c->iv;
      gctx->taglen = -1;
      gctx->iv_gen = 0;
      return 1;

    case EVP_CTRL_AEAD_SET_IVLEN:
      if (arg <= 0) {
        return 0;
      }

      // Allocate memory for IV if needed
      if (arg > EVP_MAX_IV_LENGTH && arg > gctx->ivlen) {
        if (gctx->iv != c->iv) {
          OPENSSL_free(gctx->iv);
        }
        gctx->iv = OPENSSL_malloc(arg);
        if (!gctx->iv) {
          return 0;
        }
      }
      gctx->ivlen = arg;
      return 1;

    case EVP_CTRL_GET_IVLEN:
      *(int *)ptr = gctx->ivlen;
      return 1;

    case EVP_CTRL_AEAD_SET_TAG:
      if (arg <= 0 || arg > 16 || c->encrypt) {
        return 0;
      }
      OPENSSL_memcpy(c->buf, ptr, arg);
      gctx->taglen = arg;
      return 1;

    case EVP_CTRL_AEAD_GET_TAG:
      if (arg <= 0 || arg > 16 || !c->encrypt || gctx->taglen < 0) {
        return 0;
      }
      OPENSSL_memcpy(ptr, c->buf, arg);
      return 1;

    case EVP_CTRL_AEAD_SET_IV_FIXED:
      // Special case: -1 length restores whole IV
      if (arg == -1) {
        OPENSSL_memcpy(gctx->iv, ptr, gctx->ivlen);
        gctx->iv_gen = 1;
        return 1;
      }
      // Fixed field must be at least 4 bytes and invocation field
      // at least 8.
      if (arg < 4 || (gctx->ivlen - arg) < 8) {
        return 0;
      }
      OPENSSL_memcpy(gctx->iv, ptr, arg);
      // |RAND_bytes| calls within the fipsmodule should be wrapped with state
      // lock functions to avoid updating the service indicator with the DRBG
      // functions.
      FIPS_service_indicator_lock_state();
      if (c->encrypt) {
        AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(gctx->iv + arg, gctx->ivlen - arg));
      }
      FIPS_service_indicator_unlock_state();
      gctx->iv_gen = 1;
      return 1;

    case EVP_CTRL_GCM_IV_GEN: {
      if (gctx->iv_gen == 0 || gctx->key_set == 0) {
        return 0;
      }
      CRYPTO_gcm128_setiv(&gctx->gcm, &gctx->ks.ks, gctx->iv, gctx->ivlen);
      if (arg <= 0 || arg > gctx->ivlen) {
        arg = gctx->ivlen;
      }
      OPENSSL_memcpy(ptr, gctx->iv + gctx->ivlen - arg, arg);
      // Invocation field will be at least 8 bytes in size, so no need to check
      // wrap around or increment more than last 8 bytes.
      uint8_t *ctr = gctx->iv + gctx->ivlen - 8;
      CRYPTO_store_u64_be(ctr, CRYPTO_load_u64_be(ctr) + 1);
      gctx->iv_set = 1;
      return 1;
    }

    case EVP_CTRL_GCM_SET_IV_INV:
      if (gctx->iv_gen == 0 || gctx->key_set == 0 || c->encrypt) {
        return 0;
      }
      if (arg <= 0 || arg > gctx->ivlen) {
        return 0;
      }
      OPENSSL_memcpy(gctx->iv + gctx->ivlen - arg, ptr, arg);
      CRYPTO_gcm128_setiv(&gctx->gcm, &gctx->ks.ks, gctx->iv, gctx->ivlen);
      gctx->iv_set = 1;
      return 1;

    case EVP_CTRL_COPY: {
      EVP_CIPHER_CTX *out = ptr;
      EVP_AES_GCM_CTX *gctx_out = aes_gcm_from_cipher_ctx(out);
      // |EVP_CIPHER_CTX_copy| copies this generically, but we must redo it in
      // case |out->cipher_data| and |in->cipher_data| are differently aligned.
      OPENSSL_memcpy(gctx_out, gctx, sizeof(EVP_AES_GCM_CTX));
      if (gctx->iv == c->iv) {
        gctx_out->iv = out->iv;
      } else {
        gctx_out->iv = OPENSSL_memdup(gctx->iv, gctx->ivlen);
        if (!gctx_out->iv) {
          return 0;
        }
      }
      return 1;
    }

    default:
      return -1;
  }
}

static int aes_gcm_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out, const uint8_t *in,
                          size_t len) {
  EVP_AES_GCM_CTX *gctx = aes_gcm_from_cipher_ctx(ctx);

  // If not set up, return error
  if (!gctx->key_set) {
    return -1;
  }
  if (!gctx->iv_set) {
    return -1;
  }

  if (len > INT_MAX) {
    // This function signature can only express up to |INT_MAX| bytes encrypted.
    //
    // TODO(https://crbug.com/boringssl/494): Make the internal |EVP_CIPHER|
    // calling convention |size_t|-clean.
    return -1;
  }

  if (in) {
    if (out == NULL) {
      if (!CRYPTO_gcm128_aad(&gctx->gcm, in, len)) {
        return -1;
      }
    } else if (ctx->encrypt) {
      if (gctx->ctr) {
        if (!CRYPTO_gcm128_encrypt_ctr32(&gctx->gcm, &gctx->ks.ks, in, out, len,
                                         gctx->ctr)) {
          return -1;
        }
      } else {
        if (!CRYPTO_gcm128_encrypt(&gctx->gcm, &gctx->ks.ks, in, out, len)) {
          return -1;
        }
      }
    } else {
      if (gctx->ctr) {
        if (!CRYPTO_gcm128_decrypt_ctr32(&gctx->gcm, &gctx->ks.ks, in, out, len,
                                         gctx->ctr)) {
          return -1;
        }
      } else {
        if (!CRYPTO_gcm128_decrypt(&gctx->gcm, &gctx->ks.ks, in, out, len)) {
          return -1;
        }
      }
    }
    return (int)len;
  } else {
    if (!ctx->encrypt) {
      if (gctx->taglen < 0 ||
          !CRYPTO_gcm128_finish(&gctx->gcm, ctx->buf, gctx->taglen)) {
        return -1;
      }
      gctx->iv_set = 0;
      return 0;
    }
    CRYPTO_gcm128_tag(&gctx->gcm, ctx->buf, 16);
    gctx->taglen = 16;
    // Don't reuse the IV
    gctx->iv_set = 0;
    return 0;
  }
}

static int aes_xts_init_key(EVP_CIPHER_CTX *ctx, const uint8_t *key,
                            const uint8_t *iv, int enc) {
  EVP_AES_XTS_CTX *xctx = ctx->cipher_data;
  if (!iv && !key) {
    return 1;
  }

  if (key) {
    // Verify that the two keys are different.
    //
    // This addresses the vulnerability described in Rogaway's
    // September 2004 paper:
    //
    //      "Efficient Instantiations of Tweakable Blockciphers and
    //       Refinements to Modes OCB and PMAC".
    //      (http://web.cs.ucdavis.edu/~rogaway/papers/offsets.pdf)
    //
    // FIPS 140-2 IG A.9 XTS-AES Key Generation Requirements states
    // that:
    //      "The check for Key_1 != Key_2 shall be done at any place
    //       BEFORE using the keys in the XTS-AES algorithm to process
    //       data with them."
    //
    // key_len is two AES keys

    if (CRYPTO_memcmp(key, key + ctx->key_len / 2, ctx->key_len / 2) == 0) {
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_XTS_DUPLICATED_KEYS);
      return 0;
    }

    if (enc) {
      AES_set_encrypt_key(key, ctx->key_len * 4, &xctx->ks1.ks);
      xctx->xts.block1 = AES_encrypt;
    } else {
      AES_set_decrypt_key(key, ctx->key_len * 4, &xctx->ks1.ks);
      xctx->xts.block1 = AES_decrypt;
    }

    AES_set_encrypt_key(key + ctx->key_len / 2, ctx->key_len * 4,
                        &xctx->ks2.ks);
    xctx->xts.block2 = AES_encrypt;
    xctx->xts.key1 = &xctx->ks1.ks;
  }

  if (iv) {
    xctx->xts.key2 = &xctx->ks2.ks;
    OPENSSL_memcpy(ctx->iv, iv, 16);
  }

  return 1;
}

static int aes_xts_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out, const uint8_t *in,
                          size_t len) {
  EVP_AES_XTS_CTX *xctx = ctx->cipher_data;
  if (!xctx->xts.key1 || !xctx->xts.key2 || !out || !in ||
      len < AES_BLOCK_SIZE) {
    return 0;
  }

  // Impose a limit of 2^20 blocks per data unit as specified by
  // IEEE Std 1619-2018.  The earlier and obsolete IEEE Std 1619-2007
  // indicated that this was a SHOULD NOT rather than a MUST NOT.
  // NIST SP 800-38E mandates the same limit.
  if (len > XTS_MAX_BLOCKS_PER_DATA_UNIT * AES_BLOCK_SIZE) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_XTS_DATA_UNIT_IS_TOO_LARGE);
    return 0;
  }

  if (hwaes_xts_available()) {
    return aes_hw_xts_cipher(in, out, len, xctx->xts.key1, xctx->xts.key2,
                             ctx->iv, ctx->encrypt);
  } else {
    return CRYPTO_xts128_encrypt(&xctx->xts, ctx->iv, in, out, len,
                                 ctx->encrypt);
  }
}

static int aes_xts_ctrl(EVP_CIPHER_CTX *c, int type, int arg, void *ptr) {
  EVP_AES_XTS_CTX *xctx = c->cipher_data;
  if (type == EVP_CTRL_COPY) {
    EVP_CIPHER_CTX *out = ptr;
    EVP_AES_XTS_CTX *xctx_out = out->cipher_data;
    if (xctx->xts.key1) {
      if (xctx->xts.key1 != &xctx->ks1.ks) {
        return 0;
      }
      xctx_out->xts.key1 = &xctx_out->ks1.ks;
    }
    if (xctx->xts.key2) {
      if (xctx->xts.key2 != &xctx->ks2.ks) {
        return 0;
      }
      xctx_out->xts.key2 = &xctx_out->ks2.ks;
    }
    return 1;
  } else if (type != EVP_CTRL_INIT) {
    return -1;
  }
  // key1 and key2 are used as an indicator both key and IV are set
  xctx->xts.key1 = NULL;
  xctx->xts.key2 = NULL;
  return 1;
}

static int aes_wrap_init_key(EVP_CIPHER_CTX *ctx, const uint8_t *key,
                            const uint8_t *iv, int enc) {
  EVP_AES_WRAP_CTX *wctx = ctx->cipher_data;
  if (iv == NULL && key == NULL) {
    return 1;
  }
  if (key != NULL) {
    if (ctx->encrypt) {
      AES_set_encrypt_key(key, EVP_CIPHER_CTX_key_length(ctx) * 8,
                          &wctx->ks.ks);
    } else {
      AES_set_decrypt_key(key, EVP_CIPHER_CTX_key_length(ctx) * 8,
                          &wctx->ks.ks);
    }
    if (iv == NULL) {
      wctx->iv = NULL;
    }
  }
  if (iv != NULL) {
    OPENSSL_memcpy(ctx->iv, iv, ctx->cipher->iv_len);
    wctx->iv = ctx->iv;
  }
  return 1;
}

static int aes_wrap_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out, const uint8_t *in,
                           size_t inlen) {
  EVP_AES_WRAP_CTX *wctx = ctx->cipher_data;
  // There is no final operation, so we always return zero length here.
  if (in == NULL) {
    return 0;
  }

  // Internal calls to |AES_wrap/unwrap_key| within the fipsmodule should be
  // wrapped with state lock functions to avoid updating the service indicator.
  // When consuming via |EVP_CIPHER|, |EVP_CipherFinal(_ex)| should be the
  // function that indicates approval.
  int ret;
  FIPS_service_indicator_lock_state();
  if (ctx->encrypt) {
    ret = AES_wrap_key(&wctx->ks.ks, wctx->iv, out, in, inlen);
  } else {
    ret = AES_unwrap_key(&wctx->ks.ks, wctx->iv, out, in, inlen);
  }
  FIPS_service_indicator_unlock_state();
  return ret;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_128_cbc) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_128_cbc;
  out->block_size = 16;
  out->key_len = 16;
  out->iv_len = 16;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_CBC_MODE;
  out->init = aes_init_key;
  out->cipher = aes_cbc_cipher;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_128_ctr) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_128_ctr;
  out->block_size = 1;
  out->key_len = 16;
  out->iv_len = 16;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_CTR_MODE;
  out->init = aes_init_key;
  out->cipher = aes_ctr_cipher;
}

DEFINE_LOCAL_DATA(EVP_CIPHER, aes_128_ecb_generic) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_128_ecb;
  out->block_size = 16;
  out->key_len = 16;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_ECB_MODE;
  out->init = aes_init_key;
  out->cipher = aes_ecb_cipher;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_128_ofb) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_128_ofb128;
  out->block_size = 1;
  out->key_len = 16;
  out->iv_len = 16;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_OFB_MODE;
  out->init = aes_init_key;
  out->cipher = aes_ofb_cipher;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_128_gcm) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_128_gcm;
  out->block_size = 1;
  out->key_len = 16;
  out->iv_len = AES_GCM_NONCE_LENGTH;
  out->ctx_size = sizeof(EVP_AES_GCM_CTX) + EVP_AES_GCM_CTX_PADDING;
  out->flags = EVP_CIPH_GCM_MODE | EVP_CIPH_CUSTOM_IV | EVP_CIPH_CUSTOM_COPY |
               EVP_CIPH_FLAG_CUSTOM_CIPHER | EVP_CIPH_ALWAYS_CALL_INIT |
               EVP_CIPH_CTRL_INIT | EVP_CIPH_FLAG_AEAD_CIPHER;
  out->init = aes_gcm_init_key;
  out->cipher = aes_gcm_cipher;
  out->cleanup = aes_gcm_cleanup;
  out->ctrl = aes_gcm_ctrl;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_192_cbc) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_192_cbc;
  out->block_size = 16;
  out->key_len = 24;
  out->iv_len = 16;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_CBC_MODE;
  out->init = aes_init_key;
  out->cipher = aes_cbc_cipher;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_192_ctr) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_192_ctr;
  out->block_size = 1;
  out->key_len = 24;
  out->iv_len = 16;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_CTR_MODE;
  out->init = aes_init_key;
  out->cipher = aes_ctr_cipher;
}

DEFINE_LOCAL_DATA(EVP_CIPHER, aes_192_ecb_generic) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_192_ecb;
  out->block_size = 16;
  out->key_len = 24;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_ECB_MODE;
  out->init = aes_init_key;
  out->cipher = aes_ecb_cipher;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_192_ofb) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_192_ofb128;
  out->block_size = 1;
  out->key_len = 24;
  out->iv_len = 16;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_OFB_MODE;
  out->init = aes_init_key;
  out->cipher = aes_ofb_cipher;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_192_gcm) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_192_gcm;
  out->block_size = 1;
  out->key_len = 24;
  out->iv_len = AES_GCM_NONCE_LENGTH;
  out->ctx_size = sizeof(EVP_AES_GCM_CTX) + EVP_AES_GCM_CTX_PADDING;
  out->flags = EVP_CIPH_GCM_MODE | EVP_CIPH_CUSTOM_IV | EVP_CIPH_CUSTOM_COPY |
               EVP_CIPH_FLAG_CUSTOM_CIPHER | EVP_CIPH_ALWAYS_CALL_INIT |
               EVP_CIPH_CTRL_INIT | EVP_CIPH_FLAG_AEAD_CIPHER;
  out->init = aes_gcm_init_key;
  out->cipher = aes_gcm_cipher;
  out->cleanup = aes_gcm_cleanup;
  out->ctrl = aes_gcm_ctrl;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_256_cbc) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_256_cbc;
  out->block_size = 16;
  out->key_len = 32;
  out->iv_len = 16;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_CBC_MODE;
  out->init = aes_init_key;
  out->cipher = aes_cbc_cipher;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_256_ctr) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_256_ctr;
  out->block_size = 1;
  out->key_len = 32;
  out->iv_len = 16;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_CTR_MODE;
  out->init = aes_init_key;
  out->cipher = aes_ctr_cipher;
}

DEFINE_LOCAL_DATA(EVP_CIPHER, aes_256_ecb_generic) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_256_ecb;
  out->block_size = 16;
  out->key_len = 32;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_ECB_MODE;
  out->init = aes_init_key;
  out->cipher = aes_ecb_cipher;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_256_ofb) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_256_ofb128;
  out->block_size = 1;
  out->key_len = 32;
  out->iv_len = 16;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_OFB_MODE;
  out->init = aes_init_key;
  out->cipher = aes_ofb_cipher;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_256_wrap) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_id_aes256_wrap;
  out->block_size = 8;
  out->key_len = 32;
  out->iv_len = 8;
  out->ctx_size = sizeof(EVP_AES_WRAP_CTX);
  out->flags = EVP_CIPH_WRAP_MODE | EVP_CIPH_CUSTOM_IV |
               EVP_CIPH_FLAG_CUSTOM_CIPHER | EVP_CIPH_ALWAYS_CALL_INIT;
  out->init = aes_wrap_init_key;
  out->cipher = aes_wrap_cipher;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_256_gcm) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_256_gcm;
  out->block_size = 1;
  out->key_len = 32;
  out->iv_len = AES_GCM_NONCE_LENGTH;
  out->ctx_size = sizeof(EVP_AES_GCM_CTX) + EVP_AES_GCM_CTX_PADDING;
  out->flags = EVP_CIPH_GCM_MODE | EVP_CIPH_CUSTOM_IV | EVP_CIPH_CUSTOM_COPY |
               EVP_CIPH_FLAG_CUSTOM_CIPHER | EVP_CIPH_ALWAYS_CALL_INIT |
               EVP_CIPH_CTRL_INIT | EVP_CIPH_FLAG_AEAD_CIPHER;
  out->init = aes_gcm_init_key;
  out->cipher = aes_gcm_cipher;
  out->cleanup = aes_gcm_cleanup;
  out->ctrl = aes_gcm_ctrl;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_256_xts) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_256_xts;
  out->block_size = 1;
  out->key_len = 64;
  out->iv_len = 16;
  out->ctx_size = sizeof(EVP_AES_XTS_CTX);
  out->flags = EVP_CIPH_XTS_MODE | EVP_CIPH_CUSTOM_IV |
               EVP_CIPH_ALWAYS_CALL_INIT | EVP_CIPH_CTRL_INIT |
               EVP_CIPH_CUSTOM_COPY;
  out->init = aes_xts_init_key;
  out->cipher = aes_xts_cipher;
  out->ctrl = aes_xts_ctrl;
}

#if defined(HWAES_ECB)

static int aes_hw_ecb_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out,
                             const uint8_t *in, size_t len) {
  size_t bl = ctx->cipher->block_size;

  if (len < bl) {
    return 1;
  }

  aes_hw_ecb_encrypt(in, out, len, ctx->cipher_data, ctx->encrypt);

  return 1;
}

DEFINE_LOCAL_DATA(EVP_CIPHER, aes_hw_128_ecb) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_128_ecb;
  out->block_size = 16;
  out->key_len = 16;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_ECB_MODE;
  out->init = aes_init_key;
  out->cipher = aes_hw_ecb_cipher;
}

DEFINE_LOCAL_DATA(EVP_CIPHER, aes_hw_192_ecb) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_192_ecb;
  out->block_size = 16;
  out->key_len = 24;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_ECB_MODE;
  out->init = aes_init_key;
  out->cipher = aes_hw_ecb_cipher;
}

DEFINE_LOCAL_DATA(EVP_CIPHER, aes_hw_256_ecb) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_256_ecb;
  out->block_size = 16;
  out->key_len = 32;
  out->ctx_size = sizeof(EVP_AES_KEY);
  out->flags = EVP_CIPH_ECB_MODE;
  out->init = aes_init_key;
  out->cipher = aes_hw_ecb_cipher;
}

#define EVP_ECB_CIPHER_FUNCTION(keybits)            \
  const EVP_CIPHER *EVP_aes_##keybits##_ecb(void) { \
    if (hwaes_capable()) {                          \
      return aes_hw_##keybits##_ecb();              \
    }                                               \
    return aes_##keybits##_ecb_generic();           \
  }

#else

#define EVP_ECB_CIPHER_FUNCTION(keybits)            \
  const EVP_CIPHER *EVP_aes_##keybits##_ecb(void) { \
    return aes_##keybits##_ecb_generic();           \
  }

#endif  // HWAES_ECB

EVP_ECB_CIPHER_FUNCTION(128)
EVP_ECB_CIPHER_FUNCTION(192)
EVP_ECB_CIPHER_FUNCTION(256)


#define EVP_AEAD_AES_GCM_TAG_LEN 16

struct aead_aes_gcm_ctx {
  union {
    double align;
    AES_KEY ks;
  } ks;
  GCM128_KEY gcm_key;
  ctr128_f ctr;
};

static int aead_aes_gcm_init_impl(struct aead_aes_gcm_ctx *gcm_ctx,
                                  size_t *out_tag_len, const uint8_t *key,
                                  size_t key_len, size_t tag_len) {
  const size_t key_bits = key_len * 8;

  if (key_bits != 128 && key_bits != 192 && key_bits != 256) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_KEY_LENGTH);
    return 0;  // EVP_AEAD_CTX_init should catch this.
  }

  if (tag_len == EVP_AEAD_DEFAULT_TAG_LENGTH) {
    tag_len = EVP_AEAD_AES_GCM_TAG_LEN;
  }

  if (tag_len > EVP_AEAD_AES_GCM_TAG_LEN) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TAG_TOO_LARGE);
    return 0;
  }

  gcm_ctx->ctr =
      aes_ctr_set_key(&gcm_ctx->ks.ks, &gcm_ctx->gcm_key, NULL, key, key_len);
  *out_tag_len = tag_len;
  return 1;
}

OPENSSL_STATIC_ASSERT(sizeof(((EVP_AEAD_CTX *)NULL)->state) >=
                          sizeof(struct aead_aes_gcm_ctx),
                      AEAD_state_is_too_small)
OPENSSL_STATIC_ASSERT(alignof(union evp_aead_ctx_st_state) >=
                          alignof(struct aead_aes_gcm_ctx),
                      AEAD_state_has_insufficient_alignment)

static int aead_aes_gcm_init(EVP_AEAD_CTX *ctx, const uint8_t *key,
                             size_t key_len, size_t requested_tag_len) {
  struct aead_aes_gcm_ctx *gcm_ctx = (struct aead_aes_gcm_ctx *)&ctx->state;

  size_t actual_tag_len;
  if (!aead_aes_gcm_init_impl(gcm_ctx, &actual_tag_len, key, key_len,
                              requested_tag_len)) {
    return 0;
  }

  ctx->tag_len = actual_tag_len;
  return 1;
}

static void aead_aes_gcm_cleanup(EVP_AEAD_CTX *ctx) {}

static int aead_aes_gcm_seal_scatter_impl(
    const struct aead_aes_gcm_ctx *gcm_ctx, uint8_t *out, uint8_t *out_tag,
    size_t *out_tag_len, size_t max_out_tag_len, const uint8_t *nonce,
    size_t nonce_len, const uint8_t *in, size_t in_len, const uint8_t *extra_in,
    size_t extra_in_len, const uint8_t *ad, size_t ad_len, size_t tag_len) {
  if (extra_in_len + tag_len < tag_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TOO_LARGE);
    return 0;
  }
  if (max_out_tag_len < extra_in_len + tag_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BUFFER_TOO_SMALL);
    return 0;
  }
  if (nonce_len == 0) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_NONCE_SIZE);
    return 0;
  }

  const AES_KEY *key = &gcm_ctx->ks.ks;

  GCM128_CONTEXT gcm;
  OPENSSL_memset(&gcm, 0, sizeof(gcm));
  OPENSSL_memcpy(&gcm.gcm_key, &gcm_ctx->gcm_key, sizeof(gcm.gcm_key));
  CRYPTO_gcm128_setiv(&gcm, key, nonce, nonce_len);

  if (ad_len > 0 && !CRYPTO_gcm128_aad(&gcm, ad, ad_len)) {
    return 0;
  }

  if (gcm_ctx->ctr) {
    if (!CRYPTO_gcm128_encrypt_ctr32(&gcm, key, in, out, in_len,
                                     gcm_ctx->ctr)) {
      return 0;
    }
  } else {
    if (!CRYPTO_gcm128_encrypt(&gcm, key, in, out, in_len)) {
      return 0;
    }
  }

  if (extra_in_len) {
    if (gcm_ctx->ctr) {
      if (!CRYPTO_gcm128_encrypt_ctr32(&gcm, key, extra_in, out_tag,
                                       extra_in_len, gcm_ctx->ctr)) {
        return 0;
      }
    } else {
      if (!CRYPTO_gcm128_encrypt(&gcm, key, extra_in, out_tag, extra_in_len)) {
        return 0;
      }
    }
  }

  CRYPTO_gcm128_tag(&gcm, out_tag + extra_in_len, tag_len);
  *out_tag_len = tag_len + extra_in_len;

  return 1;
}

static int aead_aes_gcm_seal_scatter(
    const EVP_AEAD_CTX *ctx, uint8_t *out, uint8_t *out_tag,
    size_t *out_tag_len, size_t max_out_tag_len, const uint8_t *nonce,
    size_t nonce_len, const uint8_t *in, size_t in_len, const uint8_t *extra_in,
    size_t extra_in_len, const uint8_t *ad, size_t ad_len) {
  const struct aead_aes_gcm_ctx *gcm_ctx =
      (const struct aead_aes_gcm_ctx *)&ctx->state;
  return aead_aes_gcm_seal_scatter_impl(
      gcm_ctx, out, out_tag, out_tag_len, max_out_tag_len, nonce, nonce_len, in,
      in_len, extra_in, extra_in_len, ad, ad_len, ctx->tag_len);
}

static int aead_aes_gcm_open_gather_impl(const struct aead_aes_gcm_ctx *gcm_ctx,
                                         uint8_t *out, const uint8_t *nonce,
                                         size_t nonce_len, const uint8_t *in,
                                         size_t in_len, const uint8_t *in_tag,
                                         size_t in_tag_len, const uint8_t *ad,
                                         size_t ad_len, size_t tag_len) {
  uint8_t tag[EVP_AEAD_AES_GCM_TAG_LEN];

  if (nonce_len == 0) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_NONCE_SIZE);
    return 0;
  }

  if (in_tag_len != tag_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
    return 0;
  }

  const AES_KEY *key = &gcm_ctx->ks.ks;

  GCM128_CONTEXT gcm;
  OPENSSL_memset(&gcm, 0, sizeof(gcm));
  OPENSSL_memcpy(&gcm.gcm_key, &gcm_ctx->gcm_key, sizeof(gcm.gcm_key));
  CRYPTO_gcm128_setiv(&gcm, key, nonce, nonce_len);

  if (!CRYPTO_gcm128_aad(&gcm, ad, ad_len)) {
    return 0;
  }

  if (gcm_ctx->ctr) {
    if (!CRYPTO_gcm128_decrypt_ctr32(&gcm, key, in, out, in_len,
                                     gcm_ctx->ctr)) {
      return 0;
    }
  } else {
    if (!CRYPTO_gcm128_decrypt(&gcm, key, in, out, in_len)) {
      return 0;
    }
  }

  CRYPTO_gcm128_tag(&gcm, tag, tag_len);
  if (CRYPTO_memcmp(tag, in_tag, tag_len) != 0) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
    return 0;
  }

  return 1;
}

static int aead_aes_gcm_open_gather(const EVP_AEAD_CTX *ctx, uint8_t *out,
                                    const uint8_t *nonce, size_t nonce_len,
                                    const uint8_t *in, size_t in_len,
                                    const uint8_t *in_tag, size_t in_tag_len,
                                    const uint8_t *ad, size_t ad_len) {
  struct aead_aes_gcm_ctx *gcm_ctx = (struct aead_aes_gcm_ctx *)&ctx->state;
  if (!aead_aes_gcm_open_gather_impl(gcm_ctx, out, nonce, nonce_len, in, in_len,
                                     in_tag, in_tag_len, ad, ad_len,
                                     ctx->tag_len)) {
    return 0;
  }

  AEAD_GCM_verify_service_indicator(ctx);
  return 1;
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_128_gcm) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 16;
  out->nonce_len = AES_GCM_NONCE_LENGTH;
  out->overhead = EVP_AEAD_AES_GCM_TAG_LEN;
  out->max_tag_len = EVP_AEAD_AES_GCM_TAG_LEN;
  out->aead_id = AEAD_AES_128_GCM_ID;
  out->seal_scatter_supports_extra_in = 1;

  out->init = aead_aes_gcm_init;
  out->cleanup = aead_aes_gcm_cleanup;
  out->seal_scatter = aead_aes_gcm_seal_scatter;
  out->open_gather = aead_aes_gcm_open_gather;
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_192_gcm) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 24;
  out->nonce_len = AES_GCM_NONCE_LENGTH;
  out->overhead = EVP_AEAD_AES_GCM_TAG_LEN;
  out->max_tag_len = EVP_AEAD_AES_GCM_TAG_LEN;
  out->aead_id = AEAD_AES_192_GCM_ID;
  out->seal_scatter_supports_extra_in = 1;

  out->init = aead_aes_gcm_init;
  out->cleanup = aead_aes_gcm_cleanup;
  out->seal_scatter = aead_aes_gcm_seal_scatter;
  out->open_gather = aead_aes_gcm_open_gather;
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_256_gcm) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 32;
  out->nonce_len = AES_GCM_NONCE_LENGTH;
  out->overhead = EVP_AEAD_AES_GCM_TAG_LEN;
  out->max_tag_len = EVP_AEAD_AES_GCM_TAG_LEN;
  out->aead_id = AEAD_AES_256_GCM_ID;
  out->seal_scatter_supports_extra_in = 1;

  out->init = aead_aes_gcm_init;
  out->cleanup = aead_aes_gcm_cleanup;
  out->seal_scatter = aead_aes_gcm_seal_scatter;
  out->open_gather = aead_aes_gcm_open_gather;
}

static int aead_aes_gcm_init_randnonce(EVP_AEAD_CTX *ctx, const uint8_t *key,
                                       size_t key_len,
                                       size_t requested_tag_len) {
  if (requested_tag_len != EVP_AEAD_DEFAULT_TAG_LENGTH) {
    if (requested_tag_len < AES_GCM_NONCE_LENGTH) {
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BUFFER_TOO_SMALL);
      return 0;
    }
    requested_tag_len -= AES_GCM_NONCE_LENGTH;
  }

  if (!aead_aes_gcm_init(ctx, key, key_len, requested_tag_len)) {
    return 0;
  }

  ctx->tag_len += AES_GCM_NONCE_LENGTH;
  return 1;
}

static int aead_aes_gcm_seal_scatter_randnonce(
    const EVP_AEAD_CTX *ctx, uint8_t *out, uint8_t *out_tag,
    size_t *out_tag_len, size_t max_out_tag_len, const uint8_t *external_nonce,
    size_t external_nonce_len, const uint8_t *in, size_t in_len,
    const uint8_t *extra_in, size_t extra_in_len, const uint8_t *ad,
    size_t ad_len) {
  if (external_nonce_len != 0) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_NONCE_SIZE);
    return 0;
  }

  uint8_t nonce[AES_GCM_NONCE_LENGTH];
  if (max_out_tag_len < sizeof(nonce)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BUFFER_TOO_SMALL);
    return 0;
  }
  // |RAND_bytes| calls within the fipsmodule should be wrapped with state lock
  // functions to avoid updating the service indicator with the DRBG functions.
  FIPS_service_indicator_lock_state();
  AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(nonce, sizeof(nonce)));
  FIPS_service_indicator_unlock_state();
  const struct aead_aes_gcm_ctx *gcm_ctx =
      (const struct aead_aes_gcm_ctx *)&ctx->state;
  if (!aead_aes_gcm_seal_scatter_impl(gcm_ctx, out, out_tag, out_tag_len,
                                      max_out_tag_len - AES_GCM_NONCE_LENGTH,
                                      nonce, sizeof(nonce), in, in_len,
                                      extra_in, extra_in_len, ad, ad_len,
                                      ctx->tag_len - AES_GCM_NONCE_LENGTH)) {
    return 0;
  }

  assert(*out_tag_len + sizeof(nonce) <= max_out_tag_len);
  memcpy(out_tag + *out_tag_len, nonce, sizeof(nonce));
  *out_tag_len += sizeof(nonce);
  // Only internal IV for AES-GCM is approved.
  AEAD_GCM_verify_service_indicator(ctx);
  return 1;
}

static int aead_aes_gcm_open_gather_randnonce(
    const EVP_AEAD_CTX *ctx, uint8_t *out, const uint8_t *external_nonce,
    size_t external_nonce_len, const uint8_t *in, size_t in_len,
    const uint8_t *in_tag, size_t in_tag_len, const uint8_t *ad,
    size_t ad_len) {
  if (external_nonce_len != 0) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_NONCE_SIZE);
    return 0;
  }

  if (in_tag_len < AES_GCM_NONCE_LENGTH) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
    return 0;
  }
  const uint8_t *nonce = in_tag + in_tag_len - AES_GCM_NONCE_LENGTH;

  const struct aead_aes_gcm_ctx *gcm_ctx =
      (const struct aead_aes_gcm_ctx *)&ctx->state;

  int ret = aead_aes_gcm_open_gather_impl(
      gcm_ctx, out, nonce, AES_GCM_NONCE_LENGTH, in, in_len, in_tag,
      in_tag_len - AES_GCM_NONCE_LENGTH, ad, ad_len,
      ctx->tag_len - AES_GCM_NONCE_LENGTH);
  // Only internal IV for AES-GCM is approved.
  if (ret) {
    AEAD_GCM_verify_service_indicator(ctx);
  }
  return ret;
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_128_gcm_randnonce) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 16;
  out->nonce_len = 0;
  out->overhead = EVP_AEAD_AES_GCM_TAG_LEN + AES_GCM_NONCE_LENGTH;
  out->max_tag_len = EVP_AEAD_AES_GCM_TAG_LEN + AES_GCM_NONCE_LENGTH;
  out->aead_id = AEAD_AES_128_GCM_RANDNONCE_ID;
  out->seal_scatter_supports_extra_in = 1;

  out->init = aead_aes_gcm_init_randnonce;
  out->cleanup = aead_aes_gcm_cleanup;
  out->seal_scatter = aead_aes_gcm_seal_scatter_randnonce;
  out->open_gather = aead_aes_gcm_open_gather_randnonce;
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_256_gcm_randnonce) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 32;
  out->nonce_len = 0;
  out->overhead = EVP_AEAD_AES_GCM_TAG_LEN + AES_GCM_NONCE_LENGTH;
  out->max_tag_len = EVP_AEAD_AES_GCM_TAG_LEN + AES_GCM_NONCE_LENGTH;
  out->aead_id = AEAD_AES_256_GCM_RANDNONCE_ID;
  out->seal_scatter_supports_extra_in = 1;

  out->init = aead_aes_gcm_init_randnonce;
  out->cleanup = aead_aes_gcm_cleanup;
  out->seal_scatter = aead_aes_gcm_seal_scatter_randnonce;
  out->open_gather = aead_aes_gcm_open_gather_randnonce;
}

struct aead_aes_gcm_tls12_ctx {
  struct aead_aes_gcm_ctx gcm_ctx;
  uint64_t min_next_nonce;
};

OPENSSL_STATIC_ASSERT(sizeof(((EVP_AEAD_CTX *)NULL)->state) >=
                          sizeof(struct aead_aes_gcm_tls12_ctx),
                      AEAD_state_is_too_small)
OPENSSL_STATIC_ASSERT(alignof(union evp_aead_ctx_st_state) >=
                          alignof(struct aead_aes_gcm_tls12_ctx),
                      AEAD_state_has_insufficient_alignment)

static int aead_aes_gcm_tls12_init(EVP_AEAD_CTX *ctx, const uint8_t *key,
                                   size_t key_len, size_t requested_tag_len) {
  struct aead_aes_gcm_tls12_ctx *gcm_ctx =
      (struct aead_aes_gcm_tls12_ctx *)&ctx->state;

  gcm_ctx->min_next_nonce = 0;

  size_t actual_tag_len;
  if (!aead_aes_gcm_init_impl(&gcm_ctx->gcm_ctx, &actual_tag_len, key, key_len,
                              requested_tag_len)) {
    return 0;
  }

  ctx->tag_len = actual_tag_len;
  return 1;
}

static int aead_aes_gcm_tls12_seal_scatter(
    const EVP_AEAD_CTX *ctx, uint8_t *out, uint8_t *out_tag,
    size_t *out_tag_len, size_t max_out_tag_len, const uint8_t *nonce,
    size_t nonce_len, const uint8_t *in, size_t in_len, const uint8_t *extra_in,
    size_t extra_in_len, const uint8_t *ad, size_t ad_len) {
  struct aead_aes_gcm_tls12_ctx *gcm_ctx =
      (struct aead_aes_gcm_tls12_ctx *)&ctx->state;

  if (nonce_len != AES_GCM_NONCE_LENGTH) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_UNSUPPORTED_NONCE_SIZE);
    return 0;
  }

  // The given nonces must be strictly monotonically increasing.
  uint64_t given_counter =
      CRYPTO_load_u64_be(nonce + nonce_len - sizeof(uint64_t));
  if (given_counter == UINT64_MAX || given_counter < gcm_ctx->min_next_nonce) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_NONCE);
    return 0;
  }

  gcm_ctx->min_next_nonce = given_counter + 1;

  if (aead_aes_gcm_seal_scatter(ctx, out, out_tag, out_tag_len, max_out_tag_len,
                                nonce, nonce_len, in, in_len, extra_in,
                                extra_in_len, ad, ad_len)) {
    AEAD_GCM_verify_service_indicator(ctx);
    return 1;
  }
  return 0;
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_128_gcm_tls12) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 16;
  out->nonce_len = AES_GCM_NONCE_LENGTH;
  out->overhead = EVP_AEAD_AES_GCM_TAG_LEN;
  out->max_tag_len = EVP_AEAD_AES_GCM_TAG_LEN;
  out->aead_id = AEAD_AES_128_GCM_TLS12_ID;
  out->seal_scatter_supports_extra_in = 1;

  out->init = aead_aes_gcm_tls12_init;
  out->cleanup = aead_aes_gcm_cleanup;
  out->seal_scatter = aead_aes_gcm_tls12_seal_scatter;
  out->open_gather = aead_aes_gcm_open_gather;
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_256_gcm_tls12) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 32;
  out->nonce_len = AES_GCM_NONCE_LENGTH;
  out->overhead = EVP_AEAD_AES_GCM_TAG_LEN;
  out->max_tag_len = EVP_AEAD_AES_GCM_TAG_LEN;
  out->aead_id = AEAD_AES_256_GCM_TLS12_ID;
  out->seal_scatter_supports_extra_in = 1;

  out->init = aead_aes_gcm_tls12_init;
  out->cleanup = aead_aes_gcm_cleanup;
  out->seal_scatter = aead_aes_gcm_tls12_seal_scatter;
  out->open_gather = aead_aes_gcm_open_gather;
}

struct aead_aes_gcm_tls13_ctx {
  struct aead_aes_gcm_ctx gcm_ctx;
  uint64_t min_next_nonce;
  uint64_t mask;
  uint8_t first;
};

OPENSSL_STATIC_ASSERT(sizeof(((EVP_AEAD_CTX *)NULL)->state) >=
                          sizeof(struct aead_aes_gcm_tls13_ctx),
                      AEAD_state_is_too_small)
OPENSSL_STATIC_ASSERT(alignof(union evp_aead_ctx_st_state) >=
                          alignof(struct aead_aes_gcm_tls13_ctx),
                      AEAD_state_has_insufficient_alignment)

static int aead_aes_gcm_tls13_init(EVP_AEAD_CTX *ctx, const uint8_t *key,
                                   size_t key_len, size_t requested_tag_len) {
  struct aead_aes_gcm_tls13_ctx *gcm_ctx =
      (struct aead_aes_gcm_tls13_ctx *)&ctx->state;

  gcm_ctx->min_next_nonce = 0;
  gcm_ctx->first = 1;

  size_t actual_tag_len;
  if (!aead_aes_gcm_init_impl(&gcm_ctx->gcm_ctx, &actual_tag_len, key, key_len,
                              requested_tag_len)) {
    return 0;
  }

  ctx->tag_len = actual_tag_len;
  return 1;
}

static int aead_aes_gcm_tls13_seal_scatter(
    const EVP_AEAD_CTX *ctx, uint8_t *out, uint8_t *out_tag,
    size_t *out_tag_len, size_t max_out_tag_len, const uint8_t *nonce,
    size_t nonce_len, const uint8_t *in, size_t in_len, const uint8_t *extra_in,
    size_t extra_in_len, const uint8_t *ad, size_t ad_len) {
  struct aead_aes_gcm_tls13_ctx *gcm_ctx =
      (struct aead_aes_gcm_tls13_ctx *)&ctx->state;

  if (nonce_len != AES_GCM_NONCE_LENGTH) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_UNSUPPORTED_NONCE_SIZE);
    return 0;
  }

  // The given nonces must be strictly monotonically increasing. See
  // https://tools.ietf.org/html/rfc8446#section-5.3 for details of the TLS 1.3
  // nonce construction.
  uint64_t given_counter =
      CRYPTO_load_u64_be(nonce + nonce_len - sizeof(uint64_t));

  if (gcm_ctx->first) {
    // In the first call the sequence number will be zero and therefore the
    // given nonce will be 0 ^ mask = mask.
    gcm_ctx->mask = given_counter;
    gcm_ctx->first = 0;
  }
  given_counter ^= gcm_ctx->mask;

  if (given_counter == UINT64_MAX || given_counter < gcm_ctx->min_next_nonce) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_NONCE);
    return 0;
  }

  gcm_ctx->min_next_nonce = given_counter + 1;

  if (aead_aes_gcm_seal_scatter(ctx, out, out_tag, out_tag_len, max_out_tag_len,
                                nonce, nonce_len, in, in_len, extra_in,
                                extra_in_len, ad, ad_len)) {
    AEAD_GCM_verify_service_indicator(ctx);
    return 1;
  }
  return 0;
}

#define AEAD_AES_GCM_TLS13_STATE_SERDE_VERSION 1

/*
 * AeadAesGCMTls13StateSerializationVersion ::= INTEGER {v1 (1)}
 *
 * AeadAesGCMTls13State ::= SEQUENCE {
 *   serializationVersion AeadAesGCMTls13StateSerializationVersion,
 *   minNextNonce   INTEGER,
 *   mask           INTEGER,
 *   first          BOOLEAN
 * }
 */
static int aead_aes_gcm_tls13_serialize_state(const EVP_AEAD_CTX *ctx,
                                              CBB *cbb) {
  struct aead_aes_gcm_tls13_ctx *gcm_ctx =
      (struct aead_aes_gcm_tls13_ctx *)&ctx->state;

  CBB state;

  if (!CBB_add_asn1(cbb, &state, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&state, AEAD_AES_GCM_TLS13_STATE_SERDE_VERSION)) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_MALLOC_FAILURE);
    return 0;
  }

  if (!CBB_add_asn1_uint64(&state, gcm_ctx->min_next_nonce)) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_MALLOC_FAILURE);
    return 0;
  }

  if (!CBB_add_asn1_uint64(&state, gcm_ctx->mask)) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_MALLOC_FAILURE);
    return 0;
  }

  if (!CBB_add_asn1_bool(&state, gcm_ctx->first ? 1 : 0)) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_MALLOC_FAILURE);
    return 0;
  }

  return CBB_flush(cbb);
}

// See |aead_aes_gcm_tls13_serialize_state| documentation string for
// serialization format.
static int aead_aes_gcm_tls13_deserialize_state(const EVP_AEAD_CTX *ctx,
                                                CBS *cbs) {
  struct aead_aes_gcm_tls13_ctx *gcm_ctx =
      (struct aead_aes_gcm_tls13_ctx *)&ctx->state;

  CBS state;

  if (!CBS_get_asn1(cbs, &state, CBS_ASN1_SEQUENCE)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_SERIALIZATION_INVALID_EVP_AEAD_CTX);
    return 0;
  }

  uint64_t serde_version;
  if (!CBS_get_asn1_uint64(&state, &serde_version) ||
      AEAD_AES_GCM_TLS13_STATE_SERDE_VERSION != serde_version) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_SERIALIZATION_INVALID_EVP_AEAD_CTX);
    return 0;
  }

  uint64_t min_next_nonce;
  if (!CBS_get_asn1_uint64(&state, &min_next_nonce)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_SERIALIZATION_INVALID_EVP_AEAD_CTX);
    return 0;
  }
  gcm_ctx->min_next_nonce = min_next_nonce;

  uint64_t mask;
  if (!CBS_get_asn1_uint64(&state, &mask)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_SERIALIZATION_INVALID_EVP_AEAD_CTX);
    return 0;
  }
  gcm_ctx->mask = mask;

  int first;
  if (!CBS_get_asn1_bool(&state, &first)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_SERIALIZATION_INVALID_EVP_AEAD_CTX);
    return 0;
  }
  gcm_ctx->first = first ? 1 : 0;

  return 1;
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_128_gcm_tls13) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 16;
  out->nonce_len = AES_GCM_NONCE_LENGTH;
  out->overhead = EVP_AEAD_AES_GCM_TAG_LEN;
  out->max_tag_len = EVP_AEAD_AES_GCM_TAG_LEN;
  out->aead_id = AEAD_AES_128_GCM_TLS13_ID;
  out->seal_scatter_supports_extra_in = 1;

  out->init = aead_aes_gcm_tls13_init;
  out->cleanup = aead_aes_gcm_cleanup;
  out->seal_scatter = aead_aes_gcm_tls13_seal_scatter;
  out->open_gather = aead_aes_gcm_open_gather;

  out->serialize_state = aead_aes_gcm_tls13_serialize_state;
  out->deserialize_state = aead_aes_gcm_tls13_deserialize_state;
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_256_gcm_tls13) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 32;
  out->nonce_len = AES_GCM_NONCE_LENGTH;
  out->overhead = EVP_AEAD_AES_GCM_TAG_LEN;
  out->max_tag_len = EVP_AEAD_AES_GCM_TAG_LEN;
  out->aead_id = AEAD_AES_256_GCM_TLS13_ID;
  out->seal_scatter_supports_extra_in = 1;

  out->init = aead_aes_gcm_tls13_init;
  out->cleanup = aead_aes_gcm_cleanup;
  out->seal_scatter = aead_aes_gcm_tls13_seal_scatter;
  out->open_gather = aead_aes_gcm_open_gather;

  out->serialize_state = aead_aes_gcm_tls13_serialize_state;
  out->deserialize_state = aead_aes_gcm_tls13_deserialize_state;
}

int EVP_has_aes_hardware(void) {
#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
  return hwaes_capable() && crypto_gcm_clmul_enabled();
#elif defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64)
  return hwaes_capable() && CRYPTO_is_ARMv8_PMULL_capable();
#elif defined(OPENSSL_PPC64LE)
  return hwaes_capable() && CRYPTO_is_PPC64LE_vcrypto_capable();
#else
  return 0;
#endif
}

OPENSSL_MSVC_PRAGMA(warning(pop))
