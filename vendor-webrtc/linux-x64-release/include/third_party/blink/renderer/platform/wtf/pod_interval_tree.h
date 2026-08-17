/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_INTERVAL_TREE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_INTERVAL_TREE_H_

#include <optional>

#include "third_party/blink/renderer/platform/wtf/pod_arena.h"
#include "third_party/blink/renderer/platform/wtf/pod_interval.h"
#include "third_party/blink/renderer/platform/wtf/pod_red_black_tree.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace WTF {

#ifndef NDEBUG
template <class T>
struct ValueToString;
#endif

template <class T, class UserData = void*>
class PODIntervalSearchAdapter {
  DISALLOW_NEW();

 public:
  typedef PODInterval<T, UserData> IntervalType;

  PODIntervalSearchAdapter(Vector<IntervalType>& result,
                           const T& low_value,
                           const T& high_value)
      : result_(result), low_value_(low_value), high_value_(high_value) {}

  const T& LowValue() const { return low_value_; }
  const T& HighValue() const { return high_value_; }
  void CollectIfNeeded(const IntervalType& data) const {
    if (data.Overlaps(low_value_, high_value_))
      result_.push_back(data);
  }

 private:
  Vector<IntervalType>& result_;
  T low_value_;
  T high_value_;
};

// An interval tree, which is a form of augmented red-black tree. It
// supports efficient (O(lg n)) insertion, removal and querying of
// intervals in the tree.
template <class T, class UserData = void*>
class PODIntervalTree final : public PODRedBlackTree<PODInterval<T, UserData>> {
 public:
  // Typedef to reduce typing when declaring intervals to be stored in
  // this tree.
  typedef PODInterval<T, UserData> IntervalType;
  typedef PODIntervalSearchAdapter<T, UserData> IntervalSearchAdapterType;

  PODIntervalTree(UninitializedTreeEnum unitialized_tree)
      : PODRedBlackTree<IntervalType>(unitialized_tree) {
    Init();
  }

  PODIntervalTree() : PODRedBlackTree<IntervalType>() { Init(); }

  explicit PODIntervalTree(scoped_refptr<PODArena> arena)
      : PODRedBlackTree<IntervalType>(arena) {
    Init();
  }

  PODIntervalTree(const PODIntervalTree&) = delete;
  PODIntervalTree& operator=(const PODIntervalTree&) = delete;

  // Returns all intervals in the tree which overlap the given query
  // interval. The returned intervals are sorted by increasing low
  // endpoint.
  Vector<IntervalType> AllOverlaps(const IntervalType& interval) const {
    Vector<IntervalType> result;
    AllOverlaps(interval, result);
    return result;
  }

  // Returns all intervals in the tree which overlap the given query
  // interval. The returned intervals are sorted by increasing low
  // endpoint.
  void AllOverlaps(const IntervalType& interval,
                   Vector<IntervalType>& result) const {
    // Explicit dereference of "this" required because of
    // inheritance rules in template classes.
    IntervalSearchAdapterType adapter(result, interval.Low(), interval.High());
    SearchForOverlapsFrom(this->Root(), adapter);
  }

  template <class AdapterType>
  void AllOverlapsWithAdapter(AdapterType& adapter) const {
    // Explicit dereference of "this" required because of
    // inheritance rules in template classes.
    SearchForOverlapsFrom(this->Root(), adapter);
  }

  // Helper to create interval objects.
  static IntervalType CreateInterval(const T& low,
                                     const T& high,
                                     const UserData data = nullptr) {
    return IntervalType(low, high, data);
  }

  bool CheckInvariants() const override {
    if (!PODRedBlackTree<IntervalType>::CheckInvariants())
      return false;
    if (!this->Root())
      return true;
    return CheckInvariantsFromNode(this->Root());
  }

  // Returns the next interval point (start or end) after the given starting
  // point (non-inclusive). If there is no such point, returns |std::nullopt|.
  std::optional<T> NextIntervalPoint(T start) const {
    return NextIntervalPoint(start, this->Root());
  }

 private:
  typedef typename PODRedBlackTree<IntervalType>::Node IntervalNode;

  // Initializes the tree.
  void Init() {
    // Explicit dereference of "this" required because of
    // inheritance rules in template classes.
    this->SetNeedsFullOrderingComparisons(true);
  }

  // Starting from the given node, adds all overlaps with the given
  // interval to the result vector. The intervals are sorted by
  // increasing low endpoint.
  template <class AdapterType>
  DISABLE_CFI_PERF static void SearchForOverlapsFrom(IntervalNode const* node,
                                                     AdapterType& adapter) {
    // This is phrased this way to avoid the need for operator
    // <= on type T.
    if (!node || adapter.HighValue() < node->Data().MinLow() ||
        node->Data().MaxHigh() < adapter.LowValue()) {
      return;
    }

    // Because the intervals are sorted by left endpoint, inorder
    // traversal produces results sorted as desired.

    // Attempt to traverse left subtree
    SearchForOverlapsFrom(node->Left(), adapter);

    // Check for overlap with current node.
    adapter.CollectIfNeeded(node->Data());

    // Attempt to traverse right subtree
    SearchForOverlapsFrom(node->Right(), adapter);
  }

  static std::optional<T> NextIntervalPoint(T start, IntervalNode const* node) {
    // If this node doesn't exist or is entirely out of scope, just return. This
    // prevents recursing deeper than necessary on the left.
    if (!node || node->Data().MaxHigh() < start) {
      return std::nullopt;
    }
    // Easy shortcut: If the lowest point in this subtree is in scope, just
    // return that. This prevents recursing deeper than necessary on the right.
    if (start < node->Data().MinLow()) {
      return node->Data().MinLow();
    }

    auto left_candidate = NextIntervalPoint(start, node->Left());

    // If the current node's low point isn't out of scope, we don't even need to
    // look at the right branch.
    if (start < node->Data().Low()) {
      if (left_candidate.has_value()) {
        return std::min(node->Data().Low(), left_candidate.value());
      } else {
        return node->Data().Low();
      }
    }

    // If the current node's high point is in scope, consider that against the
    // left branch
    std::optional<T> current_candidate;
    if (start < node->Data().High()) {
      if (left_candidate.has_value()) {
        current_candidate =
            std::min(node->Data().High(), left_candidate.value());
      } else {
        current_candidate = node->Data().High();
      }
    } else {
      current_candidate = left_candidate;
    }

    // If the current (and left) nodes fail, tail-recurse on the right node
    if (!current_candidate.has_value()) {
      return NextIntervalPoint(start, node->Right());
    }

    // Otherwise, pick the min between the |current_candidate| and the right
    // node
    auto right_candidate = NextIntervalPoint(start, node->Right());
    if (right_candidate.has_value()) {
      return std::min(current_candidate.value(), right_candidate.value());
    } else {
      return current_candidate;
    }
  }

  bool UpdateNode(IntervalNode* node) override {
    T cur_max(node->Data().High());
    T cur_min(node->Data().Low());

    IntervalNode* left = node->Left();
    if (left) {
      // Left node will always have a lower MinLow than the right node, so just
      // reassign immediately.
      cur_min = left->Data().MinLow();
      cur_max = std::max(cur_max, left->Data().MaxHigh());
    }

    IntervalNode* right = node->Right();
    if (right) {
      // Right node will always have greater min than current node or left
      // branch, so don't bother checking it.
      cur_max = std::max(cur_max, right->Data().MaxHigh());
    }

    bool updated = false;
    if (!(cur_min == node->Data().MinLow())) {
      node->Data().SetMinLow(cur_min);
      updated = true;
    }
    if (!(cur_max == node->Data().MaxHigh())) {
      node->Data().SetMaxHigh(cur_max);
      updated = true;
    }

    return updated;
  }

  static bool CheckInvariantsFromNode(IntervalNode const* node) {
    IntervalNode const* left = node->Left();
    IntervalNode const* right = node->Right();

    T observed_min_value(node->Data().Low());
    T observed_max_value(node->Data().High());

    if (left) {
      // Ensure left branch is entirely valid
      if (!CheckInvariantsFromNode(left)) {
        return false;
      }
      // Ensure that this node's MinLow is equal to MinLow of the left branch
      if (!(left->Data().MinLow() == node->Data().MinLow())) {
        LogVerificationFailedAtNode(node);
        return false;
      }
      // Ensure that this node's MaxHigh is at least MaxHigh of left branch
      if (node->Data().MaxHigh() < left->Data().MaxHigh()) {
        LogVerificationFailedAtNode(node);
        return false;
      }

      observed_min_value = left->Data().MinLow();
      observed_max_value = std::max(observed_max_value, left->Data().MaxHigh());
    }

    if (right) {
      // Ensure right branch is entirely valid
      if (!CheckInvariantsFromNode(right)) {
        return false;
      }
      // Ensure this node's MinLow is not greater than the right node's MinLow
      if (right->Data().MinLow() < node->Data().MinLow()) {
        LogVerificationFailedAtNode(node);
        return false;
      }
      // Ensure that this node's MaxHigh is at least MaxHigh of right branch
      if (node->Data().MaxHigh() < right->Data().MaxHigh()) {
        LogVerificationFailedAtNode(node);
        return false;
      }

      observed_max_value =
          std::max(observed_max_value, right->Data().MaxHigh());
    }

    // Ensure this node's MinLow is the min we actually observed
    if (!(observed_min_value == node->Data().MinLow())) {
      LogVerificationFailedAtNode(node);
      return false;
    }
    // Ensure that this node's MaxHigh is the max we actually observed
    if (!(observed_max_value == node->Data().MaxHigh())) {
      LogVerificationFailedAtNode(node);
      return false;
    }

    return true;
  }

#ifndef NDEBUG
  static void LogVerificationFailedAtNode(IntervalNode const* node) {
    DLOG(ERROR) << "PODIntervalTree verification failed at node " << node
                << ": data=" << node->Data().ToString();
  }
#else
  static void LogVerificationFailedAtNode(IntervalNode const*) {}
#endif
};

#ifndef NDEBUG
// Support for printing PODIntervals at the PODRedBlackTree level.
template <class T, class UserData>
struct ValueToString<PODInterval<T, UserData>> {
  static String ToString(const PODInterval<T, UserData>& interval) {
    return interval.ToString();
  }
};
#endif

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_INTERVAL_TREE_H_
