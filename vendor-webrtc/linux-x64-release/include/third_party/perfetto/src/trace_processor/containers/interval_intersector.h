/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_CONTAINERS_INTERVAL_INTERSECTOR_H_
#define SRC_TRACE_PROCESSOR_CONTAINERS_INTERVAL_INTERSECTOR_H_

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <limits>
#include <memory>
#include <utility>
#include <vector>

#include "perfetto/base/logging.h"
#include "src/trace_processor/containers/interval_tree.h"

namespace perfetto::trace_processor {

// Provides functionality for efficient intersection of a set of intervals with
// another interval. Operates in various modes: using interval tree, binary
// search of non overlapping intervals or linear scan if the intervals are
// overlapping but there is no need for interval tree.
class IntervalIntersector {
 public:
  // Mode of intersection. Choosing the mode strongly impacts the performance
  // of intersector.
  enum Mode {
    // Use `IntervalTree` as an underlying implementation.. Would create an
    // interval tree - with complexity of O(N) and memory complexity of O(N).
    // Query cost is O(logN).
    // NOTE: Only use if intervals are overlapping and tree would be queried
    // multiple times.
    kIntervalTree,
    // If the intervals are non overlapping we can use simple binary search.
    // There is no memory cost and algorithmic complexity of O(logN + M), where
    // logN is the cost of binary search and M is the number of results.
    // NOTE: Only use if intervals are non overlapping.
    kBinarySearch,
    // Slightly better then linear scan, we are looking for the first
    // overlapping interval and then doing linear scan of the rest. NOTE: Only
    // use if intervals are overlapping and there would be very few queries.
    kLinearScan
  };

  // Creates an interval tree from the vector of intervals if needed. Otherwise
  // copies the vector of intervals.
  explicit IntervalIntersector(const std::vector<Interval>& sorted_intervals,
                               Mode mode)
      : intervals_(sorted_intervals), mode_(mode) {
    if (sorted_intervals.empty()) {
      mode_ = kBinarySearch;
      return;
    }
    if (mode_ == kIntervalTree) {
      tree = std::make_unique<IntervalTree>(intervals_);
    }
  }

  // Modifies |res| to contain Interval::Id of all intervals that overlap
  // interval (s, e).
  template <typename T>
  void FindOverlaps(uint64_t s, uint64_t e, std::vector<T>& res) const {
    if (mode_ == kIntervalTree) {
      tree->FindOverlaps(s, e, res);
      return;
    }

    if (mode_ == kBinarySearch) {
      // Find the first interval that ends after |s|.
      auto overlap =
          std::lower_bound(intervals_.begin(), intervals_.end(), s,
                           [](const Interval& interval, uint64_t start) {
                             return interval.end <= start;
                           });

      for (; overlap != intervals_.end() && overlap->start < e; ++overlap) {
        UpdateResultVector(s, e, *overlap, res);
      }
      return;
    }

    // When using linear scan, we know only that the that if interval starts
    // after the |e|, it will not overlap. We need to go through all intervals
    // up to this point, as we don't know if any of the previous one is not
    // overlapping.
    PERFETTO_CHECK(mode_ == kLinearScan);

    auto cur_interval = intervals_.begin();

    // Go through all intervals that start before |s|.
    for (; cur_interval != intervals_.end(); ++cur_interval) {
      // An interval that ends before |s| can't overlap.
      if (cur_interval->end <= s) {
        continue;
      }

      // Escape if the interval starts after |s|.
      if (cur_interval->start > s) {
        break;
      }

      UpdateResultVector(s, e, *cur_interval, res);
    }

    // Go through all intervals that start after |s| and before |e|.
    for (; cur_interval != intervals_.end() && cur_interval->start < e;
         ++cur_interval) {
      UpdateResultVector(s, e, *cur_interval, res);
    }
  }

  // Helper function to decide which intersector mode would be in given
  // situations. Only use if the number of queries is known.
  static Mode DecideMode(bool is_nonoverlapping, uint32_t queries_count) {
    if (is_nonoverlapping) {
      return kBinarySearch;
    }
    if (queries_count < 5) {
      return kLinearScan;
    }
    return kIntervalTree;
  }

 private:
  void UpdateResultVector(uint64_t s,
                          uint64_t e,
                          const Interval& overlap,
                          std::vector<Interval>& res) const {
    Interval new_int;
    new_int.start = std::max(s, overlap.start);
    new_int.end = std::min(e, overlap.end);
    new_int.id = overlap.id;
    res.push_back(new_int);
  }

  void UpdateResultVector(uint64_t,
                          uint64_t,
                          const Interval& overlap,

                          std::vector<Id>& res) const {
    res.push_back(overlap.id);
  }
  const std::vector<Interval>& intervals_;
  Mode mode_;

  // If |use_interval_tree_|.
  std::unique_ptr<IntervalTree> tree;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_CONTAINERS_INTERVAL_INTERSECTOR_H_
