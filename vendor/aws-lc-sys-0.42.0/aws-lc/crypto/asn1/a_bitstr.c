// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>

#include <limits.h>
#include <string.h>

#include <openssl/err.h>
#include <openssl/mem.h>

#include "../internal.h"
#include "internal.h"


int ASN1_BIT_STRING_set(ASN1_BIT_STRING *x, const unsigned char *d,
                        ossl_ssize_t len) {
  return ASN1_STRING_set(x, d, len);
}

int asn1_bit_string_length(const ASN1_BIT_STRING *str,
                           uint8_t *out_padding_bits) {
  int len = str->length;
  if (str->flags & ASN1_STRING_FLAG_BITS_LEFT) {
    // If the string is already empty, it cannot have padding bits.
    *out_padding_bits = len == 0 ? 0 : str->flags & 0x07;
    return len;
  }

  // TODO(https://crbug.com/boringssl/447): If we move this logic to
  // |ASN1_BIT_STRING_set_bit|, can we remove this representation?
  while (len > 0 && str->data[len - 1] == 0) {
    len--;
  }
  uint8_t padding_bits = 0;
  if (len > 0) {
    uint8_t last = str->data[len - 1];
    assert(last != 0);
    for (; padding_bits < 7; padding_bits++) {
      if (last & (1 << padding_bits)) {
        break;
      }
    }
  }
  *out_padding_bits = padding_bits;
  return len;
}

int ASN1_BIT_STRING_num_bytes(const ASN1_BIT_STRING *str, size_t *out) {
  uint8_t padding_bits;
  int len = asn1_bit_string_length(str, &padding_bits);
  if (padding_bits != 0) {
    return 0;
  }
  *out = len;
  return 1;
}

int i2c_ASN1_BIT_STRING(const ASN1_BIT_STRING *a, unsigned char **pp) {
  if (a == NULL) {
    return 0;
  }

  uint8_t bits;
  int len = asn1_bit_string_length(a, &bits);
  if (len > INT_MAX - 1) {
    OPENSSL_PUT_ERROR(ASN1, ERR_R_OVERFLOW);
    return 0;
  }
  int ret = 1 + len;
  if (pp == NULL) {
    return ret;
  }

  uint8_t *p = *pp;
  *(p++) = bits;
  OPENSSL_memcpy(p, a->data, len);
  if (len > 0) {
    p[len - 1] &= (0xff << bits);
  }
  p += len;
  *pp = p;
  return ret;
}

ASN1_BIT_STRING *c2i_ASN1_BIT_STRING(ASN1_BIT_STRING **a,
                                     const unsigned char **pp, long len) {
  ASN1_BIT_STRING *ret = NULL;
  const unsigned char *p;
  unsigned char *s;
  int padding;

  if (len < 1) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_STRING_TOO_SHORT);
    goto err;
  }

  if (len > INT_MAX) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_STRING_TOO_LONG);
    goto err;
  }

  if ((a == NULL) || ((*a) == NULL)) {
    if ((ret = ASN1_BIT_STRING_new()) == NULL) {
      return NULL;
    }
  } else {
    ret = (*a);
  }

  p = *pp;
  padding = *(p++);
  len--;
  if (padding > 7) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_INVALID_BIT_STRING_BITS_LEFT);
    goto err;
  }

  // Unused bits in a BIT STRING must be zero.
  uint8_t padding_mask = (1 << padding) - 1;
  if (padding != 0 && (len < 1 || (p[len - 1] & padding_mask) != 0)) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_INVALID_BIT_STRING_PADDING);
    goto err;
  }

  // We do this to preserve the settings.  If we modify the settings, via
  // the _set_bit function, we will recalculate on output
  ret->flags &= ~(ASN1_STRING_FLAG_BITS_LEFT | 0x07);    // clear
  ret->flags |= (ASN1_STRING_FLAG_BITS_LEFT | padding);  // set

  if (len > 0) {
    s = OPENSSL_memdup(p, len);
    if (s == NULL) {
      goto err;
    }
    p += len;
  } else {
    s = NULL;
  }

  ret->length = (int)len;
  OPENSSL_free(ret->data);
  ret->data = s;
  ret->type = V_ASN1_BIT_STRING;
  if (a != NULL) {
    (*a) = ret;
  }
  *pp = p;
  return ret;
err:
  if ((ret != NULL) && ((a == NULL) || (*a != ret))) {
    ASN1_BIT_STRING_free(ret);
  }
  return NULL;
}

// These next 2 functions from Goetz Babin-Ebell <babinebell@trustcenter.de>
int ASN1_BIT_STRING_set_bit(ASN1_BIT_STRING *a, int n, int value) {
  int w, v, iv;
  unsigned char *c;

  // n is an index, cannot be negative
  if (n < 0) {
    return 0;
  }
  w = n / 8;
  v = 1 << (7 - (n & 0x07));
  iv = ~v;
  if (!value) {
    v = 0;
  }

  if (a == NULL) {
    return 0;
  }

  a->flags &= ~(ASN1_STRING_FLAG_BITS_LEFT | 0x07);  // clear, set on write

  if ((a->length < (w + 1)) || (a->data == NULL)) {
    if (!value) {
      return 1;  // Don't need to set
    }
    if (a->data == NULL) {
      c = (unsigned char *)OPENSSL_malloc(w + 1);
    } else {
      c = (unsigned char *)OPENSSL_realloc(a->data, w + 1);
    }
    if (c == NULL) {
      return 0;
    }
    if (w + 1 - a->length > 0) {
      OPENSSL_memset(c + a->length, 0, w + 1 - a->length);
    }
    a->data = c;
    a->length = w + 1;
  }
  a->data[w] = ((a->data[w]) & iv) | v;
  while ((a->length > 0) && (a->data[a->length - 1] == 0)) {
    a->length--;
  }
  return 1;
}

int ASN1_BIT_STRING_get_bit(const ASN1_BIT_STRING *a, int n) {
  int w, v;

  w = n / 8;
  v = 1 << (7 - (n & 0x07));
  if ((a == NULL) || (a->length < (w + 1)) || (a->data == NULL)) {
    return 0;
  }
  return ((a->data[w] & v) != 0);
}

// Checks if the given bit string contains only bits specified by
// the flags vector. Returns 0 if there is at least one bit set in 'a'
// which is not specified in 'flags', 1 otherwise.
// 'len' is the length of 'flags'.
int ASN1_BIT_STRING_check(const ASN1_BIT_STRING *a, const unsigned char *flags,
                          int flags_len) {
  int i, ok;
  // Check if there is one bit set at all.
  if (!a || !a->data) {
    return 1;
  }

  // Check each byte of the internal representation of the bit string.
  ok = 1;
  for (i = 0; i < a->length && ok; ++i) {
    unsigned char mask = i < flags_len ? ~flags[i] : 0xff;
    // We are done if there is an unneeded bit set.
    ok = (a->data[i] & mask) == 0;
  }
  return ok;
}
