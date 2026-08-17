// Copyright 2017 The BoringSSL Authors
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

#include <openssl/crypto.h>

#include "../../internal.h"
#include "../delocate.h"


int FIPS_mode(void) {
#if defined(BORINGSSL_FIPS) && !defined(OPENSSL_ASAN)
  return 1;
#else
  return 0;
#endif
}

int FIPS_mode_set(int on) { return on == FIPS_mode(); }

const char *FIPS_module_name(void) { return "BoringCrypto"; }

int CRYPTO_has_asm(void) {
#if defined(OPENSSL_NO_ASM)
  return 0;
#else
  return 1;
#endif
}

uint32_t FIPS_version(void) {
  return 0;
}

int FIPS_query_algorithm_status(const char *algorithm) {
#if defined(BORINGSSL_FIPS)
  static const char kApprovedAlgorithms[][13] = {
    "AES-CBC",
    "AES-CCM",
    "AES-CTR",
    "AES-ECB",
    "AES-GCM",
    "AES-KW",
    "AES-KWP",
    "ctrDRBG",
    "ECC-SSC",
    "ECDSA-sign",
    "ECDSA-verify",
    "FFC-SSC",
    "HMAC",
    "RSA-sign",
    "RSA-verify",
    "SHA-1",
    "SHA2-224",
    "SHA2-256",
    "SHA2-384",
    "SHA2-512",
    "SHA2-512/256",
  };
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kApprovedAlgorithms); i++) {
    if (strcmp(algorithm, kApprovedAlgorithms[i]) == 0) {
      return 1;
    }
  }
#endif  // BORINGSSL_FIPS

  return 0;
}

#if defined(BORINGSSL_FIPS_COUNTERS)

size_t FIPS_read_counter(enum fips_counter_t counter) {
  size_t index = (size_t)counter;
  if (index > fips_counter_max) {
    abort();
  }

  const size_t *array = reinterpret_cast<const size_t *>(
      CRYPTO_get_thread_local(OPENSSL_THREAD_LOCAL_FIPS_COUNTERS));
  if (!array) {
    return 0;
  }

  return array[index];
}

void boringssl_fips_inc_counter(enum fips_counter_t counter) {
  size_t index = (size_t)counter;
  if (index > fips_counter_max) {
    abort();
  }

  size_t *array = reinterpret_cast<size_t *>(
      CRYPTO_get_thread_local(OPENSSL_THREAD_LOCAL_FIPS_COUNTERS));
  if (!array) {
    const size_t num_bytes = sizeof(size_t) * (fips_counter_max + 1);
    array = reinterpret_cast<size_t *>(OPENSSL_zalloc(num_bytes));
    if (!array) {
      return;
    }

    if (!CRYPTO_set_thread_local(OPENSSL_THREAD_LOCAL_FIPS_COUNTERS, array,
                                 OPENSSL_free)) {
      // |OPENSSL_free| has already been called by |CRYPTO_set_thread_local|.
      return;
    }
  }

  array[index]++;
}

#else

size_t FIPS_read_counter(enum fips_counter_t counter) { return 0; }

// boringssl_fips_inc_counter is a no-op, inline function in internal.h in this
// case. That should let the compiler optimise away the callsites.

#endif
