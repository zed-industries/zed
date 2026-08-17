/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_FIELD_EXTRACTION_H_
#define LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_FIELD_EXTRACTION_H_

#include <cstdint>
#include <type_traits>
#include <vector>

#include "logging/rtc_event_log/encoder/rtc_event_log_encoder_common.h"
#include "rtc_base/logging.h"

namespace webrtc_event_logging {
uint8_t UnsignedBitWidth(uint64_t max_magnitude);
uint8_t SignedBitWidth(uint64_t max_pos_magnitude, uint64_t max_neg_magnitude);
uint64_t MaxUnsignedValueOfBitWidth(uint64_t bit_width);
uint64_t UnsignedDelta(uint64_t previous, uint64_t current, uint64_t bit_mask);
}  // namespace webrtc_event_logging

namespace webrtc {
template <typename T, std::enable_if_t<std::is_signed<T>::value, bool> = true>
uint64_t EncodeAsUnsigned(T value) {
  return webrtc_event_logging::ToUnsigned(value);
}

template <typename T, std::enable_if_t<std::is_unsigned<T>::value, bool> = true>
uint64_t EncodeAsUnsigned(T value) {
  return static_cast<uint64_t>(value);
}

template <typename T, std::enable_if_t<std::is_signed<T>::value, bool> = true>
T DecodeFromUnsignedToType(uint64_t value) {
  T signed_value = 0;
  bool success = webrtc_event_logging::ToSigned<T>(value, &signed_value);
  if (!success) {
    RTC_LOG(LS_ERROR) << "Failed to convert " << value << "to signed type.";
    // TODO(terelius): Propagate error?
  }
  return signed_value;
}

template <typename T, std::enable_if_t<std::is_unsigned<T>::value, bool> = true>
T DecodeFromUnsignedToType(uint64_t value) {
  // TODO(terelius): Check range?
  return static_cast<T>(value);
}

// RtcEventLogEnum<T> defines a mapping between an enum T
// and the event log encodings. To log a new enum type T,
// specialize RtcEventLogEnum<T> and add static methods
// static uint64_t Encode(T x) {}
// static RtcEventLogParseStatusOr<T> Decode(uint64_t x) {}
template <typename T>
class RtcEventLogEnum {
  static_assert(sizeof(T) != sizeof(T),
                "Missing specialisation of RtcEventLogEnum for type");
};

// Represents a vector<optional<uint64_t>> optional_values
// as a bit-vector `position_mask` which identifies the positions
// of existing values, and a (potentially shorter)
// `vector<uint64_t> values` containing the actual values.
// The bit vector is constructed such that position_mask[i]
// is true iff optional_values[i] has a value, and `values.size()`
// is equal to the number of set bits in `position_mask`.
struct ValuesWithPositions {
  std::vector<bool> position_mask;
  std::vector<uint64_t> values;
};

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_FIELD_EXTRACTION_H_
