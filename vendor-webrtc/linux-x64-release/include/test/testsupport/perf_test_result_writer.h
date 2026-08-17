/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_TESTSUPPORT_PERF_TEST_RESULT_WRITER_H_
#define TEST_TESTSUPPORT_PERF_TEST_RESULT_WRITER_H_

#include <stdio.h>

#include <string>

#include "absl/strings/string_view.h"
#include "test/testsupport/perf_test.h"

namespace webrtc {
namespace test {

// Interface for classes that write perf results to some kind of JSON format.
class PerfTestResultWriter {
 public:
  virtual ~PerfTestResultWriter() = default;

  virtual void ClearResults() = 0;
  virtual void LogResult(absl::string_view graph_name,
                         absl::string_view trace_name,
                         double value,
                         absl::string_view units,
                         bool important,
                         webrtc::test::ImproveDirection improve_direction) = 0;
  virtual void LogResultMeanAndError(
      absl::string_view graph_name,
      absl::string_view trace_name,
      double mean,
      double error,
      absl::string_view units,
      bool important,
      webrtc::test::ImproveDirection improve_direction) = 0;
  virtual void LogResultList(
      absl::string_view graph_name,
      absl::string_view trace_name,
      ArrayView<const double> values,
      absl::string_view units,
      bool important,
      webrtc::test::ImproveDirection improve_direction) = 0;

  virtual std::string Serialize() const = 0;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_TESTSUPPORT_PERF_TEST_RESULT_WRITER_H_
