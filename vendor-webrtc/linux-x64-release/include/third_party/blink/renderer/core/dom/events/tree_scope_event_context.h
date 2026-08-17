/*
 * Copyright (C) 2014 Google Inc. All Rights Reserved.
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_TREE_SCOPE_EVENT_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_TREE_SCOPE_EVENT_CONTEXT_H_

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/tree_scope.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ContainerNode;
class EventPath;
class EventTarget;
template <typename NodeType>
class StaticNodeTypeList;
using StaticNodeList = StaticNodeTypeList<Node>;
class TouchEventContext;
class TreeScope;

class CORE_EXPORT TreeScopeEventContext final
    : public GarbageCollected<TreeScopeEventContext> {
 public:
  explicit TreeScopeEventContext(TreeScope&);
  void Trace(Visitor*) const;

  TreeScope& GetTreeScope() const { return *tree_scope_; }
  ContainerNode& RootNode() const { return tree_scope_->RootNode(); }

  EventTarget* Target() const { return target_.Get(); }
  void SetTarget(EventTarget&);

  EventTarget* RelatedTarget() const { return related_target_.Get(); }
  void SetRelatedTarget(EventTarget&);

  TouchEventContext* GetTouchEventContext() const {
    return touch_event_context_.Get();
  }
  TouchEventContext& EnsureTouchEventContext();

  GCedHeapVector<Member<EventTarget>>& EnsureEventPath(EventPath&);

  bool IsInclusiveAncestorOf(const TreeScopeEventContext&) const;
  bool IsDescendantOf(const TreeScopeEventContext&) const;
#if DCHECK_IS_ON()
  bool IsExclusivePartOf(const TreeScopeEventContext&) const;
#endif
  void AddChild(TreeScopeEventContext& child) { children_.push_back(&child); }

  // For ancestor-descendant relationship check in O(1).
  // Preprocessing takes O(N).
  int CalculateTreeOrderAndSetNearestAncestorClosedTree(
      int order_number,
      TreeScopeEventContext* nearest_ancestor_closed_tree_scope_event_context);

  TreeScopeEventContext* ContainingClosedShadowTree() const {
    return containing_closed_shadow_tree_.Get();
  }

 private:
  void CheckReachableNode(EventTarget&);

  bool IsUnclosedTreeOf(const TreeScopeEventContext& other);

  Member<TreeScope> tree_scope_;
  Member<EventTarget> target_;
  Member<EventTarget> related_target_;
  Member<GCedHeapVector<Member<EventTarget>>> event_path_;
  Member<TouchEventContext> touch_event_context_;
  Member<TreeScopeEventContext> containing_closed_shadow_tree_;

  HeapVector<Member<TreeScopeEventContext>> children_;
  int pre_order_;
  int post_order_;
};

#if DCHECK_IS_ON()
inline void TreeScopeEventContext::CheckReachableNode(EventTarget& target) {
  if (!target.ToNode())
    return;
  // FIXME: Checks also for SVG elements.
  if (target.ToNode()->IsSVGElement())
    return;
  DCHECK(target.ToNode()->GetTreeScope().IsInclusiveAncestorTreeScopeOf(
      GetTreeScope()));
}
#else
inline void TreeScopeEventContext::CheckReachableNode(EventTarget&) {}
#endif

inline void TreeScopeEventContext::SetTarget(EventTarget& target) {
  CheckReachableNode(target);
  target_ = target;
}

inline void TreeScopeEventContext::SetRelatedTarget(
    EventTarget& related_target) {
  CheckReachableNode(related_target);
  related_target_ = related_target;
}

inline bool TreeScopeEventContext::IsInclusiveAncestorOf(
    const TreeScopeEventContext& other) const {
  DCHECK_NE(pre_order_, -1);
  DCHECK_NE(post_order_, -1);
  DCHECK_NE(other.pre_order_, -1);
  DCHECK_NE(other.post_order_, -1);
  return pre_order_ <= other.pre_order_ && other.post_order_ <= post_order_;
}

inline bool TreeScopeEventContext::IsDescendantOf(
    const TreeScopeEventContext& other) const {
  DCHECK_NE(pre_order_, -1);
  DCHECK_NE(post_order_, -1);
  DCHECK_NE(other.pre_order_, -1);
  DCHECK_NE(other.post_order_, -1);
  return other.pre_order_ < pre_order_ && post_order_ < other.post_order_;
}

#if DCHECK_IS_ON()
inline bool TreeScopeEventContext::IsExclusivePartOf(
    const TreeScopeEventContext& other) const {
  DCHECK_NE(pre_order_, -1);
  DCHECK_NE(post_order_, -1);
  DCHECK_NE(other.pre_order_, -1);
  DCHECK_NE(other.post_order_, -1);
  return (pre_order_ < other.pre_order_ && post_order_ < other.pre_order_) ||
         (pre_order_ > other.pre_order_ && pre_order_ > other.post_order_);
}
#endif

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_TREE_SCOPE_EVENT_CONTEXT_H_
