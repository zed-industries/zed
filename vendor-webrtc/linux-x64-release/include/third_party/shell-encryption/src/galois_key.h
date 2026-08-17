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

#ifndef RLWE_GALOIS_KEY_H_
#define RLWE_GALOIS_KEY_H_

#include <cstdint>
#include <vector>

#include "absl/strings/str_cat.h"
#include "relinearization_key.h"
#include "status_macros.h"
#include "statusor.h"
#include "third_party/shell-encryption/base/shell_encryption_export.h"
#include "third_party/shell-encryption/base/shell_encryption_export_template.h"

namespace rlwe {

// Implements a GaloisKey, a type of key-switching matrix that transforms a
// ciphertext encrypted with (1, s(x^substitution_power)) to a ciphertext that
// encrypts the same message under the canonical secret key (1, s). This can be
// viewed as a RelinearizationKey of length two and a substitution_power > 1. A
// GaloisKey can only be applied to ciphertexts whose PowerOfS exactly matches
// the substitution_power.
//
// GaloisKeys are constructed based on the secret key. Two GaloisKeys that
// correspond to the same secret key, substitution power, and use the same
// decomposition modulus will not necessarily be equal. This is due to
// randomness that is sampled when a GaloisKey is created. However, either
// GaloisKey may be used to key-switch the same ciphertext.
//
// Details can be found in Appendix D.2 of https://eprint.iacr.org/2011/566.pdf
template <typename ModularInt>
class EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) GaloisKey {
 public:
  // Initializes a GaloisKey based on a SymmetricRlweKey key that can key-switch
  // two component ciphertexts. A positive log_decomposition_modulus corresponds
  // to the decomposition modulus T. The substitution_power corresponds to the
  // power of x in the secret key polynomial s(x^substitution_power) that the
  // ciphertext is encrypted with. The prng_seed is used to generate and encode
  // the bottom row of the matrix, which consists of random entries.
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<GaloisKey> Create(
      const SymmetricRlweKey<ModularInt>& key, absl::string_view prng_seed,
      Uint64 substitution_power, Uint64 log_decomposition_modulus) {
    RLWE_ASSIGN_OR_RETURN(auto relinearization_key,
                          RelinearizationKey<ModularInt>::Create(
                              key, prng_seed, /*num_parts=*/2,
                              log_decomposition_modulus, substitution_power));
    return GaloisKey(std::move(relinearization_key));
  }

  // Takes a SymmetricRlweCiphertext with 2 components encrypted under
  // s(x^{substitution_power}) and returns a 2 component SymmetricRlweCiphertext
  // encoding the same message. The PowerOfS of the ciphertext is updated to 1.
  // Returns an error when the number of components is larger than 2.
  rlwe::StatusOr<SymmetricRlweCiphertext<ModularInt>> ApplyTo(
      const SymmetricRlweCiphertext<ModularInt>& ciphertext) const {
    if (ciphertext.PowerOfS() != SubstitutionPower()) {
      return absl::InvalidArgumentError(absl::StrCat(
          "Ciphertext PowerOfS: ", ciphertext.PowerOfS(),
          " doesn't match the key substitution power: ", SubstitutionPower()));
    }

    return relinearization_key_.ApplyTo(ciphertext);
  }

  // Returns a SerializedGaloisKey containing a representation of the
  // key-switching matrix and the power of s that corresponds to this
  // key-switching matrix.
  rlwe::StatusOr<SerializedGaloisKey> Serialize() const {
    SerializedGaloisKey output;
    RLWE_ASSIGN_OR_RETURN(*output.mutable_key(),
                          relinearization_key_.Serialize());
    return output;
  }

  // Requires that the number of NTT Polynomials in the key field of the
  // SerializedGaloisKey is (2 * num_parts * dimension) where dimension is the
  // number of digits needed to represent the modulus in base
  // 2^{log_decomposition_modulus}. Crashes for non-valid input parameters.
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<GaloisKey> Deserialize(
      const SerializedGaloisKey& serialized,
      const typename ModularInt::Params* modulus_params,
      const NttParameters<ModularInt>* ntt_params) {
    RLWE_ASSIGN_OR_RETURN(RelinearizationKey<ModularInt> key,
                          RelinearizationKey<ModularInt>::Deserialize(
                              serialized.key(), modulus_params, ntt_params));
    return GaloisKey(std::move(key));
  }
  // Substitution Power accessor.
  int SubstitutionPower() const {
    return relinearization_key_.SubstitutionPower();
  }

 private:
  GaloisKey(RelinearizationKey<ModularInt> relinearization_key)
      : relinearization_key_(std::move(relinearization_key)) {}

  // A relinearization key.
  RelinearizationKey<ModularInt> relinearization_key_;
};

template class EXPORT_TEMPLATE_DECLARE(
    SHELL_ENCRYPTION_EXPORT) GaloisKey<rlwe::MontgomeryInt<Uint16>>;
template class EXPORT_TEMPLATE_DECLARE(
    SHELL_ENCRYPTION_EXPORT) GaloisKey<rlwe::MontgomeryInt<Uint32>>;
template class EXPORT_TEMPLATE_DECLARE(
    SHELL_ENCRYPTION_EXPORT) GaloisKey<rlwe::MontgomeryInt<Uint64>>;
template class EXPORT_TEMPLATE_DECLARE(
    SHELL_ENCRYPTION_EXPORT) GaloisKey<rlwe::MontgomeryInt<absl::uint128>>;
}  //  namespace rlwe

#endif  // RLWE_GALOIS_KEY_H_
