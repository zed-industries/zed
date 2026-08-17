// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_EVENT_HANDLING_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_EVENT_HANDLING_UTIL_H_

#include "third_party/blink/public/platform/web_input_event_result.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/layout/hit_test_result.h"
#include "third_party/blink/renderer/core/page/event_with_hit_test_results.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"

namespace blink {

class ContainerNode;
class EventTarget;
class LocalFrame;
enum class DispatchEventResult;

namespace event_handling_util {

CORE_EXPORT HitTestResult HitTestResultInFrame(
    LocalFrame*,
    const HitTestLocation&,
    HitTestRequest::HitTestRequestType hit_type = HitTestRequest::kReadOnly |
                                                  HitTestRequest::kActive);

WebInputEventResult MergeEventResult(WebInputEventResult result_a,
                                     WebInputEventResult result_b);
WebInputEventResult ToWebInputEventResult(DispatchEventResult);

bool IsInDocument(EventTarget*);

ContainerNode* ParentForClickEvent(const Node&);

CORE_EXPORT PhysicalOffset
ContentPointFromRootFrame(LocalFrame*, const gfx::PointF& point_in_root_frame);

MouseEventWithHitTestResults PerformMouseEventHitTest(LocalFrame*,
                                                      const HitTestRequest&,
                                                      const WebMouseEvent&);

LocalFrame* GetTargetSubframe(const MouseEventWithHitTestResults&,
                              bool* is_remote_frame = nullptr);

LocalFrame* SubframeForTargetNode(Node*, bool* is_remote_frame = nullptr);

// Intervention: if an input event lands on a cross-origin iframe or fencedframe
// that has moved or resized recently (recent==500ms), and which contains an
// IntersectionObserver that is tracking visibility, then the event is quietly
// discarded.
bool ShouldDiscardEventTargetingFrame(const WebInputEvent& event,
                                      const LocalFrame& frame);

// If a "down" event was discarded by the above intervention, and the next down
// event arrives within `DiscardedEventMistakeInterval` with the same target as
// the discarded event, we conclude that the first event was intentional and
// should not have been discarded.
constexpr base::TimeDelta kDiscardedEventMistakeInterval = base::Seconds(5);

class PointerEventTarget {
  DISALLOW_NEW();

 public:
  void Trace(Visitor* visitor) const;

  Member<Element> target_element;
  Member<LocalFrame> target_frame;
  Member<Scrollbar> scrollbar;
};

}  // namespace event_handling_util

}  // namespace blink

WTF_ALLOW_INIT_WITH_MEM_FUNCTIONS(
    blink::event_handling_util::PointerEventTarget)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_EVENT_HANDLING_UTIL_H_
