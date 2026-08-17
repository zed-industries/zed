// Copyright 2002-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#include <openssl/aes.h>

#include <assert.h>

#include "../aes/internal.h"
#include "../service_indicator/internal.h"

namespace {
void aes_encrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key) {
  BCM_aes_encrypt(in, out, key);
}

void aes_decrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key) {
  BCM_aes_decrypt(in, out, key);
}
}

void AES_ctr128_encrypt(const uint8_t *in, uint8_t *out, size_t len,
                        const AES_KEY *key, uint8_t ivec[AES_BLOCK_SIZE],
                        uint8_t ecount_buf[AES_BLOCK_SIZE], unsigned int *num) {
  if (hwaes_capable()) {
    CRYPTO_ctr128_encrypt_ctr32(in, out, len, key, ivec, ecount_buf, num,
                                aes_hw_ctr32_encrypt_blocks);
  } else if (vpaes_capable()) {
    // TODO(davidben): On ARM, where |BSAES| is additionally defined, this could
    // use |vpaes_ctr32_encrypt_blocks_with_bsaes|.
    CRYPTO_ctr128_encrypt_ctr32(in, out, len, key, ivec, ecount_buf, num,
                                vpaes_ctr32_encrypt_blocks);
  } else {
    CRYPTO_ctr128_encrypt_ctr32(in, out, len, key, ivec, ecount_buf, num,
                                aes_nohw_ctr32_encrypt_blocks);
  }

  FIPS_service_indicator_update_state();
}

void AES_ecb_encrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key,
                     const int enc) {
  assert(in && out && key);
  assert((AES_ENCRYPT == enc) || (AES_DECRYPT == enc));

  if (AES_ENCRYPT == enc) {
    BCM_aes_encrypt(in, out, key);
  } else {
    BCM_aes_decrypt(in, out, key);
  }

  FIPS_service_indicator_update_state();
}

void AES_cbc_encrypt(const uint8_t *in, uint8_t *out, size_t len,
                     const AES_KEY *key, uint8_t *ivec, const int enc) {
  if (hwaes_capable()) {
    aes_hw_cbc_encrypt(in, out, len, key, ivec, enc);
  } else if (!vpaes_capable()) {
    aes_nohw_cbc_encrypt(in, out, len, key, ivec, enc);
  } else if (enc) {
    CRYPTO_cbc128_encrypt(in, out, len, key, ivec, aes_encrypt);
  } else {
    CRYPTO_cbc128_decrypt(in, out, len, key, ivec, aes_decrypt);
  }

  FIPS_service_indicator_update_state();
}

void AES_ofb128_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                        const AES_KEY *key, uint8_t *ivec, int *num) {
  unsigned num_u = (unsigned)(*num);
  CRYPTO_ofb128_encrypt(in, out, length, key, ivec, &num_u, aes_encrypt);
  *num = (int)num_u;
}

void AES_cfb128_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                        const AES_KEY *key, uint8_t *ivec, int *num,
                        int enc) {
  unsigned num_u = (unsigned)(*num);
  CRYPTO_cfb128_encrypt(in, out, length, key, ivec, &num_u, enc, aes_encrypt);
  *num = (int)num_u;
}
