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

#ifndef THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_ENCRYPTED_BUCKET_ID_H_
#define THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_ENCRYPTED_BUCKET_ID_H_

#include <string>

#include "third_party/private-join-and-compute/src/crypto/ec_commutative_cipher.h"
#include "third_party/private_membership/base/private_membership_export.h"
#include "third_party/private_membership/src/private_membership_rlwe.pb.h"
#include "absl/hash/hash.h"
#include "absl/strings/string_view.h"
#include "third_party/shell-encryption/src/statusor.h"

namespace private_membership {
namespace rlwe {

// Concrete data type representing the encrypted bucket id.
class PRIVATE_MEMBERSHIP_EXPORT EncryptedBucketId {
 public:
  // Creates the object from raw data.
  //
  // Returns an invalid argument error if the data and length do not match the
  // precondition that encrypted_bucket_id must have at least
  // floor(bit_length/8) bytes, at most ceil(bit_length/8) bytes, and all bits
  // after the bit_length'th must be set to 0.
  static ::rlwe::StatusOr<EncryptedBucketId> Create(
      absl::string_view encrypted_bucket_id, int bit_length);

  // Creates encrypted bucket ID from plaintext ID and parameters.
  //
  // Returns an error if commutative cipher encryption fails.
  static ::rlwe::StatusOr<EncryptedBucketId> Create(
      const RlwePlaintextId& id, const EncryptedBucketsParameters& params,
      ::private_join_and_compute::ECCommutativeCipher* ec_cipher, ::private_join_and_compute::Context* ctx);

  // Creates encrypted bucket ID from encrypted ID and parameters.
  //
  // Returns an error if parameters are invalid.
  static ::rlwe::StatusOr<EncryptedBucketId> Create(
      absl::string_view encrypted_id, const EncryptedBucketsParameters& params,
      ::private_join_and_compute::Context* ctx);

  // Converts the bucket id to an unsigned 32 bit integer representation.
  //
  // If the bit length > 32, returns an internal error.
  ::rlwe::StatusOr<uint32_t> ToUint32() const;

  bool operator==(const EncryptedBucketId& other) const {
    return (encrypted_bucket_id_bytes_ == other.encrypted_bucket_id_bytes_) &&
           (bit_length_ == other.bit_length_);
  }

  bool operator!=(const EncryptedBucketId& other) const {
    return !(*this == other);
  }

  // Hash value of the object.
  template <typename H>
  friend H AbslHashValue(H hash_state,
                         const EncryptedBucketId& encrypted_bucket_id) {
    return H::combine(std::move(hash_state),
                      encrypted_bucket_id.encrypted_bucket_id_bytes_,
                      encrypted_bucket_id.bit_length_);
  }

  inline int bit_length() const { return bit_length_; }

 private:
  EncryptedBucketId(absl::string_view encrypted_bucket_id_bytes,
                    int bit_length);

  // Raw encrypted bucket id, in bytes.
  const std::string encrypted_bucket_id_bytes_;

  // Total bit length of the encrypted bucket id.
  const int bit_length_;
};

}  // namespace rlwe
}  // namespace private_membership

#endif  // THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_ENCRYPTED_BUCKET_ID_H_
