// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/bio.h>

#include <limits.h>
#include <string.h>

#include "../internal.h"


// hexdump_ctx contains the state of a hexdump.
struct hexdump_ctx {
  BIO *bio;
  char right_chars[18];  // the contents of the right-hand side, ASCII dump.
  unsigned used;         // number of bytes in the current line.
  size_t n;              // number of bytes total.
  unsigned indent;
};

static void hexbyte(char *out, uint8_t b) {
  static const char hextable[] = "0123456789abcdef";
  out[0] = hextable[b>>4];
  out[1] = hextable[b&0x0f];
}

static char to_char(uint8_t b) {
  if (b < 32 || b > 126) {
          return '.';
  }
  return b;
}

// hexdump_write adds |len| bytes of |data| to the current hex dump described by
// |ctx|.
static int hexdump_write(struct hexdump_ctx *ctx, const uint8_t *data,
                         size_t len) {
  char buf[10];
  unsigned l;

  // Output lines look like:
  // 00000010  2e 2f 30 31 32 33 34 35  36 37 38 ... 3c 3d // |./0123456789:;<=|
  // ^ offset                          ^ extra space           ^ ASCII of line

  for (size_t i = 0; i < len; i++) {
    if (ctx->used == 0) {
      // The beginning of a line.
      if (!BIO_indent(ctx->bio, ctx->indent, UINT_MAX)) {
        return 0;
      }

      hexbyte(&buf[0], ctx->n >> 24);
      hexbyte(&buf[2], ctx->n >> 16);
      hexbyte(&buf[4], ctx->n >> 8);
      hexbyte(&buf[6], ctx->n);
      buf[8] = buf[9] = ' ';
      if (BIO_write(ctx->bio, buf, 10) < 0) {
        return 0;
      }
    }

    hexbyte(buf, data[i]);
    buf[2] = ' ';
    l = 3;
    if (ctx->used == 7) {
      // There's an additional space after the 8th byte.
      buf[3] = ' ';
      l = 4;
    } else if (ctx->used == 15) {
      // At the end of the line there's an extra space and the bar for the
      // right column.
      buf[3] = ' ';
      buf[4] = '|';
      l = 5;
    }

    if (BIO_write(ctx->bio, buf, l) < 0) {
      return 0;
    }
    ctx->right_chars[ctx->used] = to_char(data[i]);
    ctx->used++;
    ctx->n++;
    if (ctx->used == 16) {
      ctx->right_chars[16] = '|';
      ctx->right_chars[17] = '\n';
      if (BIO_write(ctx->bio, ctx->right_chars, sizeof(ctx->right_chars)) < 0) {
        return 0;
      }
      ctx->used = 0;
    }
  }

  return 1;
}

// finish flushes any buffered data in |ctx|.
static int finish(struct hexdump_ctx *ctx) {
  // See the comments in |hexdump| for the details of this format.
  const unsigned n_bytes = ctx->used;
  unsigned l;
  char buf[5];

  if (n_bytes == 0) {
    return 1;
  }

  OPENSSL_memset(buf, ' ', 4);
  buf[4] = '|';

  for (; ctx->used < 16; ctx->used++) {
    l = 3;
    if (ctx->used == 7) {
      l = 4;
    } else if (ctx->used == 15) {
      l = 5;
    }
    if (BIO_write(ctx->bio, buf, l) < 0) {
      return 0;
    }
  }

  ctx->right_chars[n_bytes] = '|';
  ctx->right_chars[n_bytes + 1] = '\n';
  if (BIO_write(ctx->bio, ctx->right_chars, n_bytes + 2) < 0) {
    return 0;
  }
  return 1;
}

int BIO_hexdump(BIO *bio, const uint8_t *data, size_t len, unsigned indent) {
  struct hexdump_ctx ctx;
  OPENSSL_memset(&ctx, 0, sizeof(ctx));
  ctx.bio = bio;
  ctx.indent = indent;

  if (!hexdump_write(&ctx, data, len) || !finish(&ctx)) {
    return 0;
  }

  return 1;
}

int BIO_dump(BIO *bio, const void *data, int len) {
  if (bio == NULL || data == NULL || len < 0) {
    return -1;
  }

  // Use a temporary memory BIO to capture the formatted output
  int ret = -1;
  BIO *mbio = BIO_new(BIO_s_mem());
  if (mbio == NULL) {
    return -1;
  }

  // Generate the hexdump to the memory BIO
  if (!BIO_hexdump(mbio, (const uint8_t *)data, (size_t)len, 0)) {
    goto err;
  }

  // Get the formatted content
  const uint8_t *contents = NULL;
  size_t content_len = 0;
  if (!BIO_mem_contents(mbio, &contents, &content_len)) {
    goto err;
  }

  // Write to the original BIO and return the exact bytes written
  ret = BIO_write(bio, contents, content_len);

err:
  BIO_free(mbio);
  return ret;
}
