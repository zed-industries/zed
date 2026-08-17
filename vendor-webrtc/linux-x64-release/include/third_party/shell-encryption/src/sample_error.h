/*
 * Copyright 2019 Google LLC.
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

#ifndef RLWE_SAMPLE_ERROR_H_
#define RLWE_SAMPLE_ERROR_H_

#include <cstdint>
#include <vector>

#include "absl/strings/str_cat.h"
#include "bits_util.h"
#include "constants.h"
#include "error_params.h"
#include "prng/prng.h"
#include "status_macros.h"
#include "statusor.h"

namespace rlwe {

// Samples a vector of coefficients from the centered binomial distribution
// with the specified variance. The RLWE proofs rely on
// sampling keys and error values from a discrete Gaussian distribution, but
// the NewHope paper [1] indicates that a centered binomial distribution is
// indistinguishable and is far more efficient, without being susceptible to
// timing attacks.
//
// [1] "Post-quantum key exchange -- a new hope", Erdem Alkim, Leo Ducas, Thomas
// Poppelmann, Peter Schwabe, USENIX Security Sumposium.
//
// All values sampled are multiplied by scalar.
template <typename ModularInt>
static rlwe::StatusOr<std::vector<ModularInt>> SampleFromErrorDistribution(
    unsigned int num_coeffs, Uint64 variance, SecurePrng* prng,
    const typename ModularInt::Params* modulus_params) {
  if (variance > kMaxVariance) {
    return absl::InvalidArgumentError(absl::StrCat(
        "The variance, ", variance, ", must be at most ", kMaxVariance, "."));
  }
  auto zero = ModularInt::ImportZero(modulus_params);
  std::vector<ModularInt> coeffs(num_coeffs, zero);

  // Sample from the centered binomial distribution. To do so, we sample k pairs
  // of bits (a, b), where k = 2 * variance. The sample of the binomial
  // distribution is the sum of the differences between each pair of bits.
  Uint64 k;
  typename ModularInt::Int coefficient;

  for (unsigned int i = 0; i < num_coeffs; i++) {
    coefficient = modulus_params->modulus;
    k = variance << 1;

    while (k > 0) {
      if (k >= 64) {
        // Use 64 bits of randomness
        RLWE_ASSIGN_OR_RETURN(auto r64, prng->Rand64());
        coefficient += rlwe::internal::CountOnes64(r64);
        RLWE_ASSIGN_OR_RETURN(r64, prng->Rand64());
        coefficient -= rlwe::internal::CountOnes64(r64);
        k -= 64;
      } else if (k >= 8) {
        // Use 8 bits of randomness
        RLWE_ASSIGN_OR_RETURN(auto r8, prng->Rand8());
        coefficient += rlwe::internal::CountOnesInByte(r8);
        RLWE_ASSIGN_OR_RETURN(r8, prng->Rand8());
        coefficient -= rlwe::internal::CountOnesInByte(r8);
        k -= 8;
      } else {
        Uint8 mask = (1 << k) - 1;
        RLWE_ASSIGN_OR_RETURN(auto r8, prng->Rand8());
        coefficient += rlwe::internal::CountOnesInByte(r8 & mask);
        RLWE_ASSIGN_OR_RETURN(r8, prng->Rand8());
        coefficient -= rlwe::internal::CountOnesInByte(r8 & mask);
        break;  // all k remaining pairs have been sampled.
      }
    }

    // coefficient is in the interval [modulus - 2k, modulus + 2k]. We reduce
    // it in [0, modulus). Since ModularInt::Int is unsigned, we create a mask
    // equal to 0xFF...FF when coefficient >= modulus, and equal to 0 otherwise.
    typename ModularInt::Int mask = -(coefficient >= modulus_params->modulus);
    coefficient -= mask & modulus_params->modulus;

    RLWE_ASSIGN_OR_RETURN(coeffs[i],
                          ModularInt::ImportInt(coefficient, modulus_params));
  }

  return coeffs;
}

}  // namespace rlwe

#endif  // RLWE_SAMPLE_ERROR_H_
