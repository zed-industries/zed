/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_METRICS_METRICS_SET_PROTO_FILE_EXPORTER_H_
#define API_TEST_METRICS_METRICS_SET_PROTO_FILE_EXPORTER_H_

#include <map>
#include <string>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/test/metrics/metric.h"
#include "api/test/metrics/metrics_exporter.h"

namespace webrtc {
namespace test {

// Exports all collected metrics to the proto file using
// `webrtc::test_metrics::MetricsSet` format.
class MetricsSetProtoFileExporter : public MetricsExporter {
 public:
  struct Options {
    explicit Options(absl::string_view export_file_path);
    Options(absl::string_view export_file_path, bool export_whole_time_series);
    Options(absl::string_view export_file_path,
            std::map<std::string, std::string> metadata);

    // File to export proto.
    std::string export_file_path;
    // If true will write all time series values to the output proto file,
    // otherwise will write stats only.
    bool export_whole_time_series = true;
    // Metadata associated to the whole MetricsSet.
    std::map<std::string, std::string> metadata;
  };

  explicit MetricsSetProtoFileExporter(const Options& options)
      : options_(options) {}

  MetricsSetProtoFileExporter(const MetricsSetProtoFileExporter&) = delete;
  MetricsSetProtoFileExporter& operator=(const MetricsSetProtoFileExporter&) =
      delete;

  bool Export(ArrayView<const Metric> metrics) override;

 private:
  const Options options_;
};

}  // namespace test
}  // namespace webrtc

#endif  // API_TEST_METRICS_METRICS_SET_PROTO_FILE_EXPORTER_H_
