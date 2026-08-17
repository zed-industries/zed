/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_TESTSUPPORT_PERF_TEST_H_
#define TEST_TESTSUPPORT_PERF_TEST_H_

#include <sstream>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/numerics/samples_stats_counter.h"

namespace webrtc {
namespace test {

enum class ImproveDirection {
  // Direction is undefined.
  kNone,
  // Smaller value is better.
  kSmallerIsBetter,
  // Bigger value is better.
  kBiggerIsBetter,
};

// Prints a performance test result.
//
// For example,
// PrintResult("ramp_up_time_", "turn_over_tcp",
//             "bwe_15s", 1234.2, "ms", false);
//
// will show up in the http://chromeperf.appspot.com under
//
// (test binary name) > (bot) > ramp_up_time_turn_over_tcp > bwe_15s.
//
// The `measurement` + `modifier` is what we're measuring. `user_story` is the
// scenario we're testing under.
//
// The binary this runs in must be hooked up as a perf test in the WebRTC
// recipes for this to actually be uploaded to chromeperf.appspot.com.
void PrintResult(absl::string_view measurement,
                 absl::string_view modifier,
                 absl::string_view user_story,
                 double value,
                 absl::string_view units,
                 bool important,
                 ImproveDirection improve_direction = ImproveDirection::kNone);

// Like PrintResult(), but prints a (mean, standard deviation) result pair.
// The |<values>| should be two comma-separated numbers, the mean and
// standard deviation (or other error metric) of the measurement.
// DEPRECATED: soon unsupported.
void PrintResultMeanAndError(
    absl::string_view measurement,
    absl::string_view modifier,
    absl::string_view user_story,
    double mean,
    double error,
    absl::string_view units,
    bool important,
    ImproveDirection improve_direction = ImproveDirection::kNone);

// Like PrintResult(), but prints an entire list of results. The `values`
// will generally be a list of comma-separated numbers. A typical
// post-processing step might produce plots of their mean and standard
// deviation.
void PrintResultList(
    absl::string_view measurement,
    absl::string_view modifier,
    absl::string_view user_story,
    ArrayView<const double> values,
    absl::string_view units,
    bool important,
    ImproveDirection improve_direction = ImproveDirection::kNone);

// Like PrintResult(), but prints a (mean, standard deviation) from stats
// counter. Also add specified metric to the plotable metrics output.
void PrintResult(absl::string_view measurement,
                 absl::string_view modifier,
                 absl::string_view user_story,
                 const SamplesStatsCounter& counter,
                 absl::string_view units,
                 bool important,
                 ImproveDirection improve_direction = ImproveDirection::kNone);

// Returns a string-encoded proto as described in
// tracing/tracing/proto/histogram.proto in
// https://github.com/catapult-project/catapult/blob/master/.
// If you want to print the proto in human readable format, use
// tracing/bin/proto2json from third_party/catapult in your WebRTC checkout.
std::string GetPerfResults();

// Print into stdout plottable metrics for further post processing.
// `desired_graphs` - list of metrics, that should be plotted. If empty - all
// available metrics will be plotted. If some of `desired_graphs` are missing
// they will be skipped.
void PrintPlottableResults(const std::vector<std::string>& desired_graphs);

// Call GetPerfResults() and write its output to a file. Returns false if we
// failed to write to the file. If you want to print the proto in human readable
// format, use tracing/bin/proto2json from third_party/catapult in your WebRTC
// checkout.
bool WritePerfResults(const std::string& output_path);

// By default, human-readable perf results are printed to stdout. Set the FILE*
// to where they should be printing instead. These results are not used to
// upload to the dashboard, however - this is only through WritePerfResults.
void SetPerfResultsOutput(FILE* output);

// Only for use by tests.
void ClearPerfResults();

}  // namespace test
}  // namespace webrtc

#endif  // TEST_TESTSUPPORT_PERF_TEST_H_
