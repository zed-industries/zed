/* Copyright 2020 The TensorFlow Authors. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_ERROR_REPORTER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_ERROR_REPORTER_H_

#include <cstdarg>
#include <string>

#include "tensorflow/lite/stateful_error_reporter.h"

namespace tflite {
namespace task {
namespace core {

// An ErrorReporter that logs to stderr and captures the last two messages.
class ErrorReporter : public tflite::StatefulErrorReporter {
 public:
  ErrorReporter() {
    last_message_[0] = '\0';
    second_last_message_[0] = '\0';
  }

  // We declared two functions with name 'Report', so that the variadic Report
  // function in tflite::ErrorReporter is hidden.
  // See https://isocpp.org/wiki/faq/strange-inheritance#hiding-rule.
  using tflite::ErrorReporter::Report;

  int Report(const char* format, std::va_list args) override;

  std::string message() override;
  std::string previous_message();

 private:
  static constexpr int kBufferSize = 1024;
  char last_message_[kBufferSize];
  char second_last_message_[kBufferSize];
};

}  // namespace core
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_ERROR_REPORTER_H_
