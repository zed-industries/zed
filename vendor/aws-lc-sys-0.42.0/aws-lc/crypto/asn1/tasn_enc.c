// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/asn1t.h>
#include <openssl/mem.h>

#include "../internal.h"
#include "internal.h"


static int asn1_item_ex_i2d_opt(ASN1_VALUE **pval, unsigned char **out,
                                const ASN1_ITEM *it, int tag, int aclass,
                                int optional);
static int asn1_i2d_ex_primitive(ASN1_VALUE **pval, unsigned char **out,
                                 const ASN1_ITEM *it, int tag, int aclass,
                                 int optional);
static int asn1_ex_i2c(ASN1_VALUE **pval, unsigned char *cont, int *out_omit,
                       int *putype, const ASN1_ITEM *it);
static int asn1_set_seq_out(STACK_OF(ASN1_VALUE) *sk, unsigned char **out,
                            int skcontlen, const ASN1_ITEM *item, int do_sort);
static int asn1_template_ex_i2d(ASN1_VALUE **pval, unsigned char **out,
                                const ASN1_TEMPLATE *tt, int tag, int aclass,
                                int optional);

// Top level i2d equivalents

int ASN1_item_i2d(ASN1_VALUE *val, unsigned char **out, const ASN1_ITEM *it) {
  if (out && !*out) {
    unsigned char *p, *buf;
    int len = ASN1_item_ex_i2d(&val, NULL, it, /*tag=*/-1, /*aclass=*/0);
    if (len <= 0) {
      return len;
    }
    buf = OPENSSL_malloc(len);
    if (!buf) {
      return -1;
    }
    p = buf;
    int len2 = ASN1_item_ex_i2d(&val, &p, it, /*tag=*/-1, /*aclass=*/0);
    if (len2 <= 0) {
      OPENSSL_free(buf);
      return len2;
    }
    assert(len == len2);
    *out = buf;
    return len;
  }

  return ASN1_item_ex_i2d(&val, out, it, /*tag=*/-1, /*aclass=*/0);
}

// Encode an item, taking care of IMPLICIT tagging (if any). This function
// performs the normal item handling: it can be used in external types.

int ASN1_item_ex_i2d(ASN1_VALUE **pval, unsigned char **out,
                     const ASN1_ITEM *it, int tag, int aclass) {
  int ret = asn1_item_ex_i2d_opt(pval, out, it, tag, aclass, /*optional=*/0);
  assert(ret != 0);
  return ret;
}

// asn1_item_ex_i2d_opt behaves like |ASN1_item_ex_i2d| but, if |optional| is
// non-zero and |*pval| is omitted, it returns zero and writes no bytes.
int asn1_item_ex_i2d_opt(ASN1_VALUE **pval, unsigned char **out,
                         const ASN1_ITEM *it, int tag, int aclass,
                         int optional) {
  const ASN1_TEMPLATE *tt = NULL;
  int i, seqcontlen, seqlen;

  // Historically, |aclass| was repurposed to pass additional flags into the
  // encoding process.
  assert((aclass & ASN1_TFLG_TAG_CLASS) == aclass);
  // If not overridding the tag, |aclass| is ignored and should be zero.
  assert(tag != -1 || aclass == 0);

  // All fields are pointers, except for boolean |ASN1_ITYPE_PRIMITIVE|s.
  // Optional primitives are handled later.
  if ((it->itype != ASN1_ITYPE_PRIMITIVE) && !*pval) {
    if (optional) {
      return 0;
    }
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_MISSING_VALUE);
    return -1;
  }

  switch (it->itype) {
    case ASN1_ITYPE_PRIMITIVE:
      if (it->templates) {
        // This is an |ASN1_ITEM_TEMPLATE|.
        if (it->templates->flags & ASN1_TFLG_OPTIONAL) {
          OPENSSL_PUT_ERROR(ASN1, ASN1_R_BAD_TEMPLATE);
          return -1;
        }
        return asn1_template_ex_i2d(pval, out, it->templates, tag, aclass,
                                    optional);
      }
      return asn1_i2d_ex_primitive(pval, out, it, tag, aclass, optional);

    case ASN1_ITYPE_MSTRING:
      // It never makes sense for multi-strings to have implicit tagging, so
      // if tag != -1, then this looks like an error in the template.
      if (tag != -1) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_BAD_TEMPLATE);
        return -1;
      }
      return asn1_i2d_ex_primitive(pval, out, it, -1, 0, optional);

    case ASN1_ITYPE_CHOICE: {
      // It never makes sense for CHOICE types to have implicit tagging, so if
      // tag != -1, then this looks like an error in the template.
      if (tag != -1) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_BAD_TEMPLATE);
        return -1;
      }
      i = asn1_get_choice_selector(pval, it);
      if (i < 0 || i >= it->tcount) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_NO_MATCHING_CHOICE_TYPE);
        return -1;
      }
      const ASN1_TEMPLATE *chtt = it->templates + i;
      if (chtt->flags & ASN1_TFLG_OPTIONAL) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_BAD_TEMPLATE);
        return -1;
      }
      ASN1_VALUE **pchval = asn1_get_field_ptr(pval, chtt);
      return asn1_template_ex_i2d(pchval, out, chtt, -1, 0, /*optional=*/0);
    }

    case ASN1_ITYPE_EXTERN: {
      // If new style i2d it does all the work
      const ASN1_EXTERN_FUNCS *ef = it->funcs;
      int ret = ef->asn1_ex_i2d(pval, out, it, tag, aclass);
      if (ret == 0) {
        // |asn1_ex_i2d| should never return zero. We have already checked
        // for optional values generically, and |ASN1_ITYPE_EXTERN| fields
        // must be pointers.
        OPENSSL_PUT_ERROR(ASN1, ERR_R_INTERNAL_ERROR);
        return -1;
      }
      return ret;
    }

    case ASN1_ITYPE_SEQUENCE: {
      i = asn1_enc_restore(&seqcontlen, out, pval, it);
      // An error occurred
      if (i < 0) {
        return -1;
      }
      // We have a valid cached encoding...
      if (i > 0) {
        return seqcontlen;
      }
      // Otherwise carry on
      seqcontlen = 0;
      // If no IMPLICIT tagging set to SEQUENCE, UNIVERSAL
      if (tag == -1) {
        tag = V_ASN1_SEQUENCE;
        aclass = V_ASN1_UNIVERSAL;
      }
      // First work out sequence content length
      for (i = 0, tt = it->templates; i < it->tcount; tt++, i++) {
        const ASN1_TEMPLATE *seqtt;
        ASN1_VALUE **pseqval;
        int tmplen;
        seqtt = asn1_do_adb(pval, tt, 1);
        if (!seqtt) {
          return -1;
        }
        pseqval = asn1_get_field_ptr(pval, seqtt);
        tmplen =
            asn1_template_ex_i2d(pseqval, NULL, seqtt, -1, 0, /*optional=*/0);
        if (tmplen == -1 || (tmplen > INT_MAX - seqcontlen)) {
          return -1;
        }
        seqcontlen += tmplen;
      }

      seqlen = ASN1_object_size(/*constructed=*/1, seqcontlen, tag);
      if (!out || seqlen == -1) {
        return seqlen;
      }
      // Output SEQUENCE header
      ASN1_put_object(out, /*constructed=*/1, seqcontlen, tag, aclass);
      for (i = 0, tt = it->templates; i < it->tcount; tt++, i++) {
        const ASN1_TEMPLATE *seqtt;
        ASN1_VALUE **pseqval;
        seqtt = asn1_do_adb(pval, tt, 1);
        if (!seqtt) {
          return -1;
        }
        pseqval = asn1_get_field_ptr(pval, seqtt);
        if (asn1_template_ex_i2d(pseqval, out, seqtt, -1, 0, /*optional=*/0) <
            0) {
          return -1;
        }
      }
      return seqlen;
    }

    default:
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_BAD_TEMPLATE);
      return -1;
  }
}

// asn1_template_ex_i2d behaves like |asn1_item_ex_i2d_opt| but uses an
// |ASN1_TEMPLATE| instead of an |ASN1_ITEM|. An |ASN1_TEMPLATE| wraps an
// |ASN1_ITEM| with modifiers such as tagging, SEQUENCE or SET, etc.
static int asn1_template_ex_i2d(ASN1_VALUE **pval, unsigned char **out,
                                const ASN1_TEMPLATE *tt, int tag, int iclass,
                                int optional) {
  int i, ret, ttag, tclass;
  size_t j;
  uint32_t flags = tt->flags;

  // Historically, |iclass| was repurposed to pass additional flags into the
  // encoding process.
  assert((iclass & ASN1_TFLG_TAG_CLASS) == iclass);
  // If not overridding the tag, |iclass| is ignored and should be zero.
  assert(tag != -1 || iclass == 0);

  // Work out tag and class to use: tagging may come either from the
  // template or the arguments, not both because this would create
  // ambiguity.
  if (flags & ASN1_TFLG_TAG_MASK) {
    // Error if argument and template tagging
    if (tag != -1) {
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_BAD_TEMPLATE);
      return -1;
    }
    // Get tagging from template
    ttag = tt->tag;
    tclass = flags & ASN1_TFLG_TAG_CLASS;
  } else if (tag != -1) {
    // No template tagging, get from arguments
    ttag = tag;
    tclass = iclass & ASN1_TFLG_TAG_CLASS;
  } else {
    ttag = -1;
    tclass = 0;
  }

  // The template may itself by marked as optional, or this may be the template
  // of an |ASN1_ITEM_TEMPLATE| type which was contained inside an outer
  // optional template. (They cannot both be true because the
  // |ASN1_ITEM_TEMPLATE| codepath rejects optional templates.)
  assert(!optional || (flags & ASN1_TFLG_OPTIONAL) == 0);
  optional = optional || (flags & ASN1_TFLG_OPTIONAL) != 0;

  // At this point 'ttag' contains the outer tag to use, and 'tclass' is the
  // class.

  if (flags & ASN1_TFLG_SK_MASK) {
    // SET OF, SEQUENCE OF
    STACK_OF(ASN1_VALUE) *sk = (STACK_OF(ASN1_VALUE) *)*pval;
    int isset, sktag, skaclass;
    int skcontlen, sklen;
    ASN1_VALUE *skitem;

    if (!*pval) {
      if (optional) {
        return 0;
      }
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_MISSING_VALUE);
      return -1;
    }

    if (flags & ASN1_TFLG_SET_OF) {
      isset = 1;
      // Historically, types with both bits set were mutated when
      // serialized to apply the sort. We no longer support this.
      assert((flags & ASN1_TFLG_SEQUENCE_OF) == 0);
    } else {
      isset = 0;
    }

    // Work out inner tag value: if EXPLICIT or no tagging use underlying
    // type.
    if ((ttag != -1) && !(flags & ASN1_TFLG_EXPTAG)) {
      sktag = ttag;
      skaclass = tclass;
    } else {
      skaclass = V_ASN1_UNIVERSAL;
      if (isset) {
        sktag = V_ASN1_SET;
      } else {
        sktag = V_ASN1_SEQUENCE;
      }
    }

    // Determine total length of items
    skcontlen = 0;
    for (j = 0; j < sk_ASN1_VALUE_num(sk); j++) {
      int tmplen;
      skitem = sk_ASN1_VALUE_value(sk, j);
      tmplen = ASN1_item_ex_i2d(&skitem, NULL, ASN1_ITEM_ptr(tt->item), -1, 0);
      if (tmplen == -1 || (skcontlen > INT_MAX - tmplen)) {
        return -1;
      }
      skcontlen += tmplen;
    }
    sklen = ASN1_object_size(/*constructed=*/1, skcontlen, sktag);
    if (sklen == -1) {
      return -1;
    }
    // If EXPLICIT need length of surrounding tag
    if (flags & ASN1_TFLG_EXPTAG) {
      ret = ASN1_object_size(/*constructed=*/1, sklen, ttag);
    } else {
      ret = sklen;
    }

    if (!out || ret == -1) {
      return ret;
    }

    // Now encode this lot...
    // EXPLICIT tag
    if (flags & ASN1_TFLG_EXPTAG) {
      ASN1_put_object(out, /*constructed=*/1, sklen, ttag, tclass);
    }
    // SET or SEQUENCE and IMPLICIT tag
    ASN1_put_object(out, /*constructed=*/1, skcontlen, sktag, skaclass);
    // And the stuff itself
    if (!asn1_set_seq_out(sk, out, skcontlen, ASN1_ITEM_ptr(tt->item), isset)) {
      return -1;
    }
    return ret;
  }

  if (flags & ASN1_TFLG_EXPTAG) {
    // EXPLICIT tagging
    // Find length of tagged item
    i = asn1_item_ex_i2d_opt(pval, NULL, ASN1_ITEM_ptr(tt->item), -1, 0,
                             optional);
    if (i <= 0) {
      return i;
    }
    // Find length of EXPLICIT tag
    ret = ASN1_object_size(/*constructed=*/1, i, ttag);
    if (out && ret != -1) {
      // Output tag and item
      ASN1_put_object(out, /*constructed=*/1, i, ttag, tclass);
      if (ASN1_item_ex_i2d(pval, out, ASN1_ITEM_ptr(tt->item), -1, 0) < 0) {
        return -1;
      }
    }
    return ret;
  }

  // Either normal or IMPLICIT tagging
  return asn1_item_ex_i2d_opt(pval, out, ASN1_ITEM_ptr(tt->item), ttag, tclass,
                              optional);
}

// Temporary structure used to hold DER encoding of items for SET OF

typedef struct {
  unsigned char *data;
  int length;
} DER_ENC;

static int der_cmp(const void *a, const void *b) {
  const DER_ENC *d1 = a, *d2 = b;
  int cmplen, i;
  cmplen = (d1->length < d2->length) ? d1->length : d2->length;
  i = OPENSSL_memcmp(d1->data, d2->data, cmplen);
  if (i) {
    return i;
  }
  return d1->length - d2->length;
}

// asn1_set_seq_out writes |sk| to |out| under the i2d output convention,
// excluding the tag and length. It returns one on success and zero on error.
// |skcontlen| must be the total encoded size. If |do_sort| is non-zero, the
// elements are sorted for a SET OF type. Each element of |sk| has type
// |item|.
static int asn1_set_seq_out(STACK_OF(ASN1_VALUE) *sk, unsigned char **out,
                            int skcontlen, const ASN1_ITEM *item, int do_sort) {
  // No need to sort if there are fewer than two items.
  if (!do_sort || sk_ASN1_VALUE_num(sk) < 2) {
    for (size_t i = 0; i < sk_ASN1_VALUE_num(sk); i++) {
      ASN1_VALUE *skitem = sk_ASN1_VALUE_value(sk, i);
      if (ASN1_item_ex_i2d(&skitem, out, item, -1, 0) < 0) {
        return 0;
      }
    }
    return 1;
  }

  int ret = 0;
  unsigned char *const buf = OPENSSL_malloc(skcontlen);
  DER_ENC *encoded = OPENSSL_calloc(sk_ASN1_VALUE_num(sk), sizeof(*encoded));
  if (encoded == NULL || buf == NULL) {
    goto err;
  }

  // Encode all the elements into |buf| and populate |encoded|.
  unsigned char *p = buf;
  for (size_t i = 0; i < sk_ASN1_VALUE_num(sk); i++) {
    ASN1_VALUE *skitem = sk_ASN1_VALUE_value(sk, i);
    encoded[i].data = p;
    encoded[i].length = ASN1_item_ex_i2d(&skitem, &p, item, -1, 0);
    if (encoded[i].length < 0) {
      goto err;
    }
    assert(p - buf <= skcontlen);
  }

  qsort(encoded, sk_ASN1_VALUE_num(sk), sizeof(*encoded), der_cmp);

  // Output the elements in sorted order.
  p = *out;
  for (size_t i = 0; i < sk_ASN1_VALUE_num(sk); i++) {
    OPENSSL_memcpy(p, encoded[i].data, encoded[i].length);
    p += encoded[i].length;
  }
  *out = p;

  ret = 1;

err:
  OPENSSL_free(encoded);
  OPENSSL_free(buf);
  return ret;
}

// asn1_i2d_ex_primitive behaves like |ASN1_item_ex_i2d| but |item| must be a
// a PRIMITIVE or MSTRING type that is not an |ASN1_ITEM_TEMPLATE|.
static int asn1_i2d_ex_primitive(ASN1_VALUE **pval, unsigned char **out,
                                 const ASN1_ITEM *it, int tag, int aclass,
                                 int optional) {
  // Get length of content octets and maybe find out the underlying type.
  int omit;
  int utype = it->utype;
  int len = asn1_ex_i2c(pval, NULL, &omit, &utype, it);
  if (len < 0) {
    return -1;
  }
  if (omit) {
    if (optional) {
      return 0;
    }
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_MISSING_VALUE);
    return -1;
  }

  // If SEQUENCE, SET or OTHER then header is included in pseudo content
  // octets so don't include tag+length. We need to check here because the
  // call to asn1_ex_i2c() could change utype.
  int usetag =
      utype != V_ASN1_SEQUENCE && utype != V_ASN1_SET && utype != V_ASN1_OTHER;

  // If not implicitly tagged get tag from underlying type
  if (tag == -1) {
    tag = utype;
  }

  // Output tag+length followed by content octets
  if (out) {
    if (usetag) {
      ASN1_put_object(out, /*constructed=*/0, len, tag, aclass);
    }
    int len2 = asn1_ex_i2c(pval, *out, &omit, &utype, it);
    if (len2 < 0) {
      return -1;
    }
    assert(len == len2);
    assert(!omit);
    *out += len;
  }

  if (usetag) {
    return ASN1_object_size(/*constructed=*/0, len, tag);
  }
  return len;
}

// asn1_ex_i2c writes the |*pval| to |cout| under the i2d output convention,
// excluding the tag and length. It returns the number of bytes written,
// possibly zero, on success or -1 on error. If |*pval| should be omitted, it
// returns zero and sets |*out_omit| to true.
//
// If |it| is an MSTRING or ANY type, it gets the underlying type from |*pval|,
// which must be an |ASN1_STRING| or |ASN1_TYPE|, respectively. It then updates
// |*putype| with the tag number of type used, or |V_ASN1_OTHER| if it was not a
// universal type. If |*putype| is set to |V_ASN1_SEQUENCE|, |V_ASN1_SET|, or
// |V_ASN1_OTHER|, it additionally outputs the tag and length, so the caller
// must not do so.
//
// Otherwise, |*putype| must contain |it->utype|.
//
// WARNING: Unlike most functions in this file, |asn1_ex_i2c| can return zero
// without omitting the element. ASN.1 values may have empty contents.
static int asn1_ex_i2c(ASN1_VALUE **pval, unsigned char *cout, int *out_omit,
                       int *putype, const ASN1_ITEM *it) {
  ASN1_BOOLEAN *tbool = NULL;
  ASN1_STRING *strtmp;
  ASN1_OBJECT *otmp;
  int utype;
  const unsigned char *cont;
  unsigned char c;
  int len;

  // Historically, |it->funcs| for primitive types contained an
  // |ASN1_PRIMITIVE_FUNCS| table of callbacks.
  assert(it->funcs == NULL);

  *out_omit = 0;

  // Should type be omitted?
  if ((it->itype != ASN1_ITYPE_PRIMITIVE) || (it->utype != V_ASN1_BOOLEAN)) {
    if (!*pval) {
      *out_omit = 1;
      return 0;
    }
  }

  if (it->itype == ASN1_ITYPE_MSTRING) {
    // If MSTRING type set the underlying type
    strtmp = (ASN1_STRING *)*pval;
    utype = strtmp->type;
    if (utype < 0 && utype != V_ASN1_OTHER) {
      // MSTRINGs can have type -1 when default-constructed.
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_WRONG_TYPE);
      return -1;
    }
    // Negative INTEGER and ENUMERATED values use |ASN1_STRING| type values
    // that do not match their corresponding utype values. INTEGERs cannot
    // participate in MSTRING types, but ENUMERATEDs can.
    //
    // TODO(davidben): Is this a bug? Although arguably one of the MSTRING
    // types should contain more values, rather than less. See
    // https://crbug.com/boringssl/412. But it is not possible to fit all
    // possible ANY values into an |ASN1_STRING|, so matching the spec here
    // is somewhat hopeless.
    if (utype == V_ASN1_NEG_INTEGER) {
      utype = V_ASN1_INTEGER;
    } else if (utype == V_ASN1_NEG_ENUMERATED) {
      utype = V_ASN1_ENUMERATED;
    }
    *putype = utype;
  } else if (it->utype == V_ASN1_ANY) {
    // If ANY set type and pointer to value
    ASN1_TYPE *typ;
    typ = (ASN1_TYPE *)*pval;
    utype = typ->type;
    if (utype < 0 && utype != V_ASN1_OTHER) {
      // |ASN1_TYPE|s can have type -1 when default-constructed.
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_WRONG_TYPE);
      return -1;
    }
    *putype = utype;
    pval = &typ->value.asn1_value;
  } else {
    utype = *putype;
  }

  switch (utype) {
    case V_ASN1_OBJECT:
      otmp = (ASN1_OBJECT *)*pval;
      cont = otmp->data;
      len = otmp->length;
      if (len == 0) {
        // Some |ASN1_OBJECT|s do not have OIDs and cannot be serialized.
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_OBJECT);
        return -1;
      }
      break;

    case V_ASN1_NULL:
      cont = NULL;
      len = 0;
      break;

    case V_ASN1_BOOLEAN:
      tbool = (ASN1_BOOLEAN *)pval;
      if (*tbool == ASN1_BOOLEAN_NONE) {
        *out_omit = 1;
        return 0;
      }
      if (it->utype != V_ASN1_ANY) {
        // Default handling if value == size field then omit
        if ((*tbool && (it->size > 0)) || (!*tbool && !it->size)) {
          *out_omit = 1;
          return 0;
        }
      }
      c = *tbool ? 0xff : 0x00;
      cont = &c;
      len = 1;
      break;

    case V_ASN1_BIT_STRING: {
      int ret =
          i2c_ASN1_BIT_STRING((ASN1_BIT_STRING *)*pval, cout ? &cout : NULL);
      // |i2c_ASN1_BIT_STRING| returns zero on error instead of -1.
      return ret <= 0 ? -1 : ret;
    }

    case V_ASN1_INTEGER:
    case V_ASN1_ENUMERATED: {
      // |i2c_ASN1_INTEGER| also handles ENUMERATED.
      int ret = i2c_ASN1_INTEGER((ASN1_INTEGER *)*pval, cout ? &cout : NULL);
      // |i2c_ASN1_INTEGER| returns zero on error instead of -1.
      return ret <= 0 ? -1 : ret;
    }

    case V_ASN1_OCTET_STRING:
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
    case V_ASN1_SEQUENCE:
    case V_ASN1_SET:
    default:
      // All based on ASN1_STRING and handled the same
      strtmp = (ASN1_STRING *)*pval;
      cont = strtmp->data;
      len = strtmp->length;

      break;
  }
  if (cout && len) {
    OPENSSL_memcpy(cout, cont, len);
  }
  return len;
}
