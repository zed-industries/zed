// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 2000.
// Copyright (c) 2000-2005 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/dh.h>

#include <assert.h>
#include <limits.h>

#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>

#include "../bytestring/internal.h"
#include "../fipsmodule/dh/internal.h"


static int parse_integer(CBS *cbs, BIGNUM **out) {
  assert(*out == NULL);
  *out = BN_new();
  if (*out == NULL) {
    return 0;
  }
  return BN_parse_asn1_unsigned(cbs, *out);
}

static int marshal_integer(CBB *cbb, BIGNUM *bn) {
  if (bn == NULL) {
    // A DH object may be missing some components.
    OPENSSL_PUT_ERROR(DH, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  return BN_marshal_asn1(cbb, bn);
}

DH *DH_parse_parameters(CBS *cbs) {
  DH *ret = DH_new();
  if (ret == NULL) {
    return NULL;
  }

  CBS child;
  if (!CBS_get_asn1(cbs, &child, CBS_ASN1_SEQUENCE) ||
      !parse_integer(&child, &ret->p) ||
      !parse_integer(&child, &ret->g)) {
    goto err;
  }

  uint64_t priv_length;
  if (CBS_len(&child) != 0) {
    if (!CBS_get_asn1_uint64(&child, &priv_length) ||
        priv_length > UINT_MAX) {
      goto err;
    }
    ret->priv_length = (unsigned)priv_length;
  }

  if (CBS_len(&child) != 0) {
    goto err;
  }

  if (!dh_check_params_fast(ret)) {
    goto err;
  }

  return ret;

err:
  OPENSSL_PUT_ERROR(DH, DH_R_DECODE_ERROR);
  DH_free(ret);
  return NULL;
}

int DH_marshal_parameters(CBB *cbb, const DH *dh) {
  CBB child;
  if (!CBB_add_asn1(cbb, &child, CBS_ASN1_SEQUENCE) ||
      !marshal_integer(&child, dh->p) ||
      !marshal_integer(&child, dh->g) ||
      (dh->priv_length != 0 &&
       !CBB_add_asn1_uint64(&child, dh->priv_length)) ||
      !CBB_flush(cbb)) {
    OPENSSL_PUT_ERROR(DH, DH_R_ENCODE_ERROR);
    return 0;
  }
  return 1;
}

DH *d2i_DHparams(DH **out, const uint8_t **inp, long len) {
  if (len < 0) {
    return NULL;
  }
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  DH *ret = DH_parse_parameters(&cbs);
  if (ret == NULL) {
    return NULL;
  }
  if (out != NULL) {
    DH_free(*out);
    *out = ret;
  }
  *inp = CBS_data(&cbs);
  return ret;
}

int i2d_DHparams(const DH *in, uint8_t **outp) {
  CBB cbb;
  if (!CBB_init(&cbb, 0) ||
      !DH_marshal_parameters(&cbb, in)) {
    CBB_cleanup(&cbb);
    return -1;
  }
  return CBB_finish_i2d(&cbb, outp);
}
