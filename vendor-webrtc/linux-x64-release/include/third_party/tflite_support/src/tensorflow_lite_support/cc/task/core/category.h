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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_CATEGORY_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_CATEGORY_H_

#include <cmath>
#include <string>

namespace tflite {
namespace task {
namespace core {

// Result for classification APIs.
struct Category {
  std::string class_name;
  double score;
  Category(const std::string& class_name, double score)
      : class_name(class_name), score(score) {}

  friend bool operator==(const Category& lhs, const Category& rhs) {
    constexpr const double kScoreTolerance = 1e-6;
    return lhs.class_name == rhs.class_name &&
           std::abs((double)(lhs.score - rhs.score)) <= kScoreTolerance;
  }

  friend bool operator!=(const Category& lhs, const Category& rhs) {
    return !(lhs == rhs);
  }
};

}  // namespace core
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_CORE_CATEGORY_H_
