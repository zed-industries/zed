/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_RANGE_BOUNDARY_POINT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_RANGE_BOUNDARY_POINT_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/dom/character_data.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/node_traversal.h"
#include "third_party/blink/renderer/core/editing/position.h"

namespace blink {

class RangeBoundaryPoint {
  DISALLOW_NEW();

 public:
  explicit RangeBoundaryPoint(Node& container);

  RangeBoundaryPoint(const RangeBoundaryPoint&);
  RangeBoundaryPoint& operator=(const RangeBoundaryPoint&);

  bool IsConnected() const;
  const Position ToPosition() const;

  Node& Container() const;
  unsigned Offset() const;
  Node* ChildBefore() const;

  void Set(Node& container, unsigned offset, Node* child_before);
  void SetOffset(unsigned);

  void SetToBeforeChild(Node&);
  void SetToStartOfNode(Node&);
  void SetToEndOfNode(Node&);

  void ChildBeforeWillBeRemoved();
  void InvalidateOffset();
  void MarkValid() const;

  void Trace(Visitor* visitor) const {
    visitor->Trace(container_node_);
    visitor->Trace(child_before_boundary_);
  }

 private:
  uint64_t DomTreeVersion() const;
  void EnsureOffsetIsValid() const;
  bool IsOffsetValid() const;

  static const unsigned kInvalidOffset = static_cast<unsigned>(-1);

  Member<Node> container_node_;
  Member<Node> child_before_boundary_;
  mutable uint64_t dom_tree_version_;
  mutable unsigned offset_in_container_;
};

inline RangeBoundaryPoint::RangeBoundaryPoint(Node& container)
    : container_node_(container),
      child_before_boundary_(nullptr),
      dom_tree_version_(DomTreeVersion()),
      offset_in_container_(0) {}

inline RangeBoundaryPoint::RangeBoundaryPoint(const RangeBoundaryPoint&) =
    default;

inline RangeBoundaryPoint& RangeBoundaryPoint::operator=(
    const RangeBoundaryPoint& other) = default;

inline Node& RangeBoundaryPoint::Container() const {
  return *container_node_;
}

inline Node* RangeBoundaryPoint::ChildBefore() const {
  return child_before_boundary_.Get();
}

inline uint64_t RangeBoundaryPoint::DomTreeVersion() const {
  return container_node_->GetDocument().DomTreeVersion();
}

inline void RangeBoundaryPoint::EnsureOffsetIsValid() const {
  if (IsOffsetValid())
    return;
  DCHECK(!container_node_->IsCharacterDataNode());
  MarkValid();
  if (!child_before_boundary_) {
    offset_in_container_ = 0;
    return;
  }
  offset_in_container_ = child_before_boundary_->NodeIndex() + 1;
}

inline bool RangeBoundaryPoint::IsConnected() const {
  return container_node_ && container_node_->isConnected();
}

inline bool RangeBoundaryPoint::IsOffsetValid() const {
  if (offset_in_container_ == kInvalidOffset) {
    DCHECK(!container_node_->IsTextNode());
    return false;
  }
  return DomTreeVersion() == dom_tree_version_ ||
         container_node_->IsCharacterDataNode();
}

inline const Position RangeBoundaryPoint::ToPosition() const {
  EnsureOffsetIsValid();
  // TODO(yosin): We should return |Position::BeforeAnchor| when
  // |container_node_| isn't a |Text| node.
  return Position(container_node_.Get(), offset_in_container_);
}

inline unsigned RangeBoundaryPoint::Offset() const {
  EnsureOffsetIsValid();
  return offset_in_container_;
}

inline void RangeBoundaryPoint::Set(Node& container,
                                    unsigned offset,
                                    Node* child_before) {
  DCHECK_GE(offset, 0u);
  DCHECK_EQ(child_before,
            offset ? NodeTraversal::ChildAt(container, offset - 1) : nullptr);
  container_node_ = container;
  offset_in_container_ = offset;
  child_before_boundary_ = child_before;
  MarkValid();
}

inline void RangeBoundaryPoint::SetOffset(unsigned offset) {
  DCHECK(container_node_);
  DCHECK(container_node_->IsCharacterDataNode());
  DCHECK_GE(offset_in_container_, 0u);
  DCHECK(!child_before_boundary_);
  offset_in_container_ = offset;
  MarkValid();
}

inline void RangeBoundaryPoint::SetToBeforeChild(Node& child) {
  DCHECK(child.parentNode());
  child_before_boundary_ = child.previousSibling();
  container_node_ = child.parentNode();
  offset_in_container_ = child_before_boundary_ ? kInvalidOffset : 0;
  MarkValid();
}

inline void RangeBoundaryPoint::SetToStartOfNode(Node& container) {
  container_node_ = &container;
  offset_in_container_ = 0;
  child_before_boundary_ = nullptr;
  MarkValid();
}

inline void RangeBoundaryPoint::SetToEndOfNode(Node& container) {
  container_node_ = &container;
  if (auto* character_data = DynamicTo<CharacterData>(container_node_.Get())) {
    offset_in_container_ = character_data->length();
    child_before_boundary_ = nullptr;
  } else {
    child_before_boundary_ = container_node_->lastChild();
    offset_in_container_ = child_before_boundary_ ? kInvalidOffset : 0;
  }
  MarkValid();
}

inline void RangeBoundaryPoint::ChildBeforeWillBeRemoved() {
  child_before_boundary_ = child_before_boundary_->previousSibling();
  if (!IsOffsetValid())
    return;
  DCHECK_GT(offset_in_container_, 0u);
  if (!child_before_boundary_)
    offset_in_container_ = 0;
  else if (offset_in_container_ > 0)
    --offset_in_container_;
  MarkValid();
}

inline void RangeBoundaryPoint::InvalidateOffset() {
  offset_in_container_ = kInvalidOffset;
}

inline void RangeBoundaryPoint::MarkValid() const {
  dom_tree_version_ = DomTreeVersion();
}

inline bool operator==(const RangeBoundaryPoint& a,
                       const RangeBoundaryPoint& b) {
  if (a.Container() != b.Container())
    return false;
  if (a.ChildBefore() || b.ChildBefore()) {
    if (a.ChildBefore() != b.ChildBefore())
      return false;
  } else {
    if (a.Offset() != b.Offset())
      return false;
  }
  return true;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_RANGE_BOUNDARY_POINT_H_
