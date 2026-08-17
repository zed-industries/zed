// Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
//
// The Elliptic Curve Public-Key Crypto Library (ECC Code) included
// herein is developed by SUN MICROSYSTEMS, INC., and is contributed
// to the OpenSSL project.
// 
// The ECDH software is originally written by Douglas Stebila of
// Sun Microsystems Laboratories.
//
// Copyright (c) 2000-2002 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ecdh.h>

#include <string.h>

#include <openssl/ec.h>
#include <openssl/ec_key.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/sha.h>

#include "../../internal.h"
#include "../ec/internal.h"
#include "../service_indicator/internal.h"


int ECDH_compute_shared_secret(uint8_t *buf, size_t *buflen, const EC_POINT *pub_key,
                               const EC_KEY *priv_key) {
  boringssl_ensure_ecc_self_test();
  if (priv_key->priv_key == NULL) {
    OPENSSL_PUT_ERROR(ECDH, ECDH_R_NO_PRIVATE_VALUE);
    return 0;
  }
  const EC_SCALAR *const priv = &priv_key->priv_key->scalar;
  const EC_GROUP *const group = EC_KEY_get0_group(priv_key);
  if (EC_GROUP_cmp(group, pub_key->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }

  // Lock state here to avoid the underlying |EC_KEY_check_fips| function
  // updating the service indicator state unintentionally.
  FIPS_service_indicator_lock_state();
  int ret = 0;

  // |EC_KEY_check_fips| is not an expensive operation on an external
  // public key so we always perform it, even in the non-FIPS build.
  EC_KEY *key_pub_key = NULL;
  key_pub_key = EC_KEY_new();
  if (key_pub_key == NULL) {
    goto end;
  }

  if (!EC_KEY_set_group(key_pub_key, group) ||
      // Creates a copy of pub_key within key_pub_key
      !EC_KEY_set_public_key(key_pub_key, pub_key) ||
      !EC_KEY_check_fips(key_pub_key)) {
    OPENSSL_PUT_ERROR(EC, EC_R_PUBLIC_KEY_VALIDATION_FAILED);
    goto end;
  }

  EC_JACOBIAN shared_point;
  if (!ec_point_mul_scalar(group, &shared_point, &pub_key->raw, priv) ||
      !ec_get_x_coordinate_as_bytes(group, buf, buflen, *buflen,
                                    &shared_point)) {
    OPENSSL_PUT_ERROR(ECDH, ECDH_R_POINT_ARITHMETIC_FAILURE);
    goto end;
  }

  ret = 1;
end:
  OPENSSL_cleanse(&shared_point, sizeof(EC_JACOBIAN));
  FIPS_service_indicator_unlock_state();
  if (key_pub_key != NULL) {
    EC_KEY_free(key_pub_key);
  }
  return ret;
}

int ECDH_compute_key_fips(uint8_t *out, size_t out_len, const EC_POINT *pub_key,
                          const EC_KEY *priv_key) {
  // Lock state here to avoid underlying |SHA*| functions updating the service
  // indicator state unintentionally.
  FIPS_service_indicator_lock_state();

  uint8_t buf[EC_MAX_BYTES];
  size_t buflen = sizeof(buf);
  int ret = 0;

  if (!ECDH_compute_shared_secret(buf, &buflen, pub_key, priv_key)) {
    goto end;
  }

  switch (out_len) {
    case SHA224_DIGEST_LENGTH:
      SHA224(buf, buflen, out);
      break;
    case SHA256_DIGEST_LENGTH:
      SHA256(buf, buflen, out);
      break;
    case SHA384_DIGEST_LENGTH:
      SHA384(buf, buflen, out);
      break;
    case SHA512_DIGEST_LENGTH:
      SHA512(buf, buflen, out);
      break;
    default:
      OPENSSL_PUT_ERROR(ECDH, ECDH_R_UNKNOWN_DIGEST_LENGTH);
      goto end;
  }
  ret = 1;

end:
  OPENSSL_cleanse(buf, sizeof(buf));
  FIPS_service_indicator_unlock_state();
  if(ret) {
    ECDH_verify_service_indicator(priv_key);
  }
  return ret;
}
