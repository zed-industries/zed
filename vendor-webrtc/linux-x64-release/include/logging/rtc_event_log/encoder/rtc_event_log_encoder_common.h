/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_ENCODER_RTC_EVENT_LOG_ENCODER_COMMON_H_
#define LOGGING_RTC_EVENT_LOG_ENCODER_RTC_EVENT_LOG_ENCODER_COMMON_H_

#include <stdint.h>

#include <limits>
#include <type_traits>

namespace webrtc {

// Convert between the packet fraction loss (a floating point number in
// the range [0.0, 1.0]), and a uint32_t with up to a fixed number of bits.
// The latter can be more efficiently stored in a protobuf and/or delta-encoded.
uint32_t ConvertPacketLossFractionToProtoFormat(float packet_loss_fraction);
bool ParsePacketLossFractionFromProtoFormat(uint32_t proto_packet_loss_fraction,
                                            float* output);

}  // namespace webrtc

namespace webrtc_event_logging {

// Produce an unsigned representation of a signed integer. On two's complement
// machines, this is equivalent to:
// static_cast<uint64_t>(static_cast<std::make_unsigned<T>>(y))
template <typename T>
uint64_t ToUnsigned(T y) {
  static_assert(std::is_integral<T>::value, "");
  static_assert(std::is_signed<T>::value, "");

  // Note that a signed integer whose width is N bits, has N-1 digits.
  static_assert(std::numeric_limits<T>::digits < 64, "");

  constexpr T MIN_T = std::numeric_limits<T>::min();
  constexpr T MAX_T = std::numeric_limits<T>::max();

  static_assert(MAX_T + MIN_T + 1 >= 0, "MAX_T >= abs(MIN_T) - 1");

  if (y >= 0) {
    return static_cast<uint64_t>(y);
  } else {
    // y is in the range [MIN_T, -1], so (y - MIN_T) is in the
    // range [0, abs(MIN_T) - 1]. This is representable in a T
    // because MAX_T >= abs(MIN_T) - 1, as per the static_assert above.
    return static_cast<uint64_t>(MAX_T) + 1 + static_cast<uint64_t>(y - MIN_T);
  }
}

// Assuming x = ToUnsigned(y), return `y`.
// Note: static_cast<T>(x) would work on most platforms and compilers, but
// involves undefined behavior. This function is well-defined, and can be
// optimized to a noop for 64 bit types, or a few arithmetic
// instructions and a single conditional jump for narrower types.
template <typename T>
bool ToSigned(uint64_t x, T* y) {
  static_assert(std::is_integral<T>::value, "");
  static_assert(std::is_signed<T>::value, "");

  // Note that a signed integer whose width is N bits, has N-1 digits.
  static_assert(std::numeric_limits<T>::digits < 64, "");

  constexpr T MIN_T = std::numeric_limits<T>::min();
  constexpr T MAX_T = std::numeric_limits<T>::max();

  using UNSIGNED_T = typename std::make_unsigned<T>::type;
  constexpr auto MAX_UNSIGNED_T = std::numeric_limits<UNSIGNED_T>::max();
  if (x > static_cast<uint64_t>(MAX_UNSIGNED_T)) {
    return false;  // `x` cannot be represented using a T.
  }

  if (x <= static_cast<uint64_t>(MAX_T)) {
    // The original value was positive, so it is safe to just static_cast.
    *y = static_cast<T>(x);
  } else {  // x > static_cast<uint64_t>(MAX_T)
    const uint64_t neg_x = x - static_cast<uint64_t>(MAX_T) - 1;
    *y = static_cast<T>(neg_x) + MIN_T;
  }

  return true;
}

}  // namespace webrtc_event_logging

#endif  // LOGGING_RTC_EVENT_LOG_ENCODER_RTC_EVENT_LOG_ENCODER_COMMON_H_
