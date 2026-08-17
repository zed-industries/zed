// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>

#include <assert.h>
#include <ctype.h>
#include <inttypes.h>
#include <limits.h>
#include <string.h>
#include <time.h>

#include <openssl/bio.h>
#include <openssl/bytestring.h>
#include <openssl/mem.h>

#include "../bytestring/internal.h"
#include "../internal.h"
#include "internal.h"


#define ESC_FLAGS                                                           \
  (ASN1_STRFLGS_ESC_2253 | ASN1_STRFLGS_ESC_QUOTE | ASN1_STRFLGS_ESC_CTRL | \
   ASN1_STRFLGS_ESC_MSB)

static int maybe_write(BIO *out, const void *buf, int len) {
  // If |out| is NULL, ignore the output but report the length.
  return out == NULL || BIO_write(out, buf, len) == len;
}

static int is_control_character(unsigned char c) { return c < 32 || c == 127; }

static int do_esc_char(uint32_t c, unsigned long flags, char *do_quotes,
                       BIO *out, int is_first, int is_last) {
  // |c| is a |uint32_t| because, depending on |ASN1_STRFLGS_UTF8_CONVERT|,
  // we may be escaping bytes or Unicode codepoints.
  char buf[16];  // Large enough for "\\W01234567".
  unsigned char u8 = (unsigned char)c;
  if (c > 0xffff) {
    snprintf(buf, sizeof(buf), "\\W%08" PRIX32, c);
  } else if (c > 0xff) {
    snprintf(buf, sizeof(buf), "\\U%04" PRIX32, c);
  } else if ((flags & ASN1_STRFLGS_ESC_MSB) && c > 0x7f) {
    snprintf(buf, sizeof(buf), "\\%02X", c);
  } else if ((flags & ASN1_STRFLGS_ESC_CTRL) && is_control_character(c)) {
    snprintf(buf, sizeof(buf), "\\%02X", c);
  } else if (flags & ASN1_STRFLGS_ESC_2253) {
    // See RFC 2253, sections 2.4 and 4.
    if (c == '\\' || c == '"') {
      // Quotes and backslashes are always escaped, quoted or not.
      snprintf(buf, sizeof(buf), "\\%c", (int)c);
    } else if (c == ',' || c == '+' || c == '<' || c == '>' || c == ';' ||
               (is_first && (c == ' ' || c == '#')) ||
               (is_last && (c == ' '))) {
      if (flags & ASN1_STRFLGS_ESC_QUOTE) {
        // No need to escape, just tell the caller to quote.
        if (do_quotes != NULL) {
          *do_quotes = 1;
        }
        return maybe_write(out, &u8, 1) ? 1 : -1;
      }
      snprintf(buf, sizeof(buf), "\\%c", (int)c);
    } else {
      return maybe_write(out, &u8, 1) ? 1 : -1;
    }
  } else if ((flags & ESC_FLAGS) && c == '\\') {
    // If any escape flags are set, also escape backslashes.
    snprintf(buf, sizeof(buf), "\\%c", (int)c);
  } else {
    return maybe_write(out, &u8, 1) ? 1 : -1;
  }

  OPENSSL_STATIC_ASSERT(sizeof(buf) < INT_MAX, len_may_not_fit_in_int);
  int len = (int)strlen(buf);
  return maybe_write(out, buf, len) ? len : -1;
}

// This function sends each character in a buffer to do_esc_char(). It
// interprets the content formats and converts to or from UTF8 as
// appropriate.

static int do_buf(const unsigned char *buf, int buflen, int encoding,
                  int utf8_convert, unsigned long flags, char *quotes,
                  BIO *out) {
  // Reject invalid UCS-4 and UCS-2 lengths without parsing.
  switch (encoding) {
    case MBSTRING_UNIV:
      if (buflen & 3) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_INVALID_UNIVERSALSTRING);
        return -1;
      }
      break;
    case MBSTRING_BMP:
      if (buflen & 1) {
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_INVALID_BMPSTRING);
        return -1;
      }
      break;
  }

  const unsigned char *p = buf;
  const unsigned char *q = buf + buflen;
  int outlen = 0;
  while (p != q) {
    const int is_first = p == buf;
    // TODO(davidben): Replace this with |cbs_get_ucs2_be|, etc., to check
    // for invalid codepoints. Before doing that, enforce it in the parser,
    // https://crbug.com/boringssl/427, so these error cases are not
    // reachable from parsed objects.
    uint32_t c = 0;
    switch (encoding) {
      case MBSTRING_UNIV:
        c = ((uint32_t)*p++) << 24;
        c |= ((uint32_t)*p++) << 16;
        c |= ((uint32_t)*p++) << 8;
        c |= *p++;
        break;

      case MBSTRING_BMP:
        c = ((uint32_t)*p++) << 8;
        c |= *p++;
        break;

      case MBSTRING_ASC:
        c = *p++;
        break;

      case MBSTRING_UTF8: {
        int consumed = UTF8_getc(p, buflen, &c);
        if (consumed < 0) {
          return -1;  // Invalid UTF8String
        }
        buflen -= consumed;
        p += consumed;
        break;
      }

      default:
        assert(0);
        return -1;
    }
    const int is_last = p == q;
    if (utf8_convert) {
      unsigned char utfbuf[6];
      int utflen;
      utflen = UTF8_putc(utfbuf, sizeof utfbuf, c);
      for (int i = 0; i < utflen; i++) {
        int len = do_esc_char(utfbuf[i], flags, quotes, out, is_first && i == 0,
                              is_last && i == utflen - 1);
        if (len < 0) {
          return -1;
        }

        // Ensure addition doesn't overflow and corrupt the signed output length
        if (len > INT_MAX - outlen) {
          OPENSSL_PUT_ERROR(ASN1, ERR_R_OVERFLOW);
          return -1;
        }
        outlen += len;
      }
    } else {
      int len = do_esc_char(c, flags, quotes, out, is_first, is_last);
      if (len < 0) {
        return -1;
      }
      if (len > INT_MAX - outlen) {
        OPENSSL_PUT_ERROR(ASN1, ERR_R_OVERFLOW);
        return -1;
      }
      outlen += len;
    }
  }
  return outlen;
}

// This function hex dumps a buffer of characters

static int do_hex_dump(BIO *out, unsigned char *buf, int buflen) {
  static const char hexdig[] = "0123456789ABCDEF";
  unsigned char *p, *q;
  char hextmp[2];
  if (out) {
    p = buf;
    q = buf + buflen;
    while (p != q) {
      hextmp[0] = hexdig[*p >> 4];
      hextmp[1] = hexdig[*p & 0xf];
      if (!maybe_write(out, hextmp, 2)) {
        return -1;
      }
      p++;
    }
  }
  return buflen << 1;
}

// "dump" a string. This is done when the type is unknown, or the flags
// request it. We can either dump the content octets or the entire DER
// encoding. This uses the RFC 2253 #01234 format.

static int do_dump(unsigned long flags, BIO *out, const ASN1_STRING *str) {
  if (!maybe_write(out, "#", 1)) {
    return -1;
  }

  // If we don't dump DER encoding just dump content octets
  if (!(flags & ASN1_STRFLGS_DUMP_DER)) {
    int outlen = do_hex_dump(out, str->data, str->length);
    if (outlen < 0) {
      return -1;
    }
    return outlen + 1;
  }

  // Placing the ASN1_STRING in a temporary ASN1_TYPE allows the DER encoding
  // to readily obtained.
  ASN1_TYPE t;
  OPENSSL_memset(&t, 0, sizeof(ASN1_TYPE));
  asn1_type_set0_string(&t, (ASN1_STRING *)str);
  unsigned char *der_buf = NULL;
  int der_len = i2d_ASN1_TYPE(&t, &der_buf);
  if (der_len < 0) {
    return -1;
  }
  int outlen = do_hex_dump(out, der_buf, der_len);
  OPENSSL_free(der_buf);
  if (outlen < 0) {
    return -1;
  }
  return outlen + 1;
}

// string_type_to_encoding returns the |MBSTRING_*| constant for the encoding
// used by the |ASN1_STRING| type |type|, or -1 if |tag| is not a string
// type.
static int string_type_to_encoding(int type) {
  // This function is sometimes passed ASN.1 universal types and sometimes
  // passed |ASN1_STRING| type values
  switch (type) {
    case V_ASN1_UTF8STRING:
      return MBSTRING_UTF8;
    case V_ASN1_NUMERICSTRING:
    case V_ASN1_PRINTABLESTRING:
    case V_ASN1_T61STRING:
    case V_ASN1_IA5STRING:
    case V_ASN1_UTCTIME:
    case V_ASN1_GENERALIZEDTIME:
    case V_ASN1_ISO64STRING:
      // |MBSTRING_ASC| refers to Latin-1, not ASCII.
      return MBSTRING_ASC;
    case V_ASN1_UNIVERSALSTRING:
      return MBSTRING_UNIV;
    case V_ASN1_BMPSTRING:
      return MBSTRING_BMP;
  }
  return -1;
}

// This is the main function, print out an ASN1_STRING taking note of various
// escape and display options. Returns number of characters written or -1 if
// an error occurred.

int ASN1_STRING_print_ex(BIO *out, const ASN1_STRING *str,
                         unsigned long flags) {
  int type = str->type;
  int outlen = 0;
  if (flags & ASN1_STRFLGS_SHOW_TYPE) {
    const char *tagname = ASN1_tag2str(type);
    outlen += strlen(tagname);
    if (!maybe_write(out, tagname, outlen) || !maybe_write(out, ":", 1)) {
      return -1;
    }
    outlen++;
  }

  // Decide what to do with |str|, either dump the contents or display it.
  int encoding;
  if (flags & ASN1_STRFLGS_DUMP_ALL) {
    // Dump everything.
    encoding = -1;
  } else if (flags & ASN1_STRFLGS_IGNORE_TYPE) {
    // Ignore the string type and interpret the contents as Latin-1.
    encoding = MBSTRING_ASC;
  } else {
    encoding = string_type_to_encoding(type);
    if (encoding == -1 && (flags & ASN1_STRFLGS_DUMP_UNKNOWN) == 0) {
      encoding = MBSTRING_ASC;
    }
  }

  if (encoding == -1) {
    int len = do_dump(flags, out, str);
    if (len < 0) {
      return -1;
    }
    outlen += len;
    return outlen;
  }

  int utf8_convert = 0;
  if (flags & ASN1_STRFLGS_UTF8_CONVERT) {
    // If the string is UTF-8, skip decoding and just interpret it as 1 byte
    // per character to avoid converting twice.
    //
    // TODO(davidben): This is not quite a valid optimization if the input
    // was invalid UTF-8.
    if (encoding == MBSTRING_UTF8) {
      encoding = MBSTRING_ASC;
    } else {
      utf8_convert = 1;
    }
  }

  // Measure the length.
  char quotes = 0;
  int len = do_buf(str->data, str->length, encoding, utf8_convert, flags,
                   &quotes, NULL);
  if (len < 0) {
    return -1;
  }
  outlen += len;
  if (quotes) {
    outlen += 2;
  }
  if (!out) {
    return outlen;
  }

  // Encode the value.
  if ((quotes && !maybe_write(out, "\"", 1)) ||
      do_buf(str->data, str->length, encoding, utf8_convert, flags, NULL, out) <
          0 ||
      (quotes && !maybe_write(out, "\"", 1))) {
    return -1;
  }
  return outlen;
}

int ASN1_STRING_print_ex_fp(FILE *fp, const ASN1_STRING *str,
                            unsigned long flags) {
  BIO *bio = NULL;
  if (fp != NULL) {
    // If |fp| is NULL, this function returns the number of bytes without
    // writing.
    bio = BIO_new_fp(fp, BIO_NOCLOSE);
    if (bio == NULL) {
      return -1;
    }
  }
  int ret = ASN1_STRING_print_ex(bio, str, flags);
  BIO_free(bio);
  return ret;
}

int ASN1_STRING_to_UTF8(unsigned char **out, const ASN1_STRING *in) {
  if (!in) {
    return -1;
  }
  int mbflag = string_type_to_encoding(in->type);
  if (mbflag == -1) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_UNKNOWN_TAG);
    return -1;
  }
  ASN1_STRING stmp, *str = &stmp;
  stmp.data = NULL;
  stmp.length = 0;
  stmp.flags = 0;
  int ret =
      ASN1_mbstring_copy(&str, in->data, in->length, mbflag, B_ASN1_UTF8STRING);
  if (ret < 0) {
    return ret;
  }
  *out = stmp.data;
  return stmp.length;
}

int ASN1_STRING_print(BIO *bp, const ASN1_STRING *v) {
  int i, n;
  char buf[80];
  const char *p;

  if (v == NULL) {
    return 0;
  }
  n = 0;
  p = (const char *)v->data;
  for (i = 0; i < v->length; i++) {
    if ((p[i] > '~') || ((p[i] < ' ') && (p[i] != '\n') && (p[i] != '\r'))) {
      buf[n] = '.';
    } else {
      buf[n] = p[i];
    }
    n++;
    if (n >= 80) {
      if (BIO_write(bp, buf, n) <= 0) {
        return 0;
      }
      n = 0;
    }
  }
  if (n > 0) {
    if (BIO_write(bp, buf, n) <= 0) {
      return 0;
    }
  }
  return 1;
}

int ASN1_TIME_print(BIO *bp, const ASN1_TIME *tm) {
  if (tm->type == V_ASN1_UTCTIME) {
    return ASN1_UTCTIME_print(bp, tm);
  }
  if (tm->type == V_ASN1_GENERALIZEDTIME) {
    return ASN1_GENERALIZEDTIME_print(bp, tm);
  }
  BIO_puts(bp, "Bad time value");
  return 0;
}

static const char *const mon[12] = {"Jan", "Feb", "Mar", "Apr", "May", "Jun",
                                    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"};

int ASN1_GENERALIZEDTIME_print(BIO *bp, const ASN1_GENERALIZEDTIME *tm) {
  CBS cbs;
  CBS_init(&cbs, tm->data, tm->length);
  struct tm utc;
  if (!CBS_parse_generalized_time(&cbs, &utc, /*allow_timezone_offset=*/0)) {
    BIO_puts(bp, "Bad time value");
    return 0;
  }

  return BIO_printf(bp, "%s %2d %02d:%02d:%02d %d GMT", mon[utc.tm_mon],
                    utc.tm_mday, utc.tm_hour, utc.tm_min, utc.tm_sec,
                    utc.tm_year + 1900) > 0;
}

int ASN1_UTCTIME_print(BIO *bp, const ASN1_UTCTIME *tm) {
  CBS cbs;
  CBS_init(&cbs, tm->data, tm->length);
  struct tm utc;
  if (!CBS_parse_utc_time(&cbs, &utc, /*allow_timezone_offset=*/0)) {
    BIO_puts(bp, "Bad time value");
    return 0;
  }

  return BIO_printf(bp, "%s %2d %02d:%02d:%02d %d GMT", mon[utc.tm_mon],
                    utc.tm_mday, utc.tm_hour, utc.tm_min, utc.tm_sec,
                    utc.tm_year + 1900) > 0;
}
