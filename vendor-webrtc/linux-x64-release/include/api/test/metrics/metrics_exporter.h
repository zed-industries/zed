/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_METRICS_METRICS_EXPORTER_H_
#define API_TEST_METRICS_METRICS_EXPORTER_H_

#include "api/array_view.h"
#include "api/test/metrics/metric.h"

namespace webrtc {
namespace test {

// Exports metrics in the requested format.
class MetricsExporter {
 public:
  virtual ~MetricsExporter() = default;

  // Exports specified metrics in a format that depends on the implementation.
  // Returns true if export succeeded, false otherwise.
  virtual bool Export(ArrayView<const Metric> metrics) = 0;
};

}  // namespace test
}  // namespace webrtc

#endif  // API_TEST_METRICS_METRICS_EXPORTER_H_
