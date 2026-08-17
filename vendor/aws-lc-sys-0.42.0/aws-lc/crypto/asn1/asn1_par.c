// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>
#include <openssl/bio.h>
#include "../internal.h"

// Forward declarations
static int asn1_parse2(BIO *bp, const uint8_t **pp, long len, long offset,
                       int depth, int indent, int dump);
static int asn1_print_info(BIO *bp, int tag, int xclass, int constructed,
                           int indent);
static int asn1_parse_constructed_type(BIO *bp, const uint8_t **current_pos,
                                       const uint8_t *total_end,
                                       const uint8_t *original_start,
                                       long *object_len, int parse_flags,
                                       long offset, int depth, int indent,
                                       int dump);
static int asn1_parse_primitive_type(BIO *bp, const uint8_t *object_start,
                                     const uint8_t *current_pos,
                                     long object_len, long header_len, int tag,
                                     int dump);

const char *ASN1_tag2str(int tag) {
  static const char *const tag2str[] = {
      "EOC",
      "BOOLEAN",
      "INTEGER",
      "BIT STRING",
      "OCTET STRING",
      "NULL",
      "OBJECT",
      "OBJECT DESCRIPTOR",
      "EXTERNAL",
      "REAL",
      "ENUMERATED",
      "<ASN1 11>",
      "UTF8STRING",
      "<ASN1 13>",
      "<ASN1 14>",
      "<ASN1 15>",
      "SEQUENCE",
      "SET",
      "NUMERICSTRING",
      "PRINTABLESTRING",
      "T61STRING",
      "VIDEOTEXSTRING",
      "IA5STRING",
      "UTCTIME",
      "GENERALIZEDTIME",
      "GRAPHICSTRING",
      "VISIBLESTRING",
      "GENERALSTRING",
      "UNIVERSALSTRING",
      "<ASN1 29>",
      "BMPSTRING",
  };

  if ((tag == V_ASN1_NEG_INTEGER) || (tag == V_ASN1_NEG_ENUMERATED)) {
    tag &= ~V_ASN1_NEG;
  }

  if (tag < 0 || tag > 30) {
    return "(unknown)";
  }
  return tag2str[tag];
}

int ASN1_parse(BIO *bp, const unsigned char *pp, long len, int indent) {
  GUARD_PTR(bp);
  GUARD_PTR(pp);
  return asn1_parse2(bp, &pp, len, 0, 0, indent, 0);
}

// Constants
#define ASN1_PARSE_MAXDEPTH 128
#define ASN1_DUMP_INDENT 6  // Because we know BIO_dump_indent()

// Copy of helper functions from original (these remain unchanged)
static int asn1_print_info(BIO *bp, int tag, int xclass, int constructed,
                           int indent) {
  static const char fmt[] = "%-18s";
  char str[128];
  const char *p;

  if (constructed & V_ASN1_CONSTRUCTED) {
    p = "cons: ";
  } else {
    p = "prim: ";
  }
  if (BIO_write(bp, p, 6) < 6) {
    goto err;
  }
  BIO_indent(bp, indent, 128);

  p = str;
  if ((xclass & V_ASN1_PRIVATE) == V_ASN1_PRIVATE) {
    BIO_snprintf(str, sizeof(str), "priv [ %d ] ", tag);
  } else if ((xclass & V_ASN1_CONTEXT_SPECIFIC) == V_ASN1_CONTEXT_SPECIFIC) {
    BIO_snprintf(str, sizeof(str), "cont [ %d ]", tag);
  } else if ((xclass & V_ASN1_APPLICATION) == V_ASN1_APPLICATION) {
    BIO_snprintf(str, sizeof(str), "appl [ %d ]", tag);
  } else if (tag > 30) {
    BIO_snprintf(str, sizeof(str), "<ASN1 %d>", tag);
  } else {
    p = ASN1_tag2str(tag);
  }

  if (BIO_printf(bp, fmt, p) <= 0) {
    goto err;
  }
  return 1;
err:
  return 0;
}

static int BIO_dump_indent(BIO *bp, const char *s, int len, int indent) {
  // Use BIO_hexdump as a replacement for BIO_dump_indent
  return BIO_hexdump(bp, (const uint8_t *)s, len, indent);
}

// Helper function to parse constructed ASN.1 types (SEQUENCE, SET, etc.)
static int asn1_parse_constructed_type(BIO *bp, const uint8_t **current_pos,
                                       const uint8_t *total_end,
                                       const uint8_t *original_start,
                                       long *object_len, int parse_flags,
                                       long offset, int depth, int indent,
                                       int dump) {
  GUARD_PTR(bp);
  GUARD_PTR(current_pos);
  GUARD_PTR(total_end);
  GUARD_PTR(original_start);
  GUARD_PTR(object_len);

  if (offset < 0 || depth < 0 || indent < 0) {
    return 0;
  }

  const uint8_t *start_pos = *current_pos;

  if (BIO_write(bp, "\n", 1) <= 0) {
    return 0;
  }

  if ((parse_flags == (V_ASN1_CONSTRUCTED | 1)) && (*object_len == 0)) {
    // Indefinite length constructed object
    for (;;) {
      const int parse_result = asn1_parse2(
          bp, current_pos, (long)(total_end - *current_pos),
          offset + (*current_pos - original_start), depth + 1, indent, dump);
      if (parse_result == 0) {
        return 0;
      }
      if ((parse_result == 2) || (*current_pos >= total_end)) {
        *object_len = *current_pos - start_pos;
        break;
      }
    }
  } else {
    // Definite length constructed object
    const uint8_t *constructed_end = *current_pos + *object_len;
    long remaining_length = *object_len;

    if (constructed_end > total_end) {
      return 0;
    }

    while (*current_pos < constructed_end) {
      start_pos = *current_pos;
      const int parse_result = asn1_parse2(
          bp, current_pos, remaining_length,
          offset + (*current_pos - original_start), depth + 1, indent, dump);
      if (parse_result == 0) {
        return 0;
      }
      remaining_length -= *current_pos - start_pos;
    }
  }
  return 1;
}

static int asn1_parse_string_type(BIO *bp, const uint8_t *data, long len) {
  if (BIO_write(bp, ":", 1) <= 0) {
    return 0;
  }
  if (len > INT_MAX) {
    return 0;
  }
  if ((len > 0) && BIO_write(bp, (const char *)data, (int)len) != (int)len) {
    return 0;
  }
  return 1;
}

static int asn1_parse_object_type(BIO *bp, const uint8_t *data, long len,
                                  int *dump_as_hex) {
  const uint8_t *parse_pos = data;
  ASN1_OBJECT *asn1_object = NULL;
  int return_code = 0;

  GUARD_PTR(bp);
  GUARD_PTR(data);

  if (d2i_ASN1_OBJECT(&asn1_object, &parse_pos, len) != NULL) {
    if (BIO_write(bp, ":", 1) <= 0) {
      goto end;
    }
    i2a_ASN1_OBJECT(bp, asn1_object);
    return_code = 1;
  } else {
    if (BIO_puts(bp, ":BAD OBJECT") <= 0) {
      goto end;
    }
    *dump_as_hex = 1;
    return_code = 1;
  }

end:
  ASN1_OBJECT_free(asn1_object);
  return return_code;
}

static int asn1_parse_boolean_type(BIO *bp, const uint8_t *data, long len,
                                   int *dump_as_hex) {
  GUARD_PTR(bp);
  GUARD_PTR(data);
  GUARD_PTR(dump_as_hex);

  if (len < 0) {
    return 0;
  }

  if (len != 1) {
    if (BIO_puts(bp, ":BAD BOOLEAN") <= 0) {
      return 0;
    }
    *dump_as_hex = 1;
  }
  if (len > 0) {
    if (BIO_printf(bp, ":%u", data[0]) <= 0) {
      return 0;
    }
  }
  return 1;
}

static int asn1_parse_octet_string_type(BIO *bp, const uint8_t *data, long len,
                                        int dump, int *newline_printed) {
  GUARD_PTR(bp);
  GUARD_PTR(data);
  GUARD_PTR(newline_printed);

  if (len < 0) {
    return 0;
  }

  const unsigned char *parse_pos = data;
  int return_code = 0;
  int dump_indent = 6;

  ASN1_OCTET_STRING *octet_string =
      d2i_ASN1_OCTET_STRING(NULL, &parse_pos, len);
  if (octet_string != NULL && octet_string->length > 0) {
    int printable = 1;
    parse_pos = octet_string->data;

    // Test whether the octet string is printable (i.e. ASCII)
    for (int i = 0; i < octet_string->length; i++) {
      if (((parse_pos[i] < ' ') && (parse_pos[i] != '\n') &&
           (parse_pos[i] != '\r') && (parse_pos[i] != '\t')) ||
          (parse_pos[i] > '~')) {
        printable = 0;
        break;
      }
    }

    if (printable) {
      // Printable string
      if (BIO_write(bp, ":", 1) <= 0) {
        goto end;
      }
      if (BIO_write(bp, (const char *)parse_pos, octet_string->length) <= 0) {
        goto end;
      }
    } else if (!dump) {
      // Not printable => print octet string as hex dump
      if (BIO_write(bp, "[HEX DUMP]:", 11) <= 0) {
        goto end;
      }
      for (int i = 0; i < octet_string->length; i++) {
        if (BIO_printf(bp, "%02X", parse_pos[i]) <= 0) {
          goto end;
        }
      }
    } else {
      // Print the normal dump
      if (!*newline_printed) {
        if (BIO_write(bp, "\n", 1) <= 0) {
          goto end;
        }
      }
      if (BIO_dump_indent(bp, (const char *)parse_pos,
                          ((dump == -1 || dump > octet_string->length)
                               ? octet_string->length
                               : dump),
                          dump_indent) <= 0) {
        goto end;
      }
      *newline_printed = 1;
    }
  }
  return_code = 1;

end:
  ASN1_OCTET_STRING_free(octet_string);
  return return_code;
}

static int asn1_parse_integer_type(BIO *bp, const uint8_t *object_start,
                                   long object_len, int header_len,
                                   int *dump_as_hex) {
  GUARD_PTR(bp);
  GUARD_PTR(object_start);
  GUARD_PTR(dump_as_hex);

  if (object_len < 0 || header_len < 0) {
    return 0;
  }

  const uint8_t *parse_pos = object_start;
  int return_code = 0;

  ASN1_INTEGER *asn1_integer =
      d2i_ASN1_INTEGER(NULL, &parse_pos, object_len + header_len);
  if (asn1_integer != NULL) {
    if (BIO_write(bp, ":", 1) <= 0) {
      goto end;
    }
    if (asn1_integer->type == V_ASN1_NEG_INTEGER) {
      if (BIO_write(bp, "-", 1) <= 0) {
        goto end;
      }
    }
    for (int i = 0; i < asn1_integer->length; i++) {
      if (BIO_printf(bp, "%02X", asn1_integer->data[i]) <= 0) {
        goto end;
      }
    }
    if (asn1_integer->length == 0) {
      if (BIO_write(bp, "00", 2) <= 0) {
        goto end;
      }
    }
    return_code = 1;
  } else {
    if (BIO_puts(bp, ":BAD INTEGER") <= 0) {
      goto end;
    }
    *dump_as_hex = 1;
    return_code = 1;
  }

end:
  ASN1_INTEGER_free(asn1_integer);
  return return_code;
}

static int asn1_parse_enumerated_type(BIO *bp, const uint8_t *object_start,
                                      long object_len, int header_len,
                                      int *dump_as_hex) {
  GUARD_PTR(bp);
  GUARD_PTR(object_start);
  GUARD_PTR(dump_as_hex);

  if (object_len < 0 || header_len < 0) {
    return 0;
  }

  const uint8_t *parse_pos = object_start;
  int return_code = 0;

  ASN1_ENUMERATED *asn1_enumerated =
      d2i_ASN1_ENUMERATED(NULL, &parse_pos, object_len + header_len);
  if (asn1_enumerated != NULL) {
    if (BIO_write(bp, ":", 1) <= 0) {
      goto end;
    }
    if (asn1_enumerated->type == V_ASN1_NEG_ENUMERATED) {
      if (BIO_write(bp, "-", 1) <= 0) {
        goto end;
      }
    }
    for (int i = 0; i < asn1_enumerated->length; i++) {
      if (BIO_printf(bp, "%02X", asn1_enumerated->data[i]) <= 0) {
        goto end;
      }
    }
    if (asn1_enumerated->length == 0) {
      if (BIO_write(bp, "00", 2) <= 0) {
        goto end;
      }
    }
    return_code = 1;
  } else {
    if (BIO_puts(bp, ":BAD ENUMERATED") <= 0) {
      goto end;
    }
    *dump_as_hex = 1;
    return_code = 1;
  }

end:
  ASN1_ENUMERATED_free(asn1_enumerated);
  return return_code;
}

// Helper function to perform hex dump for generic types
static int asn1_parse_hex_dump(BIO *bp, const uint8_t *object_start,
                               const uint8_t *current_pos, long object_len,
                               int header_len, int dump, int *newline_printed) {
  GUARD_PTR(bp);
  GUARD_PTR(object_start);
  GUARD_PTR(current_pos);
  GUARD_PTR(newline_printed);

  if (object_len < 0 || header_len < 0) {
    return 0;
  }

  const int dump_indent = 6;

  if (object_len > 0 && dump) {
    if (!*newline_printed) {
      if (BIO_write(bp, "\n", 1) <= 0) {
        return 0;
      }
    }
    if (BIO_dump_indent(bp, (const char *)current_pos,
                        ((dump == -1 || dump > object_len) ? object_len : dump),
                        dump_indent) <= 0) {
      return 0;
    }
    *newline_printed = 1;
  }
  return 1;
}

// Helper function to output hex data when dump_as_hex flag is set
static int asn1_output_hex_data(BIO *bp, const uint8_t *object_start,
                                long object_len, int header_len) {
  if (object_len < 0 || header_len < 0) {
    return 0;
  }

  const uint8_t *hex_data = object_start + header_len;

  if (BIO_puts(bp, ":[") <= 0) {
    return 0;
  }
  for (int i = 0; i < object_len; i++) {
    if (BIO_printf(bp, "%02X", hex_data[i]) <= 0) {
      return 0;
    }
  }
  if (BIO_puts(bp, "]") <= 0) {
    return 0;
  }
  return 1;
}

// Refactored main function to parse primitive ASN.1 types
static int asn1_parse_primitive_type(BIO *bp, const uint8_t *object_start,
                                     const uint8_t *current_pos,
                                     long object_len, long header_len, int tag,
                                     int dump) {
  GUARD_PTR(bp);
  GUARD_PTR(object_start);
  GUARD_PTR(current_pos);

  if (object_len < 0 || header_len < 0) {
    return 0;
  }

  int newline_printed = 0;
  int dump_as_hex = 0;
  int result = 0;

  // Handle different primitive types
  if ((tag == V_ASN1_PRINTABLESTRING) || (tag == V_ASN1_T61STRING) ||
      (tag == V_ASN1_IA5STRING) || (tag == V_ASN1_VISIBLESTRING) ||
      (tag == V_ASN1_NUMERICSTRING) || (tag == V_ASN1_UTF8STRING) ||
      (tag == V_ASN1_UTCTIME) || (tag == V_ASN1_GENERALIZEDTIME)) {
    result = asn1_parse_string_type(bp, current_pos, object_len);
  } else if (tag == V_ASN1_OBJECT) {
    result = asn1_parse_object_type(bp, object_start, object_len + header_len,
                                    &dump_as_hex);
  } else if (tag == V_ASN1_BOOLEAN) {
    result = asn1_parse_boolean_type(bp, current_pos, object_len, &dump_as_hex);
  } else if (tag == V_ASN1_BMPSTRING) {
    // Currently this is just a placeholder as in the original code
  } else if (tag == V_ASN1_OCTET_STRING) {
    result = asn1_parse_octet_string_type(
        bp, object_start, object_len + header_len, dump, &newline_printed);
  } else if (tag == V_ASN1_INTEGER) {
    result = asn1_parse_integer_type(bp, object_start, object_len, header_len,
                                     &dump_as_hex);
  } else if (tag == V_ASN1_ENUMERATED) {
    result = asn1_parse_enumerated_type(bp, object_start, object_len,
                                        header_len, &dump_as_hex);
  } else {
    result = asn1_parse_hex_dump(bp, object_start, current_pos, object_len,
                                 header_len, dump, &newline_printed);
  }

  if (!result) {
    return 0;
  }

  // Handle hex dump output if needed
  if (dump_as_hex) {
    if (!asn1_output_hex_data(bp, object_start, object_len, header_len)) {
      return 0;
    }
  }

  // Ensure we end with a newline if one wasn't already printed
  if (!newline_printed) {
    if (BIO_write(bp, "\n", 1) <= 0) {
      return 0;
    }
  }

  return 1;
}

static int asn1_parse2(BIO *bp, const uint8_t **pp, long length, long offset,
                       int depth, int indent, int dump) {
  GUARD_PTR(bp);
  GUARD_PTR(pp);

  if (length < 0 || offset < 0 || depth < 0 || indent < 0) {
    return 0;
  }

  const uint8_t *current_pos, *total_end, *object_start;
  long content_length = 0;
  int tag, xclass, return_value = 0;
  int header_length = 0, parse_flags = 0;

  if (depth > ASN1_PARSE_MAXDEPTH) {
    BIO_puts(bp, "BAD RECURSION DEPTH\n");
    return 0;
  }

  current_pos = *pp;
  total_end = current_pos + length;
  while (length > 0) {
    object_start = current_pos;
    parse_flags =
        ASN1_get_object(&current_pos, &content_length, &tag, &xclass, length);
    if (parse_flags & 0x80) {
      if (BIO_write(bp, "Error in encoding\n", 18) <= 0) {
        goto end;
      }
      return_value = 0;
      goto end;
    }
    header_length = (current_pos - object_start);
    length -= header_length;
    /*
     * if parse_flags == (CBS_ASN1_CONSTRUCTED | 1) it is a constructed
     * indefinite length object
     */
    if (BIO_printf(bp, "%5ld:", (long)offset + (long)(object_start - *pp)) <=
        0) {
      goto end;
    }

    if (parse_flags != (V_ASN1_CONSTRUCTED | 1)) {
      if (BIO_printf(bp, "d=%-2d hl=%ld l=%4ld ", depth, (long)header_length,
                     content_length) <= 0) {
        goto end;
      }
    } else {
      if (BIO_printf(bp, "d=%-2d hl=%ld l=inf  ", depth, (long)header_length) <=
          0) {
        goto end;
      }
    }
    if (!asn1_print_info(bp, tag, xclass, parse_flags, (indent) ? depth : 0)) {
      goto end;
    }
    if (parse_flags & V_ASN1_CONSTRUCTED) {
      if (content_length > length) {
        BIO_printf(bp, "length is greater than %ld\n", length);
        return_value = 0;
        goto end;
      }
      if (!asn1_parse_constructed_type(bp, &current_pos, total_end, *pp,
                                       &content_length, parse_flags, offset,
                                       depth, indent, dump)) {
        return_value = 0;
        goto end;
      }
    } else if (xclass != 0) {
      current_pos += content_length;
      if (BIO_write(bp, "\n", 1) <= 0) {
        goto end;
      }
    } else {
      if (!asn1_parse_primitive_type(bp, object_start, current_pos,
                                     content_length, header_length, tag, dump)) {
        goto end;
      }
      current_pos += content_length;
      if ((tag == V_ASN1_EOC) && (xclass == 0)) {
        return_value = 2; /* End of sequence */
        goto end;
      }
    }
    length -= content_length;
  }
  return_value = 1;
end:
  *pp = current_pos;
  return return_value;
}
