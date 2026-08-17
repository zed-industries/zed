/*
 * Copyright 2018 Google LLC.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef RLWE_RELINEARIZATION_KEY_H_
#define RLWE_RELINEARIZATION_KEY_H_

#include <cstdint>
#include <vector>

#include "sample_error.h"
#include "statusor.h"
#include "symmetric_encryption.h"
#include "third_party/shell-encryption/base/shell_encryption_export.h"
#include "third_party/shell-encryption/base/shell_encryption_export_template.h"

namespace rlwe {
// Represents a RelinearizationKey constructed from a symmetric-key. Applying a
// RelinearizationKey of order k to a ciphertext {m1}_s encrypting m1 with
// k components produces a ciphertext {m1}_s with 2 components encrypted with
// the same secret key s. This is one of two ways key-switching is used, the
// other being GaloisKeys.
//
// RelinearizationKeys are constructed based on the secret key. Two
// RelinearizationKeys that correspond to the same secret key, number of parts,
// and use the same decomposition modulus will not necessarily be equal. This is
// due to randomness that is sampled when the key is created. However, both will
// relinearize a ciphertext. This randomness is generated from a PRNG with a
// given prng_seed.
//
// This variant of key-switching is based on a decomposition modulo a general
// modulus T. We restrict this decomposition modulus T to be a power of 2, and
// log_2(T) generally ranges form 1 to log_2(q)/2. We treat T as a parameter to
// be optimized for. The matrix W is the sum of two matrices: (1) the first is
// the matrix A = [PowersofT(( s, ..., s^{k-1})) + t * e, 0], and (2) a matrix
// R which is perpendicular to (1, s). Note that A consists of "encryptions" of
// the PowersOfT (setting a = 0 in {as + et + m, -a}), and R basically consists
// of encryptions of 0 under (1,s). The sum of these two matrices yields
// essentially "encryptions" of the non-trivial powers of the length k secret
// key (s, s^2, ..., s^{k-1}) under the length 2 secret key (1,s).
//
// Details can be found in Appendix D.2 of https://eprint.iacr.org/2011/566.pdf
//
// Only MontgomeryInt types (Uint16, Uint32, Uint64, absl::uint128) are
// supported.

// The RelinearizationKey, which holds a vector of RelinearizationKeyParts of
// length (k - 1), where k is the number of parts of the ciphertext it applies
// to.
template <typename ModularInt>
class EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) RelinearizationKey {
  using ModularIntParams = typename ModularInt::Params;

 public:
  // Initializes a RelinearizationKey based on a SymmetricRlweKey key that can
  // relinearize ciphertexts with at most num_parts components.
  // A positive log_decomposition_modulus corresponds to the decomposition
  // modulus T. The prng_seed is used to generate and encode the random entries
  // that form the bottom row of the matrix. For most RelinearizationKeys, the
  // substitution_power is 1. This corresponds to the power of x in the secret
  // key polynomial s(x^substitution_power) that the ciphertext is encrypted
  // with. This power changes when substitutions of the form x->x^k (Galois
  // automorphisms) have been applied to ciphertexts, yielding an encryption
  // with (1, s^k). In that case, we would use a relinearization key with
  // substition_power = k to return the ciphertext to be encrypted with (1,s).
  // See GaloisKey for an explicit wrapper around RelinearizationKey.
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<RelinearizationKey> Create(
      const SymmetricRlweKey<ModularInt>& key, absl::string_view prng_seed,
      ssize_t num_parts, Uint64 log_decomposition_modulus,
      Uint64 substitution_power = 1);

  // Takes a SymmetricRlweCiphertext with at most num_parts components and
  // returns a 2 component SymmetricRlweCiphertext encoding the same message.
  // Returns an error when the number of components of ciphertext is larger than
  // the number of parts of the RelineraizationKey.
  rlwe::StatusOr<SymmetricRlweCiphertext<ModularInt>> ApplyTo(
      const SymmetricRlweCiphertext<ModularInt>& ciphertext) const;

  // Returns a SerializedRelinearizationKey containing a flattened
  // representation of the SerializedNttPolynomials in the key, the
  // log_decomposition_modulus, and the number of parts the key is comprised of.
  rlwe::StatusOr<SerializedRelinearizationKey> Serialize() const;

  // Requires that the number of NTT Polynomials in serialized is (num_parts) *
  // (2 * dimension) where dimension is the number of digits needed to represent
  // the modulus in base 2^{log_decomposition_modulus}. Crashes for non-valid
  // input parameters.
  static rlwe::StatusOr<RelinearizationKey> Deserialize(
      const SerializedRelinearizationKey& serialized,
      const ModularIntParams* modulus_params,
      const NttParameters<ModularInt>* ntt_params);

  // Substitution Power accessor.
  int SubstitutionPower() const { return substitution_power_; }

 private:
  // Represents part of the RelinearizationKey corresponding to a single
  // component of the secret key.
  class RelinearizationKeyPart {
   public:
    static rlwe::StatusOr<RelinearizationKeyPart> Create(
        const Polynomial<ModularInt>& key_power,
        const SymmetricRlweKey<ModularInt>& key,
        const Uint64 log_decomposition_modulus,
        const ModularInt& decomposition_modulus, int dimension,
        SecurePrng* prng, SecurePrng* prng_encryption);

    // For RelinearizationKeyPart i, this method takes the ith component of the
    // ciphertext and params, and applies the RelinearizationKeyPart matrix to
    // an expanded ciphertext component vector to produce a 2-component vector
    // of polynomials.
    rlwe::StatusOr<std::vector<Polynomial<ModularInt>>> ApplyPartTo(
        const Polynomial<ModularInt>& ciphertext_part,
        const ModularIntParams* modulus_params,
        const NttParameters<ModularInt>* ntt_params) const;

    // Creates a RelinearizationKeyPart out of a vector of Polynomials.
    static rlwe::StatusOr<RelinearizationKeyPart> Deserialize(
        const std::vector<SerializedNttPolynomial>& polynomials,
        Uint64 log_decomposition_modulus, SecurePrng* prng,
        const ModularIntParams* modulus_params,
        const NttParameters<ModularInt>* ntt_params);

    std::vector<Polynomial<ModularInt>> Matrix() const { return matrix_[0]; }

   private:
    RelinearizationKeyPart(
        std::vector<std::vector<Polynomial<ModularInt>>> matrix,
        Uint64 log_decomposition_modulus)
        : matrix_(std::move(matrix)),
          log_decomposition_modulus_(log_decomposition_modulus) {}

    std::vector<std::vector<Polynomial<ModularInt>>> matrix_;

    const Uint64 log_decomposition_modulus_;
  };

  // Creates an empty RelinearizationKey.
  RelinearizationKey(Uint64 log_decomposition_modulus,
                     const ModularInt& decomposition_modulus,
                     const ModularIntParams* params,
                     const NttParameters<ModularInt>* ntt_params)
      : log_decomposition_modulus_(log_decomposition_modulus),
        decomposition_modulus_(decomposition_modulus),
        substitution_power_(1),
        modulus_params_(params),
        ntt_params_(ntt_params) {}

  RelinearizationKey(const SymmetricRlweKey<ModularInt>& key,
                     absl::string_view prng_seed, ssize_t num_parts,
                     Uint64 log_decomposition_modulus,
                     Uint64 substitution_power,
                     ModularInt decomposition_modulus,
                     std::vector<RelinearizationKeyPart> relinearization_key);

  // Dimension of the relinearization key matrix.
  int dimension_;

  // Number of parts the key corresponds to.
  int num_parts_;

  const Uint64 log_decomposition_modulus_;

  ModularInt decomposition_modulus_;

  // Substitution power.
  int substitution_power_;

  // Modulus parameters. Does not take ownership.
  const ModularIntParams* modulus_params_;

  // NTT parameters. Does not take ownership.
  const NttParameters<ModularInt>* ntt_params_;

  // The key-switching matrix. Each component in the vector is a
  // RelinearizationKeyPart: a 2 by dimension_ matrix corresponding to a single
  // power of the key. In this way, a RelinearizationKey for a length k
  // ciphertext can be used to transform ciphertext with any number of parts up
  // to k by only using items 0 to k-1 of the relinearization_key_.
  std::vector<RelinearizationKeyPart> relinearization_key_;

  // Prng seed
  std::string prng_seed_;
};

// Instantiations of RelinearizationKey with specific MontgomeryInt classes.
// If any new types are added, montgomery.h should be updated accordingly (such
// as ensuring BigInt is correctly specialized, etc.).
extern template class EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) RelinearizationKey<MontgomeryInt<Uint16>>;
extern template class EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) RelinearizationKey<MontgomeryInt<Uint32>>;
extern template class EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) RelinearizationKey<MontgomeryInt<Uint64>>;
extern template class EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) RelinearizationKey<MontgomeryInt<absl::uint128>>;
}  // namespace rlwe

#endif  // RLWE_RELINEARIZATION_KEY_H_
