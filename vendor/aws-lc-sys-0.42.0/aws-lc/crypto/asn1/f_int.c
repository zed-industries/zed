// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>

#include <openssl/bio.h>
#include <openssl/err.h>

int a2i_ASN1_INTEGER(BIO *bp, ASN1_INTEGER *bs, char *buf, int size) {
  int i = 0, j = 0, k = 0, n = 0, again = 0, bufsize = 0;
  unsigned char *s = NULL, *sp = NULL;
  char *bufp = NULL;
  int num = 0, slen = 0, first = 1;

  bs->type = V_ASN1_INTEGER;

  bufsize = BIO_gets(bp, buf, size);
  for (;;) {
    if (bufsize < 1) {
      goto err;
    }
    i = bufsize;
    if (buf[i - 1] == '\n') {
      buf[--i] = '\0';
    }
    if (i == 0) {
      goto err;
    }
    if (buf[i - 1] == '\r') {
      buf[--i] = '\0';
    }
    if (i == 0) {
      goto err;
    }
    again = (buf[i - 1] == '\\');

    for (j = 0; j < i; j++) {
      if (!OPENSSL_isxdigit(buf[j])) {
        i = j;
        break;
      }
    }
    buf[i] = '\0';

    if (i < 2) {
      goto err;
    }

    bufp = buf;
    if (first) {
      first = 0;
      if ((bufp[0] == '0') && (bufp[1] == '0')) {
        bufp += 2;
        i -= 2;
      }
    }
    k = 0;
    if (i % 2 != 0) {
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_ODD_NUMBER_OF_CHARS);
      OPENSSL_free(s);
      return 0;
    }
    i /= 2;
    if (num + i > slen) {
      sp = OPENSSL_realloc(s, num + i);
      if (sp == NULL) {
        OPENSSL_PUT_ERROR(ASN1, ERR_R_MALLOC_FAILURE);
        OPENSSL_free(s);
        return 0;
      }
      s = sp;
      slen = num + i;
    }
    for (j = 0; j < i; j++, k += 2) {
      for (n = 0; n < 2; n++) {
        uint8_t hex = 0;
        if (!OPENSSL_fromxdigit(&hex, bufp[k + n])) {
          OPENSSL_PUT_ERROR(ASN1, ASN1_R_NON_HEX_CHARACTERS);
          goto err;
        }
        s[num + j] <<= 4;
        s[num + j] |= hex;
      }
    }
    num += i;
    if (again) {
      bufsize = BIO_gets(bp, buf, size);
    } else {
      break;
    }
  }
  bs->length = num;
  bs->data = s;
  return 1;
err:
  OPENSSL_PUT_ERROR(ASN1, ASN1_R_SHORT_LINE);
  OPENSSL_free(s);
  return 0;
}

int i2a_ASN1_INTEGER(BIO *bp, const ASN1_INTEGER *a) {
  int i, n = 0;
  static const char *h = "0123456789ABCDEF";
  char buf[2];

  if (a == NULL) {
    return 0;
  }

  if (a->type & V_ASN1_NEG) {
    if (BIO_write(bp, "-", 1) != 1) {
      goto err;
    }
    n = 1;
  }

  if (a->length == 0) {
    if (BIO_write(bp, "00", 2) != 2) {
      goto err;
    }
    n += 2;
  } else {
    for (i = 0; i < a->length; i++) {
      if ((i != 0) && (i % 35 == 0)) {
        if (BIO_write(bp, "\\\n", 2) != 2) {
          goto err;
        }
        n += 2;
      }
      buf[0] = h[((unsigned char)a->data[i] >> 4) & 0x0f];
      buf[1] = h[((unsigned char)a->data[i]) & 0x0f];
      if (BIO_write(bp, buf, 2) != 2) {
        goto err;
      }
      n += 2;
    }
  }
  return n;
err:
  return -1;
}

int i2a_ASN1_ENUMERATED(BIO *bp, const ASN1_ENUMERATED *a) {
  return i2a_ASN1_INTEGER(bp, a);
}
