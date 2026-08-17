/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_METRICS_CHROME_PERF_DASHBOARD_METRICS_EXPORTER_H_
#define API_TEST_METRICS_CHROME_PERF_DASHBOARD_METRICS_EXPORTER_H_

#include <string>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/test/metrics/metric.h"
#include "api/test/metrics/metrics_exporter.h"

namespace webrtc {
namespace test {

// Exports all collected metrics in the Chrome Perf Dashboard proto format.
class ChromePerfDashboardMetricsExporter : public MetricsExporter {
 public:
  // `export_file_path` - path where the proto file will be written.
  explicit ChromePerfDashboardMetricsExporter(
      absl::string_view export_file_path);
  ~ChromePerfDashboardMetricsExporter() override = default;

  bool Export(ArrayView<const Metric> metrics) override;

 private:
  const std::string export_file_path_;
};

}  // namespace test
}  // namespace webrtc

#endif  // API_TEST_METRICS_CHROME_PERF_DASHBOARD_METRICS_EXPORTER_H_
