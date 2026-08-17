// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/x509.h>

#include <assert.h>
#include <ctype.h>
#include <limits.h>
#include <string.h>

#include <openssl/asn1.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/obj.h>

#include "../conf/internal.h"
#include "../internal.h"
#include "internal.h"


// Although this file is in crypto/x509 for layering purposes, it emits
// errors from the ASN.1 module for OpenSSL compatibility.

// ASN1_GEN_MAX_DEPTH is the maximum number of nested TLVs allowed.
#define ASN1_GEN_MAX_DEPTH 50

// ASN1_GEN_MAX_OUTPUT is the maximum output, in bytes, allowed. This limit is
// necessary because the SEQUENCE and SET section reference mechanism allows the
// output length to grow super-linearly with the input length.
#define ASN1_GEN_MAX_OUTPUT (64 * 1024)

// ASN1_GEN_FORMAT_* are the values for the format modifiers.
#define ASN1_GEN_FORMAT_ASCII 1
#define ASN1_GEN_FORMAT_UTF8 2
#define ASN1_GEN_FORMAT_HEX 3
#define ASN1_GEN_FORMAT_BITLIST 4

// generate_v3 converts |str| into an ASN.1 structure and writes the result to
// |cbb|. It returns one on success and zero on error. |depth| bounds recursion,
// and |format| specifies the current format modifier.
//
// If |tag| is non-zero, the structure is implicitly tagged with |tag|. |tag|
// must not have the constructed bit set.
static int generate_v3(CBB *cbb, const char *str, const X509V3_CTX *cnf,
                       CBS_ASN1_TAG tag, int format, int depth);

static int bitstr_cb(const char *elem, size_t len, void *bitstr);

ASN1_TYPE *ASN1_generate_v3(const char *str, const X509V3_CTX *cnf) {
  CBB cbb;
  if (!CBB_init(&cbb, 0) ||  //
      !generate_v3(&cbb, str, cnf, /*tag=*/0, ASN1_GEN_FORMAT_ASCII,
                   /*depth=*/0)) {
    CBB_cleanup(&cbb);
    return NULL;
  }

  // While not strictly necessary to avoid a DoS (we rely on any super-linear
  // checks being performed internally), cap the overall output to
  // |ASN1_GEN_MAX_OUTPUT| so the externally-visible behavior is consistent.
  if (CBB_len(&cbb) > ASN1_GEN_MAX_OUTPUT) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_TOO_LONG);
    CBB_cleanup(&cbb);
    return NULL;
  }

  const uint8_t *der = CBB_data(&cbb);
  ASN1_TYPE *ret = d2i_ASN1_TYPE(NULL, &der, CBB_len(&cbb));
  CBB_cleanup(&cbb);
  return ret;
}

static int cbs_str_equal(const CBS *cbs, const char *str) {
  return CBS_len(cbs) == strlen(str) &&
         OPENSSL_memcmp(CBS_data(cbs), str, strlen(str)) == 0;
}

// parse_tag decodes a tag specifier in |cbs|. It returns the tag on success or
// zero on error.
static CBS_ASN1_TAG parse_tag(const CBS *cbs) {
  CBS copy = *cbs;
  uint64_t num;
  if (!CBS_get_u64_decimal(&copy, &num) ||
      num > CBS_ASN1_TAG_NUMBER_MASK) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_INVALID_NUMBER);
    return 0;
  }

  CBS_ASN1_TAG tag_class = CBS_ASN1_CONTEXT_SPECIFIC;
  // The tag may be suffixed by a class.
  uint8_t c;
  if (CBS_get_u8(&copy, &c)) {
    switch (c) {
      case 'U':
        tag_class = CBS_ASN1_UNIVERSAL;
        break;
      case 'A':
        tag_class = CBS_ASN1_APPLICATION;
        break;
      case 'P':
        tag_class = CBS_ASN1_PRIVATE;
        break;
      case 'C':
        tag_class = CBS_ASN1_CONTEXT_SPECIFIC;
        break;
      default: {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_INVALID_MODIFIER);
        return 0;
      }
    }
    if (CBS_len(&copy) != 0) {
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_INVALID_MODIFIER);
      return 0;
    }
  }

  // Tag [UNIVERSAL 0] is reserved for indefinite-length end-of-contents. We
  // also use zero in this file to indicator no explicit tagging.
  if (tag_class == CBS_ASN1_UNIVERSAL && num == 0) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_INVALID_NUMBER);
    return 0;
  }

  return tag_class | (CBS_ASN1_TAG)num;
}

static int generate_wrapped(CBB *cbb, const char *str, const X509V3_CTX *cnf,
                            CBS_ASN1_TAG tag, int padding, int format,
                            int depth) {
  CBB child;
  return CBB_add_asn1(cbb, &child, tag) &&
         (!padding || CBB_add_u8(&child, 0)) &&
         generate_v3(&child, str, cnf, /*tag=*/0, format, depth + 1) &&
         CBB_flush(cbb);
}

static int generate_v3(CBB *cbb, const char *str, const X509V3_CTX *cnf,
                       CBS_ASN1_TAG tag, int format, int depth) {
  assert((tag & CBS_ASN1_CONSTRUCTED) == 0);
  if (depth > ASN1_GEN_MAX_DEPTH) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_NESTED_TAGGING);
    return 0;
  }

  // Process modifiers. This function uses a mix of NUL-terminated strings and
  // |CBS|. Several functions only work with NUL-terminated strings, so we need
  // to keep track of when a slice spans the whole buffer.
  for (;;) {
    // Skip whitespace.
    while (*str != '\0' && OPENSSL_isspace((unsigned char)*str)) {
      str++;
    }

    // Modifiers end at commas.
    const char *comma = strchr(str, ',');
    if (comma == NULL) {
      break;
    }

    // Remove trailing whitespace.
    CBS modifier;
    CBS_init(&modifier, (const uint8_t *)str, comma - str);
    for (;;) {
      uint8_t v;
      CBS copy = modifier;
      if (!CBS_get_last_u8(&copy, &v) || !OPENSSL_isspace(v)) {
        break;
      }
      modifier = copy;
    }

    // Advance the string past the modifier, but save the original value. We
    // will need to rewind if this is not a recognized modifier.
    const char *str_old = str;
    str = comma + 1;

    // Each modifier is either NAME:VALUE or NAME.
    CBS name;
    int has_value = CBS_get_until_first(&modifier, &name, ':');
    if (has_value) {
      CBS_skip(&modifier, 1);  // Skip the colon.
    } else {
      name = modifier;
      CBS_init(&modifier, NULL, 0);
    }

    if (cbs_str_equal(&name, "FORMAT") || cbs_str_equal(&name, "FORM")) {
      if (cbs_str_equal(&modifier, "ASCII")) {
        format = ASN1_GEN_FORMAT_ASCII;
      } else if (cbs_str_equal(&modifier, "UTF8")) {
        format = ASN1_GEN_FORMAT_UTF8;
      } else if (cbs_str_equal(&modifier, "HEX")) {
        format = ASN1_GEN_FORMAT_HEX;
      } else if (cbs_str_equal(&modifier, "BITLIST")) {
        format = ASN1_GEN_FORMAT_BITLIST;
      } else {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_UNKNOWN_FORMAT);
        return 0;
      }
    } else if (cbs_str_equal(&name, "IMP") ||
               cbs_str_equal(&name, "IMPLICIT")) {
      if (tag != 0) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_NESTED_TAGGING);
        return 0;
      }
      tag = parse_tag(&modifier);
      if (tag == 0) {
        return 0;
      }
    } else if (cbs_str_equal(&name, "EXP") ||
               cbs_str_equal(&name, "EXPLICIT")) {
      // It would actually be supportable, but OpenSSL does not allow wrapping
      // an explicit tag in an implicit tag.
      if (tag != 0) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_NESTED_TAGGING);
        return 0;
      }
      tag = parse_tag(&modifier);
      return tag != 0 &&
             generate_wrapped(cbb, str, cnf, tag | CBS_ASN1_CONSTRUCTED,
                              /*padding=*/0, format, depth);
    } else if (cbs_str_equal(&name, "OCTWRAP")) {
      tag = tag == 0 ? CBS_ASN1_OCTETSTRING : tag;
      return generate_wrapped(cbb, str, cnf, tag, /*padding=*/0, format, depth);
    } else if (cbs_str_equal(&name, "BITWRAP")) {
      tag = tag == 0 ? CBS_ASN1_BITSTRING : tag;
      return generate_wrapped(cbb, str, cnf, tag, /*padding=*/1, format, depth);
    } else if (cbs_str_equal(&name, "SEQWRAP")) {
      tag = tag == 0 ? CBS_ASN1_SEQUENCE : (tag | CBS_ASN1_CONSTRUCTED);
      tag |= CBS_ASN1_CONSTRUCTED;
      return generate_wrapped(cbb, str, cnf, tag, /*padding=*/0, format, depth);
    } else if (cbs_str_equal(&name, "SETWRAP")) {
      tag = tag == 0 ? CBS_ASN1_SET : (tag | CBS_ASN1_CONSTRUCTED);
      return generate_wrapped(cbb, str, cnf, tag, /*padding=*/0, format, depth);
    } else {
      // If this was not a recognized modifier, rewind |str| to before splitting
      // on the comma. The type itself consumes all remaining input.
      str = str_old;
      break;
    }
  }

  // The final element is, like modifiers, NAME:VALUE or NAME, but VALUE spans
  // the length of the string, including any commas.
  const char *colon = strchr(str, ':');
  CBS name;
  const char *value;
  int has_value = colon != NULL;
  if (has_value) {
    CBS_init(&name, (const uint8_t *)str, colon - str);
    value = colon + 1;
  } else {
    CBS_init(&name, (const uint8_t *)str, strlen(str));
    value = "";  // Most types treat missing and empty value equivalently.
  }

  static const struct {
    const char *name;
    CBS_ASN1_TAG type;
  } kTypes[] = {
      {"BOOL", CBS_ASN1_BOOLEAN},
      {"BOOLEAN", CBS_ASN1_BOOLEAN},
      {"NULL", CBS_ASN1_NULL},
      {"INT", CBS_ASN1_INTEGER},
      {"INTEGER", CBS_ASN1_INTEGER},
      {"ENUM", CBS_ASN1_ENUMERATED},
      {"ENUMERATED", CBS_ASN1_ENUMERATED},
      {"OID", CBS_ASN1_OBJECT},
      {"OBJECT", CBS_ASN1_OBJECT},
      {"UTCTIME", CBS_ASN1_UTCTIME},
      {"UTC", CBS_ASN1_UTCTIME},
      {"GENERALIZEDTIME", CBS_ASN1_GENERALIZEDTIME},
      {"GENTIME", CBS_ASN1_GENERALIZEDTIME},
      {"OCT", CBS_ASN1_OCTETSTRING},
      {"OCTETSTRING", CBS_ASN1_OCTETSTRING},
      {"BITSTR", CBS_ASN1_BITSTRING},
      {"BITSTRING", CBS_ASN1_BITSTRING},
      {"UNIVERSALSTRING", CBS_ASN1_UNIVERSALSTRING},
      {"UNIV", CBS_ASN1_UNIVERSALSTRING},
      {"IA5", CBS_ASN1_IA5STRING},
      {"IA5STRING", CBS_ASN1_IA5STRING},
      {"UTF8", CBS_ASN1_UTF8STRING},
      {"UTF8String", CBS_ASN1_UTF8STRING},
      {"BMP", CBS_ASN1_BMPSTRING},
      {"BMPSTRING", CBS_ASN1_BMPSTRING},
      {"PRINTABLESTRING", CBS_ASN1_PRINTABLESTRING},
      {"PRINTABLE", CBS_ASN1_PRINTABLESTRING},
      {"T61", CBS_ASN1_T61STRING},
      {"T61STRING", CBS_ASN1_T61STRING},
      {"TELETEXSTRING", CBS_ASN1_T61STRING},
      {"SEQUENCE", CBS_ASN1_SEQUENCE},
      {"SEQ", CBS_ASN1_SEQUENCE},
      {"SET", CBS_ASN1_SET},
  };
  CBS_ASN1_TAG type = 0;
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kTypes); i++) {
    if (cbs_str_equal(&name, kTypes[i].name)) {
      type = kTypes[i].type;
      break;
    }
  }
  if (type == 0) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_UNKNOWN_TAG);
    return 0;
  }

  // If there is an implicit tag, use the constructed bit from the base type.
  tag = tag == 0 ? type : (tag | (type & CBS_ASN1_CONSTRUCTED));
  CBB child;
  if (!CBB_add_asn1(cbb, &child, tag)) {
    return 0;
  }

  switch (type) {
    case CBS_ASN1_NULL:
      if (*value != '\0') {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_NULL_VALUE);
        return 0;
      }
      return CBB_flush(cbb);

    case CBS_ASN1_BOOLEAN: {
      if (format != ASN1_GEN_FORMAT_ASCII) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_NOT_ASCII_FORMAT);
        return 0;
      }
      ASN1_BOOLEAN boolean;
      if (!X509V3_bool_from_string(value, &boolean)) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_BOOLEAN);
        return 0;
      }
      return CBB_add_u8(&child, boolean ? 0xff : 0x00) && CBB_flush(cbb);
    }

    case CBS_ASN1_INTEGER:
    case CBS_ASN1_ENUMERATED: {
      if (format != ASN1_GEN_FORMAT_ASCII) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_INTEGER_NOT_ASCII_FORMAT);
        return 0;
      }
      ASN1_INTEGER *obj = s2i_ASN1_INTEGER(NULL, value);
      if (obj == NULL) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_INTEGER);
        return 0;
      }
      int len = i2c_ASN1_INTEGER(obj, NULL);
      uint8_t *out;
      int ok = len > 0 &&  //
               CBB_add_space(&child, &out, len) &&
               i2c_ASN1_INTEGER(obj, &out) == len &&
               CBB_flush(cbb);
      ASN1_INTEGER_free(obj);
      return ok;
    }

    case CBS_ASN1_OBJECT: {
      if (format != ASN1_GEN_FORMAT_ASCII) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_OBJECT_NOT_ASCII_FORMAT);
        return 0;
      }
      ASN1_OBJECT *obj = OBJ_txt2obj(value, /*dont_search_names=*/0);
      if (obj == NULL || obj->length == 0) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_OBJECT);
        return 0;
      }
      int ok = CBB_add_bytes(&child, obj->data, obj->length) && CBB_flush(cbb);
      ASN1_OBJECT_free(obj);
      return ok;
    }

    case CBS_ASN1_UTCTIME:
    case CBS_ASN1_GENERALIZEDTIME: {
      if (format != ASN1_GEN_FORMAT_ASCII) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_TIME_NOT_ASCII_FORMAT);
        return 0;
      }
      CBS value_cbs;
      CBS_init(&value_cbs, (const uint8_t*)value, strlen(value));
      int ok = type == CBS_ASN1_UTCTIME
                   ? CBS_parse_utc_time(&value_cbs, NULL,
                                        /*allow_timezone_offset=*/0)
                   : CBS_parse_generalized_time(&value_cbs, NULL,
                                                /*allow_timezone_offset=*/0);
      if (!ok) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_TIME_VALUE);
        return 0;
      }
      return CBB_add_bytes(&child, (const uint8_t *)value, strlen(value)) &&
             CBB_flush(cbb);
    }

    case CBS_ASN1_UNIVERSALSTRING:
    case CBS_ASN1_IA5STRING:
    case CBS_ASN1_UTF8STRING:
    case CBS_ASN1_BMPSTRING:
    case CBS_ASN1_PRINTABLESTRING:
    case CBS_ASN1_T61STRING: {
      int encoding;
      if (format == ASN1_GEN_FORMAT_ASCII) {
        encoding = MBSTRING_ASC;
      } else if (format == ASN1_GEN_FORMAT_UTF8) {
        encoding = MBSTRING_UTF8;
      } else {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_FORMAT);
        return 0;
      }

      // |maxsize| is measured in code points, rather than bytes, but pass it in
      // as a loose cap so fuzzers can exit from excessively long inputs
      // earlier. This limit is not load-bearing because |ASN1_mbstring_ncopy|'s
      // output is already linear in the input.
      ASN1_STRING *obj = NULL;
      if (ASN1_mbstring_ncopy(&obj, (const uint8_t *)value, -1, encoding,
                              ASN1_tag2bit(type), /*minsize=*/0,
                              /*maxsize=*/ASN1_GEN_MAX_OUTPUT) <= 0) {
        return 0;
      }
      int ok = CBB_add_bytes(&child, obj->data, obj->length) && CBB_flush(cbb);
      ASN1_STRING_free(obj);
      return ok;
    }

    case CBS_ASN1_BITSTRING:
      if (format == ASN1_GEN_FORMAT_BITLIST) {
        ASN1_BIT_STRING *obj = ASN1_BIT_STRING_new();
        if (obj == NULL) {
          return 0;
        }
        if (!CONF_parse_list(value, ',', 1, bitstr_cb, obj)) {
          OPENSSL_PUT_ERROR(ASN1, ASN1_R_LIST_ERROR);
          ASN1_BIT_STRING_free(obj);
          return 0;
        }
        int len = i2c_ASN1_BIT_STRING(obj, NULL);
        uint8_t *out;
        int ok = len > 0 &&  //
                 CBB_add_space(&child, &out, len) &&
                 i2c_ASN1_BIT_STRING(obj, &out) == len &&  //
                 CBB_flush(cbb);
        ASN1_BIT_STRING_free(obj);
        return ok;
      }

      // The other formats are the same as OCTET STRING, but with the leading
      // zero bytes.
      if (!CBB_add_u8(&child, 0)) {
        return 0;
      }
      OPENSSL_FALLTHROUGH;

    case CBS_ASN1_OCTETSTRING:
      if (format == ASN1_GEN_FORMAT_ASCII) {
        return CBB_add_bytes(&child, (const uint8_t *)value, strlen(value)) &&
               CBB_flush(cbb);
      }
      if (format == ASN1_GEN_FORMAT_HEX) {
        size_t len;
        uint8_t *data = x509v3_hex_to_bytes(value, &len);
        if (data == NULL) {
          OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_HEX);
          return 0;
        }
        int ok = CBB_add_bytes(&child, data, len) && CBB_flush(cbb);
        OPENSSL_free(data);
        return ok;
      }

      OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_BITSTRING_FORMAT);
      return 0;

    case CBS_ASN1_SEQUENCE:
    case CBS_ASN1_SET:
      if (has_value) {
        if (cnf == NULL) {
          OPENSSL_PUT_ERROR(ASN1, ASN1_R_SEQUENCE_OR_SET_NEEDS_CONFIG);
          return 0;
        }
        const STACK_OF(CONF_VALUE) *section = X509V3_get_section(cnf, value);
        if (section == NULL) {
          OPENSSL_PUT_ERROR(ASN1, ASN1_R_SEQUENCE_OR_SET_NEEDS_CONFIG);
          return 0;
        }
        for (size_t i = 0; i < sk_CONF_VALUE_num(section); i++) {
          const CONF_VALUE *conf = sk_CONF_VALUE_value(section, i);
          if (!generate_v3(&child, conf->value, cnf, /*tag=*/0,
                           ASN1_GEN_FORMAT_ASCII, depth + 1)) {
            return 0;
          }
          // This recursive call, by referencing |section|, is the one place
          // where |generate_v3|'s output can be super-linear in the input.
          // Check bounds here.
          if (CBB_len(&child) > ASN1_GEN_MAX_OUTPUT) {
            OPENSSL_PUT_ERROR(ASN1, ASN1_R_TOO_LONG);
            return 0;
          }
        }
      }
      if (type == CBS_ASN1_SET) {
        // The SET type here is a SET OF and must be sorted.
        return CBB_flush_asn1_set_of(&child) && CBB_flush(cbb);
      }
      return CBB_flush(cbb);

    default:
      OPENSSL_PUT_ERROR(ASN1, ERR_R_INTERNAL_ERROR);
      return 0;
  }
}

static int bitstr_cb(const char *elem, size_t len, void *bitstr) {
  CBS cbs;
  CBS_init(&cbs, (const uint8_t *)elem, len);
  uint64_t bitnum;
  if (!CBS_get_u64_decimal(&cbs, &bitnum) || CBS_len(&cbs) != 0 ||
      // Cap the highest allowed bit so this mechanism cannot be used to create
      // extremely large allocations with short inputs. The highest named bit in
      // RFC 5280 is 8, so 256 should give comfortable margin but still only
      // allow a 32-byte allocation.
      //
      // We do not consider this function to be safe with untrusted inputs (even
      // without bugs, it is prone to string injection vulnerabilities), so DoS
      // is not truly a concern, but the limit is necessary to keep fuzzing
      // effective.
      bitnum > 256) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_INVALID_NUMBER);
    return 0;
  }
  if (!ASN1_BIT_STRING_set_bit(bitstr, (int)bitnum, 1)) {
    return 0;
  }
  return 1;
}
