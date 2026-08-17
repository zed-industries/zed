// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>

#include <limits.h>
#include <string.h>

#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../bytestring/internal.h"
#include "internal.h"

// These functions take a string in UTF8, ASCII or multibyte form and a mask
// of permissible ASN1 string types. It then works out the minimal type
// (using the order Printable < IA5 < T61 < BMP < Universal < UTF8) and
// creates a string of the correct type with the supplied data. Yes this is
// horrible: it has to be :-( The 'ncopy' form checks minimum and maximum
// size limits too.

int ASN1_mbstring_copy(ASN1_STRING **out, const unsigned char *in,
                       ossl_ssize_t len, int inform, unsigned long mask) {
  return ASN1_mbstring_ncopy(out, in, len, inform, mask, /*minsize=*/0,
                             /*maxsize=*/0);
}

OPENSSL_DECLARE_ERROR_REASON(ASN1, INVALID_BMPSTRING)
OPENSSL_DECLARE_ERROR_REASON(ASN1, INVALID_UNIVERSALSTRING)
OPENSSL_DECLARE_ERROR_REASON(ASN1, INVALID_UTF8STRING)

int ASN1_mbstring_ncopy(ASN1_STRING **out, const unsigned char *in,
                        ossl_ssize_t len, int inform, unsigned long mask,
                        ossl_ssize_t minsize, ossl_ssize_t maxsize) {
  if (len < -1) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_FORMAT);
    return -1;
  }
  if (len == -1) {
    len = strlen((const char *)in);
  }
  if (!mask) {
    mask = DIRSTRING_TYPE;
  }

  int (*decode_func)(CBS *, uint32_t *);
  int error;
  switch (inform) {
    case MBSTRING_BMP:
      decode_func = cbs_get_ucs2_be;
      error = ASN1_R_INVALID_BMPSTRING;
      break;

    case MBSTRING_UNIV:
      decode_func = cbs_get_utf32_be;
      error = ASN1_R_INVALID_UNIVERSALSTRING;
      break;

    case MBSTRING_UTF8:
      decode_func = cbs_get_utf8;
      error = ASN1_R_INVALID_UTF8STRING;
      break;

    case MBSTRING_ASC:
      decode_func = cbs_get_latin1;
      error = ERR_R_INTERNAL_ERROR;  // Latin-1 inputs are never invalid.
      break;

    default:
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_UNKNOWN_FORMAT);
      return -1;
  }

  // Check |minsize| and |maxsize| and work out the minimal type, if any.
  CBS cbs;
  CBS_init(&cbs, in, len);
  size_t utf8_len = 0, nchar = 0;
  while (CBS_len(&cbs) != 0) {
    uint32_t c;
    if (!decode_func(&cbs, &c)) {
      OPENSSL_PUT_ERROR(ASN1, error);
      return -1;
    }
    if (nchar == 0 && (inform == MBSTRING_BMP || inform == MBSTRING_UNIV) &&
        c == 0xfeff) {
      // Reject byte-order mark. We could drop it but that would mean
      // adding ambiguity around whether a BOM was included or not when
      // matching strings.
      //
      // For a little-endian UCS-2 string, the BOM will appear as 0xfffe
      // and will be rejected as noncharacter, below.
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_CHARACTERS);
      return -1;
    }

    // Update which output formats are still possible.
    if ((mask & B_ASN1_PRINTABLESTRING) && !asn1_is_printable(c)) {
      mask &= ~B_ASN1_PRINTABLESTRING;
    }
    if ((mask & B_ASN1_IA5STRING) && (c > 127)) {
      mask &= ~B_ASN1_IA5STRING;
    }
    if ((mask & B_ASN1_T61STRING) && (c > 0xff)) {
      mask &= ~B_ASN1_T61STRING;
    }
    if ((mask & B_ASN1_BMPSTRING) && (c > 0xffff)) {
      mask &= ~B_ASN1_BMPSTRING;
    }
    if (!mask) {
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_CHARACTERS);
      return -1;
    }

    nchar++;
    utf8_len += cbb_get_utf8_len(c);
    if (maxsize > 0 && nchar > (size_t)maxsize) {
      OPENSSL_PUT_ERROR(ASN1, ASN1_R_STRING_TOO_LONG);
#if defined(OPENSSL_WINDOWS)
      ERR_add_error_dataf("maxsize=%lu", (unsigned long)maxsize);
#else
      ERR_add_error_dataf("maxsize=%zu", (size_t)maxsize);
#endif
      return -1;
    }
  }

  if (minsize > 0 && nchar < (size_t)minsize) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_STRING_TOO_SHORT);
#if defined(OPENSSL_WINDOWS)
    ERR_add_error_dataf("minsize=%lu", (unsigned long)minsize);
#else
    ERR_add_error_dataf("minsize=%zu", (size_t)minsize);
#endif
    return -1;
  }

  // Now work out output format and string type
  int str_type;
  int (*encode_func)(CBB *, uint32_t) = cbb_add_latin1;
  size_t size_estimate = nchar;
  int outform = MBSTRING_ASC;
  if (mask & B_ASN1_PRINTABLESTRING) {
    str_type = V_ASN1_PRINTABLESTRING;
  } else if (mask & B_ASN1_IA5STRING) {
    str_type = V_ASN1_IA5STRING;
  } else if (mask & B_ASN1_T61STRING) {
    str_type = V_ASN1_T61STRING;
  } else if (mask & B_ASN1_BMPSTRING) {
    str_type = V_ASN1_BMPSTRING;
    outform = MBSTRING_BMP;
    encode_func = cbb_add_ucs2_be;
    size_estimate = 2 * nchar;
  } else if (mask & B_ASN1_UNIVERSALSTRING) {
    str_type = V_ASN1_UNIVERSALSTRING;
    encode_func = cbb_add_utf32_be;
    size_estimate = 4 * nchar;
    outform = MBSTRING_UNIV;
  } else if (mask & B_ASN1_UTF8STRING) {
    str_type = V_ASN1_UTF8STRING;
    outform = MBSTRING_UTF8;
    encode_func = cbb_add_utf8;
    size_estimate = utf8_len;
  } else {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_ILLEGAL_CHARACTERS);
    return -1;
  }

  if (!out) {
    return str_type;
  }

  int free_dest = 0;
  ASN1_STRING *dest;
  if (*out) {
    dest = *out;
  } else {
    free_dest = 1;
    dest = ASN1_STRING_type_new(str_type);
    if (!dest) {
      return -1;
    }
  }

  CBB cbb;
  CBB_zero(&cbb);
  // If both the same type just copy across
  if (inform == outform) {
    if (!ASN1_STRING_set(dest, in, len)) {
      goto err;
    }
    dest->type = str_type;
    *out = dest;
    return str_type;
  }
  if (!CBB_init(&cbb, size_estimate + 1)) {
    goto err;
  }
  CBS_init(&cbs, in, len);
  while (CBS_len(&cbs) != 0) {
    uint32_t c;
    if (!decode_func(&cbs, &c) || !encode_func(&cbb, c)) {
      OPENSSL_PUT_ERROR(ASN1, ERR_R_INTERNAL_ERROR);
      goto err;
    }
  }
  uint8_t *data = NULL;
  size_t data_len;
  if (// OpenSSL historically NUL-terminated this value with a single byte,
      // even for |MBSTRING_BMP| and |MBSTRING_UNIV|.
      !CBB_add_u8(&cbb, 0) || !CBB_finish(&cbb, &data, &data_len) ||
      data_len < 1 || data_len > INT_MAX) {
    OPENSSL_PUT_ERROR(ASN1, ERR_R_INTERNAL_ERROR);
    OPENSSL_free(data);
    goto err;
  }
  dest->type = str_type;
  ASN1_STRING_set0(dest, data, (int)data_len - 1);
  *out = dest;
  return str_type;

err:
  if (free_dest) {
    ASN1_STRING_free(dest);
  }
  CBB_cleanup(&cbb);
  return -1;
}

int asn1_is_printable(uint32_t value) {
  if (value > 0x7f) {
    return 0;
  }
  return OPENSSL_isalnum(value) || //
         value == ' ' || value == '\'' || value == '(' || value == ')' ||
         value == '+' || value == ',' || value == '-' || value == '.' ||
         value == '/' || value == ':' || value == '=' || value == '?';
}
