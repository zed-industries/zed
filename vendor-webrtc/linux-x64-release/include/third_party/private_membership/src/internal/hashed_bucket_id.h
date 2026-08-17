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

#ifndef THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_HASHED_BUCKET_ID_H_
#define THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_HASHED_BUCKET_ID_H_

#include <string>

#include "third_party/private-join-and-compute/src/crypto/ec_commutative_cipher.h"
#include "third_party/private_membership/src/private_membership_rlwe.pb.h"
#include "absl/hash/hash.h"
#include "absl/strings/string_view.h"
#include "third_party/shell-encryption/src/statusor.h"

namespace private_membership {
namespace rlwe {

// Concrete data type representing the hashed bucket id.
class HashedBucketId {
 public:
  // Creates the object from raw data.
  //
  // Returns an invalid argument error if the data and length do not match the
  // precondition that hashed_bucket_id must have at least floor(bit_length/8)
  // bytes, at most ceil(bit_length/8) bytes, and all bits after the
  // bit_length'th must be set to 0.
  static ::rlwe::StatusOr<HashedBucketId> Create(
      absl::string_view hashed_bucket_id, int bit_length);

  // Creates the object from the plaintet ID and parameters.
  static ::rlwe::StatusOr<HashedBucketId> Create(
      const RlwePlaintextId& id, const HashedBucketsParameters& params,
      ::private_join_and_compute::Context* ctx);

  // Creates the object from the API proto.
  //
  // Returns an invalid argument error if the API proto doesn't satisfy the
  // precondition (see the proto doc comment for more details on the
  // precondition).
  static ::rlwe::StatusOr<HashedBucketId> CreateFromApiProto(
      const private_membership::rlwe::PrivateMembershipRlweQuery::
          HashedBucketId& api_hashed_bucket_id);

  // Creates and returns an API proto representing the object.
  private_membership::rlwe::PrivateMembershipRlweQuery::HashedBucketId
  ToApiProto() const;
  //

  // Converts the bucket id to an unsigned 32 bit integer representation.
  //
  // If the bit length > 32, returns an internal error.
  ::rlwe::StatusOr<uint32_t> ToUint32() const;

  // Creates a debug string for logging.
  std::string DebugString() const;

  bool operator==(const HashedBucketId& other) const {
    return (hashed_bucket_id_bytes_ == other.hashed_bucket_id_bytes_) &&
           (bit_length_ == other.bit_length_);
  }

  bool operator!=(const HashedBucketId& other) const {
    return !(*this == other);
  }

  // Hash value of the object.
  template <typename H>
  friend H AbslHashValue(H hash_state, const HashedBucketId& hashed_bucket_id) {
    return H::combine(std::move(hash_state),
                      hashed_bucket_id.hashed_bucket_id_bytes_,
                      hashed_bucket_id.bit_length_);
  }

  inline int bit_length() const { return bit_length_; }

 private:
  HashedBucketId(absl::string_view hashed_bucket_id_bytes, int bit_length);

  // Raw hashed bucket id, in bytes.
  const std::string hashed_bucket_id_bytes_;

  // Total bit length of the hashed bucket id.
  const int bit_length_;
};

}  // namespace rlwe
}  // namespace private_membership

#endif  // THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_HASHED_BUCKET_ID_H_
