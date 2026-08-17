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

#ifndef THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_AES_CTR_256_WITH_FIXED_IV_H_
#define THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_AES_CTR_256_WITH_FIXED_IV_H_

#include <memory>
#include <string>

#include "third_party/private_membership/base/private_membership_export.h"
#include "absl/strings/string_view.h"
#include <openssl/evp.h>
#include "third_party/shell-encryption/src/statusor.h"

namespace private_membership {

// Wrapper around OpenSSL AES-CTR with 256 bit keys and a fixed IV. As a result,
// the encryptions will be deterministic (two encryptions of the a message under
// the same key results in the same two ciphertexts).
//
// Security: This cipher does not provide probabilistic encryption or
// authentication of the message. This should only be used in the scenario where
// each key will encrypt at most one message.
class PRIVATE_MEMBERSHIP_EXPORT AesCtr256WithFixedIV {
 public:
  // Create a cipher with the input key.
  //
  // Returns INVALID_ARGUMENT if key is an invalid size.
  static ::rlwe::StatusOr<std::unique_ptr<AesCtr256WithFixedIV>> Create(
      absl::string_view key);

  // Encrypts the plaintext.
  //
  // Returns INTERNAL on internal cryptographic errors.
  ::rlwe::StatusOr<std::string> Encrypt(absl::string_view plaintext) const;

  // Decrypts the ciphertext.
  //
  // Returns INTERNAL on internal cryptographic errors.
  ::rlwe::StatusOr<std::string> Decrypt(absl::string_view ciphertext) const;

  // The key size in bytes.
  static constexpr uint8_t kKeySize = 32;

 private:
  explicit AesCtr256WithFixedIV(absl::string_view key);

  static constexpr uint8_t kIVSize = 16;

  const std::string key_;
  const std::string iv_;
};

}  // namespace private_membership

#endif  // THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_AES_CTR_256_WITH_FIXED_IV_H_
