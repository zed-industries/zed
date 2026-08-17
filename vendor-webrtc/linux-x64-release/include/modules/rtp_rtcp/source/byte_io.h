/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_BYTE_IO_H_
#define MODULES_RTP_RTCP_SOURCE_BYTE_IO_H_

// This file contains classes for reading and writing integer types from/to
// byte array representations. Signed/unsigned, partial (whole byte) sizes,
// and big/little endian byte order is all supported.
//
// Usage examples:
//
// uint8_t* buffer = ...;
//
// // Read an unsigned 4 byte integer in big endian format
// uint32_t val = ByteReader<uint32_t>::ReadBigEndian(buffer);
//
// // Read a signed 24-bit (3 byte) integer in little endian format
// int32_t val = ByteReader<int32_t, 3>::ReadLittle(buffer);
//
// // Write an unsigned 8 byte integer in little endian format
// ByteWriter<uint64_t>::WriteLittleEndian(buffer, val);
//
// Write an unsigned 40-bit (5 byte) integer in big endian format
// ByteWriter<uint64_t, 5>::WriteBigEndian(buffer, val);
//
// These classes are implemented as recursive templetizations, intended to make
// it easy for the compiler to completely inline the reading/writing.

#include <stdint.h>

#include <limits>

namespace webrtc {

// According to ISO C standard ISO/IEC 9899, section 6.2.6.2 (2), the three
// representations of signed integers allowed are two's complement, one's
// complement and sign/magnitude. We can detect which is used by looking at
// the two last bits of -1, which will be 11 in two's complement, 10 in one's
// complement and 01 in sign/magnitude.
// TODO(sprang): In the unlikely event that we actually need to support a
// platform that doesn't use two's complement, implement conversion to/from
// wire format.

// Assume the if any one signed integer type is two's complement, then all
// other will be too.
static_assert(
    (-1 & 0x03) == 0x03,
    "Only two's complement representation of signed integers supported.");

// Plain const char* won't work for static_assert, use #define instead.
#define kSizeErrorMsg "Byte size must be less than or equal to data type size."

// Utility class for getting the unsigned equivalent of a signed type.
template <typename T>
struct UnsignedOf;

// Class for reading integers from a sequence of bytes.
// T = type of integer, B = bytes to read, is_signed = true if signed integer.
// If is_signed is true and B < sizeof(T), sign extension might be needed.
template <typename T,
          unsigned int B = sizeof(T),
          bool is_signed = std::numeric_limits<T>::is_signed>
class ByteReader;

// Specialization of ByteReader for unsigned types.
template <typename T, unsigned int B>
class ByteReader<T, B, false> {
 public:
  static T ReadBigEndian(const uint8_t* data) {
    static_assert(B <= sizeof(T), kSizeErrorMsg);
    return InternalReadBigEndian(data);
  }

  static T ReadLittleEndian(const uint8_t* data) {
    static_assert(B <= sizeof(T), kSizeErrorMsg);
    return InternalReadLittleEndian(data);
  }

 private:
  static T InternalReadBigEndian(const uint8_t* data) {
    T val(0);
    for (unsigned int i = 0; i < B; ++i)
      val |= static_cast<T>(data[i]) << ((B - 1 - i) * 8);
    return val;
  }

  static T InternalReadLittleEndian(const uint8_t* data) {
    T val(0);
    for (unsigned int i = 0; i < B; ++i)
      val |= static_cast<T>(data[i]) << (i * 8);
    return val;
  }
};

// Specialization of ByteReader for signed types.
template <typename T, unsigned int B>
class ByteReader<T, B, true> {
 public:
  typedef typename UnsignedOf<T>::Type U;

  static T ReadBigEndian(const uint8_t* data) {
    U unsigned_val = ByteReader<T, B, false>::ReadBigEndian(data);
    if (B < sizeof(T))
      unsigned_val = SignExtend(unsigned_val);
    return ReinterpretAsSigned(unsigned_val);
  }

  static T ReadLittleEndian(const uint8_t* data) {
    U unsigned_val = ByteReader<T, B, false>::ReadLittleEndian(data);
    if (B < sizeof(T))
      unsigned_val = SignExtend(unsigned_val);
    return ReinterpretAsSigned(unsigned_val);
  }

 private:
  // As a hack to avoid implementation-specific or undefined behavior when
  // bit-shifting or casting signed integers, read as a signed equivalent
  // instead and convert to signed. This is safe since we have asserted that
  // two's complement for is used.
  static T ReinterpretAsSigned(U unsigned_val) {
    // An unsigned value with only the highest order bit set (ex 0x80).
    const U kUnsignedHighestBitMask = static_cast<U>(1)
                                      << ((sizeof(U) * 8) - 1);
    // A signed value with only the highest bit set. Since this is two's
    // complement form, we can use the min value from std::numeric_limits.
    const T kSignedHighestBitMask = std::numeric_limits<T>::min();

    T val;
    if ((unsigned_val & kUnsignedHighestBitMask) != 0) {
      // Casting is only safe when unsigned value can be represented in the
      // signed target type, so mask out highest bit and mask it back manually.
      val = static_cast<T>(unsigned_val & ~kUnsignedHighestBitMask);
      val |= kSignedHighestBitMask;
    } else {
      val = static_cast<T>(unsigned_val);
    }
    return val;
  }

  // If number of bytes is less than native data type (eg 24 bit, in int32_t),
  // and the most significant bit of the actual data is set, we must sign
  // extend the remaining byte(s) with ones so that the correct negative
  // number is retained.
  // Ex: 0x810A0B -> 0xFF810A0B, but 0x710A0B -> 0x00710A0B
  static U SignExtend(const U val) {
    const uint8_t kMsb = static_cast<uint8_t>(val >> ((B - 1) * 8));
    if ((kMsb & 0x80) != 0) {
      // Create a mask where all bits used by the B bytes are set to one,
      // for instance 0x00FFFFFF for B = 3. Bit-wise invert that mask (to
      // (0xFF000000 in the example above) and add it to the input value.
      // The "B % sizeof(T)" is a workaround to undefined values warnings for
      // B == sizeof(T), in which case this code won't be called anyway.
      const U kUsedBitsMask = (1 << ((B % sizeof(T)) * 8)) - 1;
      return ~kUsedBitsMask | val;
    }
    return val;
  }
};

// Class for writing integers to a sequence of bytes
// T = type of integer, B = bytes to write
template <typename T,
          unsigned int B = sizeof(T),
          bool is_signed = std::numeric_limits<T>::is_signed>
class ByteWriter;

// Specialization of ByteWriter for unsigned types.
template <typename T, unsigned int B>
class ByteWriter<T, B, false> {
 public:
  static void WriteBigEndian(uint8_t* data, T val) {
    static_assert(B <= sizeof(T), kSizeErrorMsg);
    for (unsigned int i = 0; i < B; ++i) {
      data[i] = val >> ((B - 1 - i) * 8);
    }
  }

  static void WriteLittleEndian(uint8_t* data, T val) {
    static_assert(B <= sizeof(T), kSizeErrorMsg);
    for (unsigned int i = 0; i < B; ++i) {
      data[i] = val >> (i * 8);
    }
  }
};

// Specialization of ByteWriter for signed types.
template <typename T, unsigned int B>
class ByteWriter<T, B, true> {
 public:
  typedef typename UnsignedOf<T>::Type U;

  static void WriteBigEndian(uint8_t* data, T val) {
    ByteWriter<U, B, false>::WriteBigEndian(data, ReinterpretAsUnsigned(val));
  }

  static void WriteLittleEndian(uint8_t* data, T val) {
    ByteWriter<U, B, false>::WriteLittleEndian(data,
                                               ReinterpretAsUnsigned(val));
  }

 private:
  static U ReinterpretAsUnsigned(T val) {
    // According to ISO C standard ISO/IEC 9899, section 6.3.1.3 (1, 2) a
    // conversion from signed to unsigned keeps the value if the new type can
    // represent it, and otherwise adds one more than the max value of T until
    // the value is in range. For two's complement, this fortunately means
    // that the bit-wise value will be intact. Thus, since we have asserted that
    // two's complement form is actually used, a simple cast is sufficient.
    return static_cast<U>(val);
  }
};

// ----- Below follows specializations of UnsignedOf utility class -----

template <>
struct UnsignedOf<int8_t> {
  typedef uint8_t Type;
};
template <>
struct UnsignedOf<int16_t> {
  typedef uint16_t Type;
};
template <>
struct UnsignedOf<int32_t> {
  typedef uint32_t Type;
};
template <>
struct UnsignedOf<int64_t> {
  typedef uint64_t Type;
};

// ----- Below follows specializations for unsigned, B in { 1, 2, 4, 8 } -----

// TODO(sprang): Check if these actually help or if generic cases will be
// unrolled to and optimized to similar performance.

// Specializations for single bytes
template <typename T>
class ByteReader<T, 1, false> {
 public:
  static T ReadBigEndian(const uint8_t* data) {
    static_assert(sizeof(T) == 1, kSizeErrorMsg);
    return data[0];
  }

  static T ReadLittleEndian(const uint8_t* data) {
    static_assert(sizeof(T) == 1, kSizeErrorMsg);
    return data[0];
  }
};

template <typename T>
class ByteWriter<T, 1, false> {
 public:
  static void WriteBigEndian(uint8_t* data, T val) {
    static_assert(sizeof(T) == 1, kSizeErrorMsg);
    data[0] = val;
  }

  static void WriteLittleEndian(uint8_t* data, T val) {
    static_assert(sizeof(T) == 1, kSizeErrorMsg);
    data[0] = val;
  }
};

// Specializations for two byte words
template <typename T>
class ByteReader<T, 2, false> {
 public:
  static T ReadBigEndian(const uint8_t* data) {
    static_assert(sizeof(T) >= 2, kSizeErrorMsg);
    return (data[0] << 8) | data[1];
  }

  static T ReadLittleEndian(const uint8_t* data) {
    static_assert(sizeof(T) >= 2, kSizeErrorMsg);
    return data[0] | (data[1] << 8);
  }
};

template <typename T>
class ByteWriter<T, 2, false> {
 public:
  static void WriteBigEndian(uint8_t* data, T val) {
    static_assert(sizeof(T) >= 2, kSizeErrorMsg);
    data[0] = val >> 8;
    data[1] = val;
  }

  static void WriteLittleEndian(uint8_t* data, T val) {
    static_assert(sizeof(T) >= 2, kSizeErrorMsg);
    data[0] = val;
    data[1] = val >> 8;
  }
};

// Specializations for four byte words.
template <typename T>
class ByteReader<T, 4, false> {
 public:
  static T ReadBigEndian(const uint8_t* data) {
    static_assert(sizeof(T) >= 4, kSizeErrorMsg);
    return (Get(data, 0) << 24) | (Get(data, 1) << 16) | (Get(data, 2) << 8) |
           Get(data, 3);
  }

  static T ReadLittleEndian(const uint8_t* data) {
    static_assert(sizeof(T) >= 4, kSizeErrorMsg);
    return Get(data, 0) | (Get(data, 1) << 8) | (Get(data, 2) << 16) |
           (Get(data, 3) << 24);
  }

 private:
  inline static T Get(const uint8_t* data, unsigned int index) {
    return static_cast<T>(data[index]);
  }
};

// Specializations for four byte words.
template <typename T>
class ByteWriter<T, 4, false> {
 public:
  static void WriteBigEndian(uint8_t* data, T val) {
    static_assert(sizeof(T) >= 4, kSizeErrorMsg);
    data[0] = val >> 24;
    data[1] = val >> 16;
    data[2] = val >> 8;
    data[3] = val;
  }

  static void WriteLittleEndian(uint8_t* data, T val) {
    static_assert(sizeof(T) >= 4, kSizeErrorMsg);
    data[0] = val;
    data[1] = val >> 8;
    data[2] = val >> 16;
    data[3] = val >> 24;
  }
};

// Specializations for eight byte words.
template <typename T>
class ByteReader<T, 8, false> {
 public:
  static T ReadBigEndian(const uint8_t* data) {
    static_assert(sizeof(T) >= 8, kSizeErrorMsg);
    return (Get(data, 0) << 56) | (Get(data, 1) << 48) | (Get(data, 2) << 40) |
           (Get(data, 3) << 32) | (Get(data, 4) << 24) | (Get(data, 5) << 16) |
           (Get(data, 6) << 8) | Get(data, 7);
  }

  static T ReadLittleEndian(const uint8_t* data) {
    static_assert(sizeof(T) >= 8, kSizeErrorMsg);
    return Get(data, 0) | (Get(data, 1) << 8) | (Get(data, 2) << 16) |
           (Get(data, 3) << 24) | (Get(data, 4) << 32) | (Get(data, 5) << 40) |
           (Get(data, 6) << 48) | (Get(data, 7) << 56);
  }

 private:
  inline static T Get(const uint8_t* data, unsigned int index) {
    return static_cast<T>(data[index]);
  }
};

template <typename T>
class ByteWriter<T, 8, false> {
 public:
  static void WriteBigEndian(uint8_t* data, T val) {
    static_assert(sizeof(T) >= 8, kSizeErrorMsg);
    data[0] = val >> 56;
    data[1] = val >> 48;
    data[2] = val >> 40;
    data[3] = val >> 32;
    data[4] = val >> 24;
    data[5] = val >> 16;
    data[6] = val >> 8;
    data[7] = val;
  }

  static void WriteLittleEndian(uint8_t* data, T val) {
    static_assert(sizeof(T) >= 8, kSizeErrorMsg);
    data[0] = val;
    data[1] = val >> 8;
    data[2] = val >> 16;
    data[3] = val >> 24;
    data[4] = val >> 32;
    data[5] = val >> 40;
    data[6] = val >> 48;
    data[7] = val >> 56;
  }
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_BYTE_IO_H_
