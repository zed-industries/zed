// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>

#include <string.h>

#include <openssl/asn1t.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/obj.h>

#include "../internal.h"
#include "internal.h"


static int asn1_item_ex_combine_new(ASN1_VALUE **pval, const ASN1_ITEM *it,
                                    int combine);
static void asn1_item_clear(ASN1_VALUE **pval, const ASN1_ITEM *it);
static int ASN1_template_new(ASN1_VALUE **pval, const ASN1_TEMPLATE *tt);
static void asn1_template_clear(ASN1_VALUE **pval, const ASN1_TEMPLATE *tt);
static int ASN1_primitive_new(ASN1_VALUE **pval, const ASN1_ITEM *it);
static void asn1_primitive_clear(ASN1_VALUE **pval, const ASN1_ITEM *it);

ASN1_VALUE *ASN1_item_new(const ASN1_ITEM *it) {
  ASN1_VALUE *ret = NULL;
  if (ASN1_item_ex_new(&ret, it) > 0) {
    return ret;
  }
  return NULL;
}

// Allocate an ASN1 structure

int ASN1_item_ex_new(ASN1_VALUE **pval, const ASN1_ITEM *it) {
  return asn1_item_ex_combine_new(pval, it, 0);
}

static int asn1_item_ex_combine_new(ASN1_VALUE **pval, const ASN1_ITEM *it,
                                    int combine) {
  const ASN1_TEMPLATE *tt = NULL;
  const ASN1_EXTERN_FUNCS *ef;
  ASN1_VALUE **pseqval;
  int i;

  switch (it->itype) {
    case ASN1_ITYPE_EXTERN:
      ef = it->funcs;
      if (ef && ef->asn1_ex_new) {
        if (!ef->asn1_ex_new(pval, it)) {
          goto memerr;
        }
      }
      break;

    case ASN1_ITYPE_PRIMITIVE:
      if (it->templates) {
        if (!ASN1_template_new(pval, it->templates)) {
          goto memerr;
        }
      } else if (!ASN1_primitive_new(pval, it)) {
        goto memerr;
      }
      break;

    case ASN1_ITYPE_MSTRING:
      if (!ASN1_primitive_new(pval, it)) {
        goto memerr;
      }
      break;

    case ASN1_ITYPE_CHOICE: {
      const ASN1_AUX *aux = it->funcs;
      ASN1_aux_cb *asn1_cb = aux != NULL ? aux->asn1_cb : NULL;
      if (asn1_cb) {
        i = asn1_cb(ASN1_OP_NEW_PRE, pval, it, NULL);
        if (!i) {
          goto auxerr;
        }
        if (i == 2) {
          return 1;
        }
      }
      if (!combine) {
        *pval = OPENSSL_zalloc(it->size);
        if (!*pval) {
          goto memerr;
        }
      }
      asn1_set_choice_selector(pval, -1, it);
      if (asn1_cb && !asn1_cb(ASN1_OP_NEW_POST, pval, it, NULL)) {
        goto auxerr2;
      }
      break;
    }

    case ASN1_ITYPE_SEQUENCE: {
      const ASN1_AUX *aux = it->funcs;
      ASN1_aux_cb *asn1_cb = aux != NULL ? aux->asn1_cb : NULL;
      if (asn1_cb) {
        i = asn1_cb(ASN1_OP_NEW_PRE, pval, it, NULL);
        if (!i) {
          goto auxerr;
        }
        if (i == 2) {
          return 1;
        }
      }
      if (!combine) {
        *pval = OPENSSL_zalloc(it->size);
        if (!*pval) {
          goto memerr;
        }
        asn1_refcount_set_one(pval, it);
        asn1_enc_init(pval, it);
      }
      for (i = 0, tt = it->templates; i < it->tcount; tt++, i++) {
        pseqval = asn1_get_field_ptr(pval, tt);
        if (!ASN1_template_new(pseqval, tt)) {
          goto memerr2;
        }
      }
      if (asn1_cb && !asn1_cb(ASN1_OP_NEW_POST, pval, it, NULL)) {
        goto auxerr2;
      }
      break;
    }
  }
  return 1;

memerr2:
  asn1_item_combine_free(pval, it, combine);
memerr:
  return 0;

auxerr2:
  asn1_item_combine_free(pval, it, combine);
auxerr:
  OPENSSL_PUT_ERROR(ASN1, ASN1_R_AUX_ERROR);
  return 0;
}

static void asn1_item_clear(ASN1_VALUE **pval, const ASN1_ITEM *it) {
  switch (it->itype) {
    case ASN1_ITYPE_EXTERN:
      *pval = NULL;
      break;

    case ASN1_ITYPE_PRIMITIVE:
      if (it->templates) {
        asn1_template_clear(pval, it->templates);
      } else {
        asn1_primitive_clear(pval, it);
      }
      break;

    case ASN1_ITYPE_MSTRING:
      asn1_primitive_clear(pval, it);
      break;

    case ASN1_ITYPE_CHOICE:
    case ASN1_ITYPE_SEQUENCE:
      *pval = NULL;
      break;
  }
}

static int ASN1_template_new(ASN1_VALUE **pval, const ASN1_TEMPLATE *tt) {
  const ASN1_ITEM *it = ASN1_ITEM_ptr(tt->item);
  int ret;
  if (tt->flags & ASN1_TFLG_OPTIONAL) {
    asn1_template_clear(pval, tt);
    return 1;
  }
  // If ANY DEFINED BY nothing to do

  if (tt->flags & ASN1_TFLG_ADB_MASK) {
    *pval = NULL;
    return 1;
  }
  // If SET OF or SEQUENCE OF, its a STACK
  if (tt->flags & ASN1_TFLG_SK_MASK) {
    STACK_OF(ASN1_VALUE) *skval;
    skval = sk_ASN1_VALUE_new_null();
    if (!skval) {
      ret = 0;
      goto done;
    }
    *pval = (ASN1_VALUE *)skval;
    ret = 1;
    goto done;
  }
  // Otherwise pass it back to the item routine
  ret = asn1_item_ex_combine_new(pval, it, tt->flags & ASN1_TFLG_COMBINE);
done:
  return ret;
}

static void asn1_template_clear(ASN1_VALUE **pval, const ASN1_TEMPLATE *tt) {
  // If ADB or STACK just NULL the field
  if (tt->flags & (ASN1_TFLG_ADB_MASK | ASN1_TFLG_SK_MASK)) {
    *pval = NULL;
  } else {
    asn1_item_clear(pval, ASN1_ITEM_ptr(tt->item));
  }
}

// NB: could probably combine most of the real XXX_new() behaviour and junk
// all the old functions.

static int ASN1_primitive_new(ASN1_VALUE **pval, const ASN1_ITEM *it) {
  if (!it) {
    return 0;
  }

  // Historically, |it->funcs| for primitive types contained an
  // |ASN1_PRIMITIVE_FUNCS| table of calbacks.
  assert(it->funcs == NULL);

  int utype;
  if (it->itype == ASN1_ITYPE_MSTRING) {
    utype = -1;
  } else {
    utype = it->utype;
  }
  switch (utype) {
    case V_ASN1_OBJECT:
      *pval = (ASN1_VALUE *)OBJ_get_undef();
      return 1;

    case V_ASN1_BOOLEAN:
      *(ASN1_BOOLEAN *)pval = (ASN1_BOOLEAN)it->size;
      return 1;

    case V_ASN1_NULL:
      *pval = (ASN1_VALUE *)1;
      return 1;

    case V_ASN1_ANY: {
      ASN1_TYPE *typ = OPENSSL_zalloc(sizeof(ASN1_TYPE));
      if (!typ) {
        return 0;
      }
      typ->type = -1;
      *pval = (ASN1_VALUE *)typ;
      break;
    }

    default:
      *pval = (ASN1_VALUE *)ASN1_STRING_type_new(utype);
      break;
  }
  if (*pval) {
    return 1;
  }
  return 0;
}

static void asn1_primitive_clear(ASN1_VALUE **pval, const ASN1_ITEM *it) {
  int utype;
  // Historically, |it->funcs| for primitive types contained an
  // |ASN1_PRIMITIVE_FUNCS| table of calbacks.
  assert(it == NULL || it->funcs == NULL);
  if (!it || (it->itype == ASN1_ITYPE_MSTRING)) {
    utype = -1;
  } else {
    utype = it->utype;
  }
  if (utype == V_ASN1_BOOLEAN) {
    *(ASN1_BOOLEAN *)pval = (ASN1_BOOLEAN)it->size;
  } else {
    *pval = NULL;
  }
}
