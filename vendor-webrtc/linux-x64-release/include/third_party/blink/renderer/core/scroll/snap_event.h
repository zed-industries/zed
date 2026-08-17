// Copyright 2023 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SNAP_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SNAP_EVENT_H_

#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/static_node_list.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// This class implements the SnapEvent interface for scroll-snap-related
// JavaScript events, scrollsnapchange and scrollsnapchanging.
// SnapEvents are sent to a scroller when it snaps to a different element from
// the element to which it was previously snapped along either axis.
// https://drafts.csswg.org/css-scroll-snap-2/#scrollsnapchange-and-scrollsnapchanging
class SnapEvent : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static SnapEvent* Create(const AtomicString& type,
                           Bubbles bubbles,
                           Member<Node>& block_target,
                           Member<Node>& inline_target);
  SnapEvent(const AtomicString& type,
            Bubbles bubbles,
            Member<Node>& block_target,
            Member<Node>& inline_target);

  Node* snapTargetBlock() { return snap_target_block_.Get(); }
  Node* snapTargetInline() { return snap_target_inline_.Get(); }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(snap_target_block_);
    visitor->Trace(snap_target_inline_);
    Event::Trace(visitor);
  }

 private:
  // This are the elements that the scrolling container selected as snap targets
  // for the related snap event.
  Member<Node> snap_target_block_;
  Member<Node> snap_target_inline_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SNAP_EVENT_H_
