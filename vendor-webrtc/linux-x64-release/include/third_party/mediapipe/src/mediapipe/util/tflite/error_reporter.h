// Copyright 2022 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_UTIL_TFLITE_ERROR_REPORTER_H_
#define MEDIAPIPE_UTIL_TFLITE_ERROR_REPORTER_H_

#include <string>

#include "tensorflow/lite/core/api/error_reporter.h"
#include "tensorflow/lite/stateful_error_reporter.h"

namespace mediapipe::util::tflite {

// An ErrorReporter that logs to stderr and captures the last two messages.
class ErrorReporter : public ::tflite::StatefulErrorReporter {
 public:
  static constexpr int kBufferSize = 1024;

  ErrorReporter();

  // We declared two functions with name 'Report', so that the variadic Report
  // function in tflite::ErrorReporter is hidden.
  // See https://isocpp.org/wiki/faq/strange-inheritance#hiding-rule.
  using ::tflite::ErrorReporter::Report;

  int Report(const char* format, std::va_list args) override;

  // Returns true if any error was reported.
  bool HasError() const;

  std::string message() override;
  std::string previous_message();

 private:
  char message_[kBufferSize];
  char previous_message_[kBufferSize];
};

}  // namespace mediapipe::util::tflite

#endif  // MEDIAPIPE_UTIL_TFLITE_ERROR_REPORTER_H_
