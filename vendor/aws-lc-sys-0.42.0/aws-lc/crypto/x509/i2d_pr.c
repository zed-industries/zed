// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>
#include <openssl/bytestring.h>
#include <openssl/dsa.h>
#include <openssl/ec_key.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/rsa.h>

#include "../bytestring/internal.h"


int i2d_PrivateKey(const EVP_PKEY *a, uint8_t **pp) {
  switch (EVP_PKEY_id(a)) {
    case EVP_PKEY_RSA:
      return i2d_RSAPrivateKey(EVP_PKEY_get0_RSA(a), pp);
    case EVP_PKEY_EC:
      return i2d_ECPrivateKey(EVP_PKEY_get0_EC_KEY(a), pp);
    case EVP_PKEY_DSA:
      return i2d_DSAPrivateKey(EVP_PKEY_get0_DSA(a), pp);
    default: {
      // Fall back to PKCS#8 for key types without legacy formats (e.g.
      // Ed25519). Initial capacity of 128 should hold most asymmetric keys.
      CBB cbb;
      if (!CBB_init(&cbb, 128) ||
          !EVP_marshal_private_key(&cbb, a)) {
        CBB_cleanup(&cbb);
        // Although this file is in crypto/x509 for layering reasons, it emits
        // an error code from ASN1 for OpenSSL compatibility.
        OPENSSL_PUT_ERROR(ASN1, ASN1_R_UNSUPPORTED_PUBLIC_KEY_TYPE);
        return -1;
      }
      return CBB_finish_i2d(&cbb, pp);
    }
  }
}
