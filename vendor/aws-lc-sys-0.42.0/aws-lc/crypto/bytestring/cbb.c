// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/bytestring.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/mem.h>
#include <openssl/err.h>

#include "../internal.h"


void CBB_zero(CBB *cbb) {
  OPENSSL_memset(cbb, 0, sizeof(CBB));
}

static void cbb_init(CBB *cbb, uint8_t *buf, size_t cap, int can_resize) {
  cbb->is_child = 0;
  cbb->child = NULL;
  cbb->u.base.buf = buf;
  cbb->u.base.len = 0;
  cbb->u.base.cap = cap;
  cbb->u.base.can_resize = can_resize;
  cbb->u.base.error = 0;
}

int CBB_init(CBB *cbb, size_t initial_capacity) {
  CBB_zero(cbb);

  uint8_t *buf = OPENSSL_malloc(initial_capacity);
  if (initial_capacity > 0 && buf == NULL) {
    return 0;
  }

  cbb_init(cbb, buf, initial_capacity, /*can_resize=*/1);
  return 1;
}

int CBB_init_fixed(CBB *cbb, uint8_t *buf, size_t len) {
  CBB_zero(cbb);
  cbb_init(cbb, buf, len, /*can_resize=*/0);
  return 1;
}

void CBB_cleanup(CBB *cbb) {
  // Child |CBB|s are non-owning. They are implicitly discarded and should not
  // be used with |CBB_cleanup| or |ScopedCBB|.
  assert(!cbb->is_child);
  if (cbb->is_child) {
    return;
  }

  if (cbb->u.base.can_resize) {
    OPENSSL_free(cbb->u.base.buf);
  }
}

static int cbb_buffer_reserve(struct cbb_buffer_st *base, uint8_t **out,
                              size_t len) {
  if (base == NULL) {
    return 0;
  }

  size_t newlen = base->len + len;
  if (newlen < base->len) {
    // Overflow
    OPENSSL_PUT_ERROR(CRYPTO, ERR_R_OVERFLOW);
    goto err;
  }

  if (newlen > base->cap) {
    if (!base->can_resize) {
      OPENSSL_PUT_ERROR(CRYPTO, ERR_R_OVERFLOW);
      goto err;
    }

    size_t newcap = base->cap * 2;
    if (newcap < base->cap || newcap < newlen) {
      newcap = newlen;
    }
    uint8_t *newbuf = OPENSSL_realloc(base->buf, newcap);
    if (newbuf == NULL) {
      goto err;
    }

    base->buf = newbuf;
    base->cap = newcap;
  }

  if (out) {
    *out = base->buf + base->len;
  }

  return 1;

err:
  base->error = 1;
  return 0;
}

static int cbb_buffer_add(struct cbb_buffer_st *base, uint8_t **out,
                          size_t len) {
  if (!cbb_buffer_reserve(base, out, len)) {
    return 0;
  }
  // This will not overflow or |cbb_buffer_reserve| would have failed.
  base->len += len;
  return 1;
}

int CBB_finish(CBB *cbb, uint8_t **out_data, size_t *out_len) {
  if (cbb->is_child) {
    OPENSSL_PUT_ERROR(CRYPTO, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  if (!CBB_flush(cbb)) {
    return 0;
  }

  if (cbb->u.base.can_resize && (out_data == NULL || out_len == NULL)) {
    // |out_data| and |out_len| can only be NULL if the CBB is fixed.
    return 0;
  }

  if (out_data != NULL) {
    *out_data = cbb->u.base.buf;
  }
  if (out_len != NULL) {
    *out_len = cbb->u.base.len;
  }
  cbb->u.base.buf = NULL;
  CBB_cleanup(cbb);
  return 1;
}

static struct cbb_buffer_st *cbb_get_base(CBB *cbb) {
  if (cbb->is_child) {
    return cbb->u.child.base;
  }
  return &cbb->u.base;
}

static void cbb_on_error(CBB *cbb) {
  // Due to C's lack of destructors and |CBB|'s auto-flushing API, a failing
  // |CBB|-taking function may leave a dangling pointer to a child |CBB|. As a
  // result, the convention is callers may not write to |CBB|s that have failed.
  // But, as a safety measure, we lock the |CBB| into an error state. Once the
  // error bit is set, |cbb->child| will not be read.
  //
  // TODO(davidben): This still isn't quite ideal. A |CBB| function *outside*
  // this file may originate an error while the |CBB| points to a local child.
  // In that case we don't set the error bit and are reliant on the error
  // convention. Perhaps we allow |CBB_cleanup| on child |CBB|s and make every
  // child's |CBB_cleanup| set the error bit if unflushed. That will be
  // convenient for C++ callers, but very tedious for C callers. So C callers
  // perhaps should get a |CBB_on_error| function that can be, less tediously,
  // stuck in a |goto err| block.
  cbb_get_base(cbb)->error = 1;

  // Clearing the pointer is not strictly necessary, but GCC's dangling pointer
  // warning does not know |cbb->child| will not be read once |error| is set
  // above.
  cbb->child = NULL;
}

// CBB_flush recurses and then writes out any pending length prefix. The
// current length of the underlying base is taken to be the length of the
// length-prefixed data.
int CBB_flush(CBB *cbb) {
  // If |base| has hit an error, the buffer is in an undefined state, so
  // fail all following calls. In particular, |cbb->child| may point to invalid
  // memory.
  struct cbb_buffer_st *base = cbb_get_base(cbb);
  if (base == NULL || base->error) {
    return 0;
  }

  if (cbb->child == NULL) {
    // Nothing to flush.
    return 1;
  }

  assert(cbb->child->is_child);
  struct cbb_child_st *child = &cbb->child->u.child;
  assert(child->base == base);
  size_t child_start = child->offset + child->pending_len_len;

  if (!CBB_flush(cbb->child) ||
      child_start < child->offset ||
      base->len < child_start) {
    goto err;
  }

  size_t len = base->len - child_start;

  if (child->pending_is_asn1) {
    // For ASN.1 we assume that we'll only need a single byte for the length.
    // If that turned out to be incorrect, we have to move the contents along
    // in order to make space.
    uint8_t len_len;
    uint8_t initial_length_byte;

    assert (child->pending_len_len == 1);

    if (len > 0xfffffffe) {
      OPENSSL_PUT_ERROR(CRYPTO, ERR_R_OVERFLOW);
      // Too large.
      goto err;
    } else if (len > 0xffffff) {
      len_len = 5;
      initial_length_byte = 0x80 | 4;
    } else if (len > 0xffff) {
      len_len = 4;
      initial_length_byte = 0x80 | 3;
    } else if (len > 0xff) {
      len_len = 3;
      initial_length_byte = 0x80 | 2;
    } else if (len > 0x7f) {
      len_len = 2;
      initial_length_byte = 0x80 | 1;
    } else {
      len_len = 1;
      initial_length_byte = (uint8_t)len;
      len = 0;
    }

    if (len_len != 1) {
      // We need to move the contents along in order to make space.
      size_t extra_bytes = len_len - 1;
      if (!cbb_buffer_add(base, NULL, extra_bytes)) {
        goto err;
      }
      OPENSSL_memmove(base->buf + child_start + extra_bytes,
                      base->buf + child_start, len);
    }
    base->buf[child->offset++] = initial_length_byte;
    child->pending_len_len = len_len - 1;
  }

  for (size_t i = child->pending_len_len - 1; i < child->pending_len_len; i--) {
    base->buf[child->offset + i] = (uint8_t)len;
    len >>= 8;
  }
  if (len != 0) {
    OPENSSL_PUT_ERROR(CRYPTO, ERR_R_OVERFLOW);
    goto err;
  }

  child->base = NULL;
  cbb->child = NULL;

  return 1;

err:
  cbb_on_error(cbb);
  return 0;
}

const uint8_t *CBB_data(const CBB *cbb) {
  assert(cbb->child == NULL);
  if (cbb->is_child) {
    return cbb->u.child.base->buf + cbb->u.child.offset +
           cbb->u.child.pending_len_len;
  }
  return cbb->u.base.buf;
}

size_t CBB_len(const CBB *cbb) {
  assert(cbb->child == NULL);
  if (cbb->is_child) {
    assert(cbb->u.child.offset + cbb->u.child.pending_len_len <=
           cbb->u.child.base->len);
    return cbb->u.child.base->len - cbb->u.child.offset -
           cbb->u.child.pending_len_len;
  }
  return cbb->u.base.len;
}

static int cbb_add_child(CBB *cbb, CBB *out_child, uint8_t len_len,
                         int is_asn1) {
  assert(cbb->child == NULL);
  assert(!is_asn1 || len_len == 1);
  struct cbb_buffer_st *base = cbb_get_base(cbb);
  size_t offset = base->len;

  // Reserve space for the length prefix.
  uint8_t *prefix_bytes;
  if (!cbb_buffer_add(base, &prefix_bytes, len_len)) {
    return 0;
  }
  OPENSSL_memset(prefix_bytes, 0, len_len);

  CBB_zero(out_child);
  out_child->is_child = 1;
  out_child->u.child.base = base;
  out_child->u.child.offset = offset;
  out_child->u.child.pending_len_len = len_len;
  out_child->u.child.pending_is_asn1 = is_asn1;
  cbb->child = out_child;
  return 1;
}

static int cbb_add_length_prefixed(CBB *cbb, CBB *out_contents,
                                   uint8_t len_len) {
  if (!CBB_flush(cbb)) {
    return 0;
  }

  return cbb_add_child(cbb, out_contents, len_len, /*is_asn1=*/0);
}

int CBB_add_u8_length_prefixed(CBB *cbb, CBB *out_contents) {
  return cbb_add_length_prefixed(cbb, out_contents, 1);
}

int CBB_add_u16_length_prefixed(CBB *cbb, CBB *out_contents) {
  return cbb_add_length_prefixed(cbb, out_contents, 2);
}

int CBB_add_u24_length_prefixed(CBB *cbb, CBB *out_contents) {
  return cbb_add_length_prefixed(cbb, out_contents, 3);
}

// add_base128_integer encodes |v| as a big-endian base-128 integer where the
// high bit of each byte indicates where there is more data. This is the
// encoding used in DER for both high tag number form and OID components.
static int add_base128_integer(CBB *cbb, uint64_t v) {
  unsigned len_len = 0;
  uint64_t copy = v;
  while (copy > 0) {
    len_len++;
    copy >>= 7;
  }
  if (len_len == 0) {
    len_len = 1;  // Zero is encoded with one byte.
  }
  for (unsigned i = len_len - 1; i < len_len; i--) {
    uint8_t byte = (v >> (7 * i)) & 0x7f;
    if (i != 0) {
      // The high bit denotes whether there is more data.
      byte |= 0x80;
    }
    if (!CBB_add_u8(cbb, byte)) {
      return 0;
    }
  }
  return 1;
}

int CBB_add_asn1(CBB *cbb, CBB *out_contents, CBS_ASN1_TAG tag) {
  if (!CBB_flush(cbb)) {
    return 0;
  }

  // Split the tag into leading bits and tag number.
  uint8_t tag_bits = (tag >> CBS_ASN1_TAG_SHIFT) & 0xe0;
  CBS_ASN1_TAG tag_number = tag & CBS_ASN1_TAG_NUMBER_MASK;
  if (tag_number >= 0x1f) {
    // Set all the bits in the tag number to signal high tag number form.
    if (!CBB_add_u8(cbb, tag_bits | 0x1f) ||
        !add_base128_integer(cbb, tag_number)) {
      return 0;
    }
  } else if (!CBB_add_u8(cbb, tag_bits | tag_number)) {
    return 0;
  }

  // Reserve one byte of length prefix. |CBB_flush| will finish it later.
  return cbb_add_child(cbb, out_contents, /*len_len=*/1, /*is_asn1=*/1);
}

int CBB_add_bytes(CBB *cbb, const uint8_t *data, size_t len) {
  uint8_t *out;
  if (!CBB_add_space(cbb, &out, len)) {
    return 0;
  }
  OPENSSL_memcpy(out, data, len);
  return 1;
}

int CBB_add_zeros(CBB *cbb, size_t len) {
  uint8_t *out;
  if (!CBB_add_space(cbb, &out, len)) {
    return 0;
  }
  OPENSSL_memset(out, 0, len);
  return 1;
}

int CBB_add_space(CBB *cbb, uint8_t **out_data, size_t len) {
  if (!CBB_flush(cbb) ||
      !cbb_buffer_add(cbb_get_base(cbb), out_data, len)) {
    return 0;
  }
  return 1;
}

int CBB_reserve(CBB *cbb, uint8_t **out_data, size_t len) {
  if (!CBB_flush(cbb) ||
      !cbb_buffer_reserve(cbb_get_base(cbb), out_data, len)) {
    return 0;
  }
  return 1;
}

int CBB_did_write(CBB *cbb, size_t len) {
  struct cbb_buffer_st *base = cbb_get_base(cbb);
  size_t newlen = base->len + len;
  if (cbb->child != NULL ||
      newlen < base->len ||
      newlen > base->cap) {
    return 0;
  }
  base->len = newlen;
  return 1;
}

static int cbb_add_u(CBB *cbb, uint64_t v, size_t len_len) {
  uint8_t *buf;
  if (!CBB_add_space(cbb, &buf, len_len)) {
    return 0;
  }

  for (size_t i = len_len - 1; i < len_len; i--) {
    buf[i] = v;
    v >>= 8;
  }

  // |v| must fit in |len_len| bytes.
  if (v != 0) {
    cbb_on_error(cbb);
    return 0;
  }

  return 1;
}

int CBB_add_u8(CBB *cbb, uint8_t value) {
  return cbb_add_u(cbb, value, 1);
}

int CBB_add_u16(CBB *cbb, uint16_t value) {
  return cbb_add_u(cbb, value, 2);
}

int CBB_add_u16le(CBB *cbb, uint16_t value) {
  return CBB_add_u16(cbb, CRYPTO_bswap2(value));
}

int CBB_add_u24(CBB *cbb, uint32_t value) {
  return cbb_add_u(cbb, value, 3);
}

int CBB_add_u32(CBB *cbb, uint32_t value) {
  return cbb_add_u(cbb, value, 4);
}

int CBB_add_u32le(CBB *cbb, uint32_t value) {
  return CBB_add_u32(cbb, CRYPTO_bswap4(value));
}

int CBB_add_u64(CBB *cbb, uint64_t value) {
  return cbb_add_u(cbb, value, 8);
}

int CBB_add_u64le(CBB *cbb, uint64_t value) {
  return CBB_add_u64(cbb, CRYPTO_bswap8(value));
}

void CBB_discard_child(CBB *cbb) {
  if (cbb->child == NULL) {
    return;
  }

  struct cbb_buffer_st *base = cbb_get_base(cbb);
  assert(cbb->child->is_child);
  base->len = cbb->child->u.child.offset;

  cbb->child->u.child.base = NULL;
  cbb->child = NULL;
}

int CBB_add_asn1_uint64(CBB *cbb, uint64_t value) {
  return CBB_add_asn1_uint64_with_tag(cbb, value, CBS_ASN1_INTEGER);
}

int CBB_add_asn1_uint64_with_tag(CBB *cbb, uint64_t value, CBS_ASN1_TAG tag) {
  CBB child;
  if (!CBB_add_asn1(cbb, &child, tag)) {
    goto err;
  }

  int started = 0;
  for (size_t i = 0; i < 8; i++) {
    uint8_t byte = (value >> 8 * (7 - i)) & 0xff;
    if (!started) {
      if (byte == 0) {
        // Don't encode leading zeros.
        continue;
      }
      // If the high bit is set, add a padding byte to make it
      // unsigned.
      if ((byte & 0x80) && !CBB_add_u8(&child, 0)) {
        goto err;
      }
      started = 1;
    }
    if (!CBB_add_u8(&child, byte)) {
      goto err;
    }
  }

  // 0 is encoded as a single 0, not the empty string.
  if (!started && !CBB_add_u8(&child, 0)) {
    goto err;
  }

  return CBB_flush(cbb);

err:
  cbb_on_error(cbb);
  return 0;
}

int CBB_add_asn1_int64(CBB *cbb, int64_t value) {
  return CBB_add_asn1_int64_with_tag(cbb, value, CBS_ASN1_INTEGER);
}

int CBB_add_asn1_int64_with_tag(CBB *cbb, int64_t value, CBS_ASN1_TAG tag) {
  if (value >= 0) {
    return CBB_add_asn1_uint64_with_tag(cbb, (uint64_t)value, tag);
  }

  uint8_t bytes[sizeof(int64_t)];
  memcpy(bytes, &value, sizeof(value));
  // Skip leading sign-extension bytes unless they are necessary.
#ifdef OPENSSL_BIG_ENDIAN
  int start = 0;
  while (start < 7 && (bytes[start] == 0xff && (bytes[start + 1] & 0x80))) {
    start++;
  }
#else
  int start = 7;
  while (start > 0 && (bytes[start] == 0xff && (bytes[start - 1] & 0x80))) {
    start--;
  }
#endif
  CBB child;
  if (!CBB_add_asn1(cbb, &child, tag)) {
    goto err;
  }
#ifdef OPENSSL_BIG_ENDIAN
  for (int i = start; i <= 7; i++) {
#else
  for (int i = start; i >= 0; i--) {
#endif
    if (!CBB_add_u8(&child, bytes[i])) {
      goto err;
    }
  }
  return CBB_flush(cbb);

err:
  cbb_on_error(cbb);
  return 0;
}

int CBB_add_asn1_octet_string(CBB *cbb, const uint8_t *data, size_t data_len) {
  CBB child;
  if (!CBB_add_asn1(cbb, &child, CBS_ASN1_OCTETSTRING) ||
      !CBB_add_bytes(&child, data, data_len) ||
      !CBB_flush(cbb)) {
    cbb_on_error(cbb);
    return 0;
  }

  return 1;
}

int CBB_add_asn1_bool(CBB *cbb, int value) {
  CBB child;
  if (!CBB_add_asn1(cbb, &child, CBS_ASN1_BOOLEAN) ||
      !CBB_add_u8(&child, value != 0 ? 0xff : 0) ||
      !CBB_flush(cbb)) {
    cbb_on_error(cbb);
    return 0;
  }

  return 1;
}

// parse_dotted_decimal parses one decimal component from |cbs|, where |cbs| is
// an OID literal, e.g., "1.2.840.113554.4.1.72585". It consumes both the
// component and the dot, so |cbs| may be passed into the function again for the
// next value.
static int parse_dotted_decimal(CBS *cbs, uint64_t *out) {
  if (!CBS_get_u64_decimal(cbs, out)) {
    return 0;
  }

  // The integer must have either ended at the end of the string, or a
  // non-terminal dot, which should be consumed. If the string ends with a dot,
  // this is not a valid OID string.
  uint8_t dot;
  return !CBS_get_u8(cbs, &dot) || (dot == '.' && CBS_len(cbs) > 0);
}

int CBB_add_asn1_oid_from_text(CBB *cbb, const char *text, size_t len) {
  if (!CBB_flush(cbb)) {
    return 0;
  }

  if (len > (size_t)SHRT_MAX) {
    OPENSSL_PUT_ERROR(CRYPTO, ERR_R_OVERFLOW);
    return 0;
  }

  CBS cbs;
  CBS_init(&cbs, (const uint8_t *)text, len);

  // OIDs must have at least two components.
  uint64_t a, b;
  if (!parse_dotted_decimal(&cbs, &a) ||
      !parse_dotted_decimal(&cbs, &b)) {
    return 0;
  }

  // The first component is encoded as 40 * |a| + |b|. This assumes that |a| is
  // 0, 1, or 2 and that, when it is 0 or 1, |b| is at most 39.
  if (a > 2 ||
      (a < 2 && b > 39) ||
      b > UINT64_MAX - 80 ||
      !add_base128_integer(cbb, 40u * a + b)) {
    return 0;
  }

  // The remaining components are encoded unmodified.
  while (CBS_len(&cbs) > 0) {
    if (!parse_dotted_decimal(&cbs, &a) ||
        !add_base128_integer(cbb, a)) {
      return 0;
    }
  }

  return 1;
}

static int compare_set_of_element(const void *a_ptr, const void *b_ptr) {
  // See X.690, section 11.6 for the ordering. They are sorted in ascending
  // order by their DER encoding.
  const CBS *a = a_ptr, *b = b_ptr;
  size_t a_len = CBS_len(a), b_len = CBS_len(b);
  size_t min_len = a_len < b_len ? a_len : b_len;
  int ret = OPENSSL_memcmp(CBS_data(a), CBS_data(b), min_len);
  if (ret != 0) {
    return ret;
  }
  if (a_len == b_len) {
    return 0;
  }
  // If one is a prefix of the other, the shorter one sorts first. (This is not
  // actually reachable. No DER encoding is a prefix of another DER encoding.)
  return a_len < b_len ? -1 : 1;
}

int CBB_flush_asn1_set_of(CBB *cbb) {
  if (!CBB_flush(cbb)) {
    return 0;
  }

  CBS cbs;
  size_t num_children = 0;
  CBS_init(&cbs, CBB_data(cbb), CBB_len(cbb));
  while (CBS_len(&cbs) != 0) {
    if (!CBS_get_any_asn1_element(&cbs, NULL, NULL, NULL)) {
      OPENSSL_PUT_ERROR(CRYPTO, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
      return 0;
    }
    num_children++;
  }

  if (num_children < 2) {
    return 1;  // Nothing to do. This is the common case for X.509.
  }

  // Parse out the children and sort. We alias them into a copy of so they
  // remain valid as we rewrite |cbb|.
  int ret = 0;
  size_t buf_len = CBB_len(cbb);
  uint8_t *buf = OPENSSL_memdup(CBB_data(cbb), buf_len);
  CBS *children = OPENSSL_calloc(num_children, sizeof(CBS));
  if (buf == NULL || children == NULL) {
    goto err;
  }
  CBS_init(&cbs, buf, buf_len);
  for (size_t i = 0; i < num_children; i++) {
    if (!CBS_get_any_asn1_element(&cbs, &children[i], NULL, NULL)) {
      goto err;
    }
  }
  qsort(children, num_children, sizeof(CBS), compare_set_of_element);

  // Write the contents back in the new order.
  uint8_t *out = (uint8_t *)CBB_data(cbb);
  size_t offset = 0;
  for (size_t i = 0; i < num_children; i++) {
    OPENSSL_memcpy(out + offset, CBS_data(&children[i]), CBS_len(&children[i]));
    offset += CBS_len(&children[i]);
  }
  assert(offset == buf_len);

  ret = 1;

err:
  OPENSSL_free(buf);
  OPENSSL_free(children);
  return ret;
}
