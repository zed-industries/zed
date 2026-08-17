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

#ifndef THIRD_PARTY_CENTIPEDE_ROLLING_HASH_H_
#define THIRD_PARTY_CENTIPEDE_ROLLING_HASH_H_

#include <cstddef>
#include <cstdint>

namespace fuzztest::internal {

// Computes a rolling hash for a fixed-size window in a sequence of 32-bit ints.
// Inspired by https://en.wikipedia.org/wiki/Rolling_hash#Rabin_fingerprint.
//
// Objects of this class must be created as global or TLS.
// The typical non-test usage is to create on TLS.
// Which is why we pass `window_size` via a separate function.
// There is no CTOR, the objects are zero-initialized.
// We currently do not use a CTOR with absl::ConstInitType so that the objects
// can be declared as __thread.
// TODO(kcc): reconsider once we can use c++20; the current warning is
// error: constexpr constructor that does not initialize all members is a
// C++20 extension
class RollingHash {
 public:
  // Resets the object to use the specified window size.
  void Reset(size_t window_size) {
    hash_ = 0;
    multiplier_power_window_size_ = 1;
    for (size_t i = 0; i < window_size; ++i) {
      multiplier_power_window_size_ *= kMultiplier;
    }
  }

  // Updates the hash by adding `add` and removing `remove`.
  uint32_t Update(uint32_t hash, uint32_t add, uint32_t remove) const {
    // Intermediate computations are done in 64-bit.
    return hash * kMultiplier - remove * multiplier_power_window_size_ + add;
  }

  void Update(uint32_t add, uint32_t remove) {
    hash_ = Update(hash_, add, remove);
  }

  // Returns the hash as a 32-bit int.
  uint32_t Hash() const { return hash_; }

  // Test-only function to use as a slow but simple reference implementation.
  // Adds `add` to `hash`.
  static uint32_t TestOnlyUpdate(uint32_t hash, uint32_t add) {
    return hash * kMultiplier + add;
  }

 private:
  // A prime number less than 2**32 (https://t5k.org/lists/2small/0bit.html).
  static constexpr uint64_t kMultiplier = (1ULL << 32) - 267;

  uint32_t hash_;
  uint64_t multiplier_power_window_size_;  // kMultiplier ** window_size.
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_ROLLING_HASH_H_
