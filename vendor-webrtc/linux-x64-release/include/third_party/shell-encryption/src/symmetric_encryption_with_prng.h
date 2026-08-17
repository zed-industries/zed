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

// Implementation of compressed ciphertexts.
//
// In the RLWE encryption scheme, the second component of the ciphertext, a, is
// drawn uniformly at random (that is, each coefficient is drawn uniformly at
// random). If we the second components of a series of ciphertexts are
// constructed using the pseudorandom outputs of a PRNG, the second components
// can be encoded using the PRNG's seed, which is the functionality provided
// here.
//
// An important note is that the order of encrypting the ciphertexts is
// important. When decompressing, the order of the ciphertexts must be given in
// the exact same order that the ciphertexts were encrypted to ensure the same
// random polynomials are constructed for the correct ciphertext. Since a party
// without the private key will be decompressing, it is impossible to check if
// the decompressions were performed properly.

#ifndef RLWE_SYMMETRIC_ENCRYPTION_WITH_PRNG_H_
#define RLWE_SYMMETRIC_ENCRYPTION_WITH_PRNG_H_

#include <vector>

#include "polynomial.h"
#include "prng/integral_prng_types.h"
#include "prng/prng.h"
#include "status_macros.h"
#include "statusor.h"
#include "symmetric_encryption.h"

namespace rlwe {

// Encrypts a set of plaintexts using randomness-of-encryption sampled using the
// specified PRNG.
//
// When encrypting a plaintext, the c0 component of a ciphertext incorporates a
// polynomial "a" where each coefficient is drawn uniformly and independently at
// random. The c1 component of the ciphertext is exactly "-a" to ensure
// homomorphic operations and decryption may occur. Instead of sending "a"
// explicitly, we may construct "a" using the seed of a pseudorandom number
// generator. Then, we may simply send the seed used to generate all "a"s
// together with only the c0 components of each ciphertext. This would allow the
// server to replay the PRNG and recover all "a"s and thereby reconstruct the
// c1 components of the ciphertexts. As a result, the required communication
// would be cut almost in half.
//
// Note, the c0 components of the ciphertext have an additional error vector
// added to them. This error vector is drawn separately from the PRNG used to
// compute all "a"s. The error vector remaining hidden is important for
// security.
//
// Importantly, the order of the compressed ciphertexts must remain static
// before decompression. Otherwise, the decompression will be incorrect.
//
// Two PRNGs are being passed: one is used to sample random polynomials that
// is required to be used for decryption. The other PRNG is used to sample error
// that does not need to be used for decryption.
template <typename ModularInt>
rlwe::StatusOr<std::vector<Polynomial<ModularInt>>> EncryptWithPrng(
    const SymmetricRlweKey<ModularInt>& key,
    const std::vector<Polynomial<ModularInt>>& plaintexts, SecurePrng* prng,
    SecurePrng* prng_encryption) {
  std::vector<Polynomial<ModularInt>> c0s(plaintexts.size());
  for (int i = 0; i < c0s.size(); ++i) {
    RLWE_ASSIGN_OR_RETURN(auto a, SamplePolynomialFromPrng<ModularInt>(
                                      key.Len(), prng, key.ModulusParams()));
    RLWE_ASSIGN_OR_RETURN(
        c0s[i], internal::Encrypt(key, plaintexts[i], a, prng_encryption));
  }
  return c0s;
}

// Given a list of compressed encryption and the seed of the PRNG used for
// compression, this function will decompress the encryptions so the ciphertexts
// can now be used in homomorphic operations.
//
// The compressed ciphertexts must be given in the same order as they were given
// to be encrypted.
template <typename ModularInt>
rlwe::StatusOr<std::vector<SymmetricRlweCiphertext<ModularInt>>> ExpandFromPrng(
    std::vector<Polynomial<ModularInt>> c0,
    const typename ModularInt::Params* modulus_params,
    const NttParameters<ModularInt>* ntt_params,
    const ErrorParams<ModularInt>* error_params, SecurePrng* prng) {
  std::vector<SymmetricRlweCiphertext<ModularInt>> ciphertexts;
  for (int i = 0; i < c0.size(); ++i) {
    RLWE_ASSIGN_OR_RETURN(auto a, SamplePolynomialFromPrng<ModularInt>(
                                      c0[i].Len(), prng, modulus_params));
    // Ciphertexts that can be expanded from PRNG must be fresh encryptions.
    ciphertexts.emplace_back(
        std::vector<Polynomial<ModularInt>>(
            {std::move(c0[i]), std::move(a.NegateInPlace(modulus_params))}),
        1, error_params->B_encryption(), modulus_params, error_params);
  }
  return ciphertexts;
}

}  // namespace rlwe

#endif  // RLWE_SYMMETRIC_ENCRYPTION_WITH_PRNG_H_
