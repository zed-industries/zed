// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TRAVERSAL_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TRAVERSAL_RANGE_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Node;

template <class Iterator>
class TraversalRange {
  STACK_ALLOCATED();

 public:
  using StartNodeType = typename Iterator::StartNodeType;
  explicit TraversalRange(const StartNodeType* start) : start_(start) {}
  Iterator begin() { return Iterator(start_); }
  Iterator end() { return Iterator::End(); }

 private:
  const StartNodeType* start_;
};

template <class Traversal>
class TraversalIteratorBase {
  STACK_ALLOCATED();

 public:
  using NodeType = typename Traversal::TraversalNodeType;
  NodeType& operator*() { return *current_; }
  bool operator!=(const TraversalIteratorBase& rval) const {
    return current_ != rval.current_;
  }

 protected:
  explicit TraversalIteratorBase(NodeType* current) : current_(current) {}

  NodeType* current_;
};

template <class Traversal>
class TraversalIterator : public TraversalIteratorBase<Traversal> {
  STACK_ALLOCATED();

 public:
  using StartNodeType = typename Traversal::TraversalNodeType;
  using TraversalIteratorBase<Traversal>::current_;

  explicit TraversalIterator(const StartNodeType* start)
      : TraversalIteratorBase<Traversal>(const_cast<StartNodeType*>(start)) {}

  void operator++() { current_ = Traversal::Next(*current_); }

  static TraversalIterator End() { return TraversalIterator(); }

 private:
  TraversalIterator() : TraversalIteratorBase<Traversal>(nullptr) {}
};

template <class Traversal>
class TraversalDescendantIterator : public TraversalIteratorBase<Traversal> {
  STACK_ALLOCATED();

 public:
  using StartNodeType = Node;
  using TraversalIteratorBase<Traversal>::current_;

  explicit TraversalDescendantIterator(const StartNodeType* start)
      : TraversalIteratorBase<Traversal>(start ? Traversal::FirstWithin(*start)
                                               : nullptr),
        root_(start) {}

  void operator++() { current_ = Traversal::Next(*current_, root_); }
  static TraversalDescendantIterator End() {
    return TraversalDescendantIterator();
  }

 private:
  TraversalDescendantIterator() : TraversalIteratorBase<Traversal>(nullptr) {}
  const StartNodeType* root_ = nullptr;
};

template <class Traversal>
class TraversalInclusiveDescendantIterator
    : public TraversalIteratorBase<Traversal> {
  STACK_ALLOCATED();

 public:
  using StartNodeType = typename Traversal::TraversalNodeType;
  using TraversalIteratorBase<Traversal>::current_;

  explicit TraversalInclusiveDescendantIterator(const StartNodeType* start)
      : TraversalIteratorBase<Traversal>(const_cast<StartNodeType*>(start)),
        root_(start) {}
  void operator++() { current_ = Traversal::Next(*current_, root_); }
  static TraversalInclusiveDescendantIterator End() {
    return TraversalInclusiveDescendantIterator(nullptr);
  }

 private:
  const StartNodeType* root_;
};

template <class Traversal>
class TraversalParent {
 public:
  using TraversalNodeType = typename Traversal::TraversalNodeType;
  static TraversalNodeType* Next(const TraversalNodeType& node) {
    return Traversal::Parent(node);
  }
};

template <class Traversal>
class TraversalSibling {
 public:
  using TraversalNodeType = typename Traversal::TraversalNodeType;
  static TraversalNodeType* Next(const TraversalNodeType& node) {
    return Traversal::NextSibling(node);
  }
};

template <class T>
using TraversalNextRange = TraversalRange<TraversalIterator<T>>;

template <class T>
using TraversalAncestorRange =
    TraversalRange<TraversalIterator<TraversalParent<T>>>;

template <class T>
using TraversalSiblingRange =
    TraversalRange<TraversalIterator<TraversalSibling<T>>>;

template <class T>
using TraversalDescendantRange = TraversalRange<TraversalDescendantIterator<T>>;

template <class T>
using TraversalInclusiveDescendantRange =
    TraversalRange<TraversalInclusiveDescendantIterator<T>>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TRAVERSAL_RANGE_H_
