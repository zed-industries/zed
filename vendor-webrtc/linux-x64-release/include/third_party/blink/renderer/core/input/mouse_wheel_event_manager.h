// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_MOUSE_WHEEL_EVENT_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_MOUSE_WHEEL_EVENT_MANAGER_H_

#include "third_party/blink/public/platform/web_input_event_result.h"
#include "third_party/blink/renderer/core/input/scroll_manager.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class Document;
class LocalFrame;
class LocalFrameView;
class Node;
class WebMouseWheelEvent;

class MouseWheelEventManager final
    : public GarbageCollected<MouseWheelEventManager> {
 public:
  explicit MouseWheelEventManager(LocalFrame&, ScrollManager&);
  MouseWheelEventManager(const MouseWheelEventManager&) = delete;
  MouseWheelEventManager& operator=(const MouseWheelEventManager&) = delete;
  void Trace(Visitor*) const;

  void Clear();

  WebInputEventResult HandleWheelEvent(const WebMouseWheelEvent&);

  void ElementRemoved(Node* target);

 private:
  Node* FindTargetNode(const WebMouseWheelEvent&,
                       const Document*,
                       const LocalFrameView*);

  const Member<LocalFrame> frame_;
  Member<Node> wheel_target_;
  Member<ScrollManager> scroll_manager_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_MOUSE_WHEEL_EVENT_MANAGER_H_
