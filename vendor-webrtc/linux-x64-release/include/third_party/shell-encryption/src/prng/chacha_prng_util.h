/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// An implementation of a PRNG using the ChaCha20 stream cipher. Since this is
// a stream cipher, the key stream can be obtained by "encrypting" the plaintext
// 0....0.

#ifndef RLWE_CHACHA_PRNG_UTIL_H_
#define RLWE_CHACHA_PRNG_UTIL_H_

#include <vector>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "integral_types.h"
#include "statusor.h"
#include "third_party/shell-encryption/base/shell_encryption_export.h"

namespace rlwe {
namespace internal {

const int kChaChaKeyBytesSize = 32;
const int kChaChaNonceSize = 12;
const int kChaChaOutputBytes = 255 * 32;

// Once pseudorandom output is exhausted, the salt is updated to construct
// new pseudorandom output.
SHELL_ENCRYPTION_EXPORT absl::Status ChaChaPrngResalt(
    absl::string_view key,
    int buffer_size,
    int* salt_counter,
    int* position_in_buffer,
    std::vector<Uint8>* buffer);

// Generates a secure key for instantiating an CHACHA.
SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<std::string> ChaChaPrngGenerateKey();

// Returns 8 bits of randomness.
//
// Fails on internal cryptographic errors.
SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<Uint8> ChaChaPrngRand8(
    absl::string_view key,
    int* position_in_buffer,
    int* salt_counter,
    std::vector<Uint8>* buffer);

// Returns 64 bits of randomness.
//
// Fails on internal cryptographic errors.
SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<Uint64> ChaChaPrngRand64(
    absl::string_view key,
    int* position_in_buffer,
    int* salt_counter,
    std::vector<Uint8>* buffer);

}  // namespace internal
}  // namespace rlwe

#endif  // RLWE_CHACHA_PRNG_UTIL_H_
