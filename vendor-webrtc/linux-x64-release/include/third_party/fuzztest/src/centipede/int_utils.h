// Copyright 2023 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_CENTIPEDE_INT_UTILS_H_
#define THIRD_PARTY_CENTIPEDE_INT_UTILS_H_

#include <cstdint>

namespace fuzztest::internal {

// Computes a hash of `bits`. The purpose is to use the result for XOR-ing with
// some other values, such that all resulting bits look random.
inline uint64_t Hash64Bits(uint64_t bits) {
  // This particular prime number seems to mix bits well.
  // TODO(kcc): find a more scientific way to mix bits, e.g. switch to Murmur.
  constexpr uint64_t kPrime = 13441014529ULL;
  return bits * kPrime;
}

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_INT_UTILS_H_
