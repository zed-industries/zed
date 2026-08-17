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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_NODE_EVENT_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_NODE_EVENT_CONTEXT_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/tree_scope_event_context.h"

namespace blink {

class EventTarget;
class Node;
class TouchEventContext;

class CORE_EXPORT NodeEventContext {
  DISALLOW_NEW();

 public:
  // FIXME: Use ContainerNode instead of Node.
  NodeEventContext(Node&, EventTarget& current_target);
  void Trace(Visitor*) const;

  Node& GetNode() const { return *node_; }

  void SetTreeScopeEventContext(
      TreeScopeEventContext* tree_scope_event_context) {
    tree_scope_event_context_ = tree_scope_event_context;
  }
  TreeScopeEventContext& GetTreeScopeEventContext() {
    DCHECK(tree_scope_event_context_);
    return *tree_scope_event_context_;
  }

  EventTarget* Target() const { return tree_scope_event_context_->Target(); }
  EventTarget* RelatedTarget() const {
    return tree_scope_event_context_->RelatedTarget();
  }
  TouchEventContext* GetTouchEventContext() const {
    return tree_scope_event_context_->GetTouchEventContext();
  }

  bool CurrentTargetSameAsTarget() const {
    return current_target_.Get() == Target();
  }
  void HandleLocalEvents(Event&) const;

 private:
  Member<Node> node_;
  Member<EventTarget> current_target_;
  Member<TreeScopeEventContext> tree_scope_event_context_;
};

}  // namespace blink

namespace WTF {

template <>
struct VectorTraits<blink::NodeEventContext>
    : SimpleClassVectorTraits<blink::NodeEventContext> {
  static constexpr bool kCanTraceConcurrently = true;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_NODE_EVENT_CONTEXT_H_
