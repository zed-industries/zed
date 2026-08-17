/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NUMERICS_SEQUENCE_NUMBER_UNWRAPPER_H_
#define RTC_BASE_NUMERICS_SEQUENCE_NUMBER_UNWRAPPER_H_

#include <stdint.h>

#include <limits>
#include <optional>
#include <type_traits>

#include "rtc_base/numerics/sequence_number_util.h"

namespace webrtc {

// A sequence number unwrapper where the first unwrapped value equals the
// first value being unwrapped.
template <typename T, T M = 0>
class SeqNumUnwrapper {
  static_assert(
      std::is_unsigned<T>::value &&
          std::numeric_limits<T>::max() < std::numeric_limits<int64_t>::max(),
      "Type unwrapped must be an unsigned integer smaller than int64_t.");

 public:
  // Unwraps `value` and updates the internal state of the unwrapper.
  int64_t Unwrap(T value) {
    if (!last_value_) {
      last_unwrapped_ = {value};
    } else {
      last_unwrapped_ += Delta(*last_value_, value);
    }

    last_value_ = value;
    return last_unwrapped_;
  }

  // Returns the `value` without updating the internal state of the unwrapper.
  int64_t PeekUnwrap(T value) const {
    if (!last_value_) {
      return value;
    }
    return last_unwrapped_ + Delta(*last_value_, value);
  }

  // Resets the unwrapper to its initial state. Unwrapped sequence numbers will
  // being at 0 after resetting.
  void Reset() {
    last_unwrapped_ = 0;
    last_value_.reset();
  }

 private:
  static int64_t Delta(T last_value, T new_value) {
    constexpr int64_t kBackwardAdjustment =
        M == 0 ? int64_t{std::numeric_limits<T>::max()} + 1 : M;
    int64_t result = ForwardDiff<T, M>(last_value, new_value);
    if (!AheadOrAt<T, M>(new_value, last_value)) {
      result -= kBackwardAdjustment;
    }
    return result;
  }

  int64_t last_unwrapped_ = 0;
  std::optional<T> last_value_;
};

using RtpTimestampUnwrapper = SeqNumUnwrapper<uint32_t>;
using RtpSequenceNumberUnwrapper = SeqNumUnwrapper<uint16_t>;

}  // namespace webrtc

#endif  // RTC_BASE_NUMERICS_SEQUENCE_NUMBER_UNWRAPPER_H_
