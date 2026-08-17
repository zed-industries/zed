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

#ifndef THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_PRIVATE_MEMBERSHIP_RLWE_CLIENT_H_
#define THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_PRIVATE_MEMBERSHIP_RLWE_CLIENT_H_

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "third_party/private-join-and-compute/src/crypto/ec_commutative_cipher.h"
#include "third_party/private_membership/src/private_membership.pb.h"
#include "third_party/private_membership/base/private_membership_export.h"
#include "third_party/private_membership/src/private_membership_rlwe.pb.h"
#include "third_party/private_membership/src/internal/constants.h"
#include "absl/container/flat_hash_map.h"
#include "third_party/shell-encryption/src/montgomery.h"
#include "third_party/shell-encryption/src/prng/prng.h"
#include "third_party/shell-encryption/src/statusor.h"
#include "third_party/shell-encryption/src/symmetric_encryption.h"

namespace private_membership {
namespace rlwe {

namespace internal {

// PRNG seed generator which supports deterministic seed generation.
class PRIVATE_MEMBERSHIP_EXPORT PrngSeedGenerator {
 public:
  // Creates a non deterministic PRNG seed generator.
  static std::unique_ptr<PrngSeedGenerator> Create();

  // Creates a deterministic PRNG seed generator.
  static ::rlwe::StatusOr<std::unique_ptr<PrngSeedGenerator>>
  CreateDeterministic(absl::string_view seed);

  // Generates a PRNG seed.
  ::rlwe::StatusOr<std::string> GeneratePrngSeed() const;

 private:
  PrngSeedGenerator() = default;

  explicit PrngSeedGenerator(
      std::unique_ptr<SingleThreadPrng> prng_seed_generator);

  const std::optional<std::unique_ptr<SingleThreadPrng>>
      deterministic_prng_seed_generator_;
};

class PRIVATE_MEMBERSHIP_EXPORT PrngGenerator {
 public:
  virtual ~PrngGenerator() = default;

  PrngGenerator(const PrngGenerator&) = delete;
  PrngGenerator& operator=(const PrngGenerator&) = delete;

  virtual ::rlwe::StatusOr<std::unique_ptr<::rlwe::SecurePrng>> CreatePrng(
      absl::string_view seed) const = 0;

 protected:
  PrngGenerator() = default;
};

// Implementation of a secure PRNG generator.
class SecurePrngGenerator : public PrngGenerator {
 public:
  static std::unique_ptr<PrngGenerator> Create() {
    return absl::WrapUnique<SecurePrngGenerator>(new SecurePrngGenerator());
  }

  ::rlwe::StatusOr<std::unique_ptr<::rlwe::SecurePrng>> CreatePrng(
      absl::string_view seed) const override {
    return SingleThreadPrng::Create(seed);
  }

 private:
  SecurePrngGenerator() = default;
};

// Lightweight wrapper for processing PIR related requests and responses.
// Thread safe.
class PRIVATE_MEMBERSHIP_EXPORT PirClient {
 public:
  virtual ~PirClient() = default;

  // PirClient is neither copyable nor copy assignable.
  PirClient(const PirClient&) = delete;
  PirClient& operator=(const PirClient&) = delete;

  static ::rlwe::StatusOr<std::unique_ptr<PirClient>> Create(
      const RlweParameters& rlwe_params, int total_entry_count,
      const PrngSeedGenerator* prng_seed_generator,
      const PrngGenerator* prng_generator);

  // Creates a PIR request to retrieve the entry located at index `index`.
  virtual ::rlwe::StatusOr<PirRequest> CreateRequest(int index) = 0;

  // Process the PIR response and returns the corresponding entry as raw bytes.
  virtual ::rlwe::StatusOr<std::vector<uint8_t>> ProcessResponse(
      const PirResponse& response) = 0;

 protected:
  PirClient() = default;
};

// Thread safe.
template <typename ModularInt>
class PRIVATE_MEMBERSHIP_EXPORT PirClientImpl : public PirClient {
 public:
  static ::rlwe::StatusOr<std::unique_ptr<PirClientImpl<ModularInt>>> Create(
      const RlweParameters& rlwe_params, int total_entry_count,
      const PrngSeedGenerator* prng_seed_generator,
      const PrngGenerator* prng_generator);

  ::rlwe::StatusOr<PirRequest> CreateRequest(int index) override;

  ::rlwe::StatusOr<std::vector<uint8_t>> ProcessResponse(
      const PirResponse& response) override;

 private:
  PirClientImpl(
      const RlweParameters& rlwe_params,
      std::vector<std::unique_ptr<const typename ModularInt::Params>>
          modulus_params,
      std::vector<std::unique_ptr<const ::rlwe::NttParameters<ModularInt>>>
          ntt_params,
      std::vector<std::unique_ptr<const ::rlwe::ErrorParams<ModularInt>>>
          error_params,
      const ::rlwe::SymmetricRlweKey<ModularInt>& key, int total_entry_count,
      const PrngSeedGenerator* prng_seed_generator,
      const PrngGenerator* prng_generator);

  // Maximum allowed plaintext entry size is 10 MB.
  static constexpr int64_t kMaxPlaintextEntrySize = 10000000;

  // Maximum degree is 2^20.
  static constexpr int kMaxLogDegree = 20;

  // Maximum allowed levels of recursion.
  static constexpr int kMaxLevelsOfRecursion = 100;

  // Maximum number of entries in request.
  static constexpr int kMaxRequestEntries = 10000000;

  // Parameters for protocol.
  const RlweParameters rlwe_params_;

  // Parameters for the RLWE modulus.
  const std::vector<std::unique_ptr<const typename ModularInt::Params>>
      modulus_params_;

  // Parameters to compute NTT.
  const std::vector<std::unique_ptr<const ::rlwe::NttParameters<ModularInt>>>
      ntt_params_;

  // Parameters that hold ring-specific error constants.
  const std::vector<std::unique_ptr<const ::rlwe::ErrorParams<ModularInt>>>
      error_params_;

  // Private key to encrypt/decrypt RLWE ciphertexts.
  const ::rlwe::SymmetricRlweKey<ModularInt> key_;

  // Total number of entries in the database.
  const int total_entry_count_;

  // Generates the PRNG seed.
  const PrngSeedGenerator* prng_seed_generator_;

  // Generates the PRNG.
  const PrngGenerator* prng_generator_;
};

}  // namespace internal

// Client for the Private Membership RLWE protocol.
class PRIVATE_MEMBERSHIP_EXPORT PrivateMembershipRlweClient {
 public:
  // PrivateMembershipRlweClient is neither copyable nor copy assignable.
  PrivateMembershipRlweClient(const PrivateMembershipRlweClient&) = delete;
  PrivateMembershipRlweClient& operator=(const PrivateMembershipRlweClient&) =
      delete;

  // Creates a client for the Private Membership RLWE protocol.
  //
  // Each client object generates and holds a randomly generated key.
  static ::rlwe::StatusOr<std::unique_ptr<PrivateMembershipRlweClient>> Create(
      private_membership::rlwe::RlweUseCase use_case,
      const std::vector<RlwePlaintextId>& plaintext_ids);

  // Creates a client for testing the Private Membership RLWE protocol.
  //
  // Instead of the client generating the EC cipher key internally, it will
  // instantiate the cipher with the given key. This also uses deterministic
  // PRNG, which makes the PIR request deterministic. Thus, this should never be
  // used in prod.
  static ::rlwe::StatusOr<std::unique_ptr<PrivateMembershipRlweClient>>
  CreateForTesting(private_membership::rlwe::RlweUseCase use_case,
                   const std::vector<RlwePlaintextId>& plaintext_ids,
                   absl::string_view ec_cipher_key, absl::string_view seed);

  // Creates a request proto for the first phase of the protocol.
  ::rlwe::StatusOr<private_membership::rlwe::PrivateMembershipRlweOprfRequest>
  CreateOprfRequest();

  // Creates a request proto for the second phase of the protocol.
  ::rlwe::StatusOr<private_membership::rlwe::PrivateMembershipRlweQueryRequest>
  CreateQueryRequest(
      const private_membership::rlwe::PrivateMembershipRlweOprfResponse&
          oprf_response);

  // Processes the query response from the server and returns the membership
  // response map.
  //
  // Keys of the returned map match the original plaintext ids supplied to the
  // client when it was created.
  ::rlwe::StatusOr<RlweMembershipResponses> ProcessQueryResponse(
      const private_membership::rlwe::PrivateMembershipRlweQueryResponse&
          query_response);

 private:
  static ::rlwe::StatusOr<std::unique_ptr<PrivateMembershipRlweClient>>
  CreateInternal(
      private_membership::rlwe::RlweUseCase use_case,
      const std::vector<RlwePlaintextId>& plaintext_ids,
      std::optional<std::string> ec_cipher_key,
      std::unique_ptr<internal::PrngSeedGenerator> prng_seed_generator,
      std::unique_ptr<internal::PrngGenerator> prng_generator);

  PrivateMembershipRlweClient(
      private_membership::rlwe::RlweUseCase use_case,
      const std::vector<RlwePlaintextId>& plaintext_ids,
      std::unique_ptr<::private_join_and_compute::ECCommutativeCipher> ec_cipher,
      std::unique_ptr<internal::PrngSeedGenerator> prng_seed_generator,
      std::unique_ptr<internal::PrngGenerator> prng_generator);

  // Checks whether the id corresponding to the `server_encrypted_id` is in the
  // encrypted bucket and if so, returns an associated value if there is one.
  ::rlwe::StatusOr<private_membership::MembershipResponse> CheckMembership(
      absl::string_view server_encrypted_id,
      const private_membership::rlwe::EncryptedBucket& encrypted_bucket);

  // Checks whether the OPRF response is valid.
  absl::Status ValidateOprfResponse(
      const private_membership::rlwe::PrivateMembershipRlweOprfResponse&
          oprf_response) const;

  // Checks whether the query response is valid.
  absl::Status ValidateQueryResponse(
      const private_membership::rlwe::PrivateMembershipRlweQueryResponse&
          query_response) const;

  // Maximum encrypted bucket ID length.
  static constexpr int kMaxEncryptedBucketIdLength = 26;

  // Use case of the membership query.
  const private_membership::rlwe::RlweUseCase use_case_;

  // Vector of all the identifiers to be queried.
  const std::vector<RlwePlaintextId> plaintext_ids_;

  // EC commutative cipher for encrypting/decrypting.
  const std::unique_ptr<::private_join_and_compute::ECCommutativeCipher> ec_cipher_;

  // Generates PRNG seed.
  const std::unique_ptr<internal::PrngSeedGenerator> prng_seed_generator_;

  // Generates PRNG.
  const std::unique_ptr<internal::PrngGenerator> prng_generator_;

  ::private_join_and_compute::Context context_;

  // Map of client encrypted id to plaintext id.
  absl::flat_hash_map<std::string, RlwePlaintextId>
      client_encrypted_id_to_plaintext_id_;

  // Map of client encrypted id to server key encrypted id.
  absl::flat_hash_map<std::string, std::string>
      client_encrypted_id_to_server_encrypted_id_;

  // PIR client for processing PIR requests and responses.
  std::unique_ptr<internal::PirClient> pir_client_;

  // Parameters retrieved from OPRF round.
  HashedBucketsParameters hashed_bucket_params_;
  EncryptedBucketsParameters encrypted_bucket_params_;
};

}  // namespace rlwe
}  // namespace private_membership

#endif  // THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_PRIVATE_MEMBERSHIP_RLWE_CLIENT_H_
