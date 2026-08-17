// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <string.h>

#include <openssl/buf.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "../internal.h"
#include "internal.h"


// Limit to ensure we don't overflow: much greater than
// anything enountered in practice.

#define NAME_ONELINE_MAX (1024 * 1024)

char *X509_NAME_oneline(const X509_NAME *a, char *buf, int len) {
  X509_NAME_ENTRY *ne;
  size_t i;
  int n, lold, l, l1, l2, num, j, type;
  const char *s;
  char *p;
  unsigned char *q;
  BUF_MEM *b = NULL;
  static const char hex[17] = "0123456789ABCDEF";
  int gs_doit[4];
  char tmp_buf[80];

  if (buf == NULL) {
    if ((b = BUF_MEM_new()) == NULL) {
      goto err;
    }
    if (!BUF_MEM_grow(b, 200)) {
      goto err;
    }
    b->data[0] = '\0';
    len = 200;
  } else if (len <= 0) {
    return NULL;
  }
  if (a == NULL) {
    if (b) {
      buf = b->data;
      OPENSSL_free(b);
    }
    OPENSSL_strlcpy(buf, "NO X509_NAME", len);
    return buf;
  }

  len--;  // space for '\0'
  l = 0;
  for (i = 0; i < sk_X509_NAME_ENTRY_num(a->entries); i++) {
    ne = sk_X509_NAME_ENTRY_value(a->entries, i);
    n = OBJ_obj2nid(ne->object);
    if ((n == NID_undef) || ((s = OBJ_nid2sn(n)) == NULL)) {
      i2t_ASN1_OBJECT(tmp_buf, sizeof(tmp_buf), ne->object);
      s = tmp_buf;
    }
    l1 = strlen(s);

    type = ne->value->type;
    num = ne->value->length;
    if (num > NAME_ONELINE_MAX) {
      OPENSSL_PUT_ERROR(X509, X509_R_NAME_TOO_LONG);
      goto err;
    }
    q = ne->value->data;

    if ((type == V_ASN1_GENERALSTRING) && ((num % 4) == 0)) {
      gs_doit[0] = gs_doit[1] = gs_doit[2] = gs_doit[3] = 0;
      for (j = 0; j < num; j++) {
        if (q[j] != 0) {
          gs_doit[j & 3] = 1;
        }
      }

      if (gs_doit[0] | gs_doit[1] | gs_doit[2]) {
        gs_doit[0] = gs_doit[1] = gs_doit[2] = gs_doit[3] = 1;
      } else {
        gs_doit[0] = gs_doit[1] = gs_doit[2] = 0;
        gs_doit[3] = 1;
      }
    } else {
      gs_doit[0] = gs_doit[1] = gs_doit[2] = gs_doit[3] = 1;
    }

    for (l2 = j = 0; j < num; j++) {
      if (!gs_doit[j & 3]) {
        continue;
      }
      l2++;
      if ((q[j] < ' ') || (q[j] > '~')) {
        l2 += 3;
      }
    }

    lold = l;
    l += 1 + l1 + 1 + l2;
    if (l > NAME_ONELINE_MAX) {
      OPENSSL_PUT_ERROR(X509, X509_R_NAME_TOO_LONG);
      goto err;
    }
    if (b != NULL) {
      if (!BUF_MEM_grow(b, l + 1)) {
        goto err;
      }
      p = &(b->data[lold]);
    } else if (l > len) {
      break;
    } else {
      p = &(buf[lold]);
    }
    *(p++) = '/';
    OPENSSL_memcpy(p, s, (unsigned int)l1);
    p += l1;
    *(p++) = '=';

    q = ne->value->data;

    for (j = 0; j < num; j++) {
      if (!gs_doit[j & 3]) {
        continue;
      }
      n = q[j];
      if ((n < ' ') || (n > '~')) {
        *(p++) = '\\';
        *(p++) = 'x';
        *(p++) = hex[(n >> 4) & 0x0f];
        *(p++) = hex[n & 0x0f];
      } else {
        *(p++) = n;
      }
    }
    *p = '\0';
  }
  if (b != NULL) {
    p = b->data;
    OPENSSL_free(b);
  } else {
    p = buf;
  }
  if (i == 0) {
    *p = '\0';
  }
  return p;
err:
  BUF_MEM_free(b);
  return NULL;
}
