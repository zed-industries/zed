// Copyright 2002-2016 The OpenSSL Project Authors. All Rights Reserved.
// Copyright (c) 2002, Oracle and/or its affiliates. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <openssl/ec_key.h>

#include <string.h>

#include <openssl/ec.h>
#include <openssl/ecdsa.h>
#include <openssl/engine.h>
#include <openssl/err.h>
#include <openssl/ex_data.h>
#include <openssl/mem.h>
#include <openssl/thread.h>

#include "../../internal.h"
#include "../bcm_interface.h"
#include "../delocate.h"
#include "../ecdsa/internal.h"
#include "../service_indicator/internal.h"
#include "internal.h"


DEFINE_STATIC_EX_DATA_CLASS(g_ec_ex_data_class)

static EC_WRAPPED_SCALAR *ec_wrapped_scalar_new(const EC_GROUP *group) {
  EC_WRAPPED_SCALAR *wrapped = reinterpret_cast<EC_WRAPPED_SCALAR *>(
      OPENSSL_zalloc(sizeof(EC_WRAPPED_SCALAR)));
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
  EC_KEY *ret = reinterpret_cast<EC_KEY *>(OPENSSL_zalloc(sizeof(EC_KEY)));
  if (ret == NULL) {
    return NULL;
  }

  if (engine) {
    ret->ecdsa_meth = ENGINE_get_ECDSA_method(engine);
  }
  if (ret->ecdsa_meth) {
    METHOD_ref(ret->ecdsa_meth);
  }

  ret->conv_form = POINT_CONVERSION_UNCOMPRESSED;
  ret->references = 1;

  CRYPTO_new_ex_data(&ret->ex_data);

  if (ret->ecdsa_meth && ret->ecdsa_meth->init && !ret->ecdsa_meth->init(ret)) {
    CRYPTO_free_ex_data(g_ec_ex_data_class_bss_get(), ret, &ret->ex_data);
    if (ret->ecdsa_meth) {
      METHOD_unref(ret->ecdsa_meth);
    }
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

  if (r->ecdsa_meth) {
    if (r->ecdsa_meth->finish) {
      r->ecdsa_meth->finish(r);
    }
    METHOD_unref(r->ecdsa_meth);
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

  if ((src->group != NULL && !EC_KEY_set_group(ret, src->group)) ||
      (src->pub_key != NULL && !EC_KEY_set_public_key(ret, src->pub_key)) ||
      (src->priv_key != NULL &&
       !EC_KEY_set_private_key(ret, EC_KEY_get0_private_key(src)))) {
    EC_KEY_free(ret);
    return NULL;
  }

  ret->enc_flag = src->enc_flag;
  ret->conv_form = src->conv_form;
  return ret;
}

int EC_KEY_up_ref(EC_KEY *r) {
  CRYPTO_refcount_inc(&r->references);
  return 1;
}

int EC_KEY_is_opaque(const EC_KEY *key) {
  return key->ecdsa_meth && (key->ecdsa_meth->flags & ECDSA_FLAG_OPAQUE);
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
  key->conv_form = cform;
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

  // Check the public and private keys match.
  //
  // NOTE: this is a FIPS pair-wise consistency check for the ECDH case. See SP
  // 800-56Ar3, page 36.
  if (eckey->priv_key != NULL) {
    EC_JACOBIAN point;
    if (!ec_point_mul_scalar_base(eckey->group, &point,
                                  &eckey->priv_key->scalar)) {
      OPENSSL_PUT_ERROR(EC, ERR_R_EC_LIB);
      return 0;
    }
    // Leaking this comparison only leaks whether |eckey|'s public key was
    // correct.
    if (!constant_time_declassify_int(ec_GFp_simple_points_equal(
            eckey->group, &point, &eckey->pub_key->raw))) {
      OPENSSL_PUT_ERROR(EC, EC_R_INVALID_PRIVATE_KEY);
      return 0;
    }
  }

  return 1;
}

int EC_KEY_check_fips(const EC_KEY *key) {
  int ret = 0;
  FIPS_service_indicator_lock_state();

  if (!EC_KEY_check_key(key)) {
    goto end;
  }

  if (key->priv_key) {
    uint8_t digest[BCM_SHA256_DIGEST_LENGTH] = {0};
    uint8_t sig[ECDSA_MAX_FIXED_LEN];
    size_t sig_len;
    if (!ecdsa_sign_fixed(digest, sizeof(digest), sig, &sig_len, sizeof(sig),
                          key)) {
      goto end;
    }
    if (boringssl_fips_break_test("ECDSA_PWCT")) {
      digest[0] = ~digest[0];
    }
    if (!ecdsa_verify_fixed(digest, sizeof(digest), sig, sig_len, key)) {
      OPENSSL_PUT_ERROR(EC, EC_R_PUBLIC_KEY_VALIDATION_FAILED);
      goto end;
    }
  }

  ret = 1;

end:
  FIPS_service_indicator_unlock_state();
  if (ret) {
    EC_KEY_keygen_verify_service_indicator(key);
  }

  return ret;
}

int EC_KEY_set_public_key_affine_coordinates(EC_KEY *key, const BIGNUM *x,
                                             const BIGNUM *y) {
  if (!key || !key->group || !x || !y) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  bssl::UniquePtr<EC_POINT> point(EC_POINT_new(key->group));
  if (point == nullptr ||
      !EC_POINT_set_affine_coordinates_GFp(key->group, point.get(), x, y,
                                           nullptr) ||
      !EC_KEY_set_public_key(key, point.get()) ||  //
      !EC_KEY_check_key(key)) {
    return 0;
  }

  return 1;
}

int EC_KEY_oct2key(EC_KEY *key, const uint8_t *in, size_t len, BN_CTX *ctx) {
  if (key->group == nullptr) {
    OPENSSL_PUT_ERROR(EC, EC_R_MISSING_PARAMETERS);
    return 0;
  }

  bssl::UniquePtr<EC_POINT> point(EC_POINT_new(key->group));
  return point != nullptr &&
         EC_POINT_oct2point(key->group, point.get(), in, len, ctx) &&
         EC_KEY_set_public_key(key, point.get());
}

size_t EC_KEY_key2buf(const EC_KEY *key, point_conversion_form_t form,
                      uint8_t **out_buf, BN_CTX *ctx) {
  if (key == NULL || key->pub_key == NULL || key->group == NULL) {
    OPENSSL_PUT_ERROR(EC, EC_R_MISSING_PARAMETERS);
    return 0;
  }

  return EC_POINT_point2buf(key->group, key->pub_key, form, out_buf, ctx);
}

int EC_KEY_oct2priv(EC_KEY *key, const uint8_t *in, size_t len) {
  if (key->group == NULL) {
    OPENSSL_PUT_ERROR(EC, EC_R_MISSING_PARAMETERS);
    return 0;
  }

  if (len != BN_num_bytes(EC_GROUP_get0_order(key->group))) {
    OPENSSL_PUT_ERROR(EC, EC_R_DECODE_ERROR);
    return 0;
  }

  BIGNUM *priv_key = BN_bin2bn(in, len, NULL);
  int ok = priv_key != NULL &&  //
           EC_KEY_set_private_key(key, priv_key);
  BN_free(priv_key);
  return ok;
}

size_t EC_KEY_priv2oct(const EC_KEY *key, uint8_t *out, size_t max_out) {
  if (key->group == NULL || key->priv_key == NULL) {
    OPENSSL_PUT_ERROR(EC, EC_R_MISSING_PARAMETERS);
    return 0;
  }

  size_t len = BN_num_bytes(EC_GROUP_get0_order(key->group));
  if (out == NULL) {
    return len;
  }

  if (max_out < len) {
    OPENSSL_PUT_ERROR(EC, EC_R_BUFFER_TOO_SMALL);
    return 0;
  }

  size_t bytes_written;
  ec_scalar_to_bytes(key->group, out, &bytes_written, &key->priv_key->scalar);
  assert(bytes_written == len);
  return len;
}

size_t EC_KEY_priv2buf(const EC_KEY *key, uint8_t **out_buf) {
  *out_buf = NULL;
  size_t len = EC_KEY_priv2oct(key, NULL, 0);
  if (len == 0) {
    return 0;
  }

  uint8_t *buf = reinterpret_cast<uint8_t *>(OPENSSL_malloc(len));
  if (buf == NULL) {
    return 0;
  }

  len = EC_KEY_priv2oct(key, buf, len);
  if (len == 0) {
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
  if (eckey == NULL || eckey->group == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  boringssl_ensure_ecc_self_test();

  if (EC_KEY_generate_key(eckey) && EC_KEY_check_fips(eckey)) {
    return 1;
  }

  EC_POINT_free(eckey->pub_key);
  ec_wrapped_scalar_free(eckey->priv_key);
  eckey->pub_key = NULL;
  eckey->priv_key = NULL;
  return 0;
}

int EC_KEY_get_ex_new_index(long argl, void *argp, CRYPTO_EX_unused *unused,
                            CRYPTO_EX_dup *dup_unused,
                            CRYPTO_EX_free *free_func) {
  return CRYPTO_get_ex_new_index_ex(g_ec_ex_data_class_bss_get(), argl, argp,
                                    free_func);
}

int EC_KEY_set_ex_data(EC_KEY *d, int idx, void *arg) {
  return CRYPTO_set_ex_data(&d->ex_data, idx, arg);
}

void *EC_KEY_get_ex_data(const EC_KEY *d, int idx) {
  return CRYPTO_get_ex_data(&d->ex_data, idx);
}

void EC_KEY_set_asn1_flag(EC_KEY *key, int flag) {}
