// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/asn1.h>
#include <openssl/bytestring.h>
#include <openssl/mem.h>

#include <assert.h>

#ifndef __STDC_FORMAT_MACROS
#define __STDC_FORMAT_MACROS
#endif
#include <ctype.h>
#include <inttypes.h>

#include <string.h>

#include "../asn1/internal.h"
#include "../internal.h"
#include "internal.h"


void CBS_init(CBS *cbs, const uint8_t *data, size_t len) {
  cbs->data = data;
  cbs->len = len;
}

static int cbs_get(CBS *cbs, const uint8_t **p, size_t n) {
  if (cbs->len < n) {
    return 0;
  }

  *p = cbs->data;
  cbs->data += n;
  cbs->len -= n;
  return 1;
}

int CBS_skip(CBS *cbs, size_t len) {
  const uint8_t *dummy;
  return cbs_get(cbs, &dummy, len);
}

const uint8_t *CBS_data(const CBS *cbs) { return cbs->data; }

size_t CBS_len(const CBS *cbs) { return cbs->len; }

int CBS_stow(const CBS *cbs, uint8_t **out_ptr, size_t *out_len) {
  OPENSSL_free(*out_ptr);
  *out_ptr = NULL;
  *out_len = 0;

  if (cbs->len == 0) {
    return 1;
  }
  *out_ptr = OPENSSL_memdup(cbs->data, cbs->len);
  if (*out_ptr == NULL) {
    return 0;
  }
  *out_len = cbs->len;
  return 1;
}

int CBS_strdup(const CBS *cbs, char **out_ptr) {
  if (*out_ptr != NULL) {
    OPENSSL_free(*out_ptr);
  }
  *out_ptr = OPENSSL_strndup((const char *)cbs->data, cbs->len);
  return (*out_ptr != NULL);
}

int CBS_contains_zero_byte(const CBS *cbs) {
  return OPENSSL_memchr(cbs->data, 0, cbs->len) != NULL;
}

int CBS_mem_equal(const CBS *cbs, const uint8_t *data, size_t len) {
  if (len != cbs->len) {
    return 0;
  }
  return CRYPTO_memcmp(cbs->data, data, len) == 0;
}

static int cbs_get_u(CBS *cbs, uint64_t *out, size_t len) {
  uint64_t result = 0;
  const uint8_t *data;

  if (!cbs_get(cbs, &data, len)) {
    return 0;
  }
  for (size_t i = 0; i < len; i++) {
    result <<= 8;
    result |= data[i];
  }
  *out = result;
  return 1;
}

int CBS_get_u8(CBS *cbs, uint8_t *out) {
  const uint8_t *v;
  if (!cbs_get(cbs, &v, 1)) {
    return 0;
  }
  *out = *v;
  return 1;
}

int CBS_get_u16(CBS *cbs, uint16_t *out) {
  uint64_t v;
  if (!cbs_get_u(cbs, &v, 2)) {
    return 0;
  }
  *out = v;
  return 1;
}

int CBS_get_u16le(CBS *cbs, uint16_t *out) {
  if (!CBS_get_u16(cbs, out)) {
    return 0;
  }
  *out = CRYPTO_bswap2(*out);
  return 1;
}

int CBS_get_u24(CBS *cbs, uint32_t *out) {
  uint64_t v;
  if (!cbs_get_u(cbs, &v, 3)) {
    return 0;
  }
  *out = (uint32_t)v;
  return 1;
}

int CBS_get_u32(CBS *cbs, uint32_t *out) {
  uint64_t v;
  if (!cbs_get_u(cbs, &v, 4)) {
    return 0;
  }
  *out = (uint32_t)v;
  return 1;
}

int CBS_get_u32le(CBS *cbs, uint32_t *out) {
  if (!CBS_get_u32(cbs, out)) {
    return 0;
  }
  *out = CRYPTO_bswap4(*out);
  return 1;
}

int CBS_get_u64(CBS *cbs, uint64_t *out) { return cbs_get_u(cbs, out, 8); }

int CBS_get_u64le(CBS *cbs, uint64_t *out) {
  if (!cbs_get_u(cbs, out, 8)) {
    return 0;
  }
  *out = CRYPTO_bswap8(*out);
  return 1;
}

int CBS_get_last_u8(CBS *cbs, uint8_t *out) {
  if (cbs->len == 0) {
    return 0;
  }
  *out = cbs->data[cbs->len - 1];
  cbs->len--;
  return 1;
}

int CBS_get_bytes(CBS *cbs, CBS *out, size_t len) {
  const uint8_t *v;
  if (!cbs_get(cbs, &v, len)) {
    return 0;
  }
  CBS_init(out, v, len);
  return 1;
}

int CBS_copy_bytes(CBS *cbs, uint8_t *out, size_t len) {
  const uint8_t *v;
  if (!cbs_get(cbs, &v, len)) {
    return 0;
  }
  OPENSSL_memcpy(out, v, len);
  return 1;
}

static int cbs_get_length_prefixed(CBS *cbs, CBS *out, size_t len_len) {
  uint64_t len;
  if (!cbs_get_u(cbs, &len, len_len)) {
    return 0;
  }
  // If |len_len| <= 3 then we know that |len| will fit into a |size_t|, even on
  // 32-bit systems.
  assert(len_len <= 3);
  return CBS_get_bytes(cbs, out, len);
}

int CBS_get_u8_length_prefixed(CBS *cbs, CBS *out) {
  return cbs_get_length_prefixed(cbs, out, 1);
}

int CBS_get_u16_length_prefixed(CBS *cbs, CBS *out) {
  return cbs_get_length_prefixed(cbs, out, 2);
}

int CBS_get_u24_length_prefixed(CBS *cbs, CBS *out) {
  return cbs_get_length_prefixed(cbs, out, 3);
}

int CBS_get_until_first(CBS *cbs, CBS *out, uint8_t c) {
  const uint8_t *split = OPENSSL_memchr(CBS_data(cbs), c, CBS_len(cbs));
  if (split == NULL) {
    return 0;
  }
  return CBS_get_bytes(cbs, out, split - CBS_data(cbs));
}

int CBS_get_u64_decimal(CBS *cbs, uint64_t *out) {
  uint64_t v = 0;
  int seen_digit = 0;
  while (CBS_len(cbs) != 0) {
    uint8_t c = CBS_data(cbs)[0];
    if (!OPENSSL_isdigit(c)) {
      break;
    }
    CBS_skip(cbs, 1);
    if (  // Forbid stray leading zeros.
        (v == 0 && seen_digit) ||
        // Check for overflow.
        v > UINT64_MAX / 10 ||  //
        v * 10 > UINT64_MAX - (c - '0')) {
      return 0;
    }
    v = v * 10 + (c - '0');
    seen_digit = 1;
  }

  *out = v;
  return seen_digit;
}

// parse_base128_integer reads a big-endian base-128 integer from |cbs| and sets
// |*out| to the result. This is the encoding used in DER for both high tag
// number form and OID components.
static int parse_base128_integer(CBS *cbs, uint64_t *out) {
  uint64_t v = 0;
  uint8_t b;
  do {
    if (!CBS_get_u8(cbs, &b)) {
      return 0;
    }
    if ((v >> (64 - 7)) != 0) {
      // The value is too large.
      return 0;
    }
    if (v == 0 && b == 0x80) {
      // The value must be minimally encoded.
      return 0;
    }
    v = (v << 7) | (b & 0x7f);

    // Values end at an octet with the high bit cleared.
  } while (b & 0x80);

  *out = v;
  return 1;
}

static int parse_asn1_tag(CBS *cbs, CBS_ASN1_TAG *out, int universal_tag_ok) {
  uint8_t tag_byte;
  if (!CBS_get_u8(cbs, &tag_byte)) {
    return 0;
  }

  // ITU-T X.690 section 8.1.2.3 specifies the format for identifiers with a tag
  // number no greater than 30.
  //
  // If the number portion is 31 (0x1f, the largest value that fits in the
  // allotted bits), then the tag is more than one byte long and the
  // continuation bytes contain the tag number.
  CBS_ASN1_TAG tag = ((CBS_ASN1_TAG)tag_byte & 0xe0) << CBS_ASN1_TAG_SHIFT;
  CBS_ASN1_TAG tag_number = tag_byte & 0x1f;
  if (tag_number == 0x1f) {
    uint64_t v;
    if (!parse_base128_integer(cbs, &v) ||
        // Check the tag number is within our supported bounds.
        v > CBS_ASN1_TAG_NUMBER_MASK ||
        // Small tag numbers should have used low tag number form, even in BER.
        v < 0x1f) {
      return 0;
    }
    tag_number = (CBS_ASN1_TAG)v;
  }

  tag |= tag_number;

  // Tag [UNIVERSAL 0] is reserved for use by the encoding. Reject it here to
  // avoid some ambiguity around ANY values and BER indefinite-length EOCs. See
  // https://crbug.com/boringssl/455.
  if (!universal_tag_ok && (tag & ~CBS_ASN1_CONSTRUCTED) == 0) {
    return 0;
  }

  *out = tag;
  return 1;
}

int cbs_get_any_asn1_element(CBS *cbs, CBS *out, CBS_ASN1_TAG *out_tag,
                                    size_t *out_header_len, int *out_ber_found,
                                    int *out_indefinite, int ber_ok, int universal_tag_ok) {
  CBS header = *cbs;
  CBS throwaway;

  if (out == NULL) {
    out = &throwaway;
  }
  if (ber_ok) {
    *out_ber_found = 0;
    *out_indefinite = 0;
  } else {
    assert(out_ber_found == NULL);
    assert(out_indefinite == NULL);
  }

  CBS_ASN1_TAG tag;
  if (!parse_asn1_tag(&header, &tag, universal_tag_ok)) {
    return 0;
  }
  if (out_tag != NULL) {
    *out_tag = tag;
  }

  uint8_t length_byte;
  if (!CBS_get_u8(&header, &length_byte)) {
    return 0;
  }

  size_t header_len = CBS_len(cbs) - CBS_len(&header);

  size_t len;
  // The format for the length encoding is specified in ITU-T X.690 section
  // 8.1.3.
  if ((length_byte & 0x80) == 0) {
    // Short form length.
    len = ((size_t)length_byte) + header_len;
    if (out_header_len != NULL) {
      *out_header_len = header_len;
    }
  } else {
    // The high bit indicate that this is the long form, while the next 7 bits
    // encode the number of subsequent octets used to encode the length (ITU-T
    // X.690 clause 8.1.3.5.b).
    const size_t num_bytes = length_byte & 0x7f;
    uint64_t len64;

    if (ber_ok && (tag & CBS_ASN1_CONSTRUCTED) != 0 && num_bytes == 0) {
      // indefinite length
      if (out_header_len != NULL) {
        *out_header_len = header_len;
      }
      *out_ber_found = 1;
      *out_indefinite = 1;
      return CBS_get_bytes(cbs, out, header_len);
    }

    // ITU-T X.690 clause 8.1.3.5.c specifies that the value 0xff shall not be
    // used as the first byte of the length. If this parser encounters that
    // value, num_bytes will be parsed as 127, which will fail this check.
    if (num_bytes == 0 || num_bytes > 4) {
      return 0;
    }
    if (!cbs_get_u(&header, &len64, num_bytes)) {
      return 0;
    }
    // ITU-T X.690 section 10.1 (DER length forms) requires encoding the
    // length with the minimum number of octets. BER could, technically, have
    // 125 superfluous zero bytes. We do not attempt to handle that and still
    // require that the length fit in a |uint32_t| for BER.
    if (len64 < 128) {
      // Length should have used short-form encoding.
      if (ber_ok) {
        *out_ber_found = 1;
      } else {
        return 0;
      }
    }
    if ((len64 >> ((num_bytes - 1) * 8)) == 0) {
      // Length should have been at least one byte shorter.
      if (ber_ok) {
        *out_ber_found = 1;
      } else {
        return 0;
      }
    }
    len = len64;
    if (len + header_len + num_bytes < len) {
      // Overflow.
      return 0;
    }
    len += header_len + num_bytes;
    if (out_header_len != NULL) {
      *out_header_len = header_len + num_bytes;
    }
  }

  return CBS_get_bytes(cbs, out, len);
}

int CBS_get_any_asn1(CBS *cbs, CBS *out, CBS_ASN1_TAG *out_tag) {
  size_t header_len;
  if (!CBS_get_any_asn1_element(cbs, out, out_tag, &header_len)) {
    return 0;
  }

  if (!CBS_skip(out, header_len)) {
    assert(0);
    return 0;
  }

  return 1;
}

int CBS_get_any_asn1_element(CBS *cbs, CBS *out, CBS_ASN1_TAG *out_tag,
                             size_t *out_header_len) {
  return cbs_get_any_asn1_element(cbs, out, out_tag, out_header_len, NULL, NULL,
                                  /*ber_ok=*/0, /*universal_tag_ok=*/0);
}

int CBS_get_any_ber_asn1_element(CBS *cbs, CBS *out, CBS_ASN1_TAG *out_tag,
                                 size_t *out_header_len, int *out_ber_found,
                                 int *out_indefinite) {
  int ber_found_temp;
  return cbs_get_any_asn1_element(
      cbs, out, out_tag, out_header_len,
      out_ber_found ? out_ber_found : &ber_found_temp, out_indefinite,
      /*ber_ok=*/1, /*universal_tag_ok=*/0);
}

static int cbs_get_asn1(CBS *cbs, CBS *out, CBS_ASN1_TAG tag_value,
                        int skip_header) {
  size_t header_len;
  CBS_ASN1_TAG tag;
  CBS throwaway;

  if (out == NULL) {
    out = &throwaway;
  }

  if (!CBS_get_any_asn1_element(cbs, out, &tag, &header_len) ||
      tag != tag_value) {
    return 0;
  }

  if (skip_header && !CBS_skip(out, header_len)) {
    assert(0);
    return 0;
  }

  return 1;
}

int CBS_get_asn1(CBS *cbs, CBS *out, CBS_ASN1_TAG tag_value) {
  return cbs_get_asn1(cbs, out, tag_value, 1 /* skip header */);
}

int CBS_get_asn1_element(CBS *cbs, CBS *out, CBS_ASN1_TAG tag_value) {
  return cbs_get_asn1(cbs, out, tag_value, 0 /* include header */);
}

int CBS_peek_asn1_tag(const CBS *cbs, CBS_ASN1_TAG tag_value) {
  CBS copy = *cbs;
  CBS_ASN1_TAG actual_tag;
  return parse_asn1_tag(&copy, &actual_tag, /*universal_tag_ok=*/0) && tag_value == actual_tag;
}

int CBS_get_asn1_uint64(CBS *cbs, uint64_t *out) {
  CBS bytes;
  if (!CBS_get_asn1(cbs, &bytes, CBS_ASN1_INTEGER) ||
      !CBS_is_unsigned_asn1_integer(&bytes)) {
    return 0;
  }

  *out = 0;
  const uint8_t *data = CBS_data(&bytes);
  size_t len = CBS_len(&bytes);
  for (size_t i = 0; i < len; i++) {
    if ((*out >> 56) != 0) {
      // Too large to represent as a uint64_t.
      return 0;
    }
    *out <<= 8;
    *out |= data[i];
  }

  return 1;
}

int CBS_get_asn1_int64(CBS *cbs, int64_t *out) {
  int is_negative;
  CBS bytes;
  if (!CBS_get_asn1(cbs, &bytes, CBS_ASN1_INTEGER) ||
      !CBS_is_valid_asn1_integer(&bytes, &is_negative)) {
    return 0;
  }
  const uint8_t *data = CBS_data(&bytes);
  const size_t len = CBS_len(&bytes);
  if (len > sizeof(int64_t)) {
    return 0;
  }
  uint8_t sign_extend[sizeof(int64_t)];
  memset(sign_extend, is_negative ? 0xff : 0, sizeof(sign_extend));
  // GCC 12/13 report `stringop-overflow` on the following line
  // without additional condition: `i < sizeof(int64_t)`
  for (size_t i = 0; i < len && i < sizeof(int64_t); i++) {
// `data` is big-endian.
// Values are always shifted toward the "little" end.
#ifdef OPENSSL_BIG_ENDIAN
    // Bytes are written starting at the highest index.
    sign_extend[sizeof(sign_extend) - i - 1] = data[len - i - 1];
#else
    // Bytes are written starting at the lowest index.
    sign_extend[i] = data[len - i - 1];
#endif
  }

  memcpy(out, sign_extend, sizeof(sign_extend));
  return 1;
}

int CBS_get_asn1_bool(CBS *cbs, int *out) {
  CBS bytes;
  if (!CBS_get_asn1(cbs, &bytes, CBS_ASN1_BOOLEAN) || CBS_len(&bytes) != 1) {
    return 0;
  }

  const uint8_t value = *CBS_data(&bytes);
  if (value != 0 && value != 0xff) {
    return 0;
  }

  *out = !!value;
  return 1;
}

int CBS_get_optional_asn1(CBS *cbs, CBS *out, int *out_present,
                          CBS_ASN1_TAG tag) {
  int present = 0;

  if (CBS_peek_asn1_tag(cbs, tag)) {
    if (!CBS_get_asn1(cbs, out, tag)) {
      return 0;
    }
    present = 1;
  }

  if (out_present != NULL) {
    *out_present = present;
  }

  return 1;
}

int CBS_get_optional_asn1_octet_string(CBS *cbs, CBS *out, int *out_present,
                                       CBS_ASN1_TAG tag) {
  CBS child;
  int present;
  if (!CBS_get_optional_asn1(cbs, &child, &present, tag)) {
    return 0;
  }
  if (present) {
    assert(out);
    if (!CBS_get_asn1(&child, out, CBS_ASN1_OCTETSTRING) ||
        CBS_len(&child) != 0) {
      return 0;
    }
  } else {
    CBS_init(out, NULL, 0);
  }
  if (out_present) {
    *out_present = present;
  }
  return 1;
}

int CBS_get_optional_asn1_uint64(CBS *cbs, uint64_t *out, CBS_ASN1_TAG tag,
                                 uint64_t default_value) {
  CBS child;
  int present;
  if (!CBS_get_optional_asn1(cbs, &child, &present, tag)) {
    return 0;
  }
  if (present) {
    if (!CBS_get_asn1_uint64(&child, out) || CBS_len(&child) != 0) {
      return 0;
    }
  } else {
    *out = default_value;
  }
  return 1;
}

int CBS_get_optional_asn1_bool(CBS *cbs, int *out, CBS_ASN1_TAG tag,
                               int default_value) {
  CBS child, child2;
  int present;
  if (!CBS_get_optional_asn1(cbs, &child, &present, tag)) {
    return 0;
  }
  if (present) {
    uint8_t boolean;

    if (!CBS_get_asn1(&child, &child2, CBS_ASN1_BOOLEAN) ||
        CBS_len(&child2) != 1 || CBS_len(&child) != 0) {
      return 0;
    }

    boolean = CBS_data(&child2)[0];
    if (boolean == 0) {
      *out = 0;
    } else if (boolean == 0xff) {
      *out = 1;
    } else {
      return 0;
    }
  } else {
    *out = default_value;
  }
  return 1;
}

int CBS_is_valid_asn1_bitstring(const CBS *cbs) {
  CBS in = *cbs;
  uint8_t num_unused_bits;
  if (!CBS_get_u8(&in, &num_unused_bits) || num_unused_bits > 7) {
    return 0;
  }

  if (num_unused_bits == 0) {
    return 1;
  }

  // All num_unused_bits bits must exist and be zeros.
  uint8_t last;
  if (!CBS_get_last_u8(&in, &last) ||
      (last & ((1 << num_unused_bits) - 1)) != 0) {
    return 0;
  }

  return 1;
}

int CBS_asn1_bitstring_has_bit(const CBS *cbs, unsigned bit) {
  if (!CBS_is_valid_asn1_bitstring(cbs)) {
    return 0;
  }

  const unsigned byte_num = (bit >> 3) + 1;
  const unsigned bit_num = 7 - (bit & 7);

  // Unused bits are zero, and this function does not distinguish between
  // missing and unset bits. Thus it is sufficient to do a byte-level length
  // check.
  return byte_num < CBS_len(cbs) &&
         (CBS_data(cbs)[byte_num] & (1 << bit_num)) != 0;
}

int CBS_is_valid_asn1_integer(const CBS *cbs, int *out_is_negative) {
  CBS copy = *cbs;
  uint8_t first_byte, second_byte;
  if (!CBS_get_u8(&copy, &first_byte)) {
    return 0;  // INTEGERs may not be empty.
  }
  if (out_is_negative != NULL) {
    *out_is_negative = (first_byte & 0x80) != 0;
  }
  if (!CBS_get_u8(&copy, &second_byte)) {
    return 1;  // One byte INTEGERs are always minimal.
  }
  if ((first_byte == 0x00 && (second_byte & 0x80) == 0) ||
      (first_byte == 0xff && (second_byte & 0x80) != 0)) {
    return 0;  // The value is minimal iff the first 9 bits are not all equal.
  }
  return 1;
}

int CBS_is_unsigned_asn1_integer(const CBS *cbs) {
  int is_negative;
  return CBS_is_valid_asn1_integer(cbs, &is_negative) && !is_negative;
}

static int add_decimal(CBB *out, uint64_t v) {
  char buf[DECIMAL_SIZE(uint64_t) + 1];
  snprintf(buf, sizeof(buf), "%" PRIu64, v);
  return CBB_add_bytes(out, (const uint8_t *)buf, strlen(buf));
}

int CBS_is_valid_asn1_oid(const CBS *cbs) {
  if (CBS_len(cbs) == 0) {
    return 0;  // OID encodings cannot be empty.
  }

  CBS copy = *cbs;
  uint8_t v, prev = 0;
  while (CBS_get_u8(&copy, &v)) {
    // OID encodings are a sequence of minimally-encoded base-128 integers (see
    // |parse_base128_integer|). If |prev|'s MSB was clear, it was the last byte
    // of an integer (or |v| is the first byte). |v| is then the first byte of
    // the next integer. If first byte of an integer is 0x80, it is not
    // minimally-encoded.
    if ((prev & 0x80) == 0 && v == 0x80) {
      return 0;
    }
    prev = v;
  }

  // The last byte should must end an integer encoding.
  return (prev & 0x80) == 0;
}

char *CBS_asn1_oid_to_text(const CBS *cbs) {
  CBB cbb;
  if (!CBB_init(&cbb, 32)) {
    goto err;
  }

  CBS copy = *cbs;
  // The first component is 40 * value1 + value2, where value1 is 0, 1, or 2.
  uint64_t v;
  if (!parse_base128_integer(&copy, &v)) {
    goto err;
  }

  if (v >= 80) {
    if (!CBB_add_bytes(&cbb, (const uint8_t *)"2.", 2) ||
        !add_decimal(&cbb, v - 80)) {
      goto err;
    }
  } else if (!add_decimal(&cbb, v / 40) || !CBB_add_u8(&cbb, '.') ||
             !add_decimal(&cbb, v % 40)) {
    goto err;
  }

  while (CBS_len(&copy) != 0) {
    if (!parse_base128_integer(&copy, &v) || !CBB_add_u8(&cbb, '.') ||
        !add_decimal(&cbb, v)) {
      goto err;
    }
  }

  uint8_t *txt;
  size_t txt_len;
  if (!CBB_add_u8(&cbb, '\0') || !CBB_finish(&cbb, &txt, &txt_len)) {
    goto err;
  }

  return (char *)txt;

err:
  CBB_cleanup(&cbb);
  return NULL;
}

static int cbs_get_two_digits(CBS *cbs, int *out) {
  uint8_t first_digit, second_digit;
  if (!CBS_get_u8(cbs, &first_digit)) {
    return 0;
  }
  if (!OPENSSL_isdigit(first_digit)) {
    return 0;
  }
  if (!CBS_get_u8(cbs, &second_digit)) {
    return 0;
  }
  if (!OPENSSL_isdigit(second_digit)) {
    return 0;
  }
  *out = (first_digit - '0') * 10 + (second_digit - '0');
  return 1;
}

static int is_valid_day(int year, int month, int day) {
  if (day < 1) {
    return 0;
  }
  switch (month) {
    case 1:
    case 3:
    case 5:
    case 7:
    case 8:
    case 10:
    case 12:
      return day <= 31;
    case 4:
    case 6:
    case 9:
    case 11:
      return day <= 30;
    case 2:
      if ((year % 4 == 0 && year % 100 != 0) || year % 400 == 0) {
        return day <= 29;
      } else {
        return day <= 28;
      }
    default:
      return 0;
  }
}

static int CBS_parse_rfc5280_time_internal(const CBS *cbs, int is_gentime,
                                           int allow_timezone_offset,
                                           struct tm *out_tm) {
  int year, month, day, hour, min, sec, tmp;
  CBS copy = *cbs;
  uint8_t tz;

  if (is_gentime) {
    if (!cbs_get_two_digits(&copy, &tmp)) {
      return 0;
    }
    year = tmp * 100;
    if (!cbs_get_two_digits(&copy, &tmp)) {
      return 0;
    }
    year += tmp;
  } else {
    year = 1900;
    if (!cbs_get_two_digits(&copy, &tmp)) {
      return 0;
    }
    year += tmp;
    if (year < 1950) {
      year += 100;
    }
    if (year >= 2050) {
      return 0;  // A Generalized time must be used.
    }
  }
  if (!cbs_get_two_digits(&copy, &month) || month < 1 ||
      month > 12 ||  // Reject invalid months.
      !cbs_get_two_digits(&copy, &day) ||
      !is_valid_day(year, month, day) ||  // Reject invalid days.
      !cbs_get_two_digits(&copy, &hour) ||
      hour > 23 ||  // Reject invalid hours.
      !cbs_get_two_digits(&copy, &min) ||
      min > 59 ||  // Reject invalid minutes.
      !cbs_get_two_digits(&copy, &sec) || sec > 59 || !CBS_get_u8(&copy, &tz)) {
    return 0;
  }

  int offset_sign = 0;
  switch (tz) {
    case 'Z':
      break;  // We correctly have 'Z' on the end as per spec.
    case '+':
      offset_sign = 1;
      break;  // Should not be allowed per RFC 5280.
    case '-':
      offset_sign = -1;
      break;  // Should not be allowed per RFC 5280.
    default:
      return 0;  // Reject anything else after the time.
  }

  // If allow_timezone_offset is non-zero, allow for a four digit timezone
  // offset to be specified even though this is not allowed by RFC 5280. We are
  // permissive of this for UTCTimes due to the unfortunate existence of
  // artisinally rolled long lived certificates that were baked into places that
  // are now difficult to change. These certificates were generated with the
  // 'openssl' command that permissively allowed the creation of certificates
  // with notBefore and notAfter times specified as strings for direct
  // certificate inclusion on the command line. For context see cl/237068815.
  //
  // TODO(bbe): This has been expunged from public web-pki as the ecosystem has
  // managed to encourage CA compliance with standards. We should find a way to
  // get rid of this or make it off by default.
  int offset_seconds = 0;
  if (offset_sign != 0) {
    if (!allow_timezone_offset) {
      return 0;
    }
    int offset_hours, offset_minutes;
    if (!cbs_get_two_digits(&copy, &offset_hours) ||
        offset_hours > 23 ||  // Reject invalid hours.
        !cbs_get_two_digits(&copy, &offset_minutes) ||
        offset_minutes > 59) {  // Reject invalid minutes.
      return 0;
    }
    offset_seconds = offset_sign * (offset_hours * 3600 + offset_minutes * 60);
  }

  if (CBS_len(&copy) != 0) {
    return 0;  // Reject invalid lengths.
  }

  if (out_tm != NULL) {
    // Fill in the tm fields corresponding to what we validated.
    out_tm->tm_year = year - 1900;
    out_tm->tm_mon = month - 1;
    out_tm->tm_mday = day;
    out_tm->tm_hour = hour;
    out_tm->tm_min = min;
    out_tm->tm_sec = sec;
    if (offset_seconds && !OPENSSL_gmtime_adj(out_tm, 0, offset_seconds)) {
      return 0;
    }
  }
  return 1;
}

int CBS_parse_generalized_time(const CBS *cbs, struct tm *out_tm,
                               int allow_timezone_offset) {
  return CBS_parse_rfc5280_time_internal(cbs, 1, allow_timezone_offset, out_tm);
}

int CBS_parse_utc_time(const CBS *cbs, struct tm *out_tm,
                       int allow_timezone_offset) {
  return CBS_parse_rfc5280_time_internal(cbs, 0, allow_timezone_offset, out_tm);
}

int CBS_get_optional_asn1_int64(CBS *cbs, int64_t *out, CBS_ASN1_TAG tag,
                                int64_t default_value) {
  CBS child;
  int present;
  if (!CBS_get_optional_asn1(cbs, &child, &present, tag)) {
    return 0;
  }
  if (present) {
    if (!CBS_get_asn1_int64(&child, out) || CBS_len(&child) != 0) {
      return 0;
    }
  } else {
    *out = default_value;
  }
  return 1;
}
