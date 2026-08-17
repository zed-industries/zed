/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_PATH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_PATH_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/node_event_context.h"
#include "third_party/blink/renderer/core/dom/events/tree_scope_event_context.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Event;
class EventTarget;
class Node;
class TouchEvent;
class TouchList;
class TreeScope;
class WindowEventContext;

class CORE_EXPORT EventPath final : public GarbageCollected<EventPath> {
 public:
  explicit EventPath(Node&, Event* = nullptr);
  EventPath(const EventPath&) = delete;
  EventPath& operator=(const EventPath&) = delete;

  void InitializeWith(Node&, Event*);

  const HeapVector<NodeEventContext>& NodeEventContexts() const {
    return node_event_contexts_;
  }
  HeapVector<NodeEventContext>& NodeEventContexts() {
    return node_event_contexts_;
  }
  NodeEventContext& operator[](wtf_size_t index) {
    return node_event_contexts_[index];
  }
  const NodeEventContext& operator[](wtf_size_t index) const {
    return node_event_contexts_[index];
  }
  NodeEventContext& Last() { return node_event_contexts_[size() - 1]; }

  WindowEventContext& GetWindowEventContext() {
    DCHECK(window_event_context_);
    return *window_event_context_;
  }
  void EnsureWindowEventContext();

  bool IsEmpty() const { return node_event_contexts_.empty(); }
  wtf_size_t size() const { return node_event_contexts_.size(); }

  void AdjustForRelatedTarget(Node&, EventTarget* related_target);
  void AdjustForTouchEvent(const TouchEvent&);
  // AdjustForDisabledFormControl will shrink this event path if there is a
  // disabled form control in it so that the disabled form control and its
  // parents are not included.
  void AdjustForDisabledFormControl();

  bool DisabledFormControlExistsInPath() const;
  bool HasEventListenersInPath(const AtomicString& event_type) const;

  NodeEventContext& TopNodeEventContext();

  static EventTarget& EventTargetRespectingTargetRules(Node&);

  void Trace(Visitor*) const;
  void Clear() {
    node_event_contexts_.clear();
    tree_scope_event_contexts_.clear();
  }

 private:
  EventPath() = delete;

  void Initialize();
  void CalculatePath();
  void CalculateAdjustedTargets();
  void CalculateTreeOrderAndSetNearestAncestorClosedTree();

  void Shrink(wtf_size_t new_size) {
    DCHECK(!window_event_context_);
    node_event_contexts_.Shrink(new_size);
  }

  void RetargetRelatedTarget(const Node& related_target_node);

  void ShrinkForRelatedTarget(const Node& event_target_node,
                              const Node& event_related_target_node);

  void AdjustTouchList(const TouchList*,
                       HeapVector<Member<TouchList>> adjusted_touch_list,
                       const HeapVector<Member<TreeScope>>& tree_scopes);

  TreeScopeEventContext* GetTreeScopeEventContext(TreeScope&);
  TreeScopeEventContext* EnsureTreeScopeEventContext(Node* current_target,
                                                     TreeScope*);

  using RelatedTargetMap = HeapHashMap<Member<TreeScope>, Member<EventTarget>>;

  static void BuildRelatedNodeMap(const Node&, RelatedTargetMap&);
  static EventTarget* FindRelatedNode(TreeScope&, RelatedTargetMap&);

#if DCHECK_IS_ON()
  static void CheckReachability(TreeScope&, TouchList&);
#endif

  HeapVector<NodeEventContext> node_event_contexts_;
  Member<Node> node_;
  Member<Event> event_;
  HeapVector<Member<TreeScopeEventContext>, 8> tree_scope_event_contexts_;
  Member<WindowEventContext> window_event_context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_PATH_H_
