/*
 *  Copyright 2021 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_BITSTREAM_READER_H_
#define RTC_BASE_BITSTREAM_READER_H_

#include <stdint.h>

#include "absl/base/attributes.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "rtc_base/checks.h"
#include "rtc_base/numerics/safe_conversions.h"

namespace webrtc {

// A class to parse sequence of bits. Byte order is assumed big-endian/network.
// This class is optimized for successful parsing and binary size.
// Individual calls to `Read` and `ConsumeBits` never fail. Instead they may
// change the class state into 'failure state'. User of this class should verify
// parsing by checking if class is in that 'failure state' by calling `Ok`.
// That verification can be done once after multiple reads.
class BitstreamReader {
 public:
  explicit BitstreamReader(
      ArrayView<const uint8_t> bytes ABSL_ATTRIBUTE_LIFETIME_BOUND);
  explicit BitstreamReader(
      absl::string_view bytes ABSL_ATTRIBUTE_LIFETIME_BOUND);
  BitstreamReader(const BitstreamReader&) = default;
  BitstreamReader& operator=(const BitstreamReader&) = default;
  ~BitstreamReader();

  // Return number of unread bits in the buffer, or negative number if there
  // was a reading error.
  int RemainingBitCount() const;

  // Returns `true` iff all calls to `Read` and `ConsumeBits` were successful.
  bool Ok() const { return RemainingBitCount() >= 0; }

  // Sets `BitstreamReader` into the failure state.
  void Invalidate() { remaining_bits_ = -1; }

  // Moves current read position forward. `bits` must be non-negative.
  void ConsumeBits(int bits);

  // Reads single bit. Returns 0 or 1.
  ABSL_MUST_USE_RESULT int ReadBit();

  // Reads `bits` from the bitstream. `bits` must be in range [0, 64].
  // Returns an unsigned integer in range [0, 2^bits - 1].
  // On failure sets `BitstreamReader` into the failure state and returns 0.
  ABSL_MUST_USE_RESULT uint64_t ReadBits(int bits);

  // Reads unsigned integer of fixed width.
  template <typename T,
            typename std::enable_if<std::is_unsigned<T>::value &&
                                    !std::is_same<T, bool>::value &&
                                    sizeof(T) <= 8>::type* = nullptr>
  ABSL_MUST_USE_RESULT T Read() {
    return dchecked_cast<T>(ReadBits(sizeof(T) * 8));
  }

  // Reads single bit as boolean.
  template <
      typename T,
      typename std::enable_if<std::is_same<T, bool>::value>::type* = nullptr>
  ABSL_MUST_USE_RESULT bool Read() {
    return ReadBit() != 0;
  }

  // Reads value in range [0, `num_values` - 1].
  // This encoding is similar to ReadBits(val, Ceil(Log2(num_values)),
  // but reduces wastage incurred when encoding non-power of two value ranges
  // Non symmetric values are encoded as:
  // 1) n = bit_width(num_values)
  // 2) k = (1 << n) - num_values
  // Value v in range [0, k - 1] is encoded in (n-1) bits.
  // Value v in range [k, num_values - 1] is encoded as (v+k) in n bits.
  // https://aomediacodec.github.io/av1-spec/#nsn
  uint32_t ReadNonSymmetric(uint32_t num_values);

  // Reads exponential golomb encoded value.
  // On failure sets `BitstreamReader` into the failure state and returns
  // unspecified value.
  // Exponential golomb values are encoded as:
  // 1) x = source val + 1
  // 2) In binary, write [bit_width(x) - 1] 0s, then x
  // To decode, we count the number of leading 0 bits, read that many + 1 bits,
  // and increment the result by 1.
  // Fails the parsing if the value wouldn't fit in a uint32_t.
  uint32_t ReadExponentialGolomb();

  // Reads signed exponential golomb values at the current offset. Signed
  // exponential golomb values are just the unsigned values mapped to the
  // sequence 0, 1, -1, 2, -2, etc. in order.
  // On failure sets `BitstreamReader` into the failure state and returns
  // unspecified value.
  int ReadSignedExponentialGolomb();

  // Reads a LEB128 encoded value. The value will be considered invalid if it
  // can't fit into a uint64_t.
  uint64_t ReadLeb128();

  std::string ReadString(int num_bytes);

 private:
  void set_last_read_is_verified(bool value) const;

  // Next byte with at least one unread bit.
  const uint8_t* bytes_;

  // Number of bits remained to read.
  int remaining_bits_;

  // Unused in release mode.
  mutable bool last_read_is_verified_ = true;
};

inline BitstreamReader::BitstreamReader(ArrayView<const uint8_t> bytes)
    : bytes_(bytes.data()),
      remaining_bits_(checked_cast<int>(bytes.size() * 8)) {}

inline BitstreamReader::BitstreamReader(absl::string_view bytes)
    : bytes_(reinterpret_cast<const uint8_t*>(bytes.data())),
      remaining_bits_(checked_cast<int>(bytes.size() * 8)) {}

inline BitstreamReader::~BitstreamReader() {
  RTC_DCHECK(last_read_is_verified_) << "Latest calls to Read or ConsumeBit "
                                        "were not checked with Ok function.";
}

inline void BitstreamReader::set_last_read_is_verified(bool value) const {
#ifdef RTC_DCHECK_IS_ON
  last_read_is_verified_ = value;
#endif
}

inline int BitstreamReader::RemainingBitCount() const {
  set_last_read_is_verified(true);
  return remaining_bits_;
}

}  // namespace webrtc

#endif  // RTC_BASE_BITSTREAM_READER_H_
