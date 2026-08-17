/*
 * Copyright (C) 2004, 2006, 2008 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_POSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_POSITION_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/editing_strategy.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Node;
class TreeScope;

enum class PositionAnchorType : unsigned {
  kOffsetInAnchor,
  kBeforeAnchor,
  kAfterAnchor,
  kAfterChildren,
};

// Instances of |PositionTemplate<Strategy>| are immutable.
// TODO(editing-dev): Make constructor of |PositionTemplate| take |const Node*|.
template <typename Strategy>
class PositionTemplate {
  DISALLOW_NEW();

 public:
  PositionTemplate();

  static const TreeScope* CommonAncestorTreeScope(
      const PositionTemplate<Strategy>&,
      const PositionTemplate<Strategy>& b);
  static PositionTemplate<Strategy> EditingPositionOf(const Node* anchor_node,
                                                      int offset);

  // For creating before/after positions:
  PositionTemplate(const Node* anchor_node, PositionAnchorType);

  // For creating offset positions:
  PositionTemplate(const Node& anchor_node, int offset);
  // TODO(editing-dev): We should not pass |nullptr| as |anchor_node| for
  // |Position| constructor.
  // TODO(editing-dev): This constructor should eventually go away. See bug
  // http://wkb.ug/63040.
  PositionTemplate(const Node* anchor_node, int offset);

  PositionTemplate(const PositionTemplate&);
  PositionTemplate& operator=(const PositionTemplate&);

  // Returns a newly created |Position| with |kOffsetInAnchor|. |offset| can be
  // out of bound. Out of bound position is used for computing undo/redo
  // selection for merging text typing.
  static PositionTemplate<Strategy> CreateWithoutValidation(
      const Node& container,
      int offset);

  // TODO(editing-dev): Once we get a reason to use out of bound position,
  // we should change caller to use |CreateWithoutValidation()|.
  static PositionTemplate<Strategy> CreateWithoutValidationDeprecated(
      const Node& container,
      int offset);

  explicit operator bool() const { return IsNotNull(); }

  PositionAnchorType AnchorType() const { return anchor_type_; }
  bool IsAfterAnchor() const {
    return anchor_type_ == PositionAnchorType::kAfterAnchor;
  }
  bool IsAfterChildren() const {
    return anchor_type_ == PositionAnchorType::kAfterChildren;
  }
  bool IsBeforeAnchor() const {
    return anchor_type_ == PositionAnchorType::kBeforeAnchor;
  }
  bool IsBeforeChildren() const;
  bool IsOffsetInAnchor() const {
    return anchor_type_ == PositionAnchorType::kOffsetInAnchor;
  }

  // These are always DOM compliant values.  Editing positions like [img, 0]
  // (aka [img, before]) will return img->parentNode() and img->nodeIndex() from
  // these functions.

  // null for a before/after position anchored to a node with no parent
  Node* ComputeContainerNode() const;

  // O(n) for before/after-anchored positions, O(1) for parent-anchored
  // positions
  int ComputeOffsetInContainerNode() const;

  // Convenience method for DOM positions that also fixes up some positions for
  // editing
  PositionTemplate<Strategy> ParentAnchoredEquivalent() const;

  // Returns |PositionIsAnchor| type |Position| which is compatible with
  // |RangeBoundaryPoint| as safe to pass |Range| constructor. Return value
  // of this function is different from |parentAnchoredEquivalent()| which
  // returns editing specific position.
  PositionTemplate<Strategy> ToOffsetInAnchor() const;

  // Inline O(1) access for Positions which callers know to be parent-anchored
  int OffsetInContainerNode() const {
    DCHECK(IsOffsetInAnchor());
    return offset_;
  }

  // Returns an offset for editing based on anchor type for using with
  // |AnchorNode()| function:
  //   - kOffsetInAnchor  offset_
  //   - kBeforeAnchor    0
  //   - kAfterChildren   last editing offset in anchor node
  //   - kAfterAnchor     last editing offset in anchor node
  // Editing operations will change in anchor node rather than nodes around
  // anchor node.
  int ComputeEditingOffset() const;

  // These are convenience methods which are smart about whether the position is
  // neighbor anchored or parent anchored
  Node* ComputeNodeBeforePosition() const;
  Node* ComputeNodeAfterPosition() const;

  // Returns node as |Range::firstNode()|. This position must be a
  // |PositionAnchorType::OffsetInAhcor| to behave as |Range| boundary point.
  Node* NodeAsRangeFirstNode() const;

  // Similar to |nodeAsRangeLastNode()|, but returns a node in a range.
  Node* NodeAsRangeLastNode() const;

  // Returns a node as past last as same as |Range::pastLastNode()|. This
  // function is supposed to used in HTML serialization and plain text
  // iterator. This position must be a |PositionAnchorType::OffsetInAhcor| to
  // behave as |Range| boundary point.
  Node* NodeAsRangePastLastNode() const;

  Node* CommonAncestorContainer(const PositionTemplate<Strategy>&) const;

  Node* AnchorNode() const { return anchor_node_.Get(); }

  Document* GetDocument() const {
    return anchor_node_ ? &anchor_node_->GetDocument() : nullptr;
  }

  // For PositionInFlatTree, it requires an ancestor traversal to compute the
  // value of IsConnected(), which can be expensive.
  // TODO(crbug.com/761173): Rename to |ComputeIsConnected()| to indicate the
  // cost.
  bool IsConnected() const;

  bool IsValidFor(const Document&) const;

  bool IsNull() const { return !anchor_node_; }
  bool IsNotNull() const { return anchor_node_ != nullptr; }
  bool IsOrphan() const { return anchor_node_ && !IsConnected(); }

  // Note: Comparison of positions require both parameters are non-null. You
  // should check null-position before comparing them.
  // TODO(yosin): We should use |Position::operator<()| instead of
  // |Position::compareTo()| to utilize |DCHECK_XX()|.
  int16_t CompareTo(const PositionTemplate<Strategy>&) const;
  bool operator<(const PositionTemplate<Strategy>&) const;
  bool operator<=(const PositionTemplate<Strategy>&) const;
  bool operator>(const PositionTemplate<Strategy>&) const;
  bool operator>=(const PositionTemplate<Strategy>&) const;

  bool IsEquivalent(const PositionTemplate<Strategy>&) const;

  // These can be either inside or just before/after the node, depending on
  // if the node is ignored by editing or not.
  // FIXME: These should go away. They only make sense for legacy positions.
  bool AtFirstEditingPositionForNode() const;
  bool AtLastEditingPositionForNode() const;

  bool AtStartOfTree() const;

  static PositionTemplate<Strategy> BeforeNode(const Node& anchor_node);
  static PositionTemplate<Strategy> AfterNode(const Node& anchor_node);
  static PositionTemplate<Strategy> InParentBeforeNode(const Node& anchor_node);
  static PositionTemplate<Strategy> InParentAfterNode(const Node& anchor_node);
  static int LastOffsetInNode(const Node& anchor_node);
  static PositionTemplate<Strategy> FirstPositionInNode(
      const Node& anchor_node);
  static PositionTemplate<Strategy> LastPositionInNode(const Node& anchor_node);
  static PositionTemplate<Strategy> FirstPositionInOrBeforeNode(
      const Node& anchor_node);
  static PositionTemplate<Strategy> LastPositionInOrAfterNode(
      const Node& anchor_node);

  String ToAnchorTypeAndOffsetString() const;
#if DCHECK_IS_ON()
  void ShowTreeForThis() const;
  void ShowTreeForThisInFlatTree() const;
#endif

  void Trace(Visitor*) const;

 private:
  bool IsAfterAnchorOrAfterChildren() const {
    return IsAfterAnchor() || IsAfterChildren();
  }

  // TODO(editing-dev): Since we should consider |Position| is constant in
  // tree, we should use |Member<const Node>|. see http://crbug.com/735327
  Member<Node> anchor_node_;
  // offset_ can be the offset inside anchor_node_, or if
  // EditingIgnoresContent(anchor_node_) returns true, then other places in
  // editing will treat offset_ == 0 as "before the anchor" and offset_ > 0 as
  // "after the anchor node".  See ParentAnchoredEquivalent for more info.
  int offset_ = 0;
  PositionAnchorType anchor_type_ = PositionAnchorType::kOffsetInAnchor;
};

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    PositionTemplate<EditingStrategy>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    PositionTemplate<EditingInFlatTreeStrategy>;

using Position = PositionTemplate<EditingStrategy>;
using PositionInFlatTree = PositionTemplate<EditingInFlatTreeStrategy>;

template <typename Strategy>
bool operator==(const PositionTemplate<Strategy>& a,
                const PositionTemplate<Strategy>& b) {
  if (a.IsNull())
    return b.IsNull();

  if (a.AnchorNode() != b.AnchorNode() || a.AnchorType() != b.AnchorType())
    return false;

  if (!a.IsOffsetInAnchor()) {
    // Note: |offset_| only has meaning when
    // |PositionAnchorType::OffsetInAnchor|.
    return true;
  }

  // FIXME: In <div><img></div> [div, 0] != [img, 0] even though most of the
  // editing code will treat them as identical.
  return a.OffsetInContainerNode() == b.OffsetInContainerNode();
}

template <typename Strategy>
bool operator!=(const PositionTemplate<Strategy>& a,
                const PositionTemplate<Strategy>& b) {
  return !(a == b);
}

CORE_EXPORT PositionInFlatTree ToPositionInFlatTree(const Position&);
CORE_EXPORT PositionInFlatTree ToPositionInFlatTree(const PositionInFlatTree&);
CORE_EXPORT Position ToPositionInDOMTree(const Position&);
CORE_EXPORT Position ToPositionInDOMTree(const PositionInFlatTree&);

template <typename Strategy>
PositionTemplate<Strategy> FromPositionInDOMTree(const Position&);

template <>
inline Position FromPositionInDOMTree<EditingStrategy>(
    const Position& position) {
  return position;
}

template <>
inline PositionInFlatTree FromPositionInDOMTree<EditingInFlatTreeStrategy>(
    const Position& position) {
  return ToPositionInFlatTree(position);
}

CORE_EXPORT std::ostream& operator<<(std::ostream&, PositionAnchorType);
CORE_EXPORT std::ostream& operator<<(std::ostream&, const Position&);
CORE_EXPORT std::ostream& operator<<(std::ostream&, const PositionInFlatTree&);

}  // namespace blink

#if DCHECK_IS_ON()
// Outside the blink namespace for ease of invocation from gdb.
void ShowTree(const blink::Position&);
void ShowTree(const blink::Position*);
#endif

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_POSITION_H_
