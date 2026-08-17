// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Helper class and macro to take time measurements within current scope.
// Takes time measurement within current scope. Outputs to ABSL_LOG(INFO) if
// flag --measure_time is set or if build flag SET_FLAG_MEASURE_TIME is
// defined (add --copt=-DSET_FLAG_MEASURE_TIME to your build command).
// Additionally you can limit time measurements to specific files,
// via the flag
// --measure_time_filter="<comma separated list of substrings of file names>"
// Example:
// {     // Scope to be measured
//   MEASURE_TIME << "Some additional logging and answers : " << 42;
//   ...  // instructions.
// }

#ifndef MEDIAPIPE_UTIL_TRACKING_MEASURE_TIME_H_
#define MEDIAPIPE_UTIL_TRACKING_MEASURE_TIME_H_

#include <cstdint>
#include <memory>
#include <sstream>

#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/strings/str_split.h"
#include "absl/strings/string_view.h"
#include "absl/synchronization/mutex.h"
#include "absl/time/clock.h"

extern bool flags_measure_time;

namespace mediapipe {
class MeasureTimeFilter;
}  // namespace mediapipe

#define MEASURE_TIME \
  MEASURE_TIME_PRE_IMPL(flags_measure_time, __LINE__, __FILE__)

// Add level of indirection to trigger __LINE__ macro expansion.
#define MEASURE_TIME_PRE_IMPL(show_output, line, file) \
  MEASURE_TIME_IMPL(show_output, line, file)

#define MEASURE_TIME_IMPL(show_output, line, file)                      \
  std::unique_ptr<mediapipe::ScopedWallTimer> scoped_wall_timer_##line; \
  const bool activated##line = show_output;                             \
  if (activated##line) {                                                \
    static mediapipe::ScopedWallTimer::Accumulator*                     \
        scoped_wall_timer_accum_##line =                                \
            new mediapipe::ScopedWallTimer::Accumulator;                \
    scoped_wall_timer_##line.reset(new mediapipe::ScopedWallTimer(      \
        file, line, show_output, scoped_wall_timer_accum_##line));      \
  }                                                                     \
  if (activated##line) /* NOLINT */                                     \
  scoped_wall_timer_##line->stream()

namespace mediapipe {

class ScopedWallTimer {
 public:
  // Helper class for accumulating time across multiple calls to a scoped wall
  // timer. Thread-safe (except on ANDROID, for which Mutex is mocked out).
  class Accumulator {
   public:
    Accumulator() : accum_time_(0.0f), count_(0) {}
    Accumulator(const Accumulator&) = delete;
    Accumulator& operator=(const Accumulator&) = delete;

    // Accumulates passed_time into accumulators. Returns total time and number
    // of calls made.
    void Accumulate(double passed_time, double* accum_time, int* count) {
      absl::MutexLock lock(&mutex_);
      accum_time_ += passed_time;
      ++count_;
      *accum_time = accum_time_;
      *count = count_;
    }

   private:
    double accum_time_;
    int count_;
    absl::Mutex mutex_;
  };

  // Creates a new ScopedWallTimer for current file and line. LogMessage is only
  // initialized if show_output is set to true.
  ScopedWallTimer(const char* file, int line, bool show_output,
                  Accumulator* accumulator)
      : file_(file),
        line_(line),
        show_output_(show_output),
        accumulator_(accumulator) {
    if (show_output_) {
      ABSL_CHECK(accumulator_);
      start_time_ = GetWallTime();
    }
  }
  ScopedWallTimer(const ScopedWallTimer&) = delete;
  ScopedWallTimer& operator=(const ScopedWallTimer&) = delete;

  // Destructor measures time and outputs to stream.
  ~ScopedWallTimer() {
    if (show_output_) {
      double passed_time = GetWallTime() - start_time_;
      double accum_time = 0.0;
      int count = 0;
      accumulator_->Accumulate(passed_time, &accum_time, &count);
      ABSL_LOG(INFO) << stream_.str() << " TIMES: [Curr: " << passed_time * 1e-6
                     << " ms, "
                     << "Avg: " << accum_time * 1e-6 / std::max(1, count)
                     << " ms, " << count << " calls]";
    }
  }

  std::ostream& stream() { return stream_; }

 private:
  const char* file_;
  int line_;
  bool show_output_;
  // We need to buffer information passed via stream operator <<
  // While LogMessage is adequate for this, no good equivalent exists on
  // Android, so we employ a portable ostringstream for buffering.
  std::ostringstream stream_;
  int64_t start_time_;
  Accumulator* accumulator_;

  int64_t GetWallTime() { return absl::GetCurrentTimeNanos(); }
};

class MeasureTimeFilter {
 public:
  static const MeasureTimeFilter* get() {
    static MeasureTimeFilter instance;
    return &instance;
  }
  MeasureTimeFilter(const MeasureTimeFilter&) = delete;
  MeasureTimeFilter& operator=(const MeasureTimeFilter&) = delete;

  bool Matches(const std::string& item) const {
    for (const std::string& match_item : match_items_) {
      if (item.find(match_item) != std::string::npos) {
        return true;
      }
    }
    return false;
  }

 private:
  explicit MeasureTimeFilter() {}
  explicit MeasureTimeFilter(const std::string& filter) {
    match_items_ = absl::StrSplit(filter, absl::ByChar(','));
  }
  std::vector<std::string> match_items_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_MEASURE_TIME_H_
