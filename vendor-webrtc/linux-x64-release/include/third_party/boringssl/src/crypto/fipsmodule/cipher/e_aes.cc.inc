// Copyright 2001-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/aead.h>
#include <openssl/aes.h>
#include <openssl/cipher.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/nid.h>

#include "../../internal.h"
#include "../aes/internal.h"
#include "../bcm_interface.h"
#include "../delocate.h"
#include "../service_indicator/internal.h"
#include "internal.h"


#define AES_GCM_NONCE_LENGTH 12

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
  GCM128_KEY key;
  GCM128_CONTEXT gcm;
  int key_set;  // Set if key initialised
  int iv_set;   // Set if an iv is set
  uint8_t *iv;  // Temporary IV store
  int ivlen;    // IV length
  int taglen;
  int iv_gen;  // It is OK to generate IVs
  ctr128_f ctr;
} EVP_AES_GCM_CTX;

static int aes_init_key(EVP_CIPHER_CTX *ctx, const uint8_t *key,
                        const uint8_t *iv, int enc) {
  int ret;
  EVP_AES_KEY *dat = (EVP_AES_KEY *)ctx->cipher_data;
  const int mode = ctx->cipher->flags & EVP_CIPH_MODE_MASK;

  if (mode == EVP_CIPH_CTR_MODE) {
    switch (ctx->key_len) {
      case 16:
        boringssl_fips_inc_counter(fips_counter_evp_aes_128_ctr);
        break;

      case 32:
        boringssl_fips_inc_counter(fips_counter_evp_aes_256_ctr);
        break;
    }
  }

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
#else
      dat->stream.ctr = vpaes_ctr32_encrypt_blocks;
#endif
    }
  } else {
    ret = aes_nohw_set_encrypt_key(key, ctx->key_len * 8, &dat->ks.ks);
    dat->block = aes_nohw_encrypt;
    dat->stream.cbc = NULL;
    if (mode == EVP_CIPH_CBC_MODE) {
      dat->stream.cbc = aes_nohw_cbc_encrypt;
    } else if (mode == EVP_CIPH_CTR_MODE) {
      dat->stream.ctr = aes_nohw_ctr32_encrypt_blocks;
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
  CRYPTO_ctr128_encrypt_ctr32(in, out, len, &dat->ks.ks, ctx->iv, ctx->buf,
                              &ctx->num, dat->stream.ctr);
  return 1;
}

static int aes_ofb_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out, const uint8_t *in,
                          size_t len) {
  EVP_AES_KEY *dat = (EVP_AES_KEY *)ctx->cipher_data;

  CRYPTO_ofb128_encrypt(in, out, len, &dat->ks.ks, ctx->iv, &ctx->num,
                        dat->block);
  return 1;
}

static int aes_gcm_init_key(EVP_CIPHER_CTX *ctx, const uint8_t *key,
                            const uint8_t *iv, int enc) {
  EVP_AES_GCM_CTX *gctx = reinterpret_cast<EVP_AES_GCM_CTX *>(ctx->cipher_data);
  if (!iv && !key) {
    return 1;
  }

  // We must configure first the key, then the IV, but the caller may pass both
  // together, or separately in either order.
  if (key) {
    OPENSSL_memset(&gctx->gcm, 0, sizeof(gctx->gcm));
    CRYPTO_gcm128_init_aes_key(&gctx->key, key, ctx->key_len);
    // Use the IV if specified. Otherwise, use the saved IV, if any.
    if (iv == NULL && gctx->iv_set) {
      iv = gctx->iv;
    }
    if (iv) {
      CRYPTO_gcm128_init_ctx(&gctx->key, &gctx->gcm, iv, gctx->ivlen);
      gctx->iv_set = 1;
    }
    gctx->key_set = 1;
  } else {
    if (gctx->key_set) {
      CRYPTO_gcm128_init_ctx(&gctx->key, &gctx->gcm, iv, gctx->ivlen);
    } else {
      // The caller specified the IV before the key. Save the IV for later.
      OPENSSL_memcpy(gctx->iv, iv, gctx->ivlen);
    }
    gctx->iv_set = 1;
    gctx->iv_gen = 0;
  }
  return 1;
}

static void aes_gcm_cleanup(EVP_CIPHER_CTX *c) {
  EVP_AES_GCM_CTX *gctx = reinterpret_cast<EVP_AES_GCM_CTX *>(c->cipher_data);
  OPENSSL_cleanse(&gctx->key, sizeof(gctx->key));
  OPENSSL_cleanse(&gctx->gcm, sizeof(gctx->gcm));
  if (gctx->iv != c->iv) {
    OPENSSL_free(gctx->iv);
  }
}

static int aes_gcm_ctrl(EVP_CIPHER_CTX *c, int type, int arg, void *ptr) {
  EVP_AES_GCM_CTX *gctx = reinterpret_cast<EVP_AES_GCM_CTX *>(c->cipher_data);
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
        gctx->iv = reinterpret_cast<uint8_t *>(OPENSSL_malloc(arg));
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
      if (c->encrypt) {
        // |BCM_rand_bytes| calls within the fipsmodule should be wrapped with
        // state lock functions to avoid updating the service indicator with the
        // DRBG functions.
        FIPS_service_indicator_lock_state();
        BCM_rand_bytes(gctx->iv + arg, gctx->ivlen - arg);
        FIPS_service_indicator_unlock_state();
      }
      gctx->iv_gen = 1;
      return 1;

    case EVP_CTRL_GCM_IV_GEN: {
      if (gctx->iv_gen == 0 || gctx->key_set == 0) {
        return 0;
      }
      CRYPTO_gcm128_init_ctx(&gctx->key, &gctx->gcm, gctx->iv, gctx->ivlen);
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
      OPENSSL_memcpy(gctx->iv + gctx->ivlen - arg, ptr, arg);
      CRYPTO_gcm128_init_ctx(&gctx->key, &gctx->gcm, gctx->iv, gctx->ivlen);
      gctx->iv_set = 1;
      return 1;

    case EVP_CTRL_COPY: {
      EVP_CIPHER_CTX *out = reinterpret_cast<EVP_CIPHER_CTX *>(ptr);
      EVP_AES_GCM_CTX *gctx_out =
          reinterpret_cast<EVP_AES_GCM_CTX *>(out->cipher_data);
      if (gctx->iv == c->iv) {
        gctx_out->iv = out->iv;
      } else {
        gctx_out->iv =
            reinterpret_cast<uint8_t *>(OPENSSL_memdup(gctx->iv, gctx->ivlen));
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
  EVP_AES_GCM_CTX *gctx = reinterpret_cast<EVP_AES_GCM_CTX *>(ctx->cipher_data);

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
      if (!CRYPTO_gcm128_aad(&gctx->key, &gctx->gcm, in, len)) {
        return -1;
      }
    } else if (ctx->encrypt) {
      if (!CRYPTO_gcm128_encrypt(&gctx->key, &gctx->gcm, in, out, len)) {
        return -1;
      }
    } else {
      if (!CRYPTO_gcm128_decrypt(&gctx->key, &gctx->gcm, in, out, len)) {
        return -1;
      }
    }
    return (int)len;
  } else {
    if (!ctx->encrypt) {
      if (gctx->taglen < 0 || !CRYPTO_gcm128_finish(&gctx->key, &gctx->gcm,
                                                    ctx->buf, gctx->taglen)) {
        return -1;
      }
      gctx->iv_set = 0;
      return 0;
    }
    CRYPTO_gcm128_tag(&gctx->key, &gctx->gcm, ctx->buf, 16);
    gctx->taglen = 16;
    // Don't reuse the IV
    gctx->iv_set = 0;
    return 0;
  }
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
  out->ctx_size = sizeof(EVP_AES_GCM_CTX);
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
  out->ctx_size = sizeof(EVP_AES_GCM_CTX);
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

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_256_gcm) {
  memset(out, 0, sizeof(EVP_CIPHER));

  out->nid = NID_aes_256_gcm;
  out->block_size = 1;
  out->key_len = 32;
  out->iv_len = AES_GCM_NONCE_LENGTH;
  out->ctx_size = sizeof(EVP_AES_GCM_CTX);
  out->flags = EVP_CIPH_GCM_MODE | EVP_CIPH_CUSTOM_IV | EVP_CIPH_CUSTOM_COPY |
               EVP_CIPH_FLAG_CUSTOM_CIPHER | EVP_CIPH_ALWAYS_CALL_INIT |
               EVP_CIPH_CTRL_INIT | EVP_CIPH_FLAG_AEAD_CIPHER;
  out->init = aes_gcm_init_key;
  out->cipher = aes_gcm_cipher;
  out->cleanup = aes_gcm_cleanup;
  out->ctrl = aes_gcm_ctrl;
}

#if defined(HWAES_ECB)

static int aes_hw_ecb_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out,
                             const uint8_t *in, size_t len) {
  size_t bl = ctx->cipher->block_size;

  if (len < bl) {
    return 1;
  }

  aes_hw_ecb_encrypt(in, out, len,
                     reinterpret_cast<const AES_KEY *>(ctx->cipher_data),
                     ctx->encrypt);

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

namespace {
struct aead_aes_gcm_ctx {
  GCM128_KEY key;
};
}  // namespace

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

  CRYPTO_gcm128_init_aes_key(&gcm_ctx->key, key, key_len);
  *out_tag_len = tag_len;
  return 1;
}

static_assert(sizeof(((EVP_AEAD_CTX *)NULL)->state) >=
                  sizeof(struct aead_aes_gcm_ctx),
              "AEAD state is too small");
static_assert(alignof(union evp_aead_ctx_st_state) >=
                  alignof(struct aead_aes_gcm_ctx),
              "AEAD state has insufficient alignment");

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

  const GCM128_KEY *key = &gcm_ctx->key;
  GCM128_CONTEXT gcm;
  CRYPTO_gcm128_init_ctx(key, &gcm, nonce, nonce_len);

  if (ad_len > 0 && !CRYPTO_gcm128_aad(key, &gcm, ad, ad_len)) {
    return 0;
  }

  if (!CRYPTO_gcm128_encrypt(key, &gcm, in, out, in_len)) {
    return 0;
  }

  if (extra_in_len > 0 &&
      !CRYPTO_gcm128_encrypt(key, &gcm, extra_in, out_tag, extra_in_len)) {
    return 0;
  }

  CRYPTO_gcm128_tag(key, &gcm, out_tag + extra_in_len, tag_len);
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

  const GCM128_KEY *key = &gcm_ctx->key;
  GCM128_CONTEXT gcm;
  CRYPTO_gcm128_init_ctx(key, &gcm, nonce, nonce_len);

  if (!CRYPTO_gcm128_aad(key, &gcm, ad, ad_len)) {
    return 0;
  }

  if (!CRYPTO_gcm128_decrypt(key, &gcm, in, out, in_len)) {
    return 0;
  }

  CRYPTO_gcm128_tag(key, &gcm, tag, tag_len);
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

  // |BCM_rand_bytes| calls within the fipsmodule should be wrapped with state
  // lock functions to avoid updating the service indicator with the DRBG
  // functions.
  FIPS_service_indicator_lock_state();
  BCM_rand_bytes(nonce, sizeof(nonce));
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
  if (!aead_aes_gcm_open_gather_impl(
          gcm_ctx, out, nonce, AES_GCM_NONCE_LENGTH, in, in_len, in_tag,
          in_tag_len - AES_GCM_NONCE_LENGTH, ad, ad_len,
          ctx->tag_len - AES_GCM_NONCE_LENGTH)) {
    return 0;
  }

  AEAD_GCM_verify_service_indicator(ctx);
  return 1;
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_128_gcm_randnonce) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 16;
  out->nonce_len = 0;
  out->overhead = EVP_AEAD_AES_GCM_TAG_LEN + AES_GCM_NONCE_LENGTH;
  out->max_tag_len = EVP_AEAD_AES_GCM_TAG_LEN + AES_GCM_NONCE_LENGTH;
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
  out->seal_scatter_supports_extra_in = 1;

  out->init = aead_aes_gcm_init_randnonce;
  out->cleanup = aead_aes_gcm_cleanup;
  out->seal_scatter = aead_aes_gcm_seal_scatter_randnonce;
  out->open_gather = aead_aes_gcm_open_gather_randnonce;
}

namespace {
struct aead_aes_gcm_tls12_ctx {
  struct aead_aes_gcm_ctx gcm_ctx;
  uint64_t min_next_nonce;
};
}  // namespace

static_assert(sizeof(((EVP_AEAD_CTX *)NULL)->state) >=
                  sizeof(struct aead_aes_gcm_tls12_ctx),
              "AEAD state is too small");
static_assert(alignof(union evp_aead_ctx_st_state) >=
                  alignof(struct aead_aes_gcm_tls12_ctx),
              "AEAD state has insufficient alignment");

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

  if (!aead_aes_gcm_seal_scatter(ctx, out, out_tag, out_tag_len,
                                 max_out_tag_len, nonce, nonce_len, in, in_len,
                                 extra_in, extra_in_len, ad, ad_len)) {
    return 0;
  }

  AEAD_GCM_verify_service_indicator(ctx);
  return 1;
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_128_gcm_tls12) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 16;
  out->nonce_len = AES_GCM_NONCE_LENGTH;
  out->overhead = EVP_AEAD_AES_GCM_TAG_LEN;
  out->max_tag_len = EVP_AEAD_AES_GCM_TAG_LEN;
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
  out->seal_scatter_supports_extra_in = 1;

  out->init = aead_aes_gcm_tls12_init;
  out->cleanup = aead_aes_gcm_cleanup;
  out->seal_scatter = aead_aes_gcm_tls12_seal_scatter;
  out->open_gather = aead_aes_gcm_open_gather;
}

namespace {
struct aead_aes_gcm_tls13_ctx {
  struct aead_aes_gcm_ctx gcm_ctx;
  uint64_t min_next_nonce;
  uint64_t mask;
};
}  // namespace

static_assert(sizeof(((EVP_AEAD_CTX *)NULL)->state) >=
                  sizeof(struct aead_aes_gcm_tls13_ctx),
              "AEAD state is too small");
static_assert(alignof(union evp_aead_ctx_st_state) >=
                  alignof(struct aead_aes_gcm_tls13_ctx),
              "AEAD state has insufficient alignment");

static int aead_aes_gcm_tls13_init(EVP_AEAD_CTX *ctx, const uint8_t *key,
                                   size_t key_len, size_t requested_tag_len) {
  struct aead_aes_gcm_tls13_ctx *gcm_ctx =
      (struct aead_aes_gcm_tls13_ctx *)&ctx->state;

  gcm_ctx->min_next_nonce = 0;

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

  if (gcm_ctx->min_next_nonce == 0) {
    // In the first call the sequence number will be zero and therefore the
    // given nonce will be 0 ^ mask = mask.
    gcm_ctx->mask = given_counter;
    gcm_ctx->min_next_nonce = 1;
  } else {
    given_counter ^= gcm_ctx->mask;
    if (given_counter == UINT64_MAX ||
        given_counter < gcm_ctx->min_next_nonce) {
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_NONCE);
      return 0;
    }
    gcm_ctx->min_next_nonce = given_counter + 1;
  }

  if (!aead_aes_gcm_seal_scatter(ctx, out, out_tag, out_tag_len,
                                 max_out_tag_len, nonce, nonce_len, in, in_len,
                                 extra_in, extra_in_len, ad, ad_len)) {
    return 0;
  }

  AEAD_GCM_verify_service_indicator(ctx);
  return 1;
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_128_gcm_tls13) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 16;
  out->nonce_len = AES_GCM_NONCE_LENGTH;
  out->overhead = EVP_AEAD_AES_GCM_TAG_LEN;
  out->max_tag_len = EVP_AEAD_AES_GCM_TAG_LEN;
  out->seal_scatter_supports_extra_in = 1;

  out->init = aead_aes_gcm_tls13_init;
  out->cleanup = aead_aes_gcm_cleanup;
  out->seal_scatter = aead_aes_gcm_tls13_seal_scatter;
  out->open_gather = aead_aes_gcm_open_gather;
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_256_gcm_tls13) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 32;
  out->nonce_len = AES_GCM_NONCE_LENGTH;
  out->overhead = EVP_AEAD_AES_GCM_TAG_LEN;
  out->max_tag_len = EVP_AEAD_AES_GCM_TAG_LEN;
  out->seal_scatter_supports_extra_in = 1;

  out->init = aead_aes_gcm_tls13_init;
  out->cleanup = aead_aes_gcm_cleanup;
  out->seal_scatter = aead_aes_gcm_tls13_seal_scatter;
  out->open_gather = aead_aes_gcm_open_gather;
}

int EVP_has_aes_hardware(void) {
#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
  return hwaes_capable() && crypto_gcm_clmul_enabled();
#elif defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64)
  return hwaes_capable() && CRYPTO_is_ARMv8_PMULL_capable();
#else
  return 0;
#endif
}
