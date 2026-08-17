// Originally written by Bodo Moeller for the OpenSSL project.
// Copyright (c) 1998-2005 The OpenSSL Project.  All rights reserved.
// Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
//
// The elliptic curve binary polynomial software is originally written by
// Sheueling Chang Shantz and Douglas Stebila of Sun Microsystems
// Laboratories.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ec_key.h>
#include <openssl/evp.h>

#include <string.h>

#include <openssl/ec.h>
#include <openssl/ecdsa.h>
#include <openssl/engine.h>
#include <openssl/err.h>
#include <openssl/ex_data.h>
#include <openssl/mem.h>
#include <openssl/thread.h>

#include "internal.h"
#include "../delocate.h"
#include "../service_indicator/internal.h"
#include "../../internal.h"


DEFINE_STATIC_EX_DATA_CLASS(g_ec_ex_data_class)

static EC_WRAPPED_SCALAR *ec_wrapped_scalar_new(const EC_GROUP *group) {
  EC_WRAPPED_SCALAR *wrapped = OPENSSL_zalloc(sizeof(EC_WRAPPED_SCALAR));
  if (wrapped == NULL) {
    return NULL;
  }

  wrapped->bignum.d = wrapped->scalar.words;
  wrapped->bignum.width = group->order.N.width;
  wrapped->bignum.dmax = group->order.N.width;
  wrapped->bignum.flags = BN_FLG_STATIC_DATA;
  return wrapped;
}

static void ec_wrapped_scalar_free(EC_WRAPPED_SCALAR *scalar) {
  OPENSSL_free(scalar);
}

EC_KEY *EC_KEY_new(void) { return EC_KEY_new_method(NULL); }

EC_KEY *EC_KEY_new_method(const ENGINE *engine) {
  EC_KEY *ret = OPENSSL_zalloc(sizeof(EC_KEY));
  if (ret == NULL) {
    return NULL;
  }

  if (engine) {
    // Cast away const
    ret->eckey_method = (EC_KEY_METHOD *) ENGINE_get_EC(engine);
  }

  if(ret->eckey_method == NULL) {
    ret->eckey_method = EC_KEY_get_default_method();
  }

  ret->conv_form = POINT_CONVERSION_UNCOMPRESSED;
  ret->references = 1;
  ret->group_decoded_from_explicit_params = 0;

  CRYPTO_new_ex_data(&ret->ex_data);

  if (ret->eckey_method && ret->eckey_method->init && !ret->eckey_method->init(ret)) {
    CRYPTO_free_ex_data(g_ec_ex_data_class_bss_get(), ret, &ret->ex_data);
    OPENSSL_free(ret);
    return NULL;
  }

  return ret;
}

EC_KEY *EC_KEY_new_by_curve_name(int nid) {
  EC_KEY *ret = EC_KEY_new();
  if (ret == NULL) {
    return NULL;
  }
  ret->group = EC_GROUP_new_by_curve_name(nid);
  if (ret->group == NULL) {
    EC_KEY_free(ret);
    return NULL;
  }
  return ret;
}

void EC_KEY_free(EC_KEY *r) {
  if (r == NULL) {
    return;
  }

  if (!CRYPTO_refcount_dec_and_test_zero(&r->references)) {
    return;
  }

  if (r->eckey_method && r->eckey_method->finish) {
    r->eckey_method->finish(r);
  }

  CRYPTO_free_ex_data(g_ec_ex_data_class_bss_get(), r, &r->ex_data);

  EC_GROUP_free(r->group);
  EC_POINT_free(r->pub_key);
  ec_wrapped_scalar_free(r->priv_key);

  OPENSSL_free(r);
}

EC_KEY *EC_KEY_dup(const EC_KEY *src) {
  if (src == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  EC_KEY *ret = EC_KEY_new();
  if (ret == NULL) {
    return NULL;
  }

  if ((src->group != NULL &&
       !EC_KEY_set_group(ret, src->group)) ||
      (src->pub_key != NULL &&
       !EC_KEY_set_public_key(ret, src->pub_key)) ||
      (src->priv_key != NULL &&
       !EC_KEY_set_private_key(ret, EC_KEY_get0_private_key(src)))) {
    EC_KEY_free(ret);
    return NULL;
  }

  ret->enc_flag = src->enc_flag;
  ret->conv_form = src->conv_form;
  ret->group_decoded_from_explicit_params = src->group_decoded_from_explicit_params;
  return ret;
}

int EC_KEY_up_ref(EC_KEY *r) {
  CRYPTO_refcount_inc(&r->references);
  return 1;
}

int EC_KEY_is_opaque(const EC_KEY *key) {
  return key->eckey_method && (key->eckey_method->flags & ECDSA_FLAG_OPAQUE);
}

const EC_GROUP *EC_KEY_get0_group(const EC_KEY *key) { return key->group; }

int EC_KEY_set_group(EC_KEY *key, const EC_GROUP *group) {
  // If |key| already has a group, it is an error to switch to another one.
  if (key->group != NULL) {
    if (EC_GROUP_cmp(key->group, group, NULL) != 0) {
      OPENSSL_PUT_ERROR(EC, EC_R_GROUP_MISMATCH);
      return 0;
    }
    return 1;
  }

  assert(key->priv_key == NULL);
  assert(key->pub_key == NULL);

  EC_GROUP_free(key->group);
  key->group = EC_GROUP_dup(group);
  return key->group != NULL;
}

const BIGNUM *EC_KEY_get0_private_key(const EC_KEY *key) {
  return key->priv_key != NULL ? &key->priv_key->bignum : NULL;
}

int EC_KEY_set_private_key(EC_KEY *key, const BIGNUM *priv_key) {
  if (key->group == NULL) {
    OPENSSL_PUT_ERROR(EC, EC_R_MISSING_PARAMETERS);
    return 0;
  }

  EC_WRAPPED_SCALAR *scalar = ec_wrapped_scalar_new(key->group);
  if (scalar == NULL) {
    return 0;
  }
  if (!ec_bignum_to_scalar(key->group, &scalar->scalar, priv_key) ||
      // Zero is not a valid private key, so it is safe to leak the result of
      // this comparison.
      constant_time_declassify_int(
          ec_scalar_is_zero(key->group, &scalar->scalar))) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_PRIVATE_KEY);
    ec_wrapped_scalar_free(scalar);
    return 0;
  }
  ec_wrapped_scalar_free(key->priv_key);
  key->priv_key = scalar;
  return 1;
}

const EC_POINT *EC_KEY_get0_public_key(const EC_KEY *key) {
  return key->pub_key;
}

int EC_KEY_set_public_key(EC_KEY *key, const EC_POINT *pub_key) {
  if (key->group == NULL) {
    OPENSSL_PUT_ERROR(EC, EC_R_MISSING_PARAMETERS);
    return 0;
  }

  if (pub_key != NULL && EC_POINT_is_at_infinity(key->group, pub_key)) {
    OPENSSL_PUT_ERROR(EC, EC_R_POINT_AT_INFINITY);
    return 0;
  }

  if (pub_key != NULL && EC_GROUP_cmp(key->group, pub_key->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_GROUP_MISMATCH);
    return 0;
  }

  EC_POINT_free(key->pub_key);
  key->pub_key = EC_POINT_dup(pub_key, key->group);
  return (key->pub_key == NULL) ? 0 : 1;
}

unsigned int EC_KEY_get_enc_flags(const EC_KEY *key) { return key->enc_flag; }

void EC_KEY_set_enc_flags(EC_KEY *key, unsigned int flags) {
  key->enc_flag = flags;
}

point_conversion_form_t EC_KEY_get_conv_form(const EC_KEY *key) {
  return key->conv_form;
}

void EC_KEY_set_conv_form(EC_KEY *key, point_conversion_form_t cform) {
  if (key == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return;
  }
  key->conv_form = cform;
  if (key->group != NULL && key->group->mutable_ec_group) {
    key->group->conv_form = cform;
  }
}

// ec_key_gen_pct computes the PCT: SP 800-56Arev3 Section 5.6.2.1.4 option ‘b’.
// Should only be used for NIST P-curves.
static int ec_key_gen_pct(const EC_KEY *key) {

  if (key->priv_key != NULL) {

    EC_SCALAR *priv_key_scalar = &key->priv_key->scalar;

    // In theory, we could just flip a bit in the existing private key. The
    // test below is supposed to hard abort and the code-path is only active
    // during testing the break KAT framework. But prefer to keep it local to
    // the PCT to isolate errors.
    EC_SCALAR priv_key_scalar_bit_flipped = {{0}};
    if (boringssl_fips_break_test("EC_PWCT")) {
      OPENSSL_memcpy(&priv_key_scalar_bit_flipped, priv_key_scalar,
        sizeof(priv_key_scalar->words));
      priv_key_scalar_bit_flipped.words[0] ^= 0x1;
      priv_key_scalar = &priv_key_scalar_bit_flipped;
    }

    EC_JACOBIAN point;
    if (!ec_point_mul_scalar_base(key->group, &point,
          priv_key_scalar)) {
      OPENSSL_PUT_ERROR(EC, ERR_R_EC_LIB);
      return 0;
    }
  
    // Leaking this comparison only leaks whether |key|'s public key was
    // correct.
    //
    // |ec_GFp_simple_points_equal| is quite expensive (relatively); The current
    // comparison function does a non-negligible amount of field arithmetic If
    // we were sure that points aren't in jacobian, but in affine
    // representation, then the comparison reduces to fast byte array equality.
    if (!constant_time_declassify_int(ec_GFp_simple_points_equal(
          key->group, &point, &key->pub_key->raw))) {
      OPENSSL_PUT_ERROR(EC, EC_R_INVALID_PRIVATE_KEY);
      return 0;
    }
  }

  return 1;
}

int EC_KEY_check_key(const EC_KEY *eckey) {
  if (!eckey || !eckey->group || !eckey->pub_key) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  if (EC_POINT_is_at_infinity(eckey->group, eckey->pub_key)) {
    OPENSSL_PUT_ERROR(EC, EC_R_POINT_AT_INFINITY);
    return 0;
  }

  // Test whether the public key is on the elliptic curve.
  if (!EC_POINT_is_on_curve(eckey->group, eckey->pub_key, NULL)) {
    OPENSSL_PUT_ERROR(EC, EC_R_POINT_IS_NOT_ON_CURVE);
    return 0;
  }

  // Many code-paths end up in |EC_KEY_check_key|. For example, ECDH operations.
  // Technically, the PCT is not needed for ECDH in non-FIPS. However, other
  // APIs use this function for validations e.g. the EVP private key parsing (to
  // validate that public key is indeed generate from the private key) and there
  // is currently no way to distinguish the code-paths at this level. This could
  // be optimized.
  if (!ec_key_gen_pct(eckey)) {
    return 0;
  }

  return 1;
}

int EC_KEY_check_fips(const EC_KEY *key) {

  int ret = 0;

  // Nothing obvious will falsely increment the service indicator. But lock to
  // be safe.
  FIPS_service_indicator_lock_state();

  if (EC_KEY_is_opaque(key)) {
    // Opaque keys can't be checked.
    OPENSSL_PUT_ERROR(EC, EC_R_PUBLIC_KEY_VALIDATION_FAILED);
    goto end;
  }

  if (!EC_KEY_check_key(key)) {
    goto end;
  }

  // Check that the coordinates are within the range [0,p-1], when the (raw)
  // point is affine; i.e. Z=1.
  // This is the case when validating a received public key.
  // Note: The check for x and y being negative seems superfluous since
  // ec_felem_to_bignum() calls BN_bin2bn() which sets the `neg` flag to 0.
  EC_POINT *pub_key = key->pub_key;
  EC_GROUP *group = key->pub_key->group;
  if(ec_felem_equal(group, ec_felem_one(group), &pub_key->raw.Z)) {
    BIGNUM *x = BN_new();
    BIGNUM *y = BN_new();
    int check_ret = 1;
    if (group->meth->felem_to_bytes == NULL) {
      OPENSSL_PUT_ERROR(EC, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
      check_ret = 0;
    } else if (!ec_felem_to_bignum(group, x, &pub_key->raw.X) ||
               !ec_felem_to_bignum(group, y, &pub_key->raw.Y)) {
      // Error already written to error queue by |bn_wexpand|.
      check_ret = 0;
    } else if (BN_is_negative(x) || BN_is_negative(y) ||
               BN_cmp(x, &group->field.N) >= 0 ||
               BN_cmp(y, &group->field.N) >= 0) {
      OPENSSL_PUT_ERROR(EC, EC_R_COORDINATES_OUT_OF_RANGE);
      check_ret = 0;
    }
    BN_free(x);
    BN_free(y);
    if (check_ret == 0) {
      goto end;
    }
  }

  ret = 1;
end:
  FIPS_service_indicator_unlock_state();
  if(ret){
    EC_KEY_keygen_verify_service_indicator(key);
  }
  return ret;
}

int EC_KEY_set_public_key_affine_coordinates(EC_KEY *key, const BIGNUM *x,
                                             const BIGNUM *y) {
  EC_POINT *point = NULL;
  int ok = 0;

  if (!key || !key->group || !x || !y) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  point = EC_POINT_new(key->group);
  if (point == NULL ||
      !EC_POINT_set_affine_coordinates_GFp(key->group, point, x, y, NULL) ||
      !EC_KEY_set_public_key(key, point) ||
      !EC_KEY_check_key(key)) {
    goto err;
  }

  ok = 1;

err:
  EC_POINT_free(point);
  return ok;
}

size_t EC_KEY_key2buf(const EC_KEY *key, point_conversion_form_t form,
                      unsigned char **out_buf, BN_CTX *ctx) {
  if (key == NULL || key->pub_key == NULL || key->group == NULL) {
    return 0;
  }

  const size_t len =
      EC_POINT_point2oct(key->group, key->pub_key, form, NULL, 0, ctx);
  if (len == 0) {
    return 0;
  }

  uint8_t *buf = OPENSSL_malloc(len);
  if (buf == NULL) {
    return 0;
  }

  if (EC_POINT_point2oct(key->group, key->pub_key, form, buf, len, ctx) !=
      len) {
    OPENSSL_free(buf);
    return 0;
  }

  *out_buf = buf;
  return len;
}

int EC_KEY_generate_key(EC_KEY *key) {
  if (key == NULL || key->group == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  // Check that the group order is FIPS compliant (FIPS 186-4 B.4.2).
  if (EC_GROUP_order_bits(key->group) < 160) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_GROUP_ORDER);
    return 0;
  }

  static const uint8_t kDefaultAdditionalData[32] = {0};
  EC_WRAPPED_SCALAR *priv_key = ec_wrapped_scalar_new(key->group);
  EC_POINT *pub_key = EC_POINT_new(key->group);
  if (priv_key == NULL || pub_key == NULL ||
      // Generate the private key by testing candidates (FIPS 186-4 B.4.2).
      !ec_random_nonzero_scalar(key->group, &priv_key->scalar,
                                kDefaultAdditionalData) ||
      !ec_point_mul_scalar_base(key->group, &pub_key->raw, &priv_key->scalar)) {
    EC_POINT_free(pub_key);
    ec_wrapped_scalar_free(priv_key);
    return 0;
  }

  // The public key is derived from the private key, but it is public.
  //
  // TODO(crbug.com/boringssl/677): This isn't quite right. While |pub_key|
  // represents a public point, it is still in Jacobian form and the exact
  // Jacobian representation is secret. We need to make it affine first. See
  // discussion in the bug.
  CONSTTIME_DECLASSIFY(&pub_key->raw, sizeof(pub_key->raw));

  ec_wrapped_scalar_free(key->priv_key);
  key->priv_key = priv_key;
  EC_POINT_free(key->pub_key);
  key->pub_key = pub_key;
  return 1;
}

int EC_KEY_generate_key_fips(EC_KEY *eckey) {
  int ret = 0;

  // At least |EC_KEY_check_fips| will certainly update the service indicator.
  // Hence, must lock here.
  FIPS_service_indicator_lock_state();

  boringssl_ensure_ecc_self_test();

  if (EC_KEY_generate_key(eckey) && EC_KEY_check_fips(eckey)) {
    ret = 1;
  }

  FIPS_service_indicator_unlock_state();
  if (ret) {
    EC_KEY_keygen_verify_service_indicator(eckey);
    return 1;
  }

  EC_POINT_free(eckey->pub_key);
  ec_wrapped_scalar_free(eckey->priv_key);
  eckey->pub_key = NULL;
  eckey->priv_key = NULL;

#if defined(AWSLC_FIPS)
  AWS_LC_FIPS_failure("EC keygen checks failed");
#endif
  return 0;
}

int EC_KEY_get_ex_new_index(long argl, void *argp, CRYPTO_EX_unused *unused,
                            CRYPTO_EX_dup *dup_unused,
                            CRYPTO_EX_free *free_func) {
  int index;
  if (!CRYPTO_get_ex_new_index(g_ec_ex_data_class_bss_get(), &index, argl, argp,
                               free_func)) {
    return -1;
  }
  return index;
}

int EC_KEY_set_ex_data(EC_KEY *d, int idx, void *arg) {
  return CRYPTO_set_ex_data(&d->ex_data, idx, arg);
}

void *EC_KEY_get_ex_data(const EC_KEY *d, int idx) {
  return CRYPTO_get_ex_data(&d->ex_data, idx);
}

void EC_KEY_set_asn1_flag(EC_KEY *key, int flag) {}

DEFINE_METHOD_FUNCTION(EC_KEY_METHOD, EC_KEY_OpenSSL) {
  OPENSSL_memset(out, 0, sizeof(EC_KEY_METHOD));
}

const EC_KEY_METHOD *EC_KEY_get_default_method(void) {
  return EC_KEY_OpenSSL();
}

EC_KEY_METHOD *EC_KEY_METHOD_new(const EC_KEY_METHOD *eckey_meth) {
  EC_KEY_METHOD *ret;

  ret = OPENSSL_zalloc(sizeof(EC_KEY_METHOD));
  if(ret == NULL) {
    return NULL;
  }

  if(eckey_meth) {
    *ret = *eckey_meth;
  }
  return ret;
}

void EC_KEY_METHOD_free(EC_KEY_METHOD *eckey_meth) {
  if(eckey_meth != NULL) {
    OPENSSL_free(eckey_meth);
  }
}

int EC_KEY_set_method(EC_KEY *ec, const EC_KEY_METHOD *meth) {
  if(ec == NULL || meth == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  ec->eckey_method = meth;
  return 1;
}

const EC_KEY_METHOD *EC_KEY_get_method(const EC_KEY *ec) {
  if(ec == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  return ec->eckey_method;
}

void EC_KEY_METHOD_set_init_awslc(EC_KEY_METHOD *meth, int (*init)(EC_KEY *key),
                                  void (*finish)(EC_KEY *key)) {
  if(meth == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return;
  }

  meth->init = init;
  meth->finish = finish;
}

void EC_KEY_METHOD_set_sign_awslc(EC_KEY_METHOD *meth,
                            int (*sign)(int type, const uint8_t *digest,
                                    int digest_len, uint8_t *sig,
                                    unsigned int *siglen, const BIGNUM *k_inv,
                                    const BIGNUM *r, EC_KEY *eckey),
                            ECDSA_SIG *(*sign_sig)(const uint8_t *digest,
                                    int digest_len,
                                    const BIGNUM *in_kinv, const BIGNUM *in_r,
                                    EC_KEY *eckey)) {

  if(meth == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return;
  }

  meth->sign = sign;
  meth->sign_sig = sign_sig;
}

int EC_KEY_METHOD_set_flags(EC_KEY_METHOD *meth, int flags) {
  if(!meth || flags != ECDSA_FLAG_OPAQUE) {
    return 0;
  }

  meth->flags |= flags;
  return 1;
}

int EC_KEY_decoded_from_explicit_params(const EC_KEY *key) {
  return key->group_decoded_from_explicit_params;
}
