/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_METRICS_STDOUT_METRICS_EXPORTER_H_
#define API_TEST_METRICS_STDOUT_METRICS_EXPORTER_H_

#include <cstdio>

#include "api/array_view.h"
#include "api/test/metrics/metric.h"
#include "api/test/metrics/metrics_exporter.h"

namespace webrtc {
namespace test {

// Exports all collected metrics to stdout.
class StdoutMetricsExporter : public MetricsExporter {
 public:
  StdoutMetricsExporter();
  ~StdoutMetricsExporter() override = default;

  StdoutMetricsExporter(const StdoutMetricsExporter&) = delete;
  StdoutMetricsExporter& operator=(const StdoutMetricsExporter&) = delete;

  bool Export(ArrayView<const Metric> metrics) override;

 private:
  void PrintMetric(const Metric& metric);

  FILE* const output_;
};

}  // namespace test
}  // namespace webrtc

#endif  // API_TEST_METRICS_STDOUT_METRICS_EXPORTER_H_
