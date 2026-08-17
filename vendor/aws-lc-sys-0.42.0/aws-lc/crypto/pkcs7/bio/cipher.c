// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/buffer.h>
#include <openssl/crypto.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/pkcs7.h>
#include <stdio.h>
#include "../../fipsmodule/cipher/internal.h"
#include "../internal.h"

typedef struct enc_struct {
  uint8_t done;  // indicates "EOF" for read, "flushed" for write
  uint8_t ok;    // cipher status, either 0 (error) or 1 (ok)
  int buf_off;   // start idx of buffered data
  int buf_len;   // length of buffered data
  EVP_CIPHER_CTX *cipher;
  uint8_t buf[1024 * 4];  // plaintext for read, ciphertext for writes
} BIO_ENC_CTX;

static int enc_new(BIO *b) {
  BIO_ENC_CTX *ctx;
  GUARD_PTR(b);

  if ((ctx = OPENSSL_zalloc(sizeof(*ctx))) == NULL) {
    return 0;
  }

  ctx->cipher = EVP_CIPHER_CTX_new();
  if (ctx->cipher == NULL) {
    OPENSSL_free(ctx);
    return 0;
  }
  ctx->done = 0;
  ctx->ok = 1;
  ctx->buf_off = 0;
  ctx->buf_len = 0;
  BIO_set_data(b, ctx);
  BIO_set_init(b, 1);

  return 1;
}

static int enc_free(BIO *b) {
  GUARD_PTR(b);

  BIO_ENC_CTX *ctx = BIO_get_data(b);
  if (ctx == NULL) {
    return 0;
  }

  EVP_CIPHER_CTX_free(ctx->cipher);
  OPENSSL_free(ctx);
  BIO_set_data(b, NULL);
  BIO_set_init(b, 0);

  return 1;
}

static int enc_read(BIO *b, char *out, int outl) {
  GUARD_PTR(b);
  GUARD_PTR(out);
  BIO_ENC_CTX *ctx = BIO_get_data(b);
  if (ctx == NULL || ctx->cipher == NULL || !ctx->ok || outl <= 0) {
    return 0;
  }
  BIO *next = BIO_next(b);
  if (next == NULL) {
    return 0;
  }

  int bytes_output = 0;
  int remaining = outl;
  uint8_t read_buf[sizeof(ctx->buf)];
  const int cipher_block_size = EVP_CIPHER_CTX_block_size(ctx->cipher);
  while ((!ctx->done || ctx->buf_len > 0) && remaining > 0) {
    assert(bytes_output + remaining == outl);
    if (ctx->buf_len > 0) {
      uint8_t *out_pos = ((uint8_t *)out) + bytes_output;
      int to_copy = remaining > ctx->buf_len ? ctx->buf_len : remaining;
      OPENSSL_memcpy(out_pos, &ctx->buf[ctx->buf_off], to_copy);
      // Update buffer info and counters with number of bytes processed from our
      // buffer.
      ctx->buf_len -= to_copy;
      ctx->buf_off += to_copy;
      bytes_output += to_copy;
      remaining -= to_copy;
      continue;
    }
    ctx->buf_len = 0;
    ctx->buf_off = 0;
    // |EVP_DecryptUpdate| may write up to cipher_block_size-1 more bytes than
    // requested, so only read bytes we're sure we can decrypt in place.
    int to_read = (int)sizeof(ctx->buf) - cipher_block_size + 1;
    int bytes_read = BIO_read(next, read_buf, to_read);
    if (bytes_read > 0) {
      // Decrypt ciphertext in place, update |ctx->buf_len| with num bytes
      // decrypted.
      ctx->ok = EVP_DecryptUpdate(ctx->cipher, ctx->buf, &ctx->buf_len,
                                  read_buf, bytes_read);
    } else if (BIO_eof(next)) {
      // EVP_DecryptFinal_ex may write up to one block to our buffer. If that
      // happens, continue the loop to process the decrypted block as normal.
      ctx->ok = EVP_DecryptFinal_ex(ctx->cipher, ctx->buf, &ctx->buf_len);
      ctx->done = 1;  // If we can't read any more bytes, set done.
    } else {
      // |BIO_read| returned <= 0, but no EOF. Copy retry and return.
      if (bytes_read < 0 && !BIO_should_retry(next)) {
        ctx->done = 1;
        ctx->ok = 0;
      }
      BIO_copy_next_retry(b);
      break;
    }
    if (!ctx->ok) {
      ctx->done = 1;  // Set EOF on cipher error.
    }
  }
  return bytes_output;
}

static int enc_flush(BIO *b, BIO *next, BIO_ENC_CTX *ctx) {
  GUARD_PTR(b);
  GUARD_PTR(next);
  GUARD_PTR(ctx);
  while (ctx->ok > 0 && (ctx->buf_len > 0 || !ctx->done)) {
    int bytes_written = BIO_write(next, &ctx->buf[ctx->buf_off], ctx->buf_len);
    if (ctx->buf_len > 0 && bytes_written <= 0) {
      if (bytes_written < 0 && !BIO_should_retry(next)) {
        ctx->done = 1;
        ctx->ok = 0;
      }
      BIO_copy_next_retry(b);
      return 0;
    }
    ctx->buf_off += bytes_written;
    ctx->buf_len -= bytes_written;
    if (ctx->buf_len == 0 && !ctx->done) {
      ctx->done = 1;
      ctx->buf_off = 0;
      ctx->ok = EVP_EncryptFinal_ex(ctx->cipher, ctx->buf, &ctx->buf_len);
    }
  }
  return ctx->ok;
}

static int enc_write(BIO *b, const char *in, int inl) {
  GUARD_PTR(b);
  GUARD_PTR(in);
  BIO_ENC_CTX *ctx = BIO_get_data(b);
  if (ctx == NULL || ctx->cipher == NULL || ctx->done || !ctx->ok || inl <= 0) {
    return 0;
  }
  BIO *next = BIO_next(b);
  if (next == NULL) {
    return 0;
  }

  int bytes_consumed = 0;
  int remaining = inl;
  const int max_crypt_size =
      (int)sizeof(ctx->buf) - EVP_CIPHER_CTX_block_size(ctx->cipher) + 1;
  while ((!ctx->done || ctx->buf_len > 0) && remaining > 0) {
    assert(bytes_consumed + remaining == inl);
    if (ctx->buf_len == 0) {
      ctx->buf_off = 0;
      int to_encrypt = remaining < max_crypt_size ? remaining : max_crypt_size;
      uint8_t *in_pos = ((uint8_t *)in) + bytes_consumed;
      ctx->ok = EVP_EncryptUpdate(ctx->cipher, ctx->buf, &ctx->buf_len, in_pos,
                                  to_encrypt);
      if (!ctx->ok) {
        break;
      };
      bytes_consumed += to_encrypt;
      remaining -= to_encrypt;
    }
    int bytes_written = BIO_write(next, &ctx->buf[ctx->buf_off], ctx->buf_len);
    if (bytes_written <= 0) {
      if (bytes_written < 0 && !BIO_should_retry(next)) {
        ctx->done = 1;
        ctx->ok = 0;
      }
      BIO_copy_next_retry(b);
      break;
    }
    ctx->buf_off += bytes_written;
    ctx->buf_len -= bytes_written;
  }
  return bytes_consumed;
}

static long enc_ctrl(BIO *b, int cmd, long num, void *ptr) {
  GUARD_PTR(b);
  long ret = 1;
  BIO_ENC_CTX *ctx = BIO_get_data(b);
  EVP_CIPHER_CTX **cipher_ctx;
  BIO *next = BIO_next(b);
  if (ctx == NULL) {
    return 0;
  }

  switch (cmd) {
    case BIO_CTRL_RESET:
      ctx->done = 0;
      ctx->ok = 1;
      ctx->buf_off = 0;
      ctx->buf_len = 0;
      OPENSSL_cleanse(ctx->buf, sizeof(ctx->buf));
      if (!EVP_CipherInit_ex(ctx->cipher, NULL, NULL, NULL, NULL,
                             EVP_CIPHER_CTX_encrypting(ctx->cipher))) {
        return 0;
      }
      ret = BIO_ctrl(next, cmd, num, ptr);
      break;
    case BIO_CTRL_EOF:
      if (ctx->done) {
        ret = 1;
      } else {
        ret = BIO_ctrl(next, cmd, num, ptr);
      }
      break;
    case BIO_CTRL_WPENDING:
    case BIO_CTRL_PENDING:
      // Return number of bytes left to process if we have anything buffered,
      // else consult underlying BIO.
      ret = ctx->buf_len;
      if (ret <= 0) {
        ret = BIO_ctrl(next, cmd, num, ptr);
      }
      break;
    case BIO_CTRL_FLUSH:
      ret = enc_flush(b, next, ctx);
      if (ret <= 0) {
        break;
      }
      // Flush the underlying BIO
      ret = BIO_ctrl(next, cmd, num, ptr);
      BIO_copy_next_retry(b);
      break;
    case BIO_C_GET_CIPHER_STATUS:
      ret = (long)ctx->ok;
      break;
    case BIO_C_GET_CIPHER_CTX:
      cipher_ctx = (EVP_CIPHER_CTX **)ptr;
      if (!cipher_ctx) {
        ret = 0;
        break;
      }
      *cipher_ctx = ctx->cipher;
      BIO_set_init(b, 1);
      break;
    // OpenSSL implements these, but because we don't need them and cipher BIO
    // is internal, we can fail loudly if they're called. If this case is hit,
    // it likely means you're making a change that will require implementing
    // these.
    case BIO_CTRL_DUP:
    case BIO_CTRL_GET_CALLBACK:
    case BIO_CTRL_SET_CALLBACK:
    case BIO_C_DO_STATE_MACHINE:
      OPENSSL_PUT_ERROR(PKCS7, ERR_R_BIO_LIB);
      return 0;
    default:
      ret = BIO_ctrl(next, cmd, num, ptr);
      break;
  }
  return ret;
}

int BIO_set_cipher(BIO *b, const EVP_CIPHER *c, const unsigned char *key,
                   const unsigned char *iv, int enc) {
  GUARD_PTR(b);
  GUARD_PTR(c);

  BIO_ENC_CTX *ctx = BIO_get_data(b);
  if (ctx == NULL) {
    return 0;
  }

  // We only support a modern subset of available EVP_CIPHERs. Other ciphers
  // (e.g. DES) and cipher modes (e.g. CBC, CCM) had issues with block alignment
  // and padding during testing, so they're forbidden for now.
  const EVP_CIPHER *kSupportedCiphers[] = {
      EVP_aes_128_cbc(),       EVP_aes_128_ctr(), EVP_aes_128_ofb(),
      EVP_aes_256_cbc(),       EVP_aes_256_ctr(), EVP_aes_256_ofb(),
      EVP_chacha20_poly1305(), EVP_des_ede3_cbc(),
  };
  const size_t kSupportedCiphersCount =
      sizeof(kSupportedCiphers) / sizeof(EVP_CIPHER *);
  int supported = 0;
  for (size_t i = 0; i < kSupportedCiphersCount; i++) {
    if (c == kSupportedCiphers[i]) {
      supported = 1;
      break;
    }
  }
  if (!supported) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_BIO_LIB);
    return 0;
  }

  if (!EVP_CipherInit_ex(ctx->cipher, c, NULL, key, iv, enc)) {
    return 0;
  }
  BIO_set_init(b, 1);

  return 1;
}

static const BIO_METHOD methods_enc = {
    BIO_TYPE_CIPHER,  // type
    "cipher",         // name
    enc_write,        // bwrite
    enc_read,         // bread
    NULL,             // bputs
    NULL,             // bgets
    enc_ctrl,         // ctrl
    enc_new,          // create
    enc_free,         // destroy
    NULL,             // callback_ctrl
};

const BIO_METHOD *BIO_f_cipher(void) { return &methods_enc; }

int BIO_get_cipher_ctx(BIO *b, EVP_CIPHER_CTX **ctx) {
  return BIO_ctrl(b, BIO_C_GET_CIPHER_CTX, 0, ctx);
}

int BIO_get_cipher_status(BIO *b) {
  return BIO_ctrl(b, BIO_C_GET_CIPHER_STATUS, 0, NULL);
}
