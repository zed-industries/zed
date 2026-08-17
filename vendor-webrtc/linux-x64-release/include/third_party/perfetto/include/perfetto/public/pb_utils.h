/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_PUBLIC_PB_UTILS_H_
#define INCLUDE_PERFETTO_PUBLIC_PB_UTILS_H_

#include <assert.h>
#include <stdint.h>
#include <string.h>

#include "perfetto/public/compiler.h"

// Type of fields that can be found in a protobuf serialized message.
enum PerfettoPbWireType {
  PERFETTO_PB_WIRE_TYPE_VARINT = 0,
  PERFETTO_PB_WIRE_TYPE_FIXED64 = 1,
  PERFETTO_PB_WIRE_TYPE_DELIMITED = 2,
  PERFETTO_PB_WIRE_TYPE_FIXED32 = 5,
};

// Creates a field tag, which encodes the field type and the field id.
static inline uint32_t PerfettoPbMakeTag(int32_t field_id,
                                         enum PerfettoPbWireType wire_type) {
  return ((PERFETTO_STATIC_CAST(uint32_t, field_id)) << 3) |
         PERFETTO_STATIC_CAST(uint32_t, wire_type);
}

enum {
  // Maximum bytes size of a 64-bit integer encoded as a VarInt.
  PERFETTO_PB_VARINT_MAX_SIZE_64 = 10,
  // Maximum bytes size of a 32-bit integer encoded as a VarInt.
  PERFETTO_PB_VARINT_MAX_SIZE_32 = 5,
};

// Encodes `value` as a VarInt into `*dst`.
//
// `dst` must point into a buffer big enough to represent `value`:
// PERFETTO_PB_VARINT_MAX_SIZE_* can help.
static inline uint8_t* PerfettoPbWriteVarInt(uint64_t value, uint8_t* dst) {
  uint8_t byte;
  while (value >= 0x80) {
    byte = (value & 0x7f) | 0x80;
    *dst++ = byte;
    value >>= 7;
  }
  byte = value & 0x7f;
  *dst++ = byte;

  return dst;
}

// Encodes `value` as a fixed32 (little endian) into `*dst`.
//
// `dst` must point into a buffer with at least 4 bytes of space.
static inline uint8_t* PerfettoPbWriteFixed32(uint32_t value, uint8_t* buf) {
  buf[0] = PERFETTO_STATIC_CAST(uint8_t, value);
  buf[1] = PERFETTO_STATIC_CAST(uint8_t, value >> 8);
  buf[2] = PERFETTO_STATIC_CAST(uint8_t, value >> 16);
  buf[3] = PERFETTO_STATIC_CAST(uint8_t, value >> 24);
  return buf + 4;
}

// Encodes `value` as a fixed32 (little endian) into `*dst`.
//
// `dst` must point into a buffer with at least 8 bytes of space.
static inline uint8_t* PerfettoPbWriteFixed64(uint64_t value, uint8_t* buf) {
  buf[0] = PERFETTO_STATIC_CAST(uint8_t, value);
  buf[1] = PERFETTO_STATIC_CAST(uint8_t, value >> 8);
  buf[2] = PERFETTO_STATIC_CAST(uint8_t, value >> 16);
  buf[3] = PERFETTO_STATIC_CAST(uint8_t, value >> 24);
  buf[4] = PERFETTO_STATIC_CAST(uint8_t, value >> 32);
  buf[5] = PERFETTO_STATIC_CAST(uint8_t, value >> 40);
  buf[6] = PERFETTO_STATIC_CAST(uint8_t, value >> 48);
  buf[7] = PERFETTO_STATIC_CAST(uint8_t, value >> 56);
  return buf + 8;
}

// Parses a VarInt from the encoded buffer [start, end). |end| is STL-style and
// points one byte past the end of buffer.
// The parsed int value is stored in the output arg |value|. Returns a pointer
// to the next unconsumed byte (so start < retval <= end) or |start| if the
// VarInt could not be fully parsed because there was not enough space in the
// buffer.
static inline const uint8_t* PerfettoPbParseVarInt(const uint8_t* start,
                                                   const uint8_t* end,
                                                   uint64_t* out_value) {
  const uint8_t* pos = start;
  uint64_t value = 0;
  for (uint32_t shift = 0; pos < end && shift < 64u; shift += 7) {
    // Cache *pos into |cur_byte| to prevent that the compiler dereferences the
    // pointer twice (here and in the if() below) due to char* aliasing rules.
    uint8_t cur_byte = *pos++;
    value |= PERFETTO_STATIC_CAST(uint64_t, cur_byte & 0x7f) << shift;
    if ((cur_byte & 0x80) == 0) {
      // In valid cases we get here.
      *out_value = value;
      return pos;
    }
  }
  *out_value = 0;
  return start;
}

static inline uint32_t PerfettoPbZigZagEncode32(int32_t value) {
#if defined(__cplusplus) || \
    (defined(__STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
  // Right-shift of negative values is implementation specific.
  // Assert the implementation does what we expect, which is that shifting an
  // positive int32_t by 31 gives an all 0 bitmap, and a negative int32_t gives
  // an all 1 bitmap.
  static_assert(
      PERFETTO_STATIC_CAST(uint32_t, INT32_C(-1) >> 31) == ~UINT32_C(0),
      "implementation does not support assumed rightshift");
  static_assert(PERFETTO_STATIC_CAST(uint32_t, INT32_C(1) >> 31) == UINT32_C(0),
                "implementation does not support assumed rightshift");
#endif

  return (PERFETTO_STATIC_CAST(uint32_t, value) << 1) ^
         PERFETTO_STATIC_CAST(uint32_t, value >> 31);
}

static inline uint64_t PerfettoPbZigZagEncode64(int64_t value) {
#if defined(__cplusplus) || \
    (defined(__STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
  // Right-shift of negative values is implementation specific.
  // Assert the implementation does what we expect, which is that shifting an
  // positive int64_t by 63 gives an all 0 bitmap, and a negative int64_t gives
  // an all 1 bitmap.
  static_assert(
      PERFETTO_STATIC_CAST(uint64_t, INT64_C(-1) >> 63) == ~UINT64_C(0),
      "implementation does not support assumed rightshift");
  static_assert(PERFETTO_STATIC_CAST(uint64_t, INT64_C(1) >> 63) == UINT64_C(0),
                "implementation does not support assumed rightshift");
#endif

  return (PERFETTO_STATIC_CAST(uint64_t, value) << 1) ^
         PERFETTO_STATIC_CAST(uint64_t, value >> 63);
}

static inline int32_t PerfettoPbZigZagDecode32(uint32_t value) {
  uint32_t mask =
      PERFETTO_STATIC_CAST(uint32_t, -PERFETTO_STATIC_CAST(int32_t, value & 1));
  return PERFETTO_STATIC_CAST(int32_t, ((value >> 1) ^ mask));
}

static inline int64_t PerfettoPbZigZagDecode64(uint64_t value) {
  uint64_t mask =
      PERFETTO_STATIC_CAST(uint64_t, -PERFETTO_STATIC_CAST(int64_t, value & 1));
  return PERFETTO_STATIC_CAST(int64_t, ((value >> 1) ^ mask));
}

static inline uint64_t PerfettoPbDoubleToFixed64(double value) {
  uint64_t val;
  memcpy(&val, &value, sizeof val);
  return val;
}

static inline uint32_t PerfettoPbFloatToFixed32(float value) {
  uint32_t val;
  memcpy(&val, &value, sizeof val);
  return val;
}

#endif  // INCLUDE_PERFETTO_PUBLIC_PB_UTILS_H_
