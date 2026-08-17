// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_BOUNDARY_EVENT_DISPATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_BOUNDARY_EVENT_DISPATCHER_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// This class contains the common logic of finding related node in a boundary
// crossing action and sending boundary events. The subclasses of this class
// must define what events should be sent in every case.
class BoundaryEventDispatcher {
  STACK_ALLOCATED();

 public:
  BoundaryEventDispatcher(AtomicString over_event,
                          AtomicString out_event,
                          AtomicString enter_event,
                          AtomicString leave_event)
      : over_event_(over_event),
        out_event_(out_event),
        enter_event_(enter_event),
        leave_event_(leave_event) {}
  virtual ~BoundaryEventDispatcher() = default;

  void SendBoundaryEvents(EventTarget* exited_target,
                          bool original_exited_target_removed,
                          EventTarget* entered_target);

 protected:
  virtual void Dispatch(EventTarget*,
                        EventTarget* related_target,
                        const AtomicString&,
                        bool check_for_listener) = 0;

 private:
  AtomicString over_event_;
  AtomicString out_event_;
  AtomicString enter_event_;
  AtomicString leave_event_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_BOUNDARY_EVENT_DISPATCHER_H_
