// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/bytestring.h>

#include "internal.h"


static int is_valid_code_point(uint32_t v) {
  // References in the following are to Unicode 15.0.0.
  if (// The Unicode space runs from zero to 0x10ffff (3.4 D9).
      v > 0x10ffff ||
      // Values 0x...fffe, 0x...ffff, and 0xfdd0-0xfdef are permanently reserved
      // as noncharacters (3.4 D14). See also 23.7. As our APIs are intended for
      // "open interchange", such as ASN.1, we reject them.
      (v & 0xfffe) == 0xfffe ||
      (v >= 0xfdd0 && v <= 0xfdef) ||
      // Surrogate code points are invalid (3.2 C1).
      (v >= 0xd800 && v <= 0xdfff)) {
    return 0;
  }
  return 1;
}

// BOTTOM_BITS returns a byte with the bottom |n| bits set.
#define BOTTOM_BITS(n) (uint8_t)((1u << (n)) - 1)

// TOP_BITS returns a byte with the top |n| bits set.
#define TOP_BITS(n) ((uint8_t)~BOTTOM_BITS(8 - (n)))

int cbs_get_utf8(CBS *cbs, uint32_t *out) {
  uint8_t c;
  if (!CBS_get_u8(cbs, &c)) {
    return 0;
  }
  if (c <= 0x7f) {
    *out = c;
    return 1;
  }
  uint32_t v, lower_bound;
  size_t len;
  if ((c & TOP_BITS(3)) == TOP_BITS(2)) {
    v = c & BOTTOM_BITS(5);
    len = 1;
    lower_bound = 0x80;
  } else if ((c & TOP_BITS(4)) == TOP_BITS(3)) {
    v = c & BOTTOM_BITS(4);
    len = 2;
    lower_bound = 0x800;
  } else if ((c & TOP_BITS(5)) == TOP_BITS(4)) {
    v = c & BOTTOM_BITS(3);
    len = 3;
    lower_bound = 0x10000;
  } else {
    return 0;
  }
  for (size_t i = 0; i < len; i++) {
    if (!CBS_get_u8(cbs, &c) ||
        (c & TOP_BITS(2)) != TOP_BITS(1)) {
      return 0;
    }
    v <<= 6;
    v |= c & BOTTOM_BITS(6);
  }
  if (!is_valid_code_point(v) ||
      v < lower_bound) {
    return 0;
  }
  *out = v;
  return 1;
}

int cbs_get_latin1(CBS *cbs, uint32_t *out) {
  uint8_t c;
  if (!CBS_get_u8(cbs, &c)) {
    return 0;
  }
  *out = c;
  return 1;
}

int cbs_get_ucs2_be(CBS *cbs, uint32_t *out) {
  // Note UCS-2 (used by BMPString) does not support surrogates.
  uint16_t c;
  if (!CBS_get_u16(cbs, &c) ||
      !is_valid_code_point(c)) {
    return 0;
  }
  *out = c;
  return 1;
}

int cbs_get_utf32_be(CBS *cbs, uint32_t *out) {
  return CBS_get_u32(cbs, out) && is_valid_code_point(*out);
}

size_t cbb_get_utf8_len(uint32_t u) {
  if (u <= 0x7f) {
    return 1;
  }
  if (u <= 0x7ff) {
    return 2;
  }
  if (u <= 0xffff) {
    return 3;
  }
  return 4;
}

int cbb_add_utf8(CBB *cbb, uint32_t u) {
  if (!is_valid_code_point(u)) {
    return 0;
  }
  if (u <= 0x7f) {
    return CBB_add_u8(cbb, (uint8_t)u);
  }
  if (u <= 0x7ff) {
    return CBB_add_u8(cbb, TOP_BITS(2) | (u >> 6)) &&
           CBB_add_u8(cbb, TOP_BITS(1) | (u & BOTTOM_BITS(6)));
  }
  if (u <= 0xffff) {
    return CBB_add_u8(cbb, TOP_BITS(3) | (u >> 12)) &&
           CBB_add_u8(cbb, TOP_BITS(1) | ((u >> 6) & BOTTOM_BITS(6))) &&
           CBB_add_u8(cbb, TOP_BITS(1) | (u & BOTTOM_BITS(6)));
  }
  if (u <= 0x10ffff) {
    return CBB_add_u8(cbb, TOP_BITS(4) | (u >> 18)) &&
           CBB_add_u8(cbb, TOP_BITS(1) | ((u >> 12) & BOTTOM_BITS(6))) &&
           CBB_add_u8(cbb, TOP_BITS(1) | ((u >> 6) & BOTTOM_BITS(6))) &&
           CBB_add_u8(cbb, TOP_BITS(1) | (u & BOTTOM_BITS(6)));
  }
  return 0;
}

int cbb_add_latin1(CBB *cbb, uint32_t u) {
  return u <= 0xff && CBB_add_u8(cbb, (uint8_t)u);
}

int cbb_add_ucs2_be(CBB *cbb, uint32_t u) {
  return u <= 0xffff && is_valid_code_point(u) && CBB_add_u16(cbb, (uint16_t)u);
}

int cbb_add_utf32_be(CBB *cbb, uint32_t u) {
  return is_valid_code_point(u) && CBB_add_u32(cbb, u);
}
