// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/buf.h>

#include <string.h>

#include <openssl/mem.h>
#include <openssl/err.h>

#include "../internal.h"


BUF_MEM *BUF_MEM_new(void) { return OPENSSL_zalloc(sizeof(BUF_MEM)); }

void BUF_MEM_free(BUF_MEM *buf) {
  if (buf == NULL) {
    return;
  }

  OPENSSL_free(buf->data);
  OPENSSL_free(buf);
}

int BUF_MEM_reserve(BUF_MEM *buf, size_t cap) {
  if (buf->max >= cap) {
    return 1;
  }

  size_t n = cap + 3;
  if (n < cap) {
    OPENSSL_PUT_ERROR(BUF, ERR_R_OVERFLOW);
    return 0;
  }
  n = n / 3;
  size_t alloc_size = n * 4;
  if (alloc_size / 4 != n) {
    OPENSSL_PUT_ERROR(BUF, ERR_R_OVERFLOW);
    return 0;
  }

  char *new_buf = OPENSSL_realloc(buf->data, alloc_size);
  if (new_buf == NULL) {
    return 0;
  }

  buf->data = new_buf;
  buf->max = alloc_size;
  return 1;
}

size_t BUF_MEM_grow(BUF_MEM *buf, size_t len) {
  if (!BUF_MEM_reserve(buf, len)) {
    return 0;
  }
  if (buf->length < len) {
    OPENSSL_memset(&buf->data[buf->length], 0, len - buf->length);
  }
  buf->length = len;
  return len;
}

size_t BUF_MEM_grow_clean(BUF_MEM *buf, size_t len) {
  return BUF_MEM_grow(buf, len);
}

int BUF_MEM_append(BUF_MEM *buf, const void *in, size_t len) {
  // Work around a C language bug. See https://crbug.com/1019588.
  if (len == 0) {
    return 1;
  }
  size_t new_len = buf->length + len;
  if (new_len < len) {
    OPENSSL_PUT_ERROR(BUF, ERR_R_OVERFLOW);
    return 0;
  }
  if (!BUF_MEM_reserve(buf, new_len)) {
    return 0;
  }
  OPENSSL_memcpy(buf->data + buf->length, in, len);
  buf->length = new_len;
  return 1;
}

char *BUF_strdup(const char *str) { return OPENSSL_strdup(str); }

size_t BUF_strnlen(const char *str, size_t max_len) {
  return OPENSSL_strnlen(str, max_len);
}

char *BUF_strndup(const char *str, size_t size) {
  return OPENSSL_strndup(str, size);
}

size_t BUF_strlcpy(char *dst, const char *src, size_t dst_size) {
  return OPENSSL_strlcpy(dst, src, dst_size);
}

size_t BUF_strlcat(char *dst, const char *src, size_t dst_size) {
  return OPENSSL_strlcat(dst, src, dst_size);
}

void *BUF_memdup(const void *data, size_t size) {
  return OPENSSL_memdup(data, size);
}
