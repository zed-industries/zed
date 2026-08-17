/* Copyright 2022 The TensorFlow Authors. All Rights Reserved.

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

#ifndef TENSORFLOW_LITE_SUPPORT_PYTHON_TASK_CORE_PYBINDS_TASK_UTILS_H_
#define TENSORFLOW_LITE_SUPPORT_PYTHON_TASK_CORE_PYBINDS_TASK_UTILS_H_

#include <stdexcept>

#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/proto/base_options_proto_inc.h"
#include "tensorflow_lite_support/python/task/core/proto/base_options.pb.h"

namespace tflite {
namespace task {
namespace core {

// Converts Python BaseOptions to the base options used in C++.
// Python BaseOptions is a subset of the C++ BaseOptions that strips off
// configurations that are useless in Python development.
std::unique_ptr<::tflite::task::core::BaseOptions> convert_to_cpp_base_options(
    ::tflite::python::task::core::BaseOptions options);

// Returns the object value if the status is ok, otherwise throw the runtime
// error.
template <typename T>
inline T get_value(tflite::support::StatusOr<T>& status_or_object) {
  if (status_or_object.ok()) {
    return std::move(status_or_object.value());
  } else if (absl::IsInvalidArgument(status_or_object.status())) {
    throw std::invalid_argument(
        std::string(status_or_object.status().message()));
  } else {
    throw std::runtime_error(std::string(status_or_object.status().message()));
  }
}

}  // namespace core
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_PYTHON_TASK_CORE_PYBINDS_TASK_UTILS_H_
