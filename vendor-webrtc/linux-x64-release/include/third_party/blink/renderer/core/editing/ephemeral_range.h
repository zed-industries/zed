// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EPHEMERAL_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EPHEMERAL_RANGE_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/position.h"

namespace blink {

class Document;
class AbstractRange;
class Range;

// We should restrict access to the unwanted version of |TraversalRange::end()|
// function.
template <class Iterator>
class TraversalRangeNodes : private TraversalRange<Iterator> {
  STACK_ALLOCATED();

 public:
  using StartNodeType = typename TraversalRange<Iterator>::StartNodeType;
  TraversalRangeNodes(const StartNodeType* start,
                      const StartNodeType* past_end_node)
      : TraversalRange<Iterator>(start), past_end_node_(past_end_node) {}

  using TraversalRange<Iterator>::begin;

  Iterator end() { return Iterator(past_end_node_); }

 private:
  const StartNodeType* const past_end_node_;
};

// This class acts like |TraversalNextIterator| but in addition
// it allows to set current position and checks |current_| pointer before
// dereferencing.
template <class TraversalNext>
class CheckedTraversalNextIterator
    : public TraversalIteratorBase<TraversalNext> {
  STACK_ALLOCATED();

  using TraversalIteratorBase<TraversalNext>::current_;

 public:
  using StartNodeType = typename TraversalNext::TraversalNodeType;
  explicit CheckedTraversalNextIterator(const StartNodeType* start)
      : TraversalIteratorBase<TraversalNext>(
            const_cast<StartNodeType*>(start)) {}

  void operator++() {
    DCHECK(current_);
    current_ = TraversalNext::Next(*current_);
  }
};

// Unlike |Range| objects, |EphemeralRangeTemplate| objects aren't relocated.
// You should not use |EphemeralRangeTemplate| objects after DOM modification.
//
// EphemeralRangeTemplate is supposed to use returning or passing start and end
// position.
//
//  Example usage:
//    Range* range = produceRange();
//    consumeRange(range);
//    ... no DOM modification ...
//    consumeRange2(range);
//
//  Above code should be:
//    EphemeralRangeTemplate range = produceRange();
//    consumeRange(range);
//    ... no DOM modification ...
//    consumeRange2(range);
//
//  Because of |Range| objects consume heap memory and inserted into |Range|
//  object list in |Document| for relocation. These operations are redundant
//  if |Range| objects doesn't live after DOM mutation.
//
template <typename Strategy>
class EphemeralRangeTemplate final {
  STACK_ALLOCATED();

 public:
  using RangeTraversal =
      TraversalRangeNodes<CheckedTraversalNextIterator<Strategy>>;

  EphemeralRangeTemplate(const PositionTemplate<Strategy>& start,
                         const PositionTemplate<Strategy>& end);
  EphemeralRangeTemplate(const EphemeralRangeTemplate& other);
  // |position| should be |Position::isNull()| or in-document.
  explicit EphemeralRangeTemplate(
      const PositionTemplate<Strategy>& /* position */);
  explicit EphemeralRangeTemplate(const AbstractRange*);
  // When |range| is nullptr, |EphemeralRangeTemplate| is |isNull()|.
  explicit EphemeralRangeTemplate(const Range* /* range */);
  EphemeralRangeTemplate();
  ~EphemeralRangeTemplate();

  EphemeralRangeTemplate<Strategy>& operator=(
      const EphemeralRangeTemplate<Strategy>& other);

  bool operator==(const EphemeralRangeTemplate<Strategy>& other) const;
  bool operator!=(const EphemeralRangeTemplate<Strategy>& other) const;

  Document& GetDocument() const;
  PositionTemplate<Strategy> StartPosition() const;
  PositionTemplate<Strategy> EndPosition() const;

  Node* CommonAncestorContainer() const;

  // Returns true if |start_position_| == |end_position_| or |isNull()|.
  bool IsCollapsed() const;
  bool IsNull() const {
    DCHECK(IsValid());
    return start_position_.IsNull();
  }
  bool IsNotNull() const { return !IsNull(); }

  RangeTraversal Nodes() const;

  // |node| should be in-document and valid for anchor node of
  // |PositionTemplate<Strategy>|.
  static EphemeralRangeTemplate<Strategy> RangeOfContents(
      const Node& /* node */);

#if DCHECK_IS_ON()
  void ShowTreeForThis() const;
#endif

 private:
  bool IsValid() const;

  PositionTemplate<Strategy> start_position_;
  PositionTemplate<Strategy> end_position_;
#if DCHECK_IS_ON()
  uint64_t dom_tree_version_;
#endif
};

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    EphemeralRangeTemplate<EditingStrategy>;
using EphemeralRange = EphemeralRangeTemplate<EditingStrategy>;

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    EphemeralRangeTemplate<EditingInFlatTreeStrategy>;
using EphemeralRangeInFlatTree =
    EphemeralRangeTemplate<EditingInFlatTreeStrategy>;

// Returns a newly created |Range| object from |range| or |nullptr| if
// |range.isNull()| returns true.
CORE_EXPORT Range* CreateRange(const EphemeralRange& /* range */);

CORE_EXPORT std::ostream& operator<<(std::ostream&, const EphemeralRange&);
CORE_EXPORT std::ostream& operator<<(std::ostream&,
                                     const EphemeralRangeInFlatTree&);

CORE_EXPORT EphemeralRangeInFlatTree
ToEphemeralRangeInFlatTree(const EphemeralRange&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EPHEMERAL_RANGE_H_
