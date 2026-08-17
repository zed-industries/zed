/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_METRICS_METRICS_LOGGER_H_
#define API_TEST_METRICS_METRICS_LOGGER_H_

#include <map>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/numerics/samples_stats_counter.h"
#include "api/test/metrics/metric.h"
#include "api/test/metrics/metrics_accumulator.h"
#include "api/units/timestamp.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {
namespace test {

// Provides API to log and collect performance metrics.
class MetricsLogger {
 public:
  virtual ~MetricsLogger() = default;

  // Adds a metric with a single value.
  // `metadata` - metric's level metadata to add.
  virtual void LogSingleValueMetric(
      absl::string_view name,
      absl::string_view test_case_name,
      double value,
      Unit unit,
      ImprovementDirection improvement_direction,
      std::map<std::string, std::string> metadata = {}) = 0;

  // Adds metrics with a time series created based on the provided `values`.
  // `metadata` - metric's level metadata to add.
  virtual void LogMetric(absl::string_view name,
                         absl::string_view test_case_name,
                         const SamplesStatsCounter& values,
                         Unit unit,
                         ImprovementDirection improvement_direction,
                         std::map<std::string, std::string> metadata = {}) = 0;

  // Adds metric with a time series with only stats object and without actual
  // collected values.
  // `metadata` - metric's level metadata to add.
  virtual void LogMetric(absl::string_view name,
                         absl::string_view test_case_name,
                         const Metric::Stats& metric_stats,
                         Unit unit,
                         ImprovementDirection improvement_direction,
                         std::map<std::string, std::string> metadata = {}) = 0;

  // Returns all metrics collected by this logger.
  virtual std::vector<Metric> GetCollectedMetrics() const = 0;
};

class DefaultMetricsLogger : public MetricsLogger {
 public:
  explicit DefaultMetricsLogger(webrtc::Clock* clock) : clock_(clock) {}
  ~DefaultMetricsLogger() override = default;

  void LogSingleValueMetric(
      absl::string_view name,
      absl::string_view test_case_name,
      double value,
      Unit unit,
      ImprovementDirection improvement_direction,
      std::map<std::string, std::string> metadata = {}) override;

  void LogMetric(absl::string_view name,
                 absl::string_view test_case_name,
                 const SamplesStatsCounter& values,
                 Unit unit,
                 ImprovementDirection improvement_direction,
                 std::map<std::string, std::string> metadata = {}) override;

  void LogMetric(absl::string_view name,
                 absl::string_view test_case_name,
                 const Metric::Stats& metric_stats,
                 Unit unit,
                 ImprovementDirection improvement_direction,
                 std::map<std::string, std::string> metadata = {}) override;

  // Returns all metrics collected by this logger and its `MetricsAccumulator`.
  std::vector<Metric> GetCollectedMetrics() const override;

  MetricsAccumulator* GetMetricsAccumulator() { return &metrics_accumulator_; }

 private:
  webrtc::Timestamp Now();

  webrtc::Clock* const clock_;
  MetricsAccumulator metrics_accumulator_;

  mutable Mutex mutex_;
  std::vector<Metric> metrics_ RTC_GUARDED_BY(mutex_);
};

}  // namespace test
}  // namespace webrtc

#endif  // API_TEST_METRICS_METRICS_LOGGER_H_
