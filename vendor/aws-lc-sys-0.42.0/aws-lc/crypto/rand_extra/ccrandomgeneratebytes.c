// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/rand.h>

#include "internal.h"

#if defined(OPENSSL_RAND_CCRANDOMGENERATEBYTES)

#include <CommonCrypto/CommonRandom.h>

#include <stdio.h>
#include <stdlib.h>

void CRYPTO_sysrand(uint8_t *out, size_t requested) {

  if (requested == 0) {
    return;
  }

  // To get system randomness on iOS we use |CCRandomGenerateBytes|. On MacOS we
  // use |getentropy| but iOS doesn't expose that.
  if (CCRandomGenerateBytes(out, requested) != kCCSuccess) {
    fprintf(stderr, "CCRandomGenerateBytes failed.\n");
    abort();
  }
}

#endif
