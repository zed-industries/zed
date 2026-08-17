// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/bio.h>

#include <assert.h>
#include <stdarg.h>
#include <stdio.h>

#include <openssl/err.h>
#include <openssl/mem.h>

int BIO_printf(BIO *bio, const char *format, ...) {
  va_list args;
  char buf[256], *out, out_malloced = 0;
  int out_len, ret;

  va_start(args, format);
  out_len = vsnprintf(buf, sizeof(buf), format, args);
  va_end(args);
  if (out_len < 0) {
    return -1;
  }

  if ((size_t) out_len >= sizeof(buf)) {
    const size_t requested_len = (size_t)out_len;
    // The output was truncated. Note that vsnprintf's return value does not
    // include a trailing NUL, but the buffer must be sized for it.
    out = OPENSSL_malloc(requested_len + 1);
    out_malloced = 1;
    if (out == NULL) {
      return -1;
    }
    va_start(args, format);
    out_len = vsnprintf(out, requested_len + 1, format, args);
    va_end(args);
    assert(out_len == (int)requested_len);
  } else {
    out = buf;
  }

  ret = BIO_write(bio, out, out_len);
  if (out_malloced) {
    OPENSSL_free(out);
  }

  return ret;
}
