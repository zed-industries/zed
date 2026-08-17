// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 2000
// Copyright (c) 2000-2005 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/dsa.h>

#include <assert.h>

#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "internal.h"
#include "../bytestring/internal.h"
#include "../crypto/internal.h"


// This function is in dsa_asn1.c rather than dsa.c because it is reachable from
// |EVP_PKEY| parsers. This makes it easier for the static linker to drop most
// of the DSA implementation.
int dsa_check_key(const DSA *dsa) {
  if (!dsa->p || !dsa->q || !dsa->g) {
    OPENSSL_PUT_ERROR(DSA, DSA_R_MISSING_PARAMETERS);
    return 0;
  }

  // Fully checking for invalid DSA groups is expensive, so security and
  // correctness of the signature scheme depend on how |dsa| was computed. I.e.
  // we leave "assurance of domain parameter validity" from FIPS 186-4 to the
  // caller. However, we check bounds on all values to avoid DoS vectors even
  // when domain parameters are invalid. In particular, signing will infinite
  // loop if |g| is zero.
  if (BN_is_negative(dsa->p) || BN_is_negative(dsa->q) || BN_is_zero(dsa->p) ||
      BN_is_zero(dsa->q) || !BN_is_odd(dsa->p) || !BN_is_odd(dsa->q) ||
      // |q| must be a prime divisor of |p - 1|, which implies |q < p|.
      BN_cmp(dsa->q, dsa->p) >= 0 ||
      // |g| is in the multiplicative group of |p|.
      BN_is_negative(dsa->g) || BN_is_zero(dsa->g) ||
      BN_cmp(dsa->g, dsa->p) >= 0) {
    OPENSSL_PUT_ERROR(DSA, DSA_R_INVALID_PARAMETERS);
    return 0;
  }

  // FIPS 186-4 allows only three different sizes for q.
  unsigned q_bits = BN_num_bits(dsa->q);
  if (q_bits != 160 && q_bits != 224 && q_bits != 256) {
    OPENSSL_PUT_ERROR(DSA, DSA_R_BAD_Q_VALUE);
    return 0;
  }

  // Bound |dsa->p| to avoid a DoS vector. Note this limit is much larger than
  // the one in FIPS 186-4, which only allows L = 1024, 2048, and 3072.
  if (BN_num_bits(dsa->p) > OPENSSL_DSA_MAX_MODULUS_BITS) {
    OPENSSL_PUT_ERROR(DSA, DSA_R_MODULUS_TOO_LARGE);
    return 0;
  }

  if (dsa->pub_key != NULL) {
    // The public key is also in the multiplicative group of |p|.
    if (BN_is_negative(dsa->pub_key) || BN_is_zero(dsa->pub_key) ||
        BN_cmp(dsa->pub_key, dsa->p) >= 0) {
      OPENSSL_PUT_ERROR(DSA, DSA_R_INVALID_PARAMETERS);
      return 0;
    }
  }

  if (dsa->priv_key != NULL) {
    // The private key is a non-zero element of the scalar field, determined by
    // |q|.
    if (BN_is_negative(dsa->priv_key) ||
        constant_time_declassify_int(BN_is_zero(dsa->priv_key)) ||
        constant_time_declassify_int(BN_cmp(dsa->priv_key, dsa->q) >= 0)) {
      OPENSSL_PUT_ERROR(DSA, DSA_R_INVALID_PARAMETERS);
      return 0;
    }
  }

  return 1;
}

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
    // A DSA object may be missing some components.
    OPENSSL_PUT_ERROR(DSA, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  return BN_marshal_asn1(cbb, bn);
}

DSA_SIG *DSA_SIG_parse(CBS *cbs) {
  DSA_SIG *ret = DSA_SIG_new();
  if (ret == NULL) {
    return NULL;
  }
  CBS child;
  if (!CBS_get_asn1(cbs, &child, CBS_ASN1_SEQUENCE) ||
      !parse_integer(&child, &ret->r) ||
      !parse_integer(&child, &ret->s) ||
      CBS_len(&child) != 0) {
    OPENSSL_PUT_ERROR(DSA, DSA_R_DECODE_ERROR);
    DSA_SIG_free(ret);
    return NULL;
  }
  return ret;
}

int DSA_SIG_marshal(CBB *cbb, const DSA_SIG *sig) {
  CBB child;
  if (!CBB_add_asn1(cbb, &child, CBS_ASN1_SEQUENCE) ||
      !marshal_integer(&child, sig->r) ||
      !marshal_integer(&child, sig->s) ||
      !CBB_flush(cbb)) {
    OPENSSL_PUT_ERROR(DSA, DSA_R_ENCODE_ERROR);
    return 0;
  }
  return 1;
}

DSA *DSA_parse_public_key(CBS *cbs) {
  DSA *ret = DSA_new();
  if (ret == NULL) {
    return NULL;
  }
  CBS child;
  if (!CBS_get_asn1(cbs, &child, CBS_ASN1_SEQUENCE) ||
      !parse_integer(&child, &ret->pub_key) ||
      !parse_integer(&child, &ret->p) ||
      !parse_integer(&child, &ret->q) ||
      !parse_integer(&child, &ret->g) ||
      CBS_len(&child) != 0) {
    OPENSSL_PUT_ERROR(DSA, DSA_R_DECODE_ERROR);
    goto err;
  }
  if (!dsa_check_key(ret)) {
    goto err;
  }
  return ret;

err:
  DSA_free(ret);
  return NULL;
}

int DSA_marshal_public_key(CBB *cbb, const DSA *dsa) {
  CBB child;
  if (!CBB_add_asn1(cbb, &child, CBS_ASN1_SEQUENCE) ||
      !marshal_integer(&child, dsa->pub_key) ||
      !marshal_integer(&child, dsa->p) ||
      !marshal_integer(&child, dsa->q) ||
      !marshal_integer(&child, dsa->g) ||
      !CBB_flush(cbb)) {
    OPENSSL_PUT_ERROR(DSA, DSA_R_ENCODE_ERROR);
    return 0;
  }
  return 1;
}

DSA *DSA_parse_parameters(CBS *cbs) {
  DSA *ret = DSA_new();
  if (ret == NULL) {
    return NULL;
  }
  CBS child;
  if (!CBS_get_asn1(cbs, &child, CBS_ASN1_SEQUENCE) ||
      !parse_integer(&child, &ret->p) ||
      !parse_integer(&child, &ret->q) ||
      !parse_integer(&child, &ret->g) ||
      CBS_len(&child) != 0) {
    OPENSSL_PUT_ERROR(DSA, DSA_R_DECODE_ERROR);
    goto err;
  }
  if (!dsa_check_key(ret)) {
    goto err;
  }
  return ret;

err:
  DSA_free(ret);
  return NULL;
}

int DSA_marshal_parameters(CBB *cbb, const DSA *dsa) {
  CBB child;
  if (!CBB_add_asn1(cbb, &child, CBS_ASN1_SEQUENCE) ||
      !marshal_integer(&child, dsa->p) ||
      !marshal_integer(&child, dsa->q) ||
      !marshal_integer(&child, dsa->g) ||
      !CBB_flush(cbb)) {
    OPENSSL_PUT_ERROR(DSA, DSA_R_ENCODE_ERROR);
    return 0;
  }
  return 1;
}

DSA *DSA_parse_private_key(CBS *cbs) {
  DSA *ret = DSA_new();
  if (ret == NULL) {
    return NULL;
  }

  CBS child;
  uint64_t version;
  if (!CBS_get_asn1(cbs, &child, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1_uint64(&child, &version)) {
    OPENSSL_PUT_ERROR(DSA, DSA_R_DECODE_ERROR);
    goto err;
  }

  if (version != 0) {
    OPENSSL_PUT_ERROR(DSA, DSA_R_BAD_VERSION);
    goto err;
  }

  if (!parse_integer(&child, &ret->p) ||
      !parse_integer(&child, &ret->q) ||
      !parse_integer(&child, &ret->g) ||
      !parse_integer(&child, &ret->pub_key) ||
      !parse_integer(&child, &ret->priv_key) ||
      CBS_len(&child) != 0) {
    OPENSSL_PUT_ERROR(DSA, DSA_R_DECODE_ERROR);
    goto err;
  }
  if (!dsa_check_key(ret)) {
    goto err;
  }

  return ret;

err:
  DSA_free(ret);
  return NULL;
}

int DSA_marshal_private_key(CBB *cbb, const DSA *dsa) {
  CBB child;
  if (!CBB_add_asn1(cbb, &child, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&child, 0 /* version */) ||
      !marshal_integer(&child, dsa->p) ||
      !marshal_integer(&child, dsa->q) ||
      !marshal_integer(&child, dsa->g) ||
      !marshal_integer(&child, dsa->pub_key) ||
      !marshal_integer(&child, dsa->priv_key) ||
      !CBB_flush(cbb)) {
    OPENSSL_PUT_ERROR(DSA, DSA_R_ENCODE_ERROR);
    return 0;
  }
  return 1;
}

DSA_SIG *d2i_DSA_SIG(DSA_SIG **out_sig, const uint8_t **inp, long len) {
  if (len < 0) {
    return NULL;
  }
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  DSA_SIG *ret = DSA_SIG_parse(&cbs);
  if (ret == NULL) {
    return NULL;
  }
  if (out_sig != NULL) {
    DSA_SIG_free(*out_sig);
    *out_sig = ret;
  }
  *inp = CBS_data(&cbs);
  return ret;
}

int i2d_DSA_SIG(const DSA_SIG *in, uint8_t **outp) {
  CBB cbb;
  if (!CBB_init(&cbb, 0) ||
      !DSA_SIG_marshal(&cbb, in)) {
    CBB_cleanup(&cbb);
    return -1;
  }
  return CBB_finish_i2d(&cbb, outp);
}

DSA *d2i_DSAPublicKey(DSA **out, const uint8_t **inp, long len) {
  if (len < 0) {
    return NULL;
  }
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  DSA *ret = DSA_parse_public_key(&cbs);
  if (ret == NULL) {
    return NULL;
  }
  if (out != NULL) {
    DSA_free(*out);
    *out = ret;
  }
  *inp = CBS_data(&cbs);
  return ret;
}

int i2d_DSAPublicKey(const DSA *in, uint8_t **outp) {
  CBB cbb;
  if (!CBB_init(&cbb, 0) ||
      !DSA_marshal_public_key(&cbb, in)) {
    CBB_cleanup(&cbb);
    return -1;
  }
  return CBB_finish_i2d(&cbb, outp);
}

DSA *d2i_DSAPrivateKey(DSA **out, const uint8_t **inp, long len) {
  if (len < 0) {
    return NULL;
  }
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  DSA *ret = DSA_parse_private_key(&cbs);
  if (ret == NULL) {
    return NULL;
  }
  if (out != NULL) {
    DSA_free(*out);
    *out = ret;
  }
  *inp = CBS_data(&cbs);
  return ret;
}

int i2d_DSAPrivateKey(const DSA *in, uint8_t **outp) {
  CBB cbb;
  if (!CBB_init(&cbb, 0) ||
      !DSA_marshal_private_key(&cbb, in)) {
    CBB_cleanup(&cbb);
    return -1;
  }
  return CBB_finish_i2d(&cbb, outp);
}

DSA *d2i_DSAparams(DSA **out, const uint8_t **inp, long len) {
  if (len < 0) {
    return NULL;
  }
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  DSA *ret = DSA_parse_parameters(&cbs);
  if (ret == NULL) {
    return NULL;
  }
  if (out != NULL) {
    DSA_free(*out);
    *out = ret;
  }
  *inp = CBS_data(&cbs);
  return ret;
}

int i2d_DSAparams(const DSA *in, uint8_t **outp) {
  CBB cbb;
  if (!CBB_init(&cbb, 0) ||
      !DSA_marshal_parameters(&cbb, in)) {
    CBB_cleanup(&cbb);
    return -1;
  }
  return CBB_finish_i2d(&cbb, outp);
}
