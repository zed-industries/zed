// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#include <openssl/cipher.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/nid.h>

#include "../../internal.h"
#include "../service_indicator/internal.h"
#include "internal.h"


void EVP_CIPHER_CTX_init(EVP_CIPHER_CTX *ctx) {
  OPENSSL_memset(ctx, 0, sizeof(EVP_CIPHER_CTX));
}

EVP_CIPHER_CTX *EVP_CIPHER_CTX_new(void) {
  EVP_CIPHER_CTX *ctx = reinterpret_cast<EVP_CIPHER_CTX *>(
      OPENSSL_malloc(sizeof(EVP_CIPHER_CTX)));
  if (ctx) {
    EVP_CIPHER_CTX_init(ctx);
  }
  return ctx;
}

int EVP_CIPHER_CTX_cleanup(EVP_CIPHER_CTX *c) {
  if (c->cipher != NULL && c->cipher->cleanup) {
    c->cipher->cleanup(c);
  }
  OPENSSL_free(c->cipher_data);

  OPENSSL_memset(c, 0, sizeof(EVP_CIPHER_CTX));
  return 1;
}

void EVP_CIPHER_CTX_free(EVP_CIPHER_CTX *ctx) {
  if (ctx) {
    EVP_CIPHER_CTX_cleanup(ctx);
    OPENSSL_free(ctx);
  }
}

int EVP_CIPHER_CTX_copy(EVP_CIPHER_CTX *out, const EVP_CIPHER_CTX *in) {
  if (in == NULL || in->cipher == NULL) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INPUT_NOT_INITIALIZED);
    return 0;
  }

  if (in->poisoned) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  EVP_CIPHER_CTX_cleanup(out);
  OPENSSL_memcpy(out, in, sizeof(EVP_CIPHER_CTX));

  if (in->cipher_data && in->cipher->ctx_size) {
    out->cipher_data = OPENSSL_memdup(in->cipher_data, in->cipher->ctx_size);
    if (!out->cipher_data) {
      out->cipher = NULL;
      return 0;
    }
  }

  if (in->cipher->flags & EVP_CIPH_CUSTOM_COPY) {
    if (!in->cipher->ctrl((EVP_CIPHER_CTX *)in, EVP_CTRL_COPY, 0, out)) {
      out->cipher = NULL;
      return 0;
    }
  }

  return 1;
}

int EVP_CIPHER_CTX_reset(EVP_CIPHER_CTX *ctx) {
  EVP_CIPHER_CTX_cleanup(ctx);
  EVP_CIPHER_CTX_init(ctx);
  return 1;
}

int EVP_CipherInit_ex(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *cipher,
                      ENGINE *engine, const uint8_t *key, const uint8_t *iv,
                      int enc) {
  if (enc == -1) {
    enc = ctx->encrypt;
  } else {
    if (enc) {
      enc = 1;
    }
    ctx->encrypt = enc;
  }

  if (cipher) {
    // Ensure a context left from last time is cleared (the previous check
    // attempted to avoid this if the same ENGINE and EVP_CIPHER could be
    // used).
    if (ctx->cipher) {
      EVP_CIPHER_CTX_cleanup(ctx);
      // Restore encrypt and flags
      ctx->encrypt = enc;
    }

    ctx->cipher = cipher;
    if (ctx->cipher->ctx_size) {
      ctx->cipher_data = OPENSSL_malloc(ctx->cipher->ctx_size);
      if (!ctx->cipher_data) {
        ctx->cipher = NULL;
        return 0;
      }
    } else {
      ctx->cipher_data = NULL;
    }

    ctx->key_len = cipher->key_len;
    ctx->flags = 0;

    if (ctx->cipher->flags & EVP_CIPH_CTRL_INIT) {
      if (!EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_INIT, 0, NULL)) {
        ctx->cipher = NULL;
        OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INITIALIZATION_ERROR);
        return 0;
      }
    }
  } else if (!ctx->cipher) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_NO_CIPHER_SET);
    return 0;
  }

  // we assume block size is a power of 2 in *cryptUpdate
  assert(ctx->cipher->block_size == 1 || ctx->cipher->block_size == 8 ||
         ctx->cipher->block_size == 16);

  if (!(EVP_CIPHER_CTX_flags(ctx) & EVP_CIPH_CUSTOM_IV)) {
    switch (EVP_CIPHER_CTX_mode(ctx)) {
      case EVP_CIPH_STREAM_CIPHER:
      case EVP_CIPH_ECB_MODE:
        break;

      case EVP_CIPH_CFB_MODE:
        ctx->num = 0;
        [[fallthrough]];

      case EVP_CIPH_CBC_MODE:
        assert(EVP_CIPHER_CTX_iv_length(ctx) <= sizeof(ctx->iv));
        if (iv) {
          OPENSSL_memcpy(ctx->oiv, iv, EVP_CIPHER_CTX_iv_length(ctx));
        }
        OPENSSL_memcpy(ctx->iv, ctx->oiv, EVP_CIPHER_CTX_iv_length(ctx));
        break;

      case EVP_CIPH_CTR_MODE:
      case EVP_CIPH_OFB_MODE:
        ctx->num = 0;
        // Don't reuse IV for CTR mode
        if (iv) {
          OPENSSL_memcpy(ctx->iv, iv, EVP_CIPHER_CTX_iv_length(ctx));
        }
        break;

      default:
        return 0;
    }
  }

  if (key || (ctx->cipher->flags & EVP_CIPH_ALWAYS_CALL_INIT)) {
    if (!ctx->cipher->init(ctx, key, iv, enc)) {
      return 0;
    }
  }

  ctx->buf_len = 0;
  ctx->final_used = 0;
  // Clear the poisoned flag to permit re-use of a CTX that previously had a
  // failed operation.
  ctx->poisoned = 0;
  return 1;
}

int EVP_EncryptInit_ex(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *cipher,
                       ENGINE *impl, const uint8_t *key, const uint8_t *iv) {
  return EVP_CipherInit_ex(ctx, cipher, impl, key, iv, 1);
}

int EVP_DecryptInit_ex(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *cipher,
                       ENGINE *impl, const uint8_t *key, const uint8_t *iv) {
  return EVP_CipherInit_ex(ctx, cipher, impl, key, iv, 0);
}

// block_remainder returns the number of bytes to remove from |len| to get a
// multiple of |ctx|'s block size.
static int block_remainder(const EVP_CIPHER_CTX *ctx, int len) {
  // |block_size| must be a power of two.
  assert(ctx->cipher->block_size != 0);
  assert((ctx->cipher->block_size & (ctx->cipher->block_size - 1)) == 0);
  return len & (ctx->cipher->block_size - 1);
}

int EVP_EncryptUpdate(EVP_CIPHER_CTX *ctx, uint8_t *out, int *out_len,
                      const uint8_t *in, int in_len) {
  if (ctx->poisoned) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }
  // If the first call to |cipher| succeeds and the second fails, |ctx| may be
  // left in an indeterminate state. We set a poison flag on failure to ensure
  // callers do not continue to use the object in that case.
  ctx->poisoned = 1;

  // Ciphers that use blocks may write up to |bl| extra bytes. Ensure the output
  // does not overflow |*out_len|.
  int bl = ctx->cipher->block_size;
  if (bl > 1 && in_len > INT_MAX - bl) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_OVERFLOW);
    return 0;
  }

  if (ctx->cipher->flags & EVP_CIPH_FLAG_CUSTOM_CIPHER) {
    int ret = ctx->cipher->cipher(ctx, out, in, in_len);
    if (ret < 0) {
      return 0;
    } else {
      *out_len = ret;
    }
    ctx->poisoned = 0;
    return 1;
  }

  if (in_len <= 0) {
    *out_len = 0;
    if (in_len == 0) {
      ctx->poisoned = 0;
      return 1;
    }
    return 0;
  }

  if (ctx->buf_len == 0 && block_remainder(ctx, in_len) == 0) {
    if (ctx->cipher->cipher(ctx, out, in, in_len)) {
      *out_len = in_len;
      ctx->poisoned = 0;
      return 1;
    } else {
      *out_len = 0;
      return 0;
    }
  }

  int i = ctx->buf_len;
  assert(bl <= (int)sizeof(ctx->buf));
  if (i != 0) {
    if (bl - i > in_len) {
      OPENSSL_memcpy(&ctx->buf[i], in, in_len);
      ctx->buf_len += in_len;
      *out_len = 0;
      ctx->poisoned = 0;
      return 1;
    } else {
      int j = bl - i;
      OPENSSL_memcpy(&ctx->buf[i], in, j);
      if (!ctx->cipher->cipher(ctx, out, ctx->buf, bl)) {
        return 0;
      }
      in_len -= j;
      in += j;
      out += bl;
      *out_len = bl;
    }
  } else {
    *out_len = 0;
  }

  i = block_remainder(ctx, in_len);
  in_len -= i;
  if (in_len > 0) {
    if (!ctx->cipher->cipher(ctx, out, in, in_len)) {
      return 0;
    }
    *out_len += in_len;
  }

  if (i != 0) {
    OPENSSL_memcpy(ctx->buf, &in[in_len], i);
  }
  ctx->buf_len = i;
  ctx->poisoned = 0;
  return 1;
}

int EVP_EncryptFinal_ex(EVP_CIPHER_CTX *ctx, uint8_t *out, int *out_len) {
  int n;
  unsigned int i, b, bl;

  if (ctx->poisoned) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  if (ctx->cipher->flags & EVP_CIPH_FLAG_CUSTOM_CIPHER) {
    // When EVP_CIPH_FLAG_CUSTOM_CIPHER is set, the return value of |cipher| is
    // the number of bytes written, or -1 on error. Otherwise the return value
    // is one on success and zero on error.
    const int num_bytes = ctx->cipher->cipher(ctx, out, NULL, 0);
    if (num_bytes < 0) {
      return 0;
    }
    *out_len = num_bytes;
    goto out;
  }

  b = ctx->cipher->block_size;
  assert(b <= sizeof(ctx->buf));
  if (b == 1) {
    *out_len = 0;
    goto out;
  }

  bl = ctx->buf_len;
  if (ctx->flags & EVP_CIPH_NO_PADDING) {
    if (bl) {
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_DATA_NOT_MULTIPLE_OF_BLOCK_LENGTH);
      return 0;
    }
    *out_len = 0;
    goto out;
  }

  n = b - bl;
  for (i = bl; i < b; i++) {
    ctx->buf[i] = n;
  }
  if (!ctx->cipher->cipher(ctx, out, ctx->buf, b)) {
    return 0;
  }
  *out_len = b;

out:
  EVP_Cipher_verify_service_indicator(ctx);
  return 1;
}

int EVP_DecryptUpdate(EVP_CIPHER_CTX *ctx, uint8_t *out, int *out_len,
                      const uint8_t *in, int in_len) {
  if (ctx->poisoned) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  // Ciphers that use blocks may write up to |bl| extra bytes. Ensure the output
  // does not overflow |*out_len|.
  unsigned int b = ctx->cipher->block_size;
  if (b > 1 && in_len > INT_MAX - (int)b) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_OVERFLOW);
    return 0;
  }

  if (ctx->cipher->flags & EVP_CIPH_FLAG_CUSTOM_CIPHER) {
    int r = ctx->cipher->cipher(ctx, out, in, in_len);
    if (r < 0) {
      *out_len = 0;
      return 0;
    } else {
      *out_len = r;
    }
    return 1;
  }

  if (in_len <= 0) {
    *out_len = 0;
    return in_len == 0;
  }

  if (ctx->flags & EVP_CIPH_NO_PADDING) {
    return EVP_EncryptUpdate(ctx, out, out_len, in, in_len);
  }

  assert(b <= sizeof(ctx->final));
  int fix_len = 0;
  if (ctx->final_used) {
    OPENSSL_memcpy(out, ctx->final, b);
    out += b;
    fix_len = 1;
  }

  if (!EVP_EncryptUpdate(ctx, out, out_len, in, in_len)) {
    return 0;
  }

  // if we have 'decrypted' a multiple of block size, make sure
  // we have a copy of this last block
  if (b > 1 && !ctx->buf_len) {
    *out_len -= b;
    ctx->final_used = 1;
    OPENSSL_memcpy(ctx->final, &out[*out_len], b);
  } else {
    ctx->final_used = 0;
  }

  if (fix_len) {
    *out_len += b;
  }

  return 1;
}

int EVP_DecryptFinal_ex(EVP_CIPHER_CTX *ctx, unsigned char *out, int *out_len) {
  int i, n;
  unsigned int b;
  *out_len = 0;

  if (ctx->poisoned) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  if (ctx->cipher->flags & EVP_CIPH_FLAG_CUSTOM_CIPHER) {
    i = ctx->cipher->cipher(ctx, out, NULL, 0);
    if (i < 0) {
      return 0;
    } else {
      *out_len = i;
    }
    goto out;
  }

  b = ctx->cipher->block_size;
  if (ctx->flags & EVP_CIPH_NO_PADDING) {
    if (ctx->buf_len) {
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_DATA_NOT_MULTIPLE_OF_BLOCK_LENGTH);
      return 0;
    }
    *out_len = 0;
    goto out;
  }

  if (b > 1) {
    if (ctx->buf_len || !ctx->final_used) {
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_WRONG_FINAL_BLOCK_LENGTH);
      return 0;
    }
    assert(b <= sizeof(ctx->final));

    // The following assumes that the ciphertext has been authenticated.
    // Otherwise it provides a padding oracle.
    n = ctx->final[b - 1];
    if (n == 0 || n > (int)b) {
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
      return 0;
    }

    for (i = 0; i < n; i++) {
      if (ctx->final[--b] != n) {
        OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
        return 0;
      }
    }

    n = ctx->cipher->block_size - n;
    for (i = 0; i < n; i++) {
      out[i] = ctx->final[i];
    }
    *out_len = n;
  } else {
    *out_len = 0;
  }

out:
  EVP_Cipher_verify_service_indicator(ctx);
  return 1;
}

int EVP_Cipher(EVP_CIPHER_CTX *ctx, uint8_t *out, const uint8_t *in,
               size_t in_len) {
  const int ret = ctx->cipher->cipher(ctx, out, in, in_len);

  // |EVP_CIPH_FLAG_CUSTOM_CIPHER| never sets the FIPS indicator via
  // |EVP_Cipher| because it's complicated whether the operation has completed
  // or not. E.g. AES-GCM with a non-NULL |in| argument hasn't completed an
  // operation. Callers should use the |EVP_AEAD| API or, at least,
  // |EVP_CipherUpdate| etc.
  //
  // This call can't be pushed into |EVP_Cipher_verify_service_indicator|
  // because whether |ret| indicates success or not depends on whether
  // |EVP_CIPH_FLAG_CUSTOM_CIPHER| is set. (This unreasonable, but matches
  // OpenSSL.)
  if (!(ctx->cipher->flags & EVP_CIPH_FLAG_CUSTOM_CIPHER) && ret) {
    EVP_Cipher_verify_service_indicator(ctx);
  }

  return ret;
}

int EVP_CipherUpdate(EVP_CIPHER_CTX *ctx, uint8_t *out, int *out_len,
                     const uint8_t *in, int in_len) {
  if (ctx->encrypt) {
    return EVP_EncryptUpdate(ctx, out, out_len, in, in_len);
  } else {
    return EVP_DecryptUpdate(ctx, out, out_len, in, in_len);
  }
}

int EVP_CipherFinal_ex(EVP_CIPHER_CTX *ctx, uint8_t *out, int *out_len) {
  if (ctx->encrypt) {
    return EVP_EncryptFinal_ex(ctx, out, out_len);
  } else {
    return EVP_DecryptFinal_ex(ctx, out, out_len);
  }
}

const EVP_CIPHER *EVP_CIPHER_CTX_cipher(const EVP_CIPHER_CTX *ctx) {
  return ctx->cipher;
}

int EVP_CIPHER_CTX_nid(const EVP_CIPHER_CTX *ctx) { return ctx->cipher->nid; }

int EVP_CIPHER_CTX_encrypting(const EVP_CIPHER_CTX *ctx) {
  return ctx->encrypt;
}

unsigned EVP_CIPHER_CTX_block_size(const EVP_CIPHER_CTX *ctx) {
  return ctx->cipher->block_size;
}

unsigned EVP_CIPHER_CTX_key_length(const EVP_CIPHER_CTX *ctx) {
  return ctx->key_len;
}

unsigned EVP_CIPHER_CTX_iv_length(const EVP_CIPHER_CTX *ctx) {
  if (EVP_CIPHER_mode(ctx->cipher) == EVP_CIPH_GCM_MODE) {
    int length;
    int res = EVP_CIPHER_CTX_ctrl((EVP_CIPHER_CTX *)ctx, EVP_CTRL_GET_IVLEN, 0,
                                  &length);
    // EVP_CIPHER_CTX_ctrl returning an error should be impossible under this
    // circumstance. If it somehow did, fallback to the static cipher iv_len.
    if (res == 1) {
      return length;
    }
  }
  return ctx->cipher->iv_len;
}

void *EVP_CIPHER_CTX_get_app_data(const EVP_CIPHER_CTX *ctx) {
  return ctx->app_data;
}

void EVP_CIPHER_CTX_set_app_data(EVP_CIPHER_CTX *ctx, void *data) {
  ctx->app_data = data;
}

uint32_t EVP_CIPHER_CTX_flags(const EVP_CIPHER_CTX *ctx) {
  return ctx->cipher->flags & ~EVP_CIPH_MODE_MASK;
}

uint32_t EVP_CIPHER_CTX_mode(const EVP_CIPHER_CTX *ctx) {
  return ctx->cipher->flags & EVP_CIPH_MODE_MASK;
}

int EVP_CIPHER_CTX_ctrl(EVP_CIPHER_CTX *ctx, int command, int arg, void *ptr) {
  int ret;
  if (!ctx->cipher) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_NO_CIPHER_SET);
    return 0;
  }

  if (!ctx->cipher->ctrl) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_CTRL_NOT_IMPLEMENTED);
    return 0;
  }

  ret = ctx->cipher->ctrl(ctx, command, arg, ptr);
  if (ret == -1) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_CTRL_OPERATION_NOT_IMPLEMENTED);
    return 0;
  }

  return ret;
}

int EVP_CIPHER_CTX_set_padding(EVP_CIPHER_CTX *ctx, int pad) {
  if (pad) {
    ctx->flags &= ~EVP_CIPH_NO_PADDING;
  } else {
    ctx->flags |= EVP_CIPH_NO_PADDING;
  }
  return 1;
}

int EVP_CIPHER_CTX_set_key_length(EVP_CIPHER_CTX *c, unsigned key_len) {
  if (c->key_len == key_len) {
    return 1;
  }

  if (key_len == 0 || !(c->cipher->flags & EVP_CIPH_VARIABLE_LENGTH)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_KEY_LENGTH);
    return 0;
  }

  c->key_len = key_len;
  return 1;
}

int EVP_CIPHER_nid(const EVP_CIPHER *cipher) { return cipher->nid; }

unsigned EVP_CIPHER_block_size(const EVP_CIPHER *cipher) {
  return cipher->block_size;
}

unsigned EVP_CIPHER_key_length(const EVP_CIPHER *cipher) {
  return cipher->key_len;
}

unsigned EVP_CIPHER_iv_length(const EVP_CIPHER *cipher) {
  return cipher->iv_len;
}

uint32_t EVP_CIPHER_flags(const EVP_CIPHER *cipher) {
  return cipher->flags & ~EVP_CIPH_MODE_MASK;
}

uint32_t EVP_CIPHER_mode(const EVP_CIPHER *cipher) {
  return cipher->flags & EVP_CIPH_MODE_MASK;
}

int EVP_CipherInit(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *cipher,
                   const uint8_t *key, const uint8_t *iv, int enc) {
  if (cipher) {
    EVP_CIPHER_CTX_init(ctx);
  }
  return EVP_CipherInit_ex(ctx, cipher, NULL, key, iv, enc);
}

int EVP_EncryptInit(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *cipher,
                    const uint8_t *key, const uint8_t *iv) {
  return EVP_CipherInit(ctx, cipher, key, iv, 1);
}

int EVP_DecryptInit(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *cipher,
                    const uint8_t *key, const uint8_t *iv) {
  return EVP_CipherInit(ctx, cipher, key, iv, 0);
}

int EVP_CipherFinal(EVP_CIPHER_CTX *ctx, uint8_t *out, int *out_len) {
  return EVP_CipherFinal_ex(ctx, out, out_len);
}

int EVP_EncryptFinal(EVP_CIPHER_CTX *ctx, uint8_t *out, int *out_len) {
  return EVP_EncryptFinal_ex(ctx, out, out_len);
}

int EVP_DecryptFinal(EVP_CIPHER_CTX *ctx, uint8_t *out, int *out_len) {
  return EVP_DecryptFinal_ex(ctx, out, out_len);
}

int EVP_add_cipher_alias(const char *a, const char *b) { return 1; }

void EVP_CIPHER_CTX_set_flags(const EVP_CIPHER_CTX *ctx, uint32_t flags) {}
