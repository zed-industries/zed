// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

// Ensure we can't call OPENSSL_malloc
#define _BORINGSSL_PROHIBIT_OPENSSL_MALLOC
#include <openssl/crypto.h>

#include "../../internal.h"
#include "../delocate.h"

#include "../rand/entropy/internal.h"


int FIPS_mode(void) {
#if defined(BORINGSSL_FIPS) && !defined(OPENSSL_ASAN) && !defined(OPENSSL_MSAN)
  return 1;
#else
  return 0;
#endif
}

int FIPS_is_entropy_cpu_jitter(void) {
  if (OPT_OUT_CPU_JITTER_ENTROPY_SOURCE == get_entropy_source_method_id_FOR_TESTING()) {
    return 0;
  }
  return 1;
}

uint32_t FIPS_version(void) {
#if defined(BORINGSSL_FIPS) && !defined(OPENSSL_ASAN) && !defined(OPENSSL_MSAN)
  return AWSLC_FIPS_VERSION_NUMBER;
#else
  return 0;
#endif
}

int FIPS_mode_set(int on) { return on == FIPS_mode(); }
