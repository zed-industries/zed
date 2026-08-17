// Copyright 2017 The CRC32C Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file. See the AUTHORS file for names of contributors.

#ifndef CRC32C_CRC32C_READ_LE_H_
#define CRC32C_CRC32C_READ_LE_H_

#include <cstdint>
#include <cstring>

#include "crc32c/crc32c_config.h"

namespace crc32c {

// Reads a little-endian 16-bit integer from bytes, not necessarily aligned.
inline uint16_t ReadUint16LE(const uint8_t* buffer) {
#if BYTE_ORDER_BIG_ENDIAN
  return ((uint16_t{buffer[0]}) | (uint16_t{buffer[1]} << 8));
#else   // !BYTE_ORDER_BIG_ENDIAN
  uint16_t result;
  // This should be optimized to a single instruction.
  std::memcpy(&result, buffer, sizeof(result));
  return result;
#endif  // BYTE_ORDER_BIG_ENDIAN
}

// Reads a little-endian 32-bit integer from bytes, not necessarily aligned.
inline uint32_t ReadUint32LE(const uint8_t* buffer) {
#if BYTE_ORDER_BIG_ENDIAN
  return ((uint32_t{buffer[0]}) | (uint32_t{buffer[1]} << 8) |
          (uint32_t{buffer[2]} << 16) | (uint32_t{buffer[3]} << 24));
#else   // !BYTE_ORDER_BIG_ENDIAN
  uint32_t result;
  // This should be optimized to a single instruction.
  std::memcpy(&result, buffer, sizeof(result));
  return result;
#endif  // BYTE_ORDER_BIG_ENDIAN
}

// Reads a little-endian 64-bit integer from bytes, not necessarily aligned.
inline uint64_t ReadUint64LE(const uint8_t* buffer) {
#if BYTE_ORDER_BIG_ENDIAN
  return ((uint64_t{buffer[0]}) | (uint64_t{buffer[1]} << 8) |
          (uint64_t{buffer[2]} << 16) | (uint64_t{buffer[3]} << 24) |
          (uint64_t{buffer[4]} << 32) | (uint64_t{buffer[5]} << 40) |
          (uint64_t{buffer[6]} << 48) | (uint64_t{buffer[7]} << 56));
#else   // !BYTE_ORDER_BIG_ENDIAN
  uint64_t result;
  // This should be optimized to a single instruction.
  std::memcpy(&result, buffer, sizeof(result));
  return result;
#endif  // BYTE_ORDER_BIG_ENDIAN
}

}  // namespace crc32c

#endif  // CRC32C_CRC32C_READ_LE_H_
