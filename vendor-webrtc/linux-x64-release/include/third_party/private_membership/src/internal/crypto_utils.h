// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_CRYPTO_UTILS_H_
#define THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_CRYPTO_UTILS_H_

#include <string>

#include "third_party/private-join-and-compute/src/crypto/context.h"
#include "third_party/private-join-and-compute/src/crypto/ec_commutative_cipher.h"
#include "third_party/private_membership/src/private_membership.pb.h"
#include "third_party/private_membership/base/private_membership_export.h"
#include "absl/strings/string_view.h"
#include "third_party/shell-encryption/src/statusor.h"

namespace private_membership {


// Pad to `max_byte_length` bytes.
//
// The returned bytes will be max_byte_length + 4 bytes long, where the first
// four bytes represents the length of the original bytes.
PRIVATE_MEMBERSHIP_EXPORT ::rlwe::StatusOr<std::string> Pad(
    absl::string_view bytes, int max_byte_length);

// Unpad bytes padded with the `Pad` function.
PRIVATE_MEMBERSHIP_EXPORT ::rlwe::StatusOr<std::string> Unpad(
    absl::string_view bytes);

// Hash encrypted ID.
PRIVATE_MEMBERSHIP_EXPORT std::string HashEncryptedId(
    absl::string_view encrypted_id, ::private_join_and_compute::Context* ctx);

// Compute the value encryption key.
PRIVATE_MEMBERSHIP_EXPORT ::rlwe::StatusOr<std::string> GetValueEncryptionKey(
    absl::string_view encrypted_id, ::private_join_and_compute::Context* ctx);

// Encrypts the value, first padding it to. The result will be
// max_value_byte_length+4 bytes long, the first 4 bytes being an encryption of
// the length of the original value.
//
// Returns an error if the value is longer than max_value_byte_length.
PRIVATE_MEMBERSHIP_EXPORT ::rlwe::StatusOr<std::string> EncryptValue(
    absl::string_view encrypted_id, absl::string_view value,
    uint32_t max_value_byte_length, ::private_join_and_compute::Context* ctx);

// Decrypt the encrypted_value using a key derived from the encrypted_id.
PRIVATE_MEMBERSHIP_EXPORT ::rlwe::StatusOr<std::string> DecryptValue(
    absl::string_view encrypted_id, absl::string_view encrypted_value,
    ::private_join_and_compute::Context* ctx);

// Decrypt the encrypted_value using the supplied key.
PRIVATE_MEMBERSHIP_EXPORT ::rlwe::StatusOr<std::string> DecryptValueWithKey(
    absl::string_view encryption_key, absl::string_view encrypted_value,
    ::private_join_and_compute::Context* ctx);

}  // namespace private_membership

#endif  // THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_CRYPTO_UTILS_H_
