// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/rc4.h>

const char *RC4_options(void) {
  return "rc4(ptr,int)";
}

void RC4(RC4_KEY *key, size_t len, const uint8_t *in, uint8_t *out) {
  uint32_t x = key->x;
  uint32_t y = key->y;
  uint32_t *d = key->data;

  for (size_t i = 0; i < len; i++) {
    x = (x + 1) & 0xff;
    uint32_t tx = d[x];
    y = (tx + y) & 0xff;
    uint32_t ty = d[y];
    d[x] = ty;
    d[y] = tx;
    out[i] = d[(tx + ty) & 0xff] ^ in[i];
  }

  key->x = x;
  key->y = y;
}

void RC4_set_key(RC4_KEY *rc4key, unsigned len, const uint8_t *key) {
  uint32_t *d = &rc4key->data[0];
  rc4key->x = 0;
  rc4key->y = 0;

  for (unsigned i = 0; i < 256; i++) {
    d[i] = i;
  }

  unsigned id1 = 0, id2 = 0;
  for (unsigned i = 0; i < 256; i++) {
    uint32_t tmp = d[i];
    id2 = (key[id1] + tmp + id2) & 0xff;
    if (++id1 == len) {
      id1 = 0;
    }
    d[i] = d[id2];
    d[id2] = tmp;
  }
}
