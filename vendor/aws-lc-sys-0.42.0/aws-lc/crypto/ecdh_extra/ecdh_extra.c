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

#include <limits.h>
#include <string.h>

#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../fipsmodule/ec/internal.h"
#include "../internal.h"


int ECDH_compute_key(void *out, size_t out_len, const EC_POINT *pub_key,
                     const EC_KEY *priv_key,
                     void *(*kdf)(const void *in, size_t inlen, void *out,
                                  size_t *out_len)) {
  int ret = -1;
  uint8_t buf[EC_MAX_BYTES];
  size_t buf_len = sizeof(buf);

  if (!ECDH_compute_shared_secret(buf, &buf_len, pub_key, priv_key)) {
    goto end;
  }

  if (kdf != NULL) {
    if (kdf(buf, buf_len, out, &out_len) == NULL) {
      OPENSSL_PUT_ERROR(ECDH, ECDH_R_KDF_FAILED);
      goto end;
    }
  } else {
    // no KDF, just copy as much as we can
    if (buf_len < out_len) {
      out_len = buf_len;
    }
    OPENSSL_memcpy(out, buf, out_len);
  }

  if (out_len > INT_MAX) {
    OPENSSL_PUT_ERROR(ECDH, ERR_R_OVERFLOW);
    goto end;
  }

  ret = (int)out_len;
end:
  OPENSSL_cleanse(buf, sizeof(buf));
  return ret;
}
