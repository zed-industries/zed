/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NUMERICS_PERCENTILE_FILTER_H_
#define RTC_BASE_NUMERICS_PERCENTILE_FILTER_H_

#include <stdint.h>

#include <iterator>
#include <set>

#include "rtc_base/checks.h"

namespace webrtc {

// Class to efficiently get the percentile value from a group of observations.
// The percentile is the value below which a given percentage of the
// observations fall.
template <typename T>
class PercentileFilter {
 public:
  // Construct filter. `percentile` should be between 0 and 1.
  explicit PercentileFilter(float percentile);

  // Insert one observation. The complexity of this operation is logarithmic in
  // the size of the container.
  void Insert(const T& value);

  // Remove one observation or return false if `value` doesn't exist in the
  // container. The complexity of this operation is logarithmic in the size of
  // the container.
  bool Erase(const T& value);

  // Get the percentile value. The complexity of this operation is constant.
  T GetPercentileValue() const;

  // Removes all the stored observations.
  void Reset();

 private:
  // Update iterator and index to point at target percentile value.
  void UpdatePercentileIterator();

  const float percentile_;
  std::multiset<T> set_;
  // Maintain iterator and index of current target percentile value.
  typename std::multiset<T>::iterator percentile_it_;
  int64_t percentile_index_;
};

template <typename T>
PercentileFilter<T>::PercentileFilter(float percentile)
    : percentile_(percentile),
      percentile_it_(set_.begin()),
      percentile_index_(0) {
  RTC_CHECK_GE(percentile, 0.0f);
  RTC_CHECK_LE(percentile, 1.0f);
}

template <typename T>
void PercentileFilter<T>::Insert(const T& value) {
  // Insert element at the upper bound.
  set_.insert(value);
  if (set_.size() == 1u) {
    // First element inserted - initialize percentile iterator and index.
    percentile_it_ = set_.begin();
    percentile_index_ = 0;
  } else if (value < *percentile_it_) {
    // If new element is before us, increment `percentile_index_`.
    ++percentile_index_;
  }
  UpdatePercentileIterator();
}

template <typename T>
bool PercentileFilter<T>::Erase(const T& value) {
  typename std::multiset<T>::const_iterator it = set_.lower_bound(value);
  // Ignore erase operation if the element is not present in the current set.
  if (it == set_.end() || *it != value)
    return false;
  if (it == percentile_it_) {
    // If same iterator, update to the following element. Index is not
    // affected.
    percentile_it_ = set_.erase(it);
  } else {
    set_.erase(it);
    // If erased element was before us, decrement `percentile_index_`.
    if (value <= *percentile_it_)
      --percentile_index_;
  }
  UpdatePercentileIterator();
  return true;
}

template <typename T>
void PercentileFilter<T>::UpdatePercentileIterator() {
  if (set_.empty())
    return;
  const int64_t index = static_cast<int64_t>(percentile_ * (set_.size() - 1));
  std::advance(percentile_it_, index - percentile_index_);
  percentile_index_ = index;
}

template <typename T>
T PercentileFilter<T>::GetPercentileValue() const {
  return set_.empty() ? T() : *percentile_it_;
}

template <typename T>
void PercentileFilter<T>::Reset() {
  set_.clear();
  percentile_it_ = set_.begin();
  percentile_index_ = 0;
}
}  // namespace webrtc

#endif  // RTC_BASE_NUMERICS_PERCENTILE_FILTER_H_
