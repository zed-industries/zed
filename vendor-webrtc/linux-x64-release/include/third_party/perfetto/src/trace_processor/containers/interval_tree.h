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

#ifndef SRC_TRACE_PROCESSOR_CONTAINERS_INTERVAL_TREE_H_
#define SRC_TRACE_PROCESSOR_CONTAINERS_INTERVAL_TREE_H_

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <limits>
#include <memory>
#include <utility>
#include <vector>

#include "perfetto/base/logging.h"
#include "perfetto/ext/base/small_vector.h"

namespace perfetto::trace_processor {

using Ts = uint64_t;
using Id = uint32_t;

struct Interval {
  Ts start;
  Ts end;
  Id id;
};

// An implementation of a centered interval tree data structure, designed to
// efficiently find all overlap queries on a set of intervals. Centered interval
// tree has a build complexity of O(N*logN) and a query time of O(logN + k),
// where k is the number of overlaps in the dataset.
class IntervalTree {
 public:
  // Creates an interval tree from the vector of intervals if needed. Otherwise
  // copies the vector of intervals.
  explicit IntervalTree(const std::vector<Interval>& sorted_intervals) {
    PERFETTO_CHECK(!sorted_intervals.empty());
    nodes_.reserve(sorted_intervals.size());
    Node root_node(sorted_intervals.data(), sorted_intervals.size(), nodes_);
    nodes_.emplace_back(std::move(root_node));
    root_ = nodes_.size() - 1;
  }

  // Modifies |res| to contain Interval::Id of all intervals that overlap
  // interval (s, e). Has a complexity of O(log(size of tree) + (number of
  // overlaps)).
  void FindOverlaps(uint64_t s, uint64_t e, std::vector<Id>& res) const {
    std::vector<const Node*> stack{nodes_.data() + root_};
    while (!stack.empty()) {
      const Node* n = stack.back();
      stack.pop_back();

      for (const Interval& i : n->intervals_) {
        // As we know that each interval overlaps the center, if the interval
        // starts after the |end| we know [start,end] can't intersect the
        // center.
        if (i.start > e) {
          break;
        }

        if (e > i.start && s < i.end) {
          res.push_back(i.id);
        }
      }

      if (e > n->center_ &&
          n->right_node_ != std::numeric_limits<size_t>::max()) {
        stack.push_back(&nodes_[n->right_node_]);
      }
      if (s < n->center_ &&
          n->left_node_ != std::numeric_limits<size_t>::max()) {
        stack.push_back(&nodes_[n->left_node_]);
      }
    }
  }

  // Modifies |res| to contain all overlaps (as Intervals) that overlap interval
  // (s, e). Has a complexity of O(log(size of tree) + (number of overlaps)).
  void FindOverlaps(Ts s, Ts e, std::vector<Interval>& res) const {
    std::vector<const Node*> stack{nodes_.data() + root_};
    while (!stack.empty()) {
      const Node* n = stack.back();
      stack.pop_back();

      for (const Interval& i : n->intervals_) {
        // As we know that each interval overlaps the center, if the interval
        // starts after the |end| we know [start,end] can't intersect the
        // center.
        if (i.start > e) {
          break;
        }

        if (e > i.start && s < i.end) {
          Interval new_int;
          new_int.start = std::max(s, i.start);
          new_int.end = std::min(e, i.end);
          new_int.id = i.id;
          res.push_back(new_int);
        }
      }

      if (e > n->center_ &&
          n->right_node_ != std::numeric_limits<size_t>::max()) {
        stack.push_back(&nodes_[n->right_node_]);
      }
      if (s < n->center_ &&
          n->left_node_ != std::numeric_limits<size_t>::max()) {
        stack.push_back(&nodes_[n->left_node_]);
      }
    }
  }

 private:
  struct Node {
    base::SmallVector<Interval, 2> intervals_;
    uint64_t center_ = 0;
    size_t left_node_ = std::numeric_limits<size_t>::max();
    size_t right_node_ = std::numeric_limits<size_t>::max();

    explicit Node(const Interval* intervals,
                  size_t intervals_size,
                  std::vector<Node>& nodes) {
      const Interval& mid_interval = intervals[intervals_size / 2];
      center_ = (mid_interval.start + mid_interval.end) / 2;

      // Find intervals that overlap the center_ and intervals that belong to
      // the left node (finish before the center_). If an interval starts after
      // the center break and assign all remaining intervals to the right node.
      // We can do this as the provided intervals are in sorted order.
      std::vector<Interval> left;
      for (uint32_t i = 0; i < intervals_size; i++) {
        const Interval& inter = intervals[i];
        // Starts after the center. As intervals are sorted on timestamp we
        // know the rest of intervals will go to the right node.
        if (inter.start > center_) {
          Node n(intervals + i, intervals_size - i, nodes);
          nodes.emplace_back(std::move(n));
          right_node_ = nodes.size() - 1;
          break;
        }

        // Finishes before the center.
        if (inter.end < center_) {
          left.push_back(intervals[i]);
        } else {
          // Overlaps the center.
          intervals_.emplace_back(intervals[i]);
        }
      }

      if (!left.empty()) {
        Node n(left.data(), left.size(), nodes);
        nodes.emplace_back(std::move(n));
        left_node_ = nodes.size() - 1;
      }
    }

    Node(const Node&) = delete;
    Node& operator=(const Node&) = delete;

    Node(Node&&) = default;
    Node& operator=(Node&&) = default;
  };

  size_t root_;
  std::vector<Node> nodes_;
};
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_CONTAINERS_INTERVAL_TREE_H_
