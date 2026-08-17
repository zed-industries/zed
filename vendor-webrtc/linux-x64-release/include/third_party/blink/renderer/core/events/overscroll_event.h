// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_OVERSCROLL_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_OVERSCROLL_EVENT_H_

#include "third_party/blink/renderer/core/dom/events/event.h"

namespace blink {

class OverscrollEventInit;

class OverscrollEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static OverscrollEvent* Create(const AtomicString& type,
                                 bool bubbles,
                                 double delta_x,
                                 double delta_y) {
    return MakeGarbageCollected<OverscrollEvent>(type, bubbles, delta_x,
                                                 delta_y);
  }
  static OverscrollEvent* Create(const AtomicString& type,
                                 bool bubbles,
                                 const OverscrollEventInit* initializer) {
    return MakeGarbageCollected<OverscrollEvent>(type, bubbles, initializer);
  }

  OverscrollEvent(const AtomicString&,
                  bool bubbles,
                  double delta_x,
                  double delta_y);
  OverscrollEvent(const AtomicString&,
                  bool bubbles,
                  const OverscrollEventInit*);

  double deltaX() const { return delta_x_; }
  double deltaY() const { return delta_y_; }

  void Trace(Visitor*) const override;

 private:
  double delta_x_ = 0;
  double delta_y_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_OVERSCROLL_EVENT_H_
