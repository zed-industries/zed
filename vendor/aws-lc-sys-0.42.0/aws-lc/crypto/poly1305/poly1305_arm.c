// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

// This implementation was taken from the public domain, neon2 version in
// SUPERCOP by D. J. Bernstein and Peter Schwabe.

#include <openssl/poly1305.h>

#include <string.h>

#include "../internal.h"
#include "internal.h"


#if defined(OPENSSL_POLY1305_NEON)

typedef struct {
  uint32_t v[12];  // for alignment; only using 10
} fe1305x2;

#define addmulmod openssl_poly1305_neon2_addmulmod
#define blocks openssl_poly1305_neon2_blocks

extern void addmulmod(fe1305x2 *r, const fe1305x2 *x, const fe1305x2 *y,
                      const fe1305x2 *c);

extern int blocks(fe1305x2 *h, const fe1305x2 *precomp, const uint8_t *in,
                  size_t inlen);

static void freeze(fe1305x2 *r) {
  int i;

  uint32_t x0 = r->v[0];
  uint32_t x1 = r->v[2];
  uint32_t x2 = r->v[4];
  uint32_t x3 = r->v[6];
  uint32_t x4 = r->v[8];
  uint32_t y0;
  uint32_t y1;
  uint32_t y2;
  uint32_t y3;
  uint32_t y4;
  uint32_t swap;

  for (i = 0; i < 3; ++i) {
    x1 += x0 >> 26;
    x0 &= 0x3ffffff;
    x2 += x1 >> 26;
    x1 &= 0x3ffffff;
    x3 += x2 >> 26;
    x2 &= 0x3ffffff;
    x4 += x3 >> 26;
    x3 &= 0x3ffffff;
    x0 += 5 * (x4 >> 26);
    x4 &= 0x3ffffff;
  }

  y0 = x0 + 5;
  y1 = x1 + (y0 >> 26);
  y0 &= 0x3ffffff;
  y2 = x2 + (y1 >> 26);
  y1 &= 0x3ffffff;
  y3 = x3 + (y2 >> 26);
  y2 &= 0x3ffffff;
  y4 = x4 + (y3 >> 26);
  y3 &= 0x3ffffff;
  swap = -(y4 >> 26);
  y4 &= 0x3ffffff;

  y0 ^= x0;
  y1 ^= x1;
  y2 ^= x2;
  y3 ^= x3;
  y4 ^= x4;

  y0 &= swap;
  y1 &= swap;
  y2 &= swap;
  y3 &= swap;
  y4 &= swap;

  y0 ^= x0;
  y1 ^= x1;
  y2 ^= x2;
  y3 ^= x3;
  y4 ^= x4;

  r->v[0] = y0;
  r->v[2] = y1;
  r->v[4] = y2;
  r->v[6] = y3;
  r->v[8] = y4;
}

static void store32(uint8_t out[4], uint32_t v) { OPENSSL_memcpy(out, &v, 4); }

// load32 exists to avoid breaking strict aliasing rules in
// fe1305x2_frombytearray.
static uint32_t load32(const uint8_t t[4]) {
  uint32_t tmp;
  OPENSSL_memcpy(&tmp, t, sizeof(tmp));
  return tmp;
}

static void fe1305x2_tobytearray(uint8_t r[16], fe1305x2 *x) {
  uint32_t x0 = x->v[0];
  uint32_t x1 = x->v[2];
  uint32_t x2 = x->v[4];
  uint32_t x3 = x->v[6];
  uint32_t x4 = x->v[8];

  x1 += x0 >> 26;
  x0 &= 0x3ffffff;
  x2 += x1 >> 26;
  x1 &= 0x3ffffff;
  x3 += x2 >> 26;
  x2 &= 0x3ffffff;
  x4 += x3 >> 26;
  x3 &= 0x3ffffff;

  store32(r, x0 + (x1 << 26));
  store32(r + 4, (x1 >> 6) + (x2 << 20));
  store32(r + 8, (x2 >> 12) + (x3 << 14));
  store32(r + 12, (x3 >> 18) + (x4 << 8));
}

static void fe1305x2_frombytearray(fe1305x2 *r, const uint8_t *x, size_t xlen) {
  size_t i;
  uint8_t t[17];

  for (i = 0; (i < 16) && (i < xlen); i++) {
    t[i] = x[i];
  }
  xlen -= i;
  x += i;
  t[i++] = 1;
  for (; i < 17; i++) {
    t[i] = 0;
  }

  r->v[0] = 0x3ffffff & load32(t);
  r->v[2] = 0x3ffffff & (load32(t + 3) >> 2);
  r->v[4] = 0x3ffffff & (load32(t + 6) >> 4);
  r->v[6] = 0x3ffffff & (load32(t + 9) >> 6);
  r->v[8] = load32(t + 13);

  if (xlen) {
    for (i = 0; (i < 16) && (i < xlen); i++) {
      t[i] = x[i];
    }
    t[i++] = 1;
    for (; i < 17; i++) {
      t[i] = 0;
    }

    r->v[1] = 0x3ffffff & load32(t);
    r->v[3] = 0x3ffffff & (load32(t + 3) >> 2);
    r->v[5] = 0x3ffffff & (load32(t + 6) >> 4);
    r->v[7] = 0x3ffffff & (load32(t + 9) >> 6);
    r->v[9] = load32(t + 13);
  } else {
    r->v[1] = r->v[3] = r->v[5] = r->v[7] = r->v[9] = 0;
  }
}

static const alignas(16) fe1305x2 zero;

struct poly1305_state_st {
  uint8_t data[sizeof(fe1305x2[5]) + 128];
  uint8_t buf[32];
  size_t buf_used;
  uint8_t key[16];
};

OPENSSL_STATIC_ASSERT(
    sizeof(struct poly1305_state_st) + 63 <= sizeof(poly1305_state),
    _poly1305_state_isn_t_large_enough_to_hold_aligned_poly1305_state_st)

void CRYPTO_poly1305_init_neon(poly1305_state *state, const uint8_t key[32]) {
  struct poly1305_state_st *st = poly1305_aligned_state(state);
  fe1305x2 *const r = (fe1305x2 *)(st->data + (15 & (-(int)st->data)));
  fe1305x2 *const h = r + 1;
  fe1305x2 *const c = h + 1;
  fe1305x2 *const precomp = c + 1;

  r->v[1] = r->v[0] = 0x3ffffff & load32(key);
  r->v[3] = r->v[2] = 0x3ffff03 & (load32(key + 3) >> 2);
  r->v[5] = r->v[4] = 0x3ffc0ff & (load32(key + 6) >> 4);
  r->v[7] = r->v[6] = 0x3f03fff & (load32(key + 9) >> 6);
  r->v[9] = r->v[8] = 0x00fffff & (load32(key + 12) >> 8);

  for (size_t j = 0; j < 10; j++) {
    h->v[j] = 0;  // XXX: should fast-forward a bit
  }

  addmulmod(precomp, r, r, &zero);                  // precompute r^2
  addmulmod(precomp + 1, precomp, precomp, &zero);  // precompute r^4

  OPENSSL_memcpy(st->key, key + 16, 16);
  st->buf_used = 0;
}

void CRYPTO_poly1305_update_neon(poly1305_state *state, const uint8_t *in,
                                 size_t in_len) {
  struct poly1305_state_st *st = poly1305_aligned_state(state);
  fe1305x2 *const r = (fe1305x2 *)(st->data + (15 & (-(int)st->data)));
  fe1305x2 *const h = r + 1;
  fe1305x2 *const c = h + 1;
  fe1305x2 *const precomp = c + 1;

  if (st->buf_used) {
    size_t todo = 32 - st->buf_used;
    if (todo > in_len) {
      todo = in_len;
    }
    for (size_t i = 0; i < todo; i++) {
      st->buf[st->buf_used + i] = in[i];
    }
    st->buf_used += todo;
    in_len -= todo;
    in += todo;

    if (st->buf_used == sizeof(st->buf) && in_len) {
      addmulmod(h, h, precomp, &zero);
      fe1305x2_frombytearray(c, st->buf, sizeof(st->buf));
      for (size_t i = 0; i < 10; i++) {
        h->v[i] += c->v[i];
      }
      st->buf_used = 0;
    }
  }

  while (in_len > 32) {
    size_t tlen = 1048576;
    if (in_len < tlen) {
      tlen = in_len;
    }
    tlen -= blocks(h, precomp, in, tlen);
    in_len -= tlen;
    in += tlen;
  }

  if (in_len) {
    for (size_t i = 0; i < in_len; i++) {
      st->buf[i] = in[i];
    }
    st->buf_used = in_len;
  }
}

void CRYPTO_poly1305_finish_neon(poly1305_state *state, uint8_t mac[16]) {
  struct poly1305_state_st *st = poly1305_aligned_state(state);
  fe1305x2 *const r = (fe1305x2 *)(st->data + (15 & (-(int)st->data)));
  fe1305x2 *const h = r + 1;
  fe1305x2 *const c = h + 1;
  fe1305x2 *const precomp = c + 1;

  addmulmod(h, h, precomp, &zero);

  if (st->buf_used > 16) {
    fe1305x2_frombytearray(c, st->buf, st->buf_used);
    precomp->v[1] = r->v[1];
    precomp->v[3] = r->v[3];
    precomp->v[5] = r->v[5];
    precomp->v[7] = r->v[7];
    precomp->v[9] = r->v[9];
    addmulmod(h, h, precomp, c);
  } else if (st->buf_used > 0) {
    fe1305x2_frombytearray(c, st->buf, st->buf_used);
    r->v[1] = 1;
    r->v[3] = 0;
    r->v[5] = 0;
    r->v[7] = 0;
    r->v[9] = 0;
    addmulmod(h, h, r, c);
  }

  h->v[0] += h->v[1];
  h->v[2] += h->v[3];
  h->v[4] += h->v[5];
  h->v[6] += h->v[7];
  h->v[8] += h->v[9];
  freeze(h);

  fe1305x2_frombytearray(c, st->key, 16);
  c->v[8] ^= (1 << 24);

  h->v[0] += c->v[0];
  h->v[2] += c->v[2];
  h->v[4] += c->v[4];
  h->v[6] += c->v[6];
  h->v[8] += c->v[8];
  fe1305x2_tobytearray(mac, h);
}

#endif  // OPENSSL_POLY1305_NEON
