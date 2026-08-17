// Copyright 2020 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_DSA_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_DSA_INTERNAL_H

#include <openssl/dsa.h>

#include <openssl/thread.h>

#include "../internal.h"

#if defined(__cplusplus)
extern "C" {
#endif


struct dsa_st {
  BIGNUM *p;
  BIGNUM *q;
  BIGNUM *g;

  BIGNUM *pub_key;
  BIGNUM *priv_key;

  // Normally used to cache montgomery values
  CRYPTO_MUTEX method_mont_lock;
  BN_MONT_CTX *method_mont_p;
  BN_MONT_CTX *method_mont_q;
  CRYPTO_refcount_t references;
  CRYPTO_EX_DATA ex_data;
};

// dsa_check_key performs cheap self-checks on |dsa|, and ensures it is within
// DoS bounds. It returns one on success and zero on error.
int dsa_check_key(const DSA *dsa);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_DSA_INTERNAL_H
