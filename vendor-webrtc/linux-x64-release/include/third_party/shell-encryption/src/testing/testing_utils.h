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

// This file contains constants and utilities for testing RLWE operations.

#ifndef RLWE_TESTING_TESTING_UTILS_H_
#define RLWE_TESTING_TESTING_UTILS_H_

#include <cstdint>
#include <vector>

#include "constants.h"
#include "montgomery.h"
#include "polynomial.h"
#include "status_macros.h"
#include "symmetric_encryption.h"
#include "testing/testing_prng.h"

namespace rlwe {
namespace testing {

// Set constants.
const Uint64 kDefaultLogT = 2;
const Uint64 kDefaultT = (1 << kDefaultLogT) + 1;
const Uint64 kDefaultVariance = 8;
const Uint64 kCoeffs = kDegreeBound25;
const Uint64 kLogCoeffs = kLogDegreeBound25;
const Uint32 kModulus = kModulus25;

// Construct montgomery int parameters used for testing rlwe encryption and
// decryption functionality.
inline rlwe::StatusOr<std::unique_ptr<const MontgomeryIntParams<uint>>>
ConstructMontgomeryIntParams() {
  return MontgomeryIntParams<uint>::Create(kModulus);
}

// Sample a random plaintext.
template <typename ModularInt>
std::vector<typename ModularInt::Int> SamplePlaintext(
    Uint64 num_coeffs = kCoeffs, typename ModularInt::Int t = kDefaultT) {
  // Seed for the random number generator that is used to create test
  // plaintexts.
  unsigned int seed = 1;
  std::vector<typename ModularInt::Int> plaintext(num_coeffs);
  for (unsigned int i = 0; i < num_coeffs; i++) {
    Uint64 rand = rand_r(&seed);
    typename ModularInt::Int int_rand =
        static_cast<typename ModularInt::Int>(rand);
    plaintext[i] = int_rand % t;
  }
  return plaintext;
}

// Convert a vector of integers to a vector of montgomery integers.
template <typename ModularInt>
rlwe::StatusOr<std::vector<ModularInt>> ConvertToMontgomery(
    const std::vector<typename ModularInt::Int>& coeffs,
    const rlwe::MontgomeryIntParams<typename ModularInt::Int>* params14) {
  auto val = ModularInt::ImportZero(params14);
  std::vector<ModularInt> output(coeffs.size(), val);
  for (unsigned int i = 0; i < output.size(); i++) {
    RLWE_ASSIGN_OR_RETURN(output[i],
                          ModularInt::ImportInt(coeffs[i], params14));
  }
  return output;
}

template <typename ModularInt>
rlwe::StatusOr<Polynomial<ModularInt>> GenerateRandomPlaintextPolynomial(
    int num_coeffs, Uint64 log_t, const typename ModularInt::Params* params,
    const NttParameters<ModularInt>* ntt_params) {
  if (ntt_params->number_coeffs != num_coeffs) {
    return absl::InvalidArgumentError(
        "The number of coefficients does not match that of the ntt "
        "parameters.");
  }
  typename ModularInt::Int plaintext_modulus =
      (params->One() << log_t) + params->One();
  std::vector<typename ModularInt::Int> plaintext =
      SamplePlaintext<ModularInt>(num_coeffs, plaintext_modulus);
  RLWE_ASSIGN_OR_RETURN(std::vector<ModularInt> rands,
                        ConvertToMontgomery<ModularInt>(plaintext, params));
  return Polynomial<ModularInt>::ConvertToNtt(rands, ntt_params, params);
}

}  // namespace testing
}  // namespace rlwe

#endif  // RLWE_TESTING_TESTING_UTILS_H_
