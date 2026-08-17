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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TEST_UTILS_TEST_UTILS_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TEST_UTILS_TEST_UTILS_H_

#include <glog/logging.h>
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/port/proto2.h"

namespace tflite {
namespace task {
namespace internal {

// Not part of the public API.
std::string JoinPathImpl(bool honor_abs,
                         std::initializer_list<absl::string_view> paths);

}  // namespace internal

std::string JoinPath(absl::string_view path1, absl::string_view path2);

template <typename... T>
inline std::string JoinPath(absl::string_view path1, absl::string_view path2,
                            absl::string_view path3, const T&... args) {
  return internal::JoinPathImpl(false, {path1, path2, path3, args...});
}

template <typename T>
T ParseTextProtoOrDie(const std::string& input) {
  T result;
  CHECK(tflite::support::proto::TextFormat::ParseFromString(input, &result));
  return result;
}

}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TEST_UTILS_TEST_UTILS_H_
