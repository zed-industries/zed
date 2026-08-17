/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_BYTE_ORDER_H_
#define RTC_BASE_BYTE_ORDER_H_

#include <stdint.h>

#include <cstring>

#if defined(WEBRTC_POSIX) && !defined(__native_client__)
#include <arpa/inet.h>
#endif

#include "rtc_base/system/arch.h"

#if defined(WEBRTC_MAC)
#include <libkern/OSByteOrder.h>

#define htobe16(v) OSSwapHostToBigInt16(v)
#define htobe32(v) OSSwapHostToBigInt32(v)
#define htobe64(v) OSSwapHostToBigInt64(v)
#define be16toh(v) OSSwapBigToHostInt16(v)
#define be32toh(v) OSSwapBigToHostInt32(v)
#define be64toh(v) OSSwapBigToHostInt64(v)

#define htole16(v) OSSwapHostToLittleInt16(v)
#define htole32(v) OSSwapHostToLittleInt32(v)
#define htole64(v) OSSwapHostToLittleInt64(v)
#define le16toh(v) OSSwapLittleToHostInt16(v)
#define le32toh(v) OSSwapLittleToHostInt32(v)
#define le64toh(v) OSSwapLittleToHostInt64(v)

#elif defined(WEBRTC_WIN) || defined(__native_client__)

#if defined(WEBRTC_WIN)
#include <stdlib.h>
#include <winsock2.h>
#else
#include <netinet/in.h>  // no-presubmit-check
#endif                   // defined(WEBRTC_WIN)

#if defined(WEBRTC_ARCH_LITTLE_ENDIAN)
#define htobe16(v) htons(v)
#define htobe32(v) htonl(v)
#define be16toh(v) ntohs(v)
#define be32toh(v) ntohl(v)
#define htole16(v) (v)
#define htole32(v) (v)
#define htole64(v) (v)
#define le16toh(v) (v)
#define le32toh(v) (v)
#define le64toh(v) (v)
#if defined(WEBRTC_WIN)
#define htobe64(v) _byteswap_uint64(v)
#define be64toh(v) _byteswap_uint64(v)
#endif  // defined(WEBRTC_WIN)
#if defined(__native_client__)
#define htobe64(v) __builtin_bswap64(v)
#define be64toh(v) __builtin_bswap64(v)
#endif  // defined(__native_client__)

#elif defined(WEBRTC_ARCH_BIG_ENDIAN)
#define htobe16(v) (v)
#define htobe32(v) (v)
#define be16toh(v) (v)
#define be32toh(v) (v)
#define htole16(v) __builtin_bswap16(v)
#define htole32(v) __builtin_bswap32(v)
#define htole64(v) __builtin_bswap64(v)
#define le16toh(v) __builtin_bswap16(v)
#define le32toh(v) __builtin_bswap32(v)
#define le64toh(v) __builtin_bswap64(v)
#if defined(WEBRTC_WIN)
#define htobe64(v) (v)
#define be64toh(v) (v)
#endif  // defined(WEBRTC_WIN)
#if defined(__native_client__)
#define htobe64(v) (v)
#define be64toh(v) (v)
#endif  // defined(__native_client__)
#else
#error WEBRTC_ARCH_BIG_ENDIAN or WEBRTC_ARCH_LITTLE_ENDIAN must be defined.
#endif  // defined(WEBRTC_ARCH_LITTLE_ENDIAN)

#elif defined(WEBRTC_POSIX)
#include <endian.h>
#else
#error "Missing byte order functions for this arch."
#endif  // defined(WEBRTC_MAC)

namespace webrtc {

// Reading and writing of little and big-endian numbers from memory

inline void Set8(void* memory, size_t offset, uint8_t v) {
  static_cast<uint8_t*>(memory)[offset] = v;
}

inline uint8_t Get8(const void* memory, size_t offset) {
  return static_cast<const uint8_t*>(memory)[offset];
}

inline void SetBE16(void* memory, uint16_t v) {
  uint16_t val = htobe16(v);
  memcpy(memory, &val, sizeof(val));
}

inline void SetBE32(void* memory, uint32_t v) {
  uint32_t val = htobe32(v);
  memcpy(memory, &val, sizeof(val));
}

inline void SetBE64(void* memory, uint64_t v) {
  uint64_t val = htobe64(v);
  memcpy(memory, &val, sizeof(val));
}

inline uint16_t GetBE16(const void* memory) {
  uint16_t val;
  memcpy(&val, memory, sizeof(val));
  return be16toh(val);
}

inline uint32_t GetBE32(const void* memory) {
  uint32_t val;
  memcpy(&val, memory, sizeof(val));
  return be32toh(val);
}

inline uint64_t GetBE64(const void* memory) {
  uint64_t val;
  memcpy(&val, memory, sizeof(val));
  return be64toh(val);
}

inline void SetLE16(void* memory, uint16_t v) {
  uint16_t val = htole16(v);
  memcpy(memory, &val, sizeof(val));
}

inline void SetLE32(void* memory, uint32_t v) {
  uint32_t val = htole32(v);
  memcpy(memory, &val, sizeof(val));
}

inline void SetLE64(void* memory, uint64_t v) {
  uint64_t val = htole64(v);
  memcpy(memory, &val, sizeof(val));
}

inline uint16_t GetLE16(const void* memory) {
  uint16_t val;
  memcpy(&val, memory, sizeof(val));
  return le16toh(val);
}

inline uint32_t GetLE32(const void* memory) {
  uint32_t val;
  memcpy(&val, memory, sizeof(val));
  return le32toh(val);
}

inline uint64_t GetLE64(const void* memory) {
  uint64_t val;
  memcpy(&val, memory, sizeof(val));
  return le64toh(val);
}

// Check if the current host is big endian.
inline bool IsHostBigEndian() {
#if defined(WEBRTC_ARCH_BIG_ENDIAN)
  return true;
#else
  return false;
#endif
}

inline uint16_t HostToNetwork16(uint16_t n) {
  return htobe16(n);
}

inline uint32_t HostToNetwork32(uint32_t n) {
  return htobe32(n);
}

inline uint64_t HostToNetwork64(uint64_t n) {
  return htobe64(n);
}

inline uint16_t NetworkToHost16(uint16_t n) {
  return be16toh(n);
}

inline uint32_t NetworkToHost32(uint32_t n) {
  return be32toh(n);
}

inline uint64_t NetworkToHost64(uint64_t n) {
  return be64toh(n);
}

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::Get8;
using ::webrtc::GetBE16;
using ::webrtc::GetBE32;
using ::webrtc::GetBE64;
using ::webrtc::GetLE16;
using ::webrtc::GetLE32;
using ::webrtc::GetLE64;
using ::webrtc::HostToNetwork16;
using ::webrtc::HostToNetwork32;
using ::webrtc::HostToNetwork64;
using ::webrtc::IsHostBigEndian;
using ::webrtc::NetworkToHost16;
using ::webrtc::NetworkToHost32;
using ::webrtc::NetworkToHost64;
using ::webrtc::Set8;
using ::webrtc::SetBE16;
using ::webrtc::SetBE32;
using ::webrtc::SetBE64;
using ::webrtc::SetLE16;
using ::webrtc::SetLE32;
using ::webrtc::SetLE64;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_BYTE_ORDER_H_
