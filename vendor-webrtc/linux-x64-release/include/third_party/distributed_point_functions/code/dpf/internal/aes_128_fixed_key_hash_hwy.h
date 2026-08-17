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

// Highway-specific include guard, ensuring the header can get included once per
// target architecture.
#if defined(                                                                   \
    DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_AES_128_FIXED_KEY_HASH_HWY_H_) == \
    defined(HWY_TARGET_TOGGLE)
#ifdef DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_AES_128_FIXED_KEY_HASH_HWY_H_
#undef DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_AES_128_FIXED_KEY_HASH_HWY_H_
#else
#define DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_AES_128_FIXED_KEY_HASH_HWY_H_
#endif

#include <limits>

#include "absl/numeric/int128.h"
#include "hwy/highway.h"

HWY_BEFORE_NAMESPACE();
namespace distributed_point_functions {
namespace dpf_internal {
namespace HWY_NAMESPACE {

// There is no AES support on HWY_SCALAR, but we still want to be able to
// include this header when compiling for HWY_SCALAR. The caller has to make
// sure to only call the functions defined here when not on HWY_SCALAR.
#if HWY_TARGET != HWY_SCALAR

namespace hn = hwy::HWY_NAMESPACE;

constexpr int kAesBlockSize = 16;

// Helper to convert a Highway tag to a tag for vectors of the same bit size,
// but with 64-bit lanes.
template <typename D>
constexpr auto To64(D d) {
  return hn::Repartition<uint64_t, D>();
}

// The following macros define parts of the fixed-key AES hash function
// implementation. We use macros here since Highway doesn't allow creating
// arrays of vectors/SIMD registers. That way, we can access each register by a
// unique variable name. All inputs and outputs are assumed to be of type
// hn::ScalableTag<uint8_t>.

// Loads the AES round key for the given round and key_index.
#define DPF_AES_LOAD_ROUND_KEY(key_index, round) \
  const auto round_##round##_key_##key_index =   \
      hn::LoadDup128(d, round_keys_##key_index + kAesBlockSize * round);

// Selects key_0 or key_1 for the given block_index and round, depending on the
// bits in `mask`. Keys are first converted to 64-bit vectors to apply the more
// efficient 64 bit masks.
#define DPF_AES_SELECT_KEY(block_index, round)                         \
  const auto selected_round_##round##_key_##block_index = hn::BitCast( \
      d, hn::IfThenElse(mask_##block_index,                            \
                        hn::BitCast(To64(d), round_##round##_key_1),   \
                        hn::BitCast(To64(d), round_##round##_key_0)));

// Load mask for computing {0, x.high64}, for computing sigma(x) below.
HWY_ALIGN constexpr absl::uint128 kSigmaMask =
    absl::MakeUint128(std::numeric_limits<uint64_t>::max(), 0);
#define DPF_AES_LOAD_SIGMA_MASK() \
  const auto sigma_mask =         \
      hn::LoadDup128(To64(d), reinterpret_cast<const uint64_t*>(&kSigmaMask));

// Compute sigma(x) = {x.high64, x.high64^x.low64} (in little-endian notation).
#define DPF_AES_COMPUTE_SIGMA(block_index)                                   \
  const auto in_##block_index##_64 = hn::BitCast(To64(d), in_##block_index); \
  const auto sigma_##block_index =                                           \
      hn::BitCast(d, hn::Xor(hn::Shuffle01(in_##block_index##_64),           \
                             hn::And(sigma_mask, in_##block_index##_64)));

// Performs the first round of AES for the given block_index, using sigma as the
// input.
#define DPF_AES_FIRST_ROUND(block_index) \
  out_##block_index =                    \
      hn::Xor(sigma_##block_index, selected_round_0_key_##block_index)

// Performs a middle round of AES for the given block_index.
#define DPF_AES_MIDDLE_ROUND(block_index, round) \
  out_##block_index = hn::AESRound(              \
      out_##block_index, selected_round_##round##_key_##block_index);

// Performs the last round of AES for the given block_index.
#define DPF_AES_LAST_ROUND(block_index)                   \
  out_##block_index = hn::AESLastRound(out_##block_index, \
                                       selected_round_10_key_##block_index);

// Finalize the hash by XORing with sigma.
#define DPF_AES_FINALIZE_HASH(block_index) \
  out_##block_index = hn::Xor(out_##block_index, sigma_##block_index);

// Helper macro for hashing a single vector.
#define DPF_AES_MIDDLE_ROUND_1(round) \
  DPF_AES_LOAD_ROUND_KEY(0, round);   \
  DPF_AES_LOAD_ROUND_KEY(1, round);   \
  DPF_AES_SELECT_KEY(0, round);       \
  DPF_AES_MIDDLE_ROUND(0, round);

// Hashes a vector `in_0`, writing the output to `out_0`. Each block is hashed
// using either `round_keys_0` or `round_keys_1`, which both must point to a
// byte array containing two expanded AES keys. Which key is used for each block
// depends on `mask_0`: If the mask 0, then `round_keys_0` is used, otherwise
// `round_keys_1`. Note that the masks are masks on 64 bit integers, so there
// are two mask bits per AES block. The caller is responsible for making sure
// that the masks for the two halves of any given block have the same value.
template <typename V, typename D, typename M>
void HashOneWithKeyMask(D d, V in_0, M mask_0,
                        const uint8_t* HWY_RESTRICT round_keys_0,
                        const uint8_t* HWY_RESTRICT round_keys_1, V& out_0) {
  // Compute sigma(in_0)
  DPF_AES_LOAD_SIGMA_MASK();
  DPF_AES_COMPUTE_SIGMA(0);

  // First AES round.
  DPF_AES_LOAD_ROUND_KEY(0, 0);
  DPF_AES_LOAD_ROUND_KEY(1, 0);
  DPF_AES_SELECT_KEY(0, 0);
  DPF_AES_FIRST_ROUND(0);

  // Middle AES rounds.
  DPF_AES_MIDDLE_ROUND_1(1);
  DPF_AES_MIDDLE_ROUND_1(2);
  DPF_AES_MIDDLE_ROUND_1(3);
  DPF_AES_MIDDLE_ROUND_1(4);
  DPF_AES_MIDDLE_ROUND_1(5);
  DPF_AES_MIDDLE_ROUND_1(6);
  DPF_AES_MIDDLE_ROUND_1(7);
  DPF_AES_MIDDLE_ROUND_1(8);
  DPF_AES_MIDDLE_ROUND_1(9);

  // Last AES round.
  DPF_AES_LOAD_ROUND_KEY(0, 10);
  DPF_AES_LOAD_ROUND_KEY(1, 10);
  DPF_AES_SELECT_KEY(0, 10)
  DPF_AES_LAST_ROUND(0);

  // Finalize hash.
  DPF_AES_FINALIZE_HASH(0);
}

// Helper macros for hashing four vectors in parallel.
#define DPF_AES_SELECT_KEY_4(round) \
  DPF_AES_SELECT_KEY(0, round);     \
  DPF_AES_SELECT_KEY(1, round);     \
  DPF_AES_SELECT_KEY(2, round);     \
  DPF_AES_SELECT_KEY(3, round);
#define DPF_AES_MIDDLE_ROUND_4(round) \
  DPF_AES_LOAD_ROUND_KEY(0, round);   \
  DPF_AES_LOAD_ROUND_KEY(1, round);   \
  DPF_AES_SELECT_KEY_4(round);        \
  DPF_AES_MIDDLE_ROUND(0, round);     \
  DPF_AES_MIDDLE_ROUND(1, round);     \
  DPF_AES_MIDDLE_ROUND(2, round);     \
  DPF_AES_MIDDLE_ROUND(3, round);

// Hashes four vectors `in_0, ..., in_3`, writing the results to `out_0, ...,
// out_3`. This improves pipelining of AES instructions, and improves
// performance by about 10%. Each block is hashed using either `round_keys_0` or
// `round_keys_1`, which both must point to a byte array containing two expanded
// AES keys. Which key is used for each block depends on `mask_0, ... mask_3`:
// If the mask 0, then `round_keys_0` is used, otherwise `round_keys_1`. Note
// that the masks are masks on 64 bit integers, so there are two mask bits per
// AES block. The caller is responsible for making sure that the masks for the
// two halves of any given block have the same value.
template <typename V, typename D, typename M>
void HashFourWithKeyMask(D d, V in_0, V in_1, V in_2, V in_3, M mask_0,
                         M mask_1, M mask_2, M mask_3,
                         const uint8_t* HWY_RESTRICT round_keys_0,
                         const uint8_t* HWY_RESTRICT round_keys_1, V& out_0,
                         V& out_1, V& out_2, V& out_3) {
  // Compute sigma(in_0), ..., sigma(in_3)
  DPF_AES_LOAD_SIGMA_MASK();
  DPF_AES_COMPUTE_SIGMA(0);
  DPF_AES_COMPUTE_SIGMA(1);
  DPF_AES_COMPUTE_SIGMA(2);
  DPF_AES_COMPUTE_SIGMA(3);

  // First AES round.
  DPF_AES_LOAD_ROUND_KEY(0, 0);
  DPF_AES_LOAD_ROUND_KEY(1, 0);
  DPF_AES_SELECT_KEY_4(0)
  DPF_AES_FIRST_ROUND(0);
  DPF_AES_FIRST_ROUND(1);
  DPF_AES_FIRST_ROUND(2);
  DPF_AES_FIRST_ROUND(3);

  // Middle AES rounds.
  DPF_AES_MIDDLE_ROUND_4(1);
  DPF_AES_MIDDLE_ROUND_4(2);
  DPF_AES_MIDDLE_ROUND_4(3);
  DPF_AES_MIDDLE_ROUND_4(4);
  DPF_AES_MIDDLE_ROUND_4(5);
  DPF_AES_MIDDLE_ROUND_4(6);
  DPF_AES_MIDDLE_ROUND_4(7);
  DPF_AES_MIDDLE_ROUND_4(8);
  DPF_AES_MIDDLE_ROUND_4(9);

  // Last AES round.
  DPF_AES_LOAD_ROUND_KEY(0, 10);
  DPF_AES_LOAD_ROUND_KEY(1, 10);
  DPF_AES_SELECT_KEY_4(10)
  DPF_AES_LAST_ROUND(0);
  DPF_AES_LAST_ROUND(1);
  DPF_AES_LAST_ROUND(2);
  DPF_AES_LAST_ROUND(3);

  // Finalize hash.
  DPF_AES_FINALIZE_HASH(0);
  DPF_AES_FINALIZE_HASH(1);
  DPF_AES_FINALIZE_HASH(2);
  DPF_AES_FINALIZE_HASH(3);
}

#endif  // HWY_TARGET != HWY_SCALAR

}  // namespace HWY_NAMESPACE
}  // namespace dpf_internal
}  // namespace distributed_point_functions
HWY_AFTER_NAMESPACE();

#endif  // DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_AES_128_FIXED_KEY_HASH_HWY_H_
