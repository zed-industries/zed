// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <string.h>

#include <openssl/base64.h>
#include <openssl/bio.h>
#include <openssl/buffer.h>
#include <openssl/evp.h>
#include <openssl/mem.h>

#include "../../internal.h"


#define B64_BLOCK_SIZE 1024
#define B64_BLOCK_SIZE2 768
#define B64_NONE 0
#define B64_ENCODE 1
#define B64_DECODE 2
#define EVP_ENCODE_LENGTH(l) (((l+2)/3*4)+(l/48+1)*2+80)

typedef struct b64_struct {
  int buf_len;
  int buf_off;
  int tmp_len;  // used to find the start when decoding
  int tmp_nl;   // If true, scan until '\n'
  int encode;
  int start;  // have we started decoding yet?
  int cont;   // <= 0 when finished
  EVP_ENCODE_CTX base64;
  char buf[EVP_ENCODE_LENGTH(B64_BLOCK_SIZE) + 10];
  char tmp[B64_BLOCK_SIZE];
} BIO_B64_CTX;

static int b64_new(BIO *bio) {
  BIO_B64_CTX *ctx;

  ctx = OPENSSL_zalloc(sizeof(*ctx));
  if (ctx == NULL) {
    return 0;
  }

  ctx->cont = 1;
  ctx->start = 1;

  bio->init = 1;
  bio->ptr = (char *)ctx;
  return 1;
}

static int b64_free(BIO *bio) {
  if (bio == NULL) {
    return 0;
  }
  OPENSSL_free(bio->ptr);
  bio->ptr = NULL;
  bio->init = 0;
  bio->flags = 0;
  return 1;
}

static int b64_read(BIO *b, char *out, int outl) {
  int ret = 0, i, ii, j, k, x, n, num, ret_code = 0;
  BIO_B64_CTX *ctx;
  uint8_t *p, *q;

  if (out == NULL) {
    return 0;
  }
  ctx = (BIO_B64_CTX *) b->ptr;

  if (ctx == NULL || b->next_bio == NULL) {
    return 0;
  }

  BIO_clear_retry_flags(b);

  if (ctx->encode != B64_DECODE) {
    ctx->encode = B64_DECODE;
    ctx->buf_len = 0;
    ctx->buf_off = 0;
    ctx->tmp_len = 0;
    EVP_DecodeInit(&ctx->base64);
  }

  // First check if there are bytes decoded/encoded
  if (ctx->buf_len > 0) {
    assert(ctx->buf_len >= ctx->buf_off);
    i = ctx->buf_len - ctx->buf_off;
    if (i > outl) {
      i = outl;
    }
    assert(ctx->buf_off + i < (int)sizeof(ctx->buf));
    OPENSSL_memcpy(out, &ctx->buf[ctx->buf_off], i);
    ret = i;
    out += i;
    outl -= i;
    ctx->buf_off += i;
    if (ctx->buf_len == ctx->buf_off) {
      ctx->buf_len = 0;
      ctx->buf_off = 0;
    }
  }

  // At this point, we have room of outl bytes and an empty buffer, so we
  // should read in some more.

  ret_code = 0;
  while (outl > 0) {
    if (ctx->cont <= 0) {
      break;
    }

    i = BIO_read(b->next_bio, &(ctx->tmp[ctx->tmp_len]),
                 B64_BLOCK_SIZE - ctx->tmp_len);

    if (i <= 0) {
      ret_code = i;

      // Should we continue next time we are called?
      if (!BIO_should_retry(b->next_bio)) {
        ctx->cont = i;
        // If buffer empty break
        if (ctx->tmp_len == 0) {
          break;
        } else {
          // Fall through and process what we have
          i = 0;
        }
      } else {
        // else we retry and add more data to buffer
        break;
      }
    }
    i += ctx->tmp_len;
    ctx->tmp_len = i;

    // We need to scan, a line at a time until we have a valid line if we are
    // starting.
    if (ctx->start && (BIO_test_flags(b, BIO_FLAGS_BASE64_NO_NL))) {
      // ctx->start = 1;
      ctx->tmp_len = 0;
    } else if (ctx->start) {
      q = p = (uint8_t *)ctx->tmp;
      num = 0;
      for (j = 0; j < i; j++) {
        if (*(q++) != '\n') {
          continue;
        }

        // due to a previous very long line, we need to keep on scanning for a
        // '\n' before we even start looking for base64 encoded stuff.
        if (ctx->tmp_nl) {
          p = q;
          ctx->tmp_nl = 0;
          continue;
        }

        k = EVP_DecodeUpdate(&(ctx->base64), (uint8_t *)ctx->buf, &num, p,
                             q - p);

        if (k <= 0 && num == 0 && ctx->start) {
          EVP_DecodeInit(&ctx->base64);
        } else {
          if (p != (uint8_t *)&(ctx->tmp[0])) {
            i -= (p - (uint8_t *)&(ctx->tmp[0]));
            for (x = 0; x < i; x++) {
              ctx->tmp[x] = p[x];
            }
          }
          EVP_DecodeInit(&ctx->base64);
          ctx->start = 0;
          break;
        }
        p = q;
      }

      // we fell off the end without starting
      if (j == i && num == 0) {
        // Is this is one long chunk?, if so, keep on reading until a new
        // line.
        if (p == (uint8_t *)&(ctx->tmp[0])) {
          // Check buffer full
          if (i == B64_BLOCK_SIZE) {
            ctx->tmp_nl = 1;
            ctx->tmp_len = 0;
          }
        } else if (p != q) {  // finished on a '\n'
          n = q - p;
          for (ii = 0; ii < n; ii++) {
            ctx->tmp[ii] = p[ii];
          }
          ctx->tmp_len = n;
        }
        // else finished on a '\n'
        continue;
      } else {
        ctx->tmp_len = 0;
      }
    } else if (i < B64_BLOCK_SIZE && ctx->cont > 0) {
      // If buffer isn't full and we can retry then restart to read in more
      // data.
      continue;
    }

    if (BIO_test_flags(b, BIO_FLAGS_BASE64_NO_NL)) {
      int z, jj;

      jj = i & ~3;  // process per 4
      z = EVP_DecodeBlock((uint8_t *)ctx->buf, (uint8_t *)ctx->tmp, jj);
      if (jj > 2) {
        if (ctx->tmp[jj - 1] == '=') {
          z--;
          if (ctx->tmp[jj - 2] == '=') {
            z--;
          }
        }
      }
      // z is now number of output bytes and jj is the number consumed.
      if (jj != i) {
        OPENSSL_memmove(ctx->tmp, &ctx->tmp[jj], i - jj);
        ctx->tmp_len = i - jj;
      }
      ctx->buf_len = 0;
      if (z > 0) {
        ctx->buf_len = z;
      }
      i = z;
    } else {
      i = EVP_DecodeUpdate(&(ctx->base64), (uint8_t *)ctx->buf,
                           &ctx->buf_len, (uint8_t *)ctx->tmp, i);
      ctx->tmp_len = 0;
    }
    ctx->buf_off = 0;
    if (i < 0) {
      ret_code = 0;
      ctx->buf_len = 0;
      break;
    }

    if (ctx->buf_len <= outl) {
      i = ctx->buf_len;
    } else {
      i = outl;
    }

    OPENSSL_memcpy(out, ctx->buf, i);
    ret += i;
    ctx->buf_off = i;
    if (ctx->buf_off == ctx->buf_len) {
      ctx->buf_len = 0;
      ctx->buf_off = 0;
    }
    outl -= i;
    out += i;
  }

  BIO_copy_next_retry(b);
  return ret == 0 ? ret_code : ret;
}

static int b64_write(BIO *b, const char *in, int inl) {
  int ret = 0, n, i;
  BIO_B64_CTX *ctx;

  ctx = (BIO_B64_CTX *)b->ptr;
  BIO_clear_retry_flags(b);

  if (ctx->encode != B64_ENCODE) {
    ctx->encode = B64_ENCODE;
    ctx->buf_len = 0;
    ctx->buf_off = 0;
    ctx->tmp_len = 0;
    EVP_EncodeInit(&(ctx->base64));
  }

  assert(ctx->buf_off < (int)sizeof(ctx->buf));
  assert(ctx->buf_len <= (int)sizeof(ctx->buf));
  assert(ctx->buf_len >= ctx->buf_off);

  n = ctx->buf_len - ctx->buf_off;
  while (n > 0) {
    i = BIO_write(b->next_bio, &(ctx->buf[ctx->buf_off]), n);
    if (i <= 0) {
      BIO_copy_next_retry(b);
      return i;
    }
    assert(i <= n);
    ctx->buf_off += i;
    assert(ctx->buf_off <= (int)sizeof(ctx->buf));
    assert(ctx->buf_len >= ctx->buf_off);
    n -= i;
  }

  // at this point all pending data has been written.
  ctx->buf_off = 0;
  ctx->buf_len = 0;

  if (in == NULL || inl <= 0) {
    return 0;
  }

  while (inl > 0) {
    n = (inl > B64_BLOCK_SIZE) ? B64_BLOCK_SIZE : inl;

    if (BIO_test_flags(b, BIO_FLAGS_BASE64_NO_NL)) {
      if (ctx->tmp_len > 0) {
        assert(ctx->tmp_len <= 3);
        n = 3 - ctx->tmp_len;
        // There's a theoretical possibility of this.
        if (n > inl) {
          n = inl;
        }
        OPENSSL_memcpy(&(ctx->tmp[ctx->tmp_len]), in, n);
        ctx->tmp_len += n;
        ret += n;
        if (ctx->tmp_len < 3) {
          break;
        }
        ctx->buf_len = EVP_EncodeBlock((uint8_t *)ctx->buf, (uint8_t *)ctx->tmp,
                                       ctx->tmp_len);
        assert(ctx->buf_len <= (int)sizeof(ctx->buf));
        assert(ctx->buf_len >= ctx->buf_off);

        // Since we're now done using the temporary buffer, the length should
        // be zeroed.
        ctx->tmp_len = 0;
      } else {
        if (n < 3) {
          OPENSSL_memcpy(ctx->tmp, in, n);
          ctx->tmp_len = n;
          ret += n;
          break;
        }
        n -= n % 3;
        ctx->buf_len =
            EVP_EncodeBlock((uint8_t *)ctx->buf, (const uint8_t *)in, n);
        assert(ctx->buf_len <= (int)sizeof(ctx->buf));
        assert(ctx->buf_len >= ctx->buf_off);
        ret += n;
      }
    } else {
      if(!EVP_EncodeUpdate(&(ctx->base64), (uint8_t *)ctx->buf, &ctx->buf_len,
                       (uint8_t *)in, n)) {
        return ((ret == 0) ? -1 : ret);
      }
      assert(ctx->buf_len <= (int)sizeof(ctx->buf));
      assert(ctx->buf_len >= ctx->buf_off);
      ret += n;
    }
    inl -= n;
    in += n;

    ctx->buf_off = 0;
    n = ctx->buf_len;

    while (n > 0) {
      i = BIO_write(b->next_bio, &(ctx->buf[ctx->buf_off]), n);
      if (i <= 0) {
        BIO_copy_next_retry(b);
        return ret == 0 ? i : ret;
      }
      assert(i <= n);
      n -= i;
      ctx->buf_off += i;
      assert(ctx->buf_off <= (int)sizeof(ctx->buf));
      assert(ctx->buf_len >= ctx->buf_off);
    }
    ctx->buf_len = 0;
    ctx->buf_off = 0;
  }
  return ret;
}

static long b64_ctrl(BIO *b, int cmd, long num, void *ptr) {
  BIO_B64_CTX *ctx;
  long ret = 1;
  int i;

  ctx = (BIO_B64_CTX *)b->ptr;

  switch (cmd) {
    case BIO_CTRL_RESET:
      ctx->cont = 1;
      ctx->start = 1;
      ctx->encode = B64_NONE;
      ret = BIO_ctrl(b->next_bio, cmd, num, ptr);
      break;

    case BIO_CTRL_EOF:  // More to read
      if (ctx->cont <= 0) {
        ret = 1;
      } else {
        ret = BIO_ctrl(b->next_bio, cmd, num, ptr);
      }
      break;

    case BIO_CTRL_WPENDING:  // More to write in buffer
      assert(ctx->buf_len >= ctx->buf_off);
      ret = ctx->buf_len - ctx->buf_off;
      if ((ret == 0) && (ctx->encode != B64_NONE) && (ctx->base64.data_used != 0)) {
        ret = 1;
      } else if (ret <= 0) {
        ret = BIO_ctrl(b->next_bio, cmd, num, ptr);
      }
      break;

    case BIO_CTRL_PENDING:  // More to read in buffer
      assert(ctx->buf_len >= ctx->buf_off);
      ret = ctx->buf_len - ctx->buf_off;
      if (ret <= 0) {
        ret = BIO_ctrl(b->next_bio, cmd, num, ptr);
      }
      break;

    case BIO_CTRL_FLUSH:
    // do a final write
    again:
      while (ctx->buf_len != ctx->buf_off) {
        i = b64_write(b, NULL, 0);
        if (i < 0) {
          return i;
        }
      }
      if (BIO_test_flags(b, BIO_FLAGS_BASE64_NO_NL)) {
        if (ctx->tmp_len != 0) {
          ctx->buf_len = EVP_EncodeBlock((uint8_t *)ctx->buf,
                                         (uint8_t *)ctx->tmp, ctx->tmp_len);
          ctx->buf_off = 0;
          ctx->tmp_len = 0;
          goto again;
        }
      } else if (ctx->encode != B64_NONE && ctx->base64.data_used != 0) {
        ctx->buf_off = 0;
        EVP_EncodeFinal(&(ctx->base64), (uint8_t *)ctx->buf, &(ctx->buf_len));
        // push out the bytes
        goto again;
      }
      // Finally flush the underlying BIO
      ret = BIO_ctrl(b->next_bio, cmd, num, ptr);
      break;

    case BIO_C_DO_STATE_MACHINE:
      BIO_clear_retry_flags(b);
      ret = BIO_ctrl(b->next_bio, cmd, num, ptr);
      BIO_copy_next_retry(b);
      break;

    case BIO_CTRL_INFO:
    case BIO_CTRL_GET:
    case BIO_CTRL_SET:
    default:
      ret = BIO_ctrl(b->next_bio, cmd, num, ptr);
      break;
  }
  return ret;
}

static long b64_callback_ctrl(BIO *b, int cmd, bio_info_cb fp) {
  if (b->next_bio == NULL) {
    return 0;
  }
  return BIO_callback_ctrl(b->next_bio, cmd, fp);
}

static const BIO_METHOD b64_method = {
    BIO_TYPE_BASE64, "base64 encoding", b64_write, b64_read, NULL /* puts */,
    NULL /* gets */, b64_ctrl,          b64_new,   b64_free, b64_callback_ctrl,
};

const BIO_METHOD *BIO_f_base64(void) { return &b64_method; }
