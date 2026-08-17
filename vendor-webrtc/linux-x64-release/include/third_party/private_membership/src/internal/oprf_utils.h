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

#ifndef THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_OPRF_UTILS_H_
#define THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_OPRF_UTILS_H_

#include "third_party/private-join-and-compute/src/crypto/ec_commutative_cipher.h"
#include "third_party/private_membership/src/private_membership.pb.h"
#include "third_party/private_membership/base/private_membership_export.h"
#include "third_party/shell-encryption/src/statusor.h"

namespace private_membership {

// Re-encrypts an already encrypted id with the given EC commutative cipher.
//
// Returns an INVALID_ARGUMENT error code if the given encrypted id is not a
// valid encoding of a point on the curve as defined in ANSI X9.62 ECDSA.
//
// This method is not threadsafe because ec_cipher is not thread-safe.
PRIVATE_MEMBERSHIP_EXPORT ::rlwe::StatusOr<DoublyEncryptedId> ReEncryptId(
    absl::string_view encrypted_id, ::private_join_and_compute::ECCommutativeCipher* ec_cipher);

}  // namespace private_membership

#endif  // THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_OPRF_UTILS_H_
