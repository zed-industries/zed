// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/ctrdrbg.h>

#include "internal.h"

int vm_ube_fallback_get_seed(uint8_t seed[CTR_DRBG_ENTROPY_LEN]) {
    CRYPTO_sysrand(seed, CTR_DRBG_ENTROPY_LEN);
    return 1;
}
