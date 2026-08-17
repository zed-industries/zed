// Copyright 2017 The CRC32C Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file. See the AUTHORS file for names of contributors.

#ifndef CRC32C_CRC32C_ROUND_UP_H_
#define CRC32C_CRC32C_ROUND_UP_H_

#include <cstddef>
#include <cstdint>

namespace crc32c {

// Returns the smallest number >= the given number that is evenly divided by N.
//
// N must be a power of two.
template <int N>
constexpr inline uintptr_t RoundUp(uintptr_t pointer) {
  static_assert((N & (N - 1)) == 0, "N must be a power of two");
  return (pointer + (N - 1)) & ~(N - 1);
}

// Returns the smallest address >= the given address that is aligned to N bytes.
//
// N must be a power of two.
template <int N>
constexpr inline const uint8_t* RoundUp(const uint8_t* pointer) {
  static_assert((N & (N - 1)) == 0, "N must be a power of two");
  return reinterpret_cast<uint8_t*>(
      RoundUp<N>(reinterpret_cast<uintptr_t>(pointer)));
}

}  // namespace crc32c

#endif  // CRC32C_CRC32C_ROUND_UP_H_
