// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>

#include <openssl/err.h>
#include <openssl/mem.h>


ASN1_STRING *ASN1_item_pack(void *obj, const ASN1_ITEM *it, ASN1_STRING **out) {
  uint8_t *new_data = NULL;
  int len = ASN1_item_i2d(obj, &new_data, it);
  if (len <= 0) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_ENCODE_ERROR);
    return NULL;
  }

  ASN1_STRING *ret = NULL;
  if (out == NULL || *out == NULL) {
    ret = ASN1_STRING_new();
    if (ret == NULL) {
      OPENSSL_free(new_data);
      return NULL;
    }
  } else {
    ret = *out;
  }

  ASN1_STRING_set0(ret, new_data, len);
  if (out != NULL) {
    *out = ret;
  }
  return ret;
}

void *ASN1_item_unpack(const ASN1_STRING *oct, const ASN1_ITEM *it) {
  const unsigned char *p = oct->data;
  void *ret = ASN1_item_d2i(NULL, &p, oct->length, it);
  if (ret == NULL || p != oct->data + oct->length) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_DECODE_ERROR);
    ASN1_item_free(ret, it);
    return NULL;
  }
  return ret;
}
