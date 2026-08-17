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

#ifndef TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_TOP_N_AMORTIZED_CONSTANT_H_
#define TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_TOP_N_AMORTIZED_CONSTANT_H_

#include <algorithm>
#include <vector>

#include <glog/logging.h>
template <class T, class Cmp = std::greater<T>>
class TopNAmortizedConstant {
 public:
  TopNAmortizedConstant(size_t limit, T approx_bottom)
      : limit_(limit),
        approx_bottom_(approx_bottom),
        original_approx_bottom_(approx_bottom),
        elements_(),
        cmp_() {
    DCHECK_GT(limit_, 0);
  }
  TopNAmortizedConstant() {}
  template <typename... Args>
  void emplace(Args... args) {
    if (cmp_(args..., approx_bottom_)) {
      elements_.emplace_back(T(args...));
      if (elements_.size() >= 2 * limit_) {
        PartitionAndResizeToLimit();
      }
    }
  }
  std::vector<T> TakeUnsorted() {
    DCHECK_GT(limit_, 0) << "Cannot call TakeUnsorted on uninitialized "
                            "TopNAmortizedConstant instance.";
    if (elements_.size() > limit_) PartitionAndResizeToLimit();
    auto result = std::move(elements_);
    elements_.clear();
    approx_bottom_ = original_approx_bottom_;
    return result;
  }
  const std::vector<T>& ExtractUnsorted() {
    DCHECK_GT(limit_, 0) << "Cannot call ExtractUnsorted on uninitialized "
                            "TopNAmortizedConstant instance.";
    if (elements_.size() > limit_) PartitionAndResizeToLimit();
    return elements_;
  }
  std::vector<T> Take() {
    DCHECK_GT(limit_, 0) << "Cannot call Take on uninitialized "
                            "TopNAmortizedConstant instance.";
    if (elements_.size() > limit_) PartitionAndResizeToLimit();
    std::sort(elements_.begin(), elements_.end(), cmp_);
    auto result = std::move(elements_);
    elements_.clear();
    approx_bottom_ = original_approx_bottom_;
    return result;
  }
  const T& approx_bottom() const {
    DCHECK(!elements_.empty());
    return approx_bottom_;
  }
  size_t size() const { return std::min(limit_, elements_.size()); }
  size_t limit() const { return limit_; }
  void reserve(size_t n_elements) {
    DCHECK_LE(n_elements, 2 * limit_);
    elements_.reserve(n_elements);
  }

 private:
  void PartitionAndResizeToLimit() {
    DCHECK_GT(elements_.size(), limit_);
    std::nth_element(elements_.begin(), elements_.begin() + limit_ - 1,
                     elements_.end(), cmp_);
    elements_.resize(limit_);
    approx_bottom_ = elements_.back();
  }
  size_t limit_ = 0;
  T approx_bottom_;
  T original_approx_bottom_;
  std::vector<T> elements_;
  Cmp cmp_;
};

namespace tflite {
namespace scann_ondevice {
namespace core {
struct Comparator {
  bool operator()(const std::pair<float, int>& a,
                  const std::pair<float, int>& b) const {
    return a.first < b.first;
  }
  bool operator()(float distance, int,
                  const std::pair<float, int>& other) const {
    return distance < other.first;
  }
};

using TopN = TopNAmortizedConstant<std::pair<float, int>, Comparator>;

}  // namespace core
}  // namespace scann_ondevice
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_TOP_N_AMORTIZED_CONSTANT_H_
