// Copyright 2023 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_KECCAK_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_KECCAK_INTERNAL_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif


enum boringssl_keccak_config_t {
  boringssl_sha3_256,
  boringssl_sha3_512,
  boringssl_shake128,
  boringssl_shake256,
};

enum boringssl_keccak_phase_t {
  boringssl_keccak_phase_absorb,
  boringssl_keccak_phase_squeeze,
};

struct BORINGSSL_keccak_st {
  enum boringssl_keccak_config_t config;
  enum boringssl_keccak_phase_t phase;
  uint64_t state[25];
  size_t rate_bytes;
  size_t absorb_offset;
  size_t squeeze_offset;
};

// BORINGSSL_keccak hashes |in_len| bytes from |in| and writes |out_len| bytes
// of output to |out|. If the |config| specifies a fixed-output function, like
// SHA3-256, then |out_len| must be the correct length for that function.
OPENSSL_EXPORT void BORINGSSL_keccak(uint8_t *out, size_t out_len,
                                     const uint8_t *in, size_t in_len,
                                     enum boringssl_keccak_config_t config);

// BORINGSSL_keccak_init prepares |ctx| for absorbing. The |config| must specify
// a SHAKE variant, otherwise callers should use |BORINGSSL_keccak|.
OPENSSL_EXPORT void BORINGSSL_keccak_init(
    struct BORINGSSL_keccak_st *ctx, enum boringssl_keccak_config_t config);

// BORINGSSL_keccak_absorb absorbs |in_len| bytes from |in|.
OPENSSL_EXPORT void BORINGSSL_keccak_absorb(struct BORINGSSL_keccak_st *ctx,
                                            const uint8_t *in, size_t in_len);

// BORINGSSL_keccak_squeeze writes |out_len| bytes to |out| from |ctx|.
OPENSSL_EXPORT void BORINGSSL_keccak_squeeze(struct BORINGSSL_keccak_st *ctx,
                                             uint8_t *out, size_t out_len);

#if defined(__cplusplus)
}
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_KECCAK_INTERNAL_H
