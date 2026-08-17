/*
 *  Copyright 2015 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_BIT_BUFFER_H_
#define RTC_BASE_BIT_BUFFER_H_

#include <stddef.h>  // For size_t.
#include <stdint.h>  // For integer types.

#include "absl/strings/string_view.h"
#include "api/units/data_size.h"

namespace webrtc {

// A BitBuffer API for write operations. Supports symmetric write APIs to the
// reading APIs of BitstreamReader.
// Sizes/counts specify bits/bytes, for clarity.
// Byte order is assumed big-endian/network.
class BitBufferWriter {
 public:
  static constexpr DataSize kMaxLeb128Length = webrtc::DataSize::Bytes(10);

  // Constructs a bit buffer for the writable buffer of `bytes`.
  BitBufferWriter(uint8_t* bytes, size_t byte_count);

  BitBufferWriter(const BitBufferWriter&) = delete;
  BitBufferWriter& operator=(const BitBufferWriter&) = delete;

  // Gets the current offset, in bytes/bits, from the start of the buffer. The
  // bit offset is the offset into the current byte, in the range [0,7].
  void GetCurrentOffset(size_t* out_byte_offset, size_t* out_bit_offset);

  // The remaining bits in the byte buffer.
  uint64_t RemainingBitCount() const;

  // Moves current position `byte_count` bytes forward. Returns false if
  // there aren't enough bytes left in the buffer.
  bool ConsumeBytes(size_t byte_count);
  // Moves current position `bit_count` bits forward. Returns false if
  // there aren't enough bits left in the buffer.
  bool ConsumeBits(size_t bit_count);

  // Sets the current offset to the provied byte/bit offsets. The bit
  // offset is from the given byte, in the range [0,7].
  bool Seek(size_t byte_offset, size_t bit_offset);

  // Writes byte-sized values from the buffer. Returns false if there isn't
  // enough data left for the specified type.
  bool WriteUInt8(uint8_t val);
  bool WriteUInt16(uint16_t val);
  bool WriteUInt32(uint32_t val);

  // Writes bit-sized values to the buffer. Returns false if there isn't enough
  // room left for the specified number of bits.
  bool WriteBits(uint64_t val, size_t bit_count);

  // Writes value in range [0, num_values - 1]
  // See ReadNonSymmetric documentation for the format,
  // Call SizeNonSymmetricBits to get number of bits needed to store the value.
  // Returns false if there isn't enough room left for the value.
  bool WriteNonSymmetric(uint32_t val, uint32_t num_values);
  // Returns number of bits required to store `val` with NonSymmetric encoding.
  static size_t SizeNonSymmetricBits(uint32_t val, uint32_t num_values);

  // Writes the exponential golomb encoded version of the supplied value.
  // Returns false if there isn't enough room left for the value.
  bool WriteExponentialGolomb(uint32_t val);
  // Writes the signed exponential golomb version of the supplied value.
  // Signed exponential golomb values are just the unsigned values mapped to the
  // sequence 0, 1, -1, 2, -2, etc. in order.
  bool WriteSignedExponentialGolomb(int32_t val);

  // Writes the Leb128 encoded value.
  bool WriteLeb128(uint64_t val);

  // Writes the string as bytes of data.
  bool WriteString(absl::string_view data);

 private:
  // The buffer, as a writable array.
  uint8_t* const writable_bytes_;
  // The total size of `bytes_`.
  const size_t byte_count_;
  // The current offset, in bytes, from the start of `bytes_`.
  size_t byte_offset_;
  // The current offset, in bits, into the current byte.
  size_t bit_offset_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::BitBufferWriter;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_BIT_BUFFER_H_
