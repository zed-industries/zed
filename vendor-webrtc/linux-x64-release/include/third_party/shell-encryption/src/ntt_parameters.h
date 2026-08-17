/*
 * Copyright 2017 Google LLC.
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
#ifndef RLWE_NTT_PARAMETERS_H_
#define RLWE_NTT_PARAMETERS_H_

#include <algorithm>
#include <cstdlib>
#include <vector>

#include "absl/memory/memory.h"
#include "absl/strings/str_cat.h"
#include "constants.h"
#include "status_macros.h"
#include "statusor.h"
#include "third_party/shell-encryption/base/shell_encryption_export.h"

namespace rlwe {
namespace internal {

// Fill row with every power in {0, 1, ..., n-1} (mod modulus) of base .
template <typename ModularInt>
void FillWithEveryPower(const ModularInt& base, unsigned int n,
                        std::vector<ModularInt>* row,
                        const typename ModularInt::Params* params) {
  for (unsigned int i = 0; i < n; i++) {
    (*row)[i].AddInPlace(base.ModExp(i, params), params);
  }
}

template <typename ModularInt>
rlwe::StatusOr<ModularInt> PrimitiveNthRootOfUnity(
    unsigned int log_n, const typename ModularInt::Params* params) {
  typename ModularInt::Int n = params->One() << log_n;
  typename ModularInt::Int half_n = n >> 1;

  // When the modulus is prime, the value k is a power such that any number
  // raised to it will be a n-th root of unity. (It will not necessarily be a
  // *primitive* root of unity, however).
  typename ModularInt::Int k = (params->modulus - params->One()) / n;

  // Test each number t to see whether t^k is a primitive n-th root
  // of unity - that t^{nk} is a root of unity but t^{(n/2)k} is not.
  ModularInt one = ModularInt::ImportOne(params);
  for (typename ModularInt::Int t = params->Two(); t < params->modulus;
       t = t + params->One()) {
    // Produce a candidate root of unity.
    RLWE_ASSIGN_OR_RETURN(auto mt, ModularInt::ImportInt(t, params));
    ModularInt candidate = mt.ModExp(k, params);

    // Check whether candidate^half_n = 1. If not, it is a primitive root of
    // unity.
    if (candidate.ModExp(half_n, params) != one) {
      return candidate;
    }
  }

  // Failure state. The above loop should always return successfully assuming
  // the parameters were set properly.
  return absl::UnknownError("Loop in PrimitiveNthRootOfUnity terminated.");
}

// Let psi be a primitive 2n-th root of unity, i.e., a 2n-th root of unity such
// that psi^n = -1. When performing the NTT transformation, the powers of psi in
// bitreversed order are needed. The vector produced by this helper function
// contains the powers of psi (psi^0, psi^1, psi^2, ..., psi^(n-1)).
//
// Each item of the vector is in modular integer representation.
template <typename ModularInt>
rlwe::StatusOr<std::vector<ModularInt>> NttPsis(
    unsigned int log_n, const typename ModularInt::Params* params) {
  // Obtain psi, a primitive 2n-th root of unity (hence log_n + 1).
  RLWE_ASSIGN_OR_RETURN(
      ModularInt psi,
      internal::PrimitiveNthRootOfUnity<ModularInt>(log_n + 1, params));
  unsigned int n = 1 << log_n;
  ModularInt zero = ModularInt::ImportZero(params);
  // Create a vector with the powers of psi.
  std::vector<ModularInt> row(n, zero);
  internal::FillWithEveryPower<ModularInt>(psi, n, &row, params);
  return row;
}

// Creates a vector containing the indices necessary to perform the NTT bit
// reversal operation. Index i of the returned vector contains an integer with
// the rightmost log_n bits of i reversed.
SHELL_ENCRYPTION_EXPORT std::vector<unsigned int> BitrevArray(unsigned int log_n);

// Helper function: Perform the bit-reversal operation in-place on coeffs_.
template <typename ModularInt>
static void BitrevHelper(const std::vector<unsigned int>& bitrevs,
                         std::vector<ModularInt>* item_to_reverse) {
  using std::swap;
  for (int i = 0; i < item_to_reverse->size(); i++) {
    // Only swap in one direction - don't accidentally swap twice.
    unsigned int r = bitrevs[i];
    if (static_cast<unsigned int>(i) < r) {
      swap((*item_to_reverse)[i], (*item_to_reverse)[r]);
    }
  }
}

}  // namespace internal

// The precomputed roots of unity used during the forward NTT are the
// bitreversed powers of the primitive 2n-th root of unity.
template <typename ModularInt>
rlwe::StatusOr<std::vector<ModularInt>> NttPsisBitrev(
    unsigned int log_n, const typename ModularInt::Params* params) {
  // Retrieve the table for the forward transformation.
  RLWE_ASSIGN_OR_RETURN(std::vector<ModularInt> psis,
                        internal::NttPsis<ModularInt>(log_n, params));
  // Bitreverse the vector.
  internal::BitrevHelper(internal::BitrevArray(log_n), &psis);
  return psis;
}

// The precomputed roots of unity used during the inverse NTT are the inverses
// of the bitreversed powers of the primitive 2n-th root of unity plus 1.
template <typename ModularInt>
rlwe::StatusOr<std::vector<ModularInt>> NttPsisInvBitrev(
    unsigned int log_n, const typename ModularInt::Params* params) {
  // Retrieve the table for the forward transformation.
  RLWE_ASSIGN_OR_RETURN(std::vector<ModularInt> row,
                        internal::NttPsis<ModularInt>(log_n, params));

  // Reverse the items at indices 1 through (n - 1). Multiplying index i
  // of the reversed row by index i of the original row will yield psi^n = -1.
  // (The exception is psi^0 = 1, which is already its own inverse.)
  std::reverse(row.begin() + 1, row.end());

  // Get the inverse of psi
  ModularInt psi_inv = row[1].Negate(params);
  ModularInt negative_psi_inv = row[1];

  // Bitreverse the vector.
  internal::BitrevHelper(internal::BitrevArray(log_n), &row);

  // Finally, multiply each of the items at indices 1 to (n-1) by -1. Multiply
  // every entry by psi_inv.
  row[0].MulInPlace(psi_inv, params);
  for (int i = 1; i < row.size(); i++) {
    row[i].MulInPlace(negative_psi_inv, params);
  }

  return row;
}

// A struct that stores a package of NTT Parameters
template <typename ModularInt>
struct NttParameters {
  NttParameters() = default;
  // Disallow copy and copy-assign, allow move and move-assign.
  NttParameters(const NttParameters<ModularInt>&) = delete;
  NttParameters& operator=(const NttParameters<ModularInt>&) = delete;
  NttParameters(NttParameters<ModularInt>&&) = default;
  NttParameters& operator=(NttParameters<ModularInt>&&) = default;
  ~NttParameters() = default;

  int number_coeffs;
  std::optional<ModularInt> n_inv_ptr;
  std::vector<ModularInt> psis_bitrev;
  std::vector<ModularInt> psis_inv_bitrev;
  std::vector<unsigned int> bitrevs;
};

// A convenient function that sets up all NTT parameters at once.
// Does not take ownership of params.
template <typename ModularInt>
rlwe::StatusOr<NttParameters<ModularInt>> InitializeNttParameters(
    int log_n, const typename ModularInt::Params* params) {
  // Abort if log_n is non-positive.
  if (log_n <= 0) {
    return absl::InvalidArgumentError("log_n must be positive");
  } else if (static_cast<Uint64>(log_n) > kMaxLogNumCoeffs) {
    return absl::InvalidArgumentError(absl::StrCat(
        "log_n, ", log_n, ", must be less than ", kMaxLogNumCoeffs, "."));
  }

  if (!ModularInt::Params::DoesLogNFit(log_n)) {
    return absl::InvalidArgumentError(
        absl::StrCat("log_n, ", log_n,
                     ", does not fit into underlying ModularInt::Int type."));
  }

  NttParameters<ModularInt> output;

  output.number_coeffs = 1 << log_n;
  typename ModularInt::Int two_times_n = params->One() << (log_n + 1);

  if (params->modulus % two_times_n != params->One()){
    return absl::InvalidArgumentError(
        absl::StrCat("modulus is not 1 mod 2n for logn, ", log_n));
  }

  // Compute the inverse of n.
  typename ModularInt::Int n = params->One() << log_n;
  RLWE_ASSIGN_OR_RETURN(auto mn, ModularInt::ImportInt(n, params));
  output.n_inv_ptr = mn.MultiplicativeInverse(params);

  RLWE_ASSIGN_OR_RETURN(output.psis_bitrev,
                        NttPsisBitrev<ModularInt>(log_n, params));
  RLWE_ASSIGN_OR_RETURN(output.psis_inv_bitrev,
                        NttPsisInvBitrev<ModularInt>(log_n, params));
  output.bitrevs = internal::BitrevArray(log_n);

  return output;
}

}  // namespace rlwe

#endif  // RLWE_NTT_PARAMETERS_H_
