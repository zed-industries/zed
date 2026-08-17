// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>

#include <limits.h>
#include <string.h>

#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/obj.h>

#include "../bytestring/internal.h"
#include "../internal.h"
#include "internal.h"


int i2d_ASN1_OBJECT(const ASN1_OBJECT *in, unsigned char **outp) {
  if (in == NULL) {
    OPENSSL_PUT_ERROR(ASN1, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  if (in->length <= 0) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_OBJECT);
    return -1;
  }

  CBB cbb, child;
  if (!CBB_init(&cbb, (size_t)in->length + 2) ||
      !CBB_add_asn1(&cbb, &child, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&child, in->data, in->length)) {
    CBB_cleanup(&cbb);
    return -1;
  }

  return CBB_finish_i2d(&cbb, outp);
}

int i2t_ASN1_OBJECT(char *buf, int buf_len, const ASN1_OBJECT *a) {
  return OBJ_obj2txt(buf, buf_len, a, 0);
}

static int write_str(BIO *bp, const char *str) {
  size_t len = strlen(str);
  if (len > INT_MAX) {
    OPENSSL_PUT_ERROR(ASN1, ERR_R_OVERFLOW);
    return -1;
  }
  return BIO_write(bp, str, (int)len) == (int)len ? (int)len : -1;
}

int i2a_ASN1_OBJECT(BIO *bp, const ASN1_OBJECT *a) {
  if (a == NULL || a->data == NULL) {
    return write_str(bp, "NULL");
  }

  char buf[80], *allocated = NULL;
  const char *str = buf;
  int len = i2t_ASN1_OBJECT(buf, sizeof(buf), a);
  if (len > (int)sizeof(buf) - 1) {
    // The input was truncated. Allocate a buffer that fits.
    allocated = OPENSSL_malloc(len + 1);
    if (allocated == NULL) {
      return -1;
    }
    len = i2t_ASN1_OBJECT(allocated, len + 1, a);
    str = allocated;
  }
  if (len <= 0) {
    str = "<INVALID>";
  }

  int ret = write_str(bp, str);
  OPENSSL_free(allocated);
  return ret;
}

ASN1_OBJECT *d2i_ASN1_OBJECT(ASN1_OBJECT **out, const unsigned char **inp,
                             long len) {
  if (len < 0) {
    return NULL;
  }

  CBS cbs, child;
  CBS_init(&cbs, *inp, (size_t)len);
  if (!CBS_get_asn1(&cbs, &child, CBS_ASN1_OBJECT)) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_DECODE_ERROR);
    return NULL;
  }

  const uint8_t *contents = CBS_data(&child);
  ASN1_OBJECT *ret = c2i_ASN1_OBJECT(out, &contents, CBS_len(&child));
  if (ret != NULL) {
    // |c2i_ASN1_OBJECT| should have consumed the entire input.
    assert(CBS_data(&cbs) == contents);
    *inp = CBS_data(&cbs);
  }
  return ret;
}

ASN1_OBJECT *c2i_ASN1_OBJECT(ASN1_OBJECT **out, const unsigned char **inp,
                             long len) {
  if (len < 0) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_INVALID_OBJECT_ENCODING);
    return NULL;
  }

  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  if (!CBS_is_valid_asn1_oid(&cbs)) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_INVALID_OBJECT_ENCODING);
    return NULL;
  }

  ASN1_OBJECT *ret = ASN1_OBJECT_create(NID_undef, *inp, (size_t)len,
                                        /*sn=*/NULL, /*ln=*/NULL);
  if (ret == NULL) {
    return NULL;
  }

  if (out != NULL) {
    ASN1_OBJECT_free(*out);
    *out = ret;
  }
  *inp += len;  // All bytes were consumed.
  return ret;
}

ASN1_OBJECT *ASN1_OBJECT_new(void) {
  ASN1_OBJECT *ret;

  ret = (ASN1_OBJECT *)OPENSSL_zalloc(sizeof(ASN1_OBJECT));
  if (ret == NULL) {
    return NULL;
  }
  ret->flags = ASN1_OBJECT_FLAG_DYNAMIC;
  return ret;
}

void ASN1_OBJECT_free(ASN1_OBJECT *a) {
  if (a == NULL) {
    return;
  }
  if (a->flags & ASN1_OBJECT_FLAG_DYNAMIC_STRINGS) {
    OPENSSL_free((void *)a->sn);
    OPENSSL_free((void *)a->ln);
    a->sn = a->ln = NULL;
  }
  if (a->flags & ASN1_OBJECT_FLAG_DYNAMIC_DATA) {
    OPENSSL_free((void *)a->data);
    a->data = NULL;
    a->length = 0;
  }
  if (a->flags & ASN1_OBJECT_FLAG_DYNAMIC) {
    OPENSSL_free(a);
  }
}

ASN1_OBJECT *ASN1_OBJECT_create(int nid, const unsigned char *data, size_t len,
                                const char *sn, const char *ln) {
  if (len > INT_MAX) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_STRING_TOO_LONG);
    return NULL;
  }

  ASN1_OBJECT o;
  o.sn = sn;
  o.ln = ln;
  o.data = data;
  o.nid = nid;
  o.length = (int)len;
  o.flags = ASN1_OBJECT_FLAG_DYNAMIC | ASN1_OBJECT_FLAG_DYNAMIC_STRINGS |
            ASN1_OBJECT_FLAG_DYNAMIC_DATA;
  return OBJ_dup(&o);
}
