// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>

#include <limits.h>
#include <string.h>

#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../bytestring/internal.h"
#include "../internal.h"
#include "internal.h"


// Cross-module errors from crypto/x509/i2d_pr.c.
OPENSSL_DECLARE_ERROR_REASON(ASN1, UNSUPPORTED_PUBLIC_KEY_TYPE)

// Cross-module errors from crypto/x509/algorithm.c.
OPENSSL_DECLARE_ERROR_REASON(ASN1, CONTEXT_NOT_INITIALISED)
OPENSSL_DECLARE_ERROR_REASON(ASN1, DIGEST_AND_KEY_TYPE_NOT_SUPPORTED)
OPENSSL_DECLARE_ERROR_REASON(ASN1, UNKNOWN_MESSAGE_DIGEST_ALGORITHM)
OPENSSL_DECLARE_ERROR_REASON(ASN1, UNKNOWN_SIGNATURE_ALGORITHM)
OPENSSL_DECLARE_ERROR_REASON(ASN1, WRONG_PUBLIC_KEY_TYPE)
// Cross-module errors from crypto/x509/asn1_gen.c. TODO(davidben): Remove
// these once asn1_gen.c is gone.
OPENSSL_DECLARE_ERROR_REASON(ASN1, DEPTH_EXCEEDED)
OPENSSL_DECLARE_ERROR_REASON(ASN1, ILLEGAL_BITSTRING_FORMAT)
OPENSSL_DECLARE_ERROR_REASON(ASN1, ILLEGAL_BOOLEAN)
OPENSSL_DECLARE_ERROR_REASON(ASN1, ILLEGAL_FORMAT)
OPENSSL_DECLARE_ERROR_REASON(ASN1, ILLEGAL_HEX)
OPENSSL_DECLARE_ERROR_REASON(ASN1, ILLEGAL_IMPLICIT_TAG)
OPENSSL_DECLARE_ERROR_REASON(ASN1, ILLEGAL_INTEGER)
OPENSSL_DECLARE_ERROR_REASON(ASN1, ILLEGAL_NESTED_TAGGING)
OPENSSL_DECLARE_ERROR_REASON(ASN1, ILLEGAL_NULL_VALUE)
OPENSSL_DECLARE_ERROR_REASON(ASN1, ILLEGAL_OBJECT)
OPENSSL_DECLARE_ERROR_REASON(ASN1, ILLEGAL_TIME_VALUE)
OPENSSL_DECLARE_ERROR_REASON(ASN1, INTEGER_NOT_ASCII_FORMAT)
OPENSSL_DECLARE_ERROR_REASON(ASN1, INVALID_MODIFIER)
OPENSSL_DECLARE_ERROR_REASON(ASN1, INVALID_NUMBER)
OPENSSL_DECLARE_ERROR_REASON(ASN1, LIST_ERROR)
OPENSSL_DECLARE_ERROR_REASON(ASN1, MISSING_VALUE)
OPENSSL_DECLARE_ERROR_REASON(ASN1, NOT_ASCII_FORMAT)
OPENSSL_DECLARE_ERROR_REASON(ASN1, OBJECT_NOT_ASCII_FORMAT)
OPENSSL_DECLARE_ERROR_REASON(ASN1, SEQUENCE_OR_SET_NEEDS_CONFIG)
OPENSSL_DECLARE_ERROR_REASON(ASN1, TIME_NOT_ASCII_FORMAT)
OPENSSL_DECLARE_ERROR_REASON(ASN1, UNKNOWN_FORMAT)
OPENSSL_DECLARE_ERROR_REASON(ASN1, UNKNOWN_TAG)
OPENSSL_DECLARE_ERROR_REASON(ASN1, UNSUPPORTED_TYPE)

// Limit |ASN1_STRING|s to 64 MiB of data. Most of this module, as well as
// downstream code, does not correctly handle overflow. We cap string fields
// more tightly than strictly necessary to fit in |int|. This is not expected to
// impact real world uses of this field.
//
// In particular, this limit is small enough that the bit count of a BIT STRING
// comfortably fits in an |int|, with room for arithmetic.
#define ASN1_STRING_MAX (64 * 1024 * 1024)

static void asn1_put_length(unsigned char **pp, int length);

// |ASN1_get_object| parses an ASN.1 header, including tag, class, and length
// information. The tag number is written to |*out_tag|. The class is written to
// |*out_class|. If the tag is not indefinite, the content length is written to
// |*out_len|. If the tag indicates the indefinite form, |*out_len| is set to 0.
// |inp| is advanced past the header in the input buffer.
//
// Indefinite-length encoding and universal tags are always allowed to align
// with OpenSSL behavior.
//
// The return value may have the following bits set:
//   * 0x80: error occurred while parsing.
//   * 0x20: the encoding is constructed, not primitive.
//   * 0x01: indefinite-length constructed encoding.
int ASN1_get_object(const unsigned char **inp, long *out_len, int *out_tag,
                    int *out_class, long in_len) {
  if (in_len < 0) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_HEADER_TOO_LONG);
    return 0x80;
  }

  // TODO(https://crbug.com/boringssl/354): This should use |CBS_get_asn1| to
  // reject non-minimal lengths, which are only allowed in BER. However,
  // Android sometimes needs allow a non-minimal length in certificate
  // signature fields (see b/18228011). Make this only apply to that field,
  // while requiring DER elsewhere. Better yet, it should be limited to an
  // preprocessing step in that part of Android.
  CBS_ASN1_TAG tag;
  size_t header_len;
  int indefinite;
  CBS cbs, body;
  CBS_init(&cbs, *inp, (size_t)in_len);

  int ber_found_temp;
  if (!cbs_get_any_asn1_element(&cbs, &body, &tag, &header_len, &ber_found_temp,
                                &indefinite, /*ber_ok=*/1,
                                /*universal_tag_ok=*/1) ||
      !CBS_skip(&body, header_len) ||
      // Bound the length to comfortably fit in an int. Lengths in this
      // module often switch between int and long without overflow checks.
      CBS_len(&body) > INT_MAX / 2) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_HEADER_TOO_LONG);
    return 0x80;
  }

  // Convert between tag representations.
  int tag_class = (tag & CBS_ASN1_CLASS_MASK) >> CBS_ASN1_TAG_SHIFT;
  int constructed = (indefinite ? 1 : 0) |
                    ((tag & CBS_ASN1_CONSTRUCTED) >> CBS_ASN1_TAG_SHIFT);
  int tag_number = tag & CBS_ASN1_TAG_NUMBER_MASK;

  // To avoid ambiguity with V_ASN1_NEG, impose a limit on universal tags.
  if (tag_class == V_ASN1_UNIVERSAL && tag_number > V_ASN1_MAX_UNIVERSAL) {
    // Ruby's OpenSSL::test_basic_asn1data has an assertion that violates this
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_HEADER_TOO_LONG);
    return 0x80;
  }

  *inp = CBS_data(&body);
  // If |indefinite|, |header_len| and |CBS_len(&body)| were equal prior to the
  // |CBS_skip(&body, header_len)| call above and |CBS_len(&body)| is now zero.
  *out_len = CBS_len(&body);
  *out_tag = tag_number;
  *out_class = tag_class;
  return constructed;
}

// class 0 is constructed constructed == 2 for indefinite length constructed
void ASN1_put_object(unsigned char **pp, int constructed, int length, int tag,
                     int xclass) {
  unsigned char *p = *pp;
  int i, ttag;

  i = (constructed) ? V_ASN1_CONSTRUCTED : 0;
  i |= (xclass & V_ASN1_PRIVATE);
  if (tag < 31) {
    *(p++) = i | (tag & V_ASN1_PRIMITIVE_TAG);
  } else {
    *(p++) = i | V_ASN1_PRIMITIVE_TAG;
    for (i = 0, ttag = tag; ttag > 0; i++) {
      ttag >>= 7;
    }
    ttag = i;
    while (i-- > 0) {
      p[i] = tag & 0x7f;
      if (i != (ttag - 1)) {
        p[i] |= 0x80;
      }
      tag >>= 7;
    }
    p += ttag;
  }
  if (constructed == 2) {
    *(p++) = 0x80;
  } else {
    asn1_put_length(&p, length);
  }
  *pp = p;
}

int ASN1_put_eoc(unsigned char **pp) {
  // This function is no longer used in the library, but some external code
  // uses it.
  unsigned char *p = *pp;
  *p++ = 0;
  *p++ = 0;
  *pp = p;
  return 2;
}

static void asn1_put_length(unsigned char **pp, int length) {
  unsigned char *p = *pp;
  int i, l;
  if (length <= 127) {
    *(p++) = (unsigned char)length;
  } else {
    l = length;
    for (i = 0; l > 0; i++) {
      l >>= 8;
    }
    *(p++) = i | 0x80;
    l = i;
    while (i-- > 0) {
      p[i] = length & 0xff;
      length >>= 8;
    }
    p += l;
  }
  *pp = p;
}

int ASN1_object_size(int constructed, int length, int tag) {
  int ret = 1;
  if (length < 0) {
    return -1;
  }
  if (tag >= 31) {
    while (tag > 0) {
      tag >>= 7;
      ret++;
    }
  }
  if (constructed == 2) {
    ret += 3;
  } else {
    ret++;
    if (length > 127) {
      int tmplen = length;
      while (tmplen > 0) {
        tmplen >>= 8;
        ret++;
      }
    }
  }
  if (ret >= INT_MAX - length) {
    return -1;
  }
  return ret + length;
}

int ASN1_STRING_copy(ASN1_STRING *dst, const ASN1_STRING *str) {
  if (str == NULL) {
    return 0;
  }
  if (!ASN1_STRING_set(dst, str->data, str->length)) {
    return 0;
  }
  dst->type = str->type;
  dst->flags = str->flags;
  return 1;
}

ASN1_STRING *ASN1_STRING_dup(const ASN1_STRING *str) {
  ASN1_STRING *ret;
  if (!str) {
    return NULL;
  }
  ret = ASN1_STRING_new();
  if (!ret) {
    return NULL;
  }
  if (!ASN1_STRING_copy(ret, str)) {
    ASN1_STRING_free(ret);
    return NULL;
  }
  return ret;
}

int ASN1_STRING_set(ASN1_STRING *str, const void *_data, ossl_ssize_t len_s) {
  const char *data = _data;
  size_t len;
  if (len_s < 0) {
    if (data == NULL) {
      return 0;
    }
    len = strlen(data);
  } else {
    len = (size_t)len_s;
  }

  OPENSSL_STATIC_ASSERT(ASN1_STRING_MAX < INT_MAX, len_will_not_overflow_int);
  if (len > ASN1_STRING_MAX) {
    OPENSSL_PUT_ERROR(ASN1, ERR_R_OVERFLOW);
    return 0;
  }

  if (str->length <= (int)len || str->data == NULL) {
    unsigned char *c = str->data;
    if (c == NULL) {
      str->data = OPENSSL_malloc(len + 1);
    } else {
      str->data = OPENSSL_realloc(c, len + 1);
    }

    if (str->data == NULL) {
      str->data = c;
      return 0;
    }
  }
  str->length = (int)len;
  if (data != NULL) {
    OPENSSL_memcpy(str->data, data, len);
    // Historically, OpenSSL would NUL-terminate most (but not all)
    // |ASN1_STRING|s, in case anyone accidentally passed |str->data| into a
    // function expecting a C string. We retain this behavior for compatibility,
    // but code must not rely on this. See CVE-2021-3712.
    str->data[len] = '\0';
  }
  return 1;
}

void ASN1_STRING_set0(ASN1_STRING *str, void *data, int len) {
  OPENSSL_free(str->data);
  str->data = data;
  str->length = len;
}

ASN1_STRING *ASN1_STRING_new(void) {
  return (ASN1_STRING_type_new(V_ASN1_OCTET_STRING));
}

ASN1_STRING *ASN1_STRING_type_new(int type) {
  ASN1_STRING *ret;

  ret = (ASN1_STRING *)OPENSSL_zalloc(sizeof(ASN1_STRING));
  if (ret == NULL) {
    return NULL;
  }
  ret->type = type;
  return ret;
}

void ASN1_STRING_free(ASN1_STRING *str) {
  if (str == NULL) {
    return;
  }
  OPENSSL_free(str->data);
  OPENSSL_free(str);
}

void ASN1_STRING_clear_free(ASN1_STRING *str) {
  ASN1_STRING_free(str);
}

int ASN1_STRING_cmp(const ASN1_STRING *a, const ASN1_STRING *b) {
  // Capture padding bits and implicit truncation in BIT STRINGs.
  int a_length = a->length, b_length = b->length;
  uint8_t a_padding = 0, b_padding = 0;
  if (a->type == V_ASN1_BIT_STRING) {
    a_length = asn1_bit_string_length(a, &a_padding);
  }
  if (b->type == V_ASN1_BIT_STRING) {
    b_length = asn1_bit_string_length(b, &b_padding);
  }

  if (a_length < b_length) {
    return -1;
  }
  if (a_length > b_length) {
    return 1;
  }
  // In a BIT STRING, the number of bits is 8 * length - padding. Invert this
  // comparison so we compare by lengths.
  if (a_padding > b_padding) {
    return -1;
  }
  if (a_padding < b_padding) {
    return 1;
  }

  int ret = OPENSSL_memcmp(a->data, b->data, a_length);
  if (ret != 0) {
    return ret;
  }

  // Comparing the type first is more natural, but this matches OpenSSL.
  if (a->type < b->type) {
    return -1;
  }
  if (a->type > b->type) {
    return 1;
  }
  return 0;
}

int ASN1_STRING_length(const ASN1_STRING *str) { return str->length; }

int ASN1_STRING_type(const ASN1_STRING *str) { return str->type; }

unsigned char *ASN1_STRING_data(ASN1_STRING *str) { return str->data; }

const unsigned char *ASN1_STRING_get0_data(const ASN1_STRING *str) {
  return str->data;
}
