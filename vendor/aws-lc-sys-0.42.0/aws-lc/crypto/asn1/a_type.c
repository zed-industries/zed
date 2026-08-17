// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>

#include <assert.h>

#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/obj.h>

#include "internal.h"


int ASN1_TYPE_get(const ASN1_TYPE *a) {
  switch (a->type) {
    case V_ASN1_NULL:
    case V_ASN1_BOOLEAN:
      return a->type;
    case V_ASN1_OBJECT:
      return a->value.object != NULL ? a->type : 0;
    default:
      return a->value.asn1_string != NULL ? a->type : 0;
  }
}

const void *asn1_type_value_as_pointer(const ASN1_TYPE *a) {
  switch (a->type) {
    case V_ASN1_NULL:
      return NULL;
    case V_ASN1_BOOLEAN:
      return a->value.boolean ? (void *)0xff : NULL;
    case V_ASN1_OBJECT:
      return a->value.object;
    default:
      return a->value.asn1_string;
  }
}

void asn1_type_set0_string(ASN1_TYPE *a, ASN1_STRING *str) {
  // |ASN1_STRING| types are almost the same as |ASN1_TYPE| types, except that
  // the negative flag is not reflected into |ASN1_TYPE|.
  int type = str->type;
  if (type == V_ASN1_NEG_INTEGER) {
    type = V_ASN1_INTEGER;
  } else if (type == V_ASN1_NEG_ENUMERATED) {
    type = V_ASN1_ENUMERATED;
  }

  // These types are not |ASN1_STRING| types and use a different
  // representation when stored in |ASN1_TYPE|.
  assert(type != V_ASN1_NULL && type != V_ASN1_OBJECT &&
         type != V_ASN1_BOOLEAN);
  ASN1_TYPE_set(a, type, str);
}

void asn1_type_cleanup(ASN1_TYPE *a) {
  switch (a->type) {
    case V_ASN1_NULL:
      a->value.ptr = NULL;
      break;
    case V_ASN1_BOOLEAN:
      a->value.boolean = ASN1_BOOLEAN_NONE;
      break;
    case V_ASN1_OBJECT:
      ASN1_OBJECT_free(a->value.object);
      a->value.object = NULL;
      break;
    default:
      ASN1_STRING_free(a->value.asn1_string);
      a->value.asn1_string = NULL;
      break;
  }
}

void ASN1_TYPE_set(ASN1_TYPE *a, int type, void *value) {
  asn1_type_cleanup(a);
  a->type = type;
  switch (type) {
    case V_ASN1_NULL:
      a->value.ptr = NULL;
      break;
    case V_ASN1_BOOLEAN:
      a->value.boolean = value ? ASN1_BOOLEAN_TRUE : ASN1_BOOLEAN_FALSE;
      break;
    case V_ASN1_OBJECT:
      a->value.object = value;
      break;
    default:
      a->value.asn1_string = value;
      break;
  }
}

int ASN1_TYPE_set1(ASN1_TYPE *a, int type, const void *value) {
  if (!value || (type == V_ASN1_BOOLEAN)) {
    void *p = (void *)value;
    ASN1_TYPE_set(a, type, p);
  } else if (type == V_ASN1_OBJECT) {
    ASN1_OBJECT *odup;
    odup = OBJ_dup(value);
    if (!odup) {
      return 0;
    }
    ASN1_TYPE_set(a, type, odup);
  } else {
    ASN1_STRING *sdup;
    sdup = ASN1_STRING_dup(value);
    if (!sdup) {
      return 0;
    }
    ASN1_TYPE_set(a, type, sdup);
  }
  return 1;
}

// Returns 0 if they are equal, != 0 otherwise.
int ASN1_TYPE_cmp(const ASN1_TYPE *a, const ASN1_TYPE *b) {
  int result = -1;

  if (!a || !b || a->type != b->type) {
    return -1;
  }

  switch (a->type) {
    case V_ASN1_OBJECT:
      result = OBJ_cmp(a->value.object, b->value.object);
      break;
    case V_ASN1_NULL:
      result = 0;  // They do not have content.
      break;
    case V_ASN1_BOOLEAN:
      result = a->value.boolean - b->value.boolean;
      break;
    case V_ASN1_INTEGER:
    case V_ASN1_ENUMERATED:
    case V_ASN1_BIT_STRING:
    case V_ASN1_OCTET_STRING:
    case V_ASN1_SEQUENCE:
    case V_ASN1_SET:
    case V_ASN1_NUMERICSTRING:
    case V_ASN1_PRINTABLESTRING:
    case V_ASN1_T61STRING:
    case V_ASN1_VIDEOTEXSTRING:
    case V_ASN1_IA5STRING:
    case V_ASN1_UTCTIME:
    case V_ASN1_GENERALIZEDTIME:
    case V_ASN1_GRAPHICSTRING:
    case V_ASN1_VISIBLESTRING:
    case V_ASN1_GENERALSTRING:
    case V_ASN1_UNIVERSALSTRING:
    case V_ASN1_BMPSTRING:
    case V_ASN1_UTF8STRING:
    case V_ASN1_OTHER:
    default:
      result = ASN1_STRING_cmp(a->value.asn1_string, b->value.asn1_string);
      break;
  }

  return result;
}
