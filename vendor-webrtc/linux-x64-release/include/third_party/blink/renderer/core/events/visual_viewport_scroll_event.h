// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_VISUAL_VIEWPORT_SCROLL_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_VISUAL_VIEWPORT_SCROLL_EVENT_H_

#include "third_party/blink/renderer/core/dom/events/event.h"

namespace blink {

class VisualViewportScrollEvent final : public Event {
 public:
  VisualViewportScrollEvent();
  ~VisualViewportScrollEvent() override;

  void DoneDispatchingEventAtCurrentTarget() override;

  void Trace(Visitor* visitor) const override { Event::Trace(visitor); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_VISUAL_VIEWPORT_SCROLL_EVENT_H_
