/* Copyright 2017 The CRC32C Authors. All rights reserved.
   Use of this source code is governed by a BSD-style license that can be
   found in the LICENSE file. See the AUTHORS file for names of contributors. */

#ifndef CRC32C_CRC32C_H_
#define CRC32C_CRC32C_H_

/* The API exported by the CRC32C project. */

#if defined(__cplusplus)

#include <cstddef>
#include <cstdint>
#include <string>

#else  /* !defined(__cplusplus) */

#include <stddef.h>
#include <stdint.h>

#endif  /* !defined(__cplusplus) */


/* The C API. */

#if defined(__cplusplus)
extern "C" {
#endif  /* defined(__cplusplus) */

/* Extends "crc" with the CRC32C of "count" bytes in the buffer pointed by
   "data" */
uint32_t crc32c_extend(uint32_t crc, const uint8_t* data, size_t count);

/* Computes the CRC32C of "count" bytes in the buffer pointed by "data". */
uint32_t crc32c_value(const uint8_t* data, size_t count);

#ifdef __cplusplus
}  /* end extern "C" */
#endif  /* defined(__cplusplus) */


/* The C++ API. */

#if defined(__cplusplus)

namespace crc32c {

// Extends "crc" with the CRC32C of "count" bytes in the buffer pointed by
// "data".
uint32_t Extend(uint32_t crc, const uint8_t* data, size_t count);

// Computes the CRC32C of "count" bytes in the buffer pointed by "data".
inline uint32_t Crc32c(const uint8_t* data, size_t count) {
  return Extend(0, data, count);
}

// Computes the CRC32C of "count" bytes in the buffer pointed by "data".
inline uint32_t Crc32c(const char* data, size_t count) {
  return Extend(0, reinterpret_cast<const uint8_t*>(data), count);
}

#ifdef __cpp_lib_byte
// Computes the CRC32C of "count" bytes in the buffer pointed by "data".
inline uint32_t Crc32c(const std::byte* data, size_t count) {
  return Extend(0, reinterpret_cast<const uint8_t*>(data), count);
}
#endif

// Computes the CRC32C of the string's content.
inline uint32_t Crc32c(const std::string& string) {
  return Crc32c(reinterpret_cast<const uint8_t*>(string.data()),
                string.size());
}

}  // namespace crc32c

#if __cplusplus > 201402L
#if __has_include(<string_view>)
#include <string_view>

namespace crc32c {

// Computes the CRC32C of the bytes in the string_view.
inline uint32_t Crc32c(const std::string_view& string_view) {
  return Crc32c(reinterpret_cast<const uint8_t*>(string_view.data()),
                string_view.size());
}

}  // namespace crc32c

#endif  // __has_include(<string_view>)
#endif  // __cplusplus > 201402L

#endif  /* defined(__cplusplus) */

#endif  // CRC32C_CRC32C_H_
