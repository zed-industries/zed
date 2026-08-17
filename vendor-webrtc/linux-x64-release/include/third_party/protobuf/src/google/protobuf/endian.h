// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_ENDIAN_H__
#define GOOGLE_PROTOBUF_ENDIAN_H__

#if defined(_MSC_VER)
#include <stdlib.h>
#endif

#include <cstdint>

#include "absl/base/config.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace internal {

inline uint64_t BSwap64(uint64_t host_int) {
#if defined(__GNUC__) || ABSL_HAVE_BUILTIN(__builtin_bswap64)
  return __builtin_bswap64(host_int);
#elif defined(_MSC_VER)
  return _byteswap_uint64(host_int);
#else
  return (((host_int & uint64_t{0xFF}) << 56) |
          ((host_int & uint64_t{0xFF00}) << 40) |
          ((host_int & uint64_t{0xFF0000}) << 24) |
          ((host_int & uint64_t{0xFF000000}) << 8) |
          ((host_int & uint64_t{0xFF00000000}) >> 8) |
          ((host_int & uint64_t{0xFF0000000000}) >> 24) |
          ((host_int & uint64_t{0xFF000000000000}) >> 40) |
          ((host_int & uint64_t{0xFF00000000000000}) >> 56));
#endif
}

inline uint32_t BSwap32(uint32_t host_int) {
#if defined(__GNUC__) || ABSL_HAVE_BUILTIN(__builtin_bswap32)
  return __builtin_bswap32(host_int);
#elif defined(_MSC_VER)
  return _byteswap_ulong(host_int);
#else
  return (((host_int & uint32_t{0xFF}) << 24) |
          ((host_int & uint32_t{0xFF00}) << 8) |
          ((host_int & uint32_t{0xFF0000}) >> 8) |
          ((host_int & uint32_t{0xFF000000}) >> 24));
#endif
}

inline uint16_t BSwap16(uint16_t host_int) {
#if defined(__GNUC__) || ABSL_HAVE_BUILTIN(__builtin_bswap16)
  return __builtin_bswap16(host_int);
#elif defined(_MSC_VER)
  return _byteswap_ushort(host_int);
#else
  return (((host_int & uint16_t{0xFF}) << 8) |
          ((host_int & uint16_t{0xFF00}) >> 8));
#endif
}

namespace little_endian {

inline uint16_t FromHost(uint16_t value) {
#if defined(ABSL_IS_BIG_ENDIAN)
  return BSwap16(value);
#else
  return value;
#endif
}

inline uint32_t FromHost(uint32_t value) {
#if defined(ABSL_IS_BIG_ENDIAN)
  return BSwap32(value);
#else
  return value;
#endif
}

inline uint64_t FromHost(uint64_t value) {
#if defined(ABSL_IS_BIG_ENDIAN)
  return BSwap64(value);
#else
  return value;
#endif
}

inline uint16_t ToHost(uint16_t value) {
#if defined(ABSL_IS_BIG_ENDIAN)
  return BSwap16(value);
#else
  return value;
#endif
}

inline uint32_t ToHost(uint32_t value) {
#if defined(ABSL_IS_BIG_ENDIAN)
  return BSwap32(value);
#else
  return value;
#endif
}

inline uint64_t ToHost(uint64_t value) {
#if defined(ABSL_IS_BIG_ENDIAN)
  return BSwap64(value);
#else
  return value;
#endif
}

}  // namespace little_endian

namespace big_endian {

inline uint16_t FromHost(uint16_t value) {
#if defined(ABSL_IS_BIG_ENDIAN)
  return value;
#else
  return BSwap16(value);
#endif
}

inline uint32_t FromHost(uint32_t value) {
#if defined(ABSL_IS_BIG_ENDIAN)
  return value;
#else
  return BSwap32(value);
#endif
}

inline uint64_t FromHost(uint64_t value) {
#if defined(ABSL_IS_BIG_ENDIAN)
  return value;
#else
  return BSwap64(value);
#endif
}

inline uint16_t ToHost(uint16_t value) {
#if defined(ABSL_IS_BIG_ENDIAN)
  return value;
#else
  return BSwap16(value);
#endif
}

inline uint32_t ToHost(uint32_t value) {
#if defined(ABSL_IS_BIG_ENDIAN)
  return value;
#else
  return BSwap32(value);
#endif
}

inline uint64_t ToHost(uint64_t value) {
#if defined(ABSL_IS_BIG_ENDIAN)
  return value;
#else
  return BSwap64(value);
#endif
}

}  // namespace big_endian

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_ENDIAN_H__
