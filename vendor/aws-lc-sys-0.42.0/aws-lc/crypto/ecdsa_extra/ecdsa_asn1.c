// Copyright (c) 1998-2005 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ecdsa.h>

#include <limits.h>
#include <string.h>

#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/ec_key.h>
#include <openssl/mem.h>

#include "../bytestring/internal.h"
#include "../fipsmodule/ec/internal.h"
#include "../internal.h"


size_t ECDSA_size(const EC_KEY *key) {
  if (key == NULL) {
    return 0;
  }

  size_t group_order_size;
  const EC_GROUP *group = EC_KEY_get0_group(key);
  if (group == NULL) {
    return 0;
  }

  group_order_size = BN_num_bytes(EC_GROUP_get0_order(group));

  return ECDSA_SIG_max_len(group_order_size);
}

ECDSA_SIG *ECDSA_SIG_parse(CBS *cbs) {
  ECDSA_SIG *ret = ECDSA_SIG_new();
  if (ret == NULL) {
    return NULL;
  }
  CBS child;
  if (!CBS_get_asn1(cbs, &child, CBS_ASN1_SEQUENCE) ||
      !BN_parse_asn1_unsigned(&child, ret->r) ||
      !BN_parse_asn1_unsigned(&child, ret->s) ||
      CBS_len(&child) != 0) {
    OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_BAD_SIGNATURE);
    ECDSA_SIG_free(ret);
    return NULL;
  }
  return ret;
}

ECDSA_SIG *ECDSA_SIG_from_bytes(const uint8_t *in, size_t in_len) {
  CBS cbs;
  CBS_init(&cbs, in, in_len);
  ECDSA_SIG *ret = ECDSA_SIG_parse(&cbs);
  if (ret == NULL || CBS_len(&cbs) != 0) {
    OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_BAD_SIGNATURE);
    ECDSA_SIG_free(ret);
    return NULL;
  }
  return ret;
}

int ECDSA_SIG_marshal(CBB *cbb, const ECDSA_SIG *sig) {
  CBB child;
  if (!CBB_add_asn1(cbb, &child, CBS_ASN1_SEQUENCE) ||
      !BN_marshal_asn1(&child, sig->r) ||
      !BN_marshal_asn1(&child, sig->s) ||
      !CBB_flush(cbb)) {
    OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_ENCODE_ERROR);
    return 0;
  }
  return 1;
}

int ECDSA_SIG_to_bytes(uint8_t **out_bytes, size_t *out_len,
                       const ECDSA_SIG *sig) {
  CBB cbb;
  CBB_zero(&cbb);
  if (!CBB_init(&cbb, 0) ||
      !ECDSA_SIG_marshal(&cbb, sig) ||
      !CBB_finish(&cbb, out_bytes, out_len)) {
    OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_ENCODE_ERROR);
    CBB_cleanup(&cbb);
    return 0;
  }
  return 1;
}

// der_len_len returns the number of bytes needed to represent a length of |len|
// in DER.
static size_t der_len_len(size_t len) {
  if (len < 0x80) {
    return 1;
  }
  size_t ret = 1;
  while (len > 0) {
    ret++;
    len >>= 8;
  }
  return ret;
}

size_t ECDSA_SIG_max_len(size_t order_len) {
  // Compute the maximum length of an |order_len| byte integer. Defensively
  // assume that the leading 0x00 is included.
  size_t integer_len = 1 /* tag */ + der_len_len(order_len + 1) + 1 + order_len;
  if (integer_len < order_len) {
    return 0;
  }
  // An ECDSA signature is two INTEGERs.
  size_t value_len = 2 * integer_len;
  if (value_len < integer_len) {
    return 0;
  }
  // Add the header.
  size_t ret = 1 /* tag */ + der_len_len(value_len) + value_len;
  if (ret < value_len) {
    return 0;
  }
  return ret;
}

ECDSA_SIG *d2i_ECDSA_SIG(ECDSA_SIG **out, const uint8_t **inp, long len) {
  if (len < 0) {
    return NULL;
  }
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  ECDSA_SIG *ret = ECDSA_SIG_parse(&cbs);
  if (ret == NULL) {
    return NULL;
  }
  if (out != NULL) {
    ECDSA_SIG_free(*out);
    *out = ret;
  }
  *inp = CBS_data(&cbs);
  return ret;
}

int i2d_ECDSA_SIG(const ECDSA_SIG *sig, uint8_t **outp) {
  CBB cbb;
  if (!CBB_init(&cbb, 0) ||
      !ECDSA_SIG_marshal(&cbb, sig)) {
    CBB_cleanup(&cbb);
    return -1;
  }
  return CBB_finish_i2d(&cbb, outp);
}
