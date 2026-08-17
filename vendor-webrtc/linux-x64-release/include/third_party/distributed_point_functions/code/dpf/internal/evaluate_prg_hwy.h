/*
 * Copyright 2021 Google LLC
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

#ifndef DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_EXPAND_SEEDS_HWY_H_
#define DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_EXPAND_SEEDS_HWY_H_

#include <stdint.h>

#include "absl/numeric/int128.h"
#include "absl/status/status.h"
#include "dpf/aes_128_fixed_key_hash.h"

namespace distributed_point_functions {
namespace dpf_internal {

using distributed_point_functions::Aes128FixedKeyHash;

// Extracts the lowest bit of `x` and sets it to 0 in `x`.
inline bool ExtractAndClearLowestBit(absl::uint128& x) {
  bool bit = ((x & absl::uint128{1}) != 0);
  x &= ~absl::uint128{1};
  return bit;
}

// Performs DPF evaluation of the seeds given in `seeds_in` using `prg_left` or
// `prg_right, and the given `control_bits_in`, and correction words given by
// `correction_seeds`, `correction_controls_left`, and
// `correction_controls_right`. At each level `l < num_level`, the evaluation
// for the i-th seed continues along the left or right path depending on the
// l-th most significant bit among the lowest `num_levels` bits of `paths[i]`,
// after right-shifting each `paths[i]` by `paths_rightshift`.
//
// This function takes raw pointers instead of absl::Span for performance
// reasons. No bounds checks are performed, so it is the caller's responsibility
// to ensure that
// - `seeds_in`, `control_bits_in`, `seeds_out`, and `control_bits_out` have at
//   least `num_seeds` elements, and
// - `correction_seeds`, `correction_controls_left`, and
//   `correction_controls_right` have at least `num_levels` elements.
//
// If the inputs are aligned (e.g. using HWY_ALIGN, or hwy::AllocateAligned),
// and if SIMD operations are supported, then the evaluation will be done using
// SIMD operations. Otherwise, falls back to `EvaluateSeedsNoHwy`, which is at
// least 2x slower.
//
// `num_correction_words` can either be equal to `num_levels`, or equal to
// `num_seeds * num_levels`. In the first case, the same correction word is used
// for every seed at a given level. In the second case, correction word at index
// `i * num_seeds + j` is used to correct seed `i` at level `j`.
// If `num_correction_words == num_seeds * num_levels`, then `num_seeds` should
// be smaller than or divisible by the size of a SIMD vector for optimal
// performance.
//
// Returns OK on success, INVALID_ARGUMENT in case num_correction_words is not
// equal to `num_levels` or `num_seeds * num_levels`, and INTERNAL in case of
// OpenSSL errors.
absl::Status EvaluateSeeds(
    int64_t num_seeds, int num_levels, int num_correction_words,
    const absl::uint128* seeds_in, const bool* control_bits_in,
    const absl::uint128* paths, int paths_rightshift,
    const absl::uint128* correction_seeds, const bool* correction_controls_left,
    const bool* correction_controls_right, const Aes128FixedKeyHash& prg_left,
    const Aes128FixedKeyHash& prg_right, absl::uint128* seeds_out,
    bool* control_bits_out);

// As `EvaluateSeeds`, but does not require any SIMD support.
absl::Status EvaluateSeedsNoHwy(
    int64_t num_seeds, int num_levels, int num_correction_words,
    const absl::uint128* seeds_in, const bool* control_bits_in,
    const absl::uint128* paths, int paths_rightshift,
    const absl::uint128* correction_seeds, const bool* correction_controls_left,
    const bool* correction_controls_right, const Aes128FixedKeyHash& prg_left,
    const Aes128FixedKeyHash& prg_right, absl::uint128* seeds_out,
    bool* control_bits_out);

}  // namespace dpf_internal
}  // namespace distributed_point_functions

#endif  // DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_EXPAND_SEEDS_HWY_H_
