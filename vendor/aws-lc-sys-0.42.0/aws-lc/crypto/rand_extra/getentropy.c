// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#if !defined(_DEFAULT_SOURCE)
// Needed for getentropy on musl and glibc per man pages.
#define _DEFAULT_SOURCE
#endif

#include <openssl/rand.h>

#include "internal.h"

#if defined(OPENSSL_RAND_GETENTROPY)

#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#if defined(OPENSSL_MACOS)
// MacOS does not declare getentropy in uinstd.h like other OS's. 
#include <sys/random.h>
#endif

#if !defined(GETENTROPY_MAX)
// Per POSIX 2024
// https://pubs.opengroup.org/onlinepubs/9799919799/functions/getentropy.html
#define GETENTROPY_MAX 256
#endif

void CRYPTO_sysrand(uint8_t *out, size_t requested) {
  // getentropy max request size is GETENTROPY_MAX.
  while (requested > 0) {
    size_t request_chunk = (requested > GETENTROPY_MAX) ? GETENTROPY_MAX : requested;
    if (getentropy(out, request_chunk) != 0) {
      fprintf(stderr, "getentropy failed.\n");
      abort();
    }
    requested -= request_chunk;
    out += request_chunk;
  }
}

#endif
