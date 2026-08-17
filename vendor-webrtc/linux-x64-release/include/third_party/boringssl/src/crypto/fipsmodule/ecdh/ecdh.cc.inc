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

#include <openssl/ecdh.h>

#include <string.h>

#include <openssl/ec.h>
#include <openssl/ec_key.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "../ec/internal.h"
#include "../service_indicator/internal.h"


int ECDH_compute_key_fips(uint8_t *out, size_t out_len, const EC_POINT *pub_key,
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

  EC_JACOBIAN shared_point;
  uint8_t buf[EC_MAX_BYTES];
  size_t buflen;
  if (!ec_point_mul_scalar(group, &shared_point, &pub_key->raw, priv) ||
      !ec_get_x_coordinate_as_bytes(group, buf, &buflen, sizeof(buf),
                                    &shared_point)) {
    OPENSSL_PUT_ERROR(ECDH, ECDH_R_POINT_ARITHMETIC_FAILURE);
    return 0;
  }

  FIPS_service_indicator_lock_state();
  SHA256_CTX ctx;
  SHA512_CTX ctx_512;
  switch (out_len) {
    case SHA224_DIGEST_LENGTH:
      BCM_sha224_init(&ctx);
      BCM_sha224_update(&ctx, buf, buflen);
      BCM_sha224_final(out, &ctx);
      break;
    case SHA256_DIGEST_LENGTH:
      BCM_sha256_init(&ctx);
      BCM_sha256_update(&ctx, buf, buflen);
      BCM_sha256_final(out, &ctx);
      break;
    case SHA384_DIGEST_LENGTH:
      BCM_sha384_init(&ctx_512);
      BCM_sha384_update(&ctx_512, buf, buflen);
      BCM_sha384_final(out, &ctx_512);
      break;
    case SHA512_DIGEST_LENGTH:
      BCM_sha512_init(&ctx_512);
      BCM_sha512_update(&ctx_512, buf, buflen);
      BCM_sha512_final(out, &ctx_512);
      break;
    default:
      OPENSSL_PUT_ERROR(ECDH, ECDH_R_UNKNOWN_DIGEST_LENGTH);
      FIPS_service_indicator_unlock_state();
      return 0;
  }
  FIPS_service_indicator_unlock_state();

  ECDH_verify_service_indicator(priv_key);
  return 1;
}
