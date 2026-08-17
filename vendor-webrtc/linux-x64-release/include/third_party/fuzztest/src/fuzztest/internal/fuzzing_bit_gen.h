// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef FUZZTEST_FUZZTEST_INTERNAL_FUZZING_BIT_GEN_H_
#define FUZZTEST_FUZZTEST_INTERNAL_FUZZING_BIT_GEN_H_

#include <cstdint>
#include <limits>

#include "absl/base/fast_type_id.h"
#include "absl/numeric/bits.h"
#include "absl/numeric/int128.h"
#include "absl/random/bit_gen_ref.h"
#include "absl/types/span.h"

namespace fuzztest::internal {

/// FuzzingBitGen is a BitGen instance which uses the Abseil mock mechanisms
/// to return distribution specific variates based on the fuzz data stream.
///
/// It is perhaps useful to think of the data stream as a sequence of structured
/// variates with semantic meaning, rather than just values. Recombinations of,
/// and modifications to, the sequence are useful in exploring the behavior of
/// the code under test in ways where a mere random-number generator sequence
/// would not, as changing the seed mutates the entire sequence.
///
/// NOTE: The first 8 bytes of the fuzzed data stream may be used to seed an
/// internal pnrg which is used to generate random variates for calls which
/// are not captured through mockable Abseil random distribution methods
/// (for example, calls to std::shuffle(...)). Otherwise the data stream is
/// treated as a stream where the next value in the sequence maps to the output
/// of the next distribution method.  Note that the specific sequence generated
/// by a FuzzingBitGen may vary due to the underlying code paths and whether
/// implementation details change, such as adding support for new distributions,
/// etc.
///
/// When the data stream is exhausted, absl::MockingBitGen mockable calls will
/// continue to return an arbitrary legal value, typically the minimum or mean
/// value of the distribution.
///
/// This type is thread-compatible, but not thread-safe.
class FuzzingBitGen {
 public:
  // Create a FuzzingBitGen from an unowned fuzzed `data` source, which must
  // outlive the FuzzingBitGen instance.
  //
  // The first 8 bytes of the data stream are used to seed an internal URBG used
  // for calls which are not mockable.
  explicit FuzzingBitGen(absl::Span<const uint8_t> data_stream);

  // Disallow copy, assign, and move.
  FuzzingBitGen(const FuzzingBitGen&) = delete;
  FuzzingBitGen& operator=(const FuzzingBitGen&) = delete;
  FuzzingBitGen(FuzzingBitGen&&) = default;
  FuzzingBitGen& operator=(FuzzingBitGen&&) = default;

  using result_type = uint64_t;

  static constexpr result_type(min)() {
    return (std::numeric_limits<result_type>::min)();
  }

  static constexpr result_type(max)() {
    return (std::numeric_limits<result_type>::max)();
  }

  void seed(result_type seed_value = 0) {
    absl::uint128 tmp = seed_value;
    state_ = lcg(tmp + increment());
  }

  result_type operator()();

 private:
  // Minimal implementation of a PCG64 engine equivalent to xsl_rr_128_64.
  static inline constexpr absl::uint128 multiplier() {
    return absl::MakeUint128(0x2360ed051fc65da4, 0x4385df649fccf645);
  }
  static inline constexpr absl::uint128 increment() {
    return absl::MakeUint128(0x5851f42d4c957f2d, 0x14057b7ef767814f);
  }
  inline absl::uint128 lcg(absl::uint128 s) {
    return s * multiplier() + increment();
  }
  inline result_type mix(absl::uint128 state) {
    uint64_t h = absl::Uint128High64(state);
    uint64_t rotate = h >> 58u;
    uint64_t s = absl::Uint128Low64(state) ^ h;
    return absl::rotr(s, rotate);
  }

  // InvokeMock meets the requirements of absl::BitGenRef::InvokeMock.
  // This method detects whether the key has been registered as supported,
  // and, if so, returns a value derived from `data_stream_`.
  bool InvokeMock(absl::FastTypeIdType key_id, void* args_tuple, void* result);

  absl::Span<const uint8_t> data_stream_;  // Mock data stream.
  absl::uint128 state_ = 0;                // Internal URBG state.

  template <typename>
  friend struct ::absl::random_internal::DistributionCaller;  // for InvokeMock
  friend class ::absl::random_internal::MockHelpers;          // for InvokeMock
  friend class ::absl::BitGenRef;                             // for InvokeMock
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_FUZZING_BIT_GEN_H_
