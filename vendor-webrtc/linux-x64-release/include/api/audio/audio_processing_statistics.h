/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_AUDIO_PROCESSING_STATISTICS_H_
#define API_AUDIO_AUDIO_PROCESSING_STATISTICS_H_

#include <stdint.h>

#include <optional>

#include "rtc_base/system/rtc_export.h"

namespace webrtc {
// This version of the stats uses Optionals, it will replace the regular
// AudioProcessingStatistics struct.
struct RTC_EXPORT AudioProcessingStats {
  AudioProcessingStats();
  AudioProcessingStats(const AudioProcessingStats& other);
  ~AudioProcessingStats();

  // Deprecated.
  // TODO(bugs.webrtc.org/11226): Remove.
  // True if voice is detected in the last capture frame, after processing.
  // It is conservative in flagging audio as speech, with low likelihood of
  // incorrectly flagging a frame as voice.
  // Only reported if voice detection is enabled in AudioProcessing::Config.
  std::optional<bool> voice_detected;

  // AEC Statistics.
  // ERL = 10log_10(P_far / P_echo)
  std::optional<double> echo_return_loss;
  // ERLE = 10log_10(P_echo / P_out)
  std::optional<double> echo_return_loss_enhancement;
  // Fraction of time that the AEC linear filter is divergent, in a 1-second
  // non-overlapped aggregation window.
  std::optional<double> divergent_filter_fraction;

  // The delay metrics consists of the delay median and standard deviation. It
  // also consists of the fraction of delay estimates that can make the echo
  // cancellation perform poorly. The values are aggregated until the first
  // call to `GetStatistics()` and afterwards aggregated and updated every
  // second. Note that if there are several clients pulling metrics from
  // `GetStatistics()` during a session the first call from any of them will
  // change to one second aggregation window for all.
  std::optional<int32_t> delay_median_ms;
  std::optional<int32_t> delay_standard_deviation_ms;

  // Residual echo detector likelihood.
  std::optional<double> residual_echo_likelihood;
  // Maximum residual echo likelihood from the last time period.
  std::optional<double> residual_echo_likelihood_recent_max;

  // The instantaneous delay estimate produced in the AEC. The unit is in
  // milliseconds and the value is the instantaneous value at the time of the
  // call to `GetStatistics()`.
  std::optional<int32_t> delay_ms;
};

}  // namespace webrtc

#endif  // API_AUDIO_AUDIO_PROCESSING_STATISTICS_H_
