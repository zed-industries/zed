/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_EVENTS_FIXED_LENGTH_ENCODING_PARAMETERS_V3_H_
#define LOGGING_RTC_EVENT_LOG_EVENTS_FIXED_LENGTH_ENCODING_PARAMETERS_V3_H_

#include <cstdint>
#include <optional>

#include "api/array_view.h"
#include "logging/rtc_event_log/events/rtc_event_field_extraction.h"

namespace webrtc {

// Parameters for fixed-size delta-encoding/decoding.
// These are tailored for the sequence which will be encoded (e.g. widths).
class FixedLengthEncodingParametersV3 final {
 public:
  static bool ValidParameters(uint64_t delta_bit_width,
                              bool signed_deltas,
                              uint64_t value_bit_width) {
    return (1 <= delta_bit_width && delta_bit_width <= 64 &&
            1 <= value_bit_width && value_bit_width <= 64 &&
            (delta_bit_width <= value_bit_width ||
             (signed_deltas && delta_bit_width == 64)));
  }

  static FixedLengthEncodingParametersV3 CalculateParameters(
      uint64_t base,
      ArrayView<const uint64_t> values,
      uint64_t value_bit_width,
      bool values_optional);
  static std::optional<FixedLengthEncodingParametersV3> ParseDeltaHeader(
      uint64_t header,
      uint64_t value_bit_width);

  uint64_t DeltaHeaderAsInt() const;

  // Number of bits necessary to hold the widest(*) of the deltas between the
  // values in the sequence.
  // (*) - Widest might not be the largest, if signed deltas are used.
  uint64_t delta_bit_width() const { return delta_bit_width_; }

  // Whether deltas are signed.
  bool signed_deltas() const { return signed_deltas_; }

  // Whether the values of the sequence are optional. That is, it may be
  // that some of them do not have a value (not even a sentinel value indicating
  // invalidity).
  bool values_optional() const { return values_optional_; }

  // Whether all values are equal. 64-bit signed deltas are assumed to not
  // occur, since those could equally well be represented using 64 bit unsigned
  // deltas.
  bool values_equal() const {
    return delta_bit_width() == 64 && signed_deltas();
  }

  // Number of bits necessary to hold the largest value in the sequence.
  uint64_t value_bit_width() const { return value_bit_width_; }

  // Masks where only the bits relevant to the deltas/values are turned on.
  uint64_t delta_mask() const { return delta_mask_; }
  uint64_t value_mask() const { return value_mask_; }

 private:
  FixedLengthEncodingParametersV3(uint64_t delta_bit_width,
                                  bool signed_deltas,
                                  bool values_optional,
                                  uint64_t value_bit_width)
      : delta_bit_width_(delta_bit_width),
        signed_deltas_(signed_deltas),
        values_optional_(values_optional),
        value_bit_width_(value_bit_width),
        delta_mask_(
            webrtc_event_logging::MaxUnsignedValueOfBitWidth(delta_bit_width_)),
        value_mask_(webrtc_event_logging::MaxUnsignedValueOfBitWidth(
            value_bit_width_)) {}

  uint64_t delta_bit_width_;
  bool signed_deltas_;
  bool values_optional_;
  uint64_t value_bit_width_;

  uint64_t delta_mask_;
  uint64_t value_mask_;
};

}  // namespace webrtc
#endif  // LOGGING_RTC_EVENT_LOG_EVENTS_FIXED_LENGTH_ENCODING_PARAMETERS_V3_H_
