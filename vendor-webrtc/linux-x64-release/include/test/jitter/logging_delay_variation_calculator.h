/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_JITTER_LOGGING_DELAY_VARIATION_CALCULATOR_H_
#define TEST_JITTER_LOGGING_DELAY_VARIATION_CALCULATOR_H_

#include <string>

#include "absl/strings/string_view.h"
#include "api/test/metrics/global_metrics_logger_and_exporter.h"
#include "api/units/data_size.h"
#include "test/jitter/delay_variation_calculator.h"

namespace webrtc {
namespace test {

// This class logs the results from a `DelayVariationCalculator`
// to a metrics logger. For ease of integration, logging happens at
// object destruction.
class LoggingDelayVariationCalculator {
 public:
  LoggingDelayVariationCalculator(
      absl::string_view log_type,
      DefaultMetricsLogger* logger = GetGlobalMetricsLogger())
      : log_type_(log_type), logger_(logger) {}
  ~LoggingDelayVariationCalculator() { LogMetrics(); }

  void Insert(uint32_t rtp_timestamp,
              Timestamp arrival_time,
              DataSize size,
              std::optional<int> spatial_layer = std::nullopt,
              std::optional<int> temporal_layer = std::nullopt,
              std::optional<VideoFrameType> frame_type = std::nullopt);

 private:
  void LogMetrics() const;

  const std::string log_type_;
  DefaultMetricsLogger* const logger_;
  DelayVariationCalculator calc_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_JITTER_LOGGING_DELAY_VARIATION_CALCULATOR_H_
