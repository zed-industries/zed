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

#ifndef THIRD_PARTY_CENTIPEDE_FOREACH_NONZERO_H_
#define THIRD_PARTY_CENTIPEDE_FOREACH_NONZERO_H_

// WARNING!!!: Be very careful with what STL headers or other dependencies you
// add here. This header needs to remain mostly bare-bones so that we can
// include it into runner.
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <functional>

namespace fuzztest::internal {

// Iterates over [bytes, bytes + num_bytes) and calls action(idx, bytes[idx]),
// for every non-zero bytes[idx]. Then clears those non-zero bytes.
// Optimized for the case where lots of bytes are zero.
inline void ForEachNonZeroByte(uint8_t *bytes, size_t num_bytes,
                               std::function<void(size_t, uint8_t)> action) {
  // The main loop will read words of this size.
  constexpr uintptr_t kWordSize = sizeof(uintptr_t);
  const uintptr_t initial_alignment =
      reinterpret_cast<uintptr_t>(bytes) % kWordSize;
  size_t idx = 0;
  uintptr_t alignment = initial_alignment;
  // Iterate the first few until we reach alignment by word size.
  for (; idx < num_bytes && alignment != 0;
       idx++, alignment = (alignment + 1) % kWordSize) {
    if (bytes[idx]) {
      action(idx, bytes[idx]);
      bytes[idx] = 0;
    }
  }
  // Iterate one word at a time. If the word is != 0, iterate its bytes.
  for (; idx + kWordSize - 1 < num_bytes; idx += kWordSize) {
    uintptr_t wide_load;
    __builtin_memcpy(&wide_load, bytes + idx, kWordSize);  // force inline.
    if (!wide_load) continue;
    __builtin_memset(bytes + idx, 0, kWordSize);  // // force inline.
    // This loop assumes little-endianness. (Tests will break on big-endian).
    for (size_t pos = 0; pos < kWordSize; pos++) {
      uint8_t value = wide_load >> (pos * 8);  // lowest byte is taken.
      if (value) action(idx + pos, value);
    }
  }
  // Iterate the last few.
  for (; idx < num_bytes; idx++) {
    if (bytes[idx]) {
      action(idx, bytes[idx]);
      bytes[idx] = 0;
    }
  }
}

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_FOREACH_NONZERO_H_
