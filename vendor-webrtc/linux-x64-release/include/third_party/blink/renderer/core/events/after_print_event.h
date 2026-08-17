// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_AFTER_PRINT_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_AFTER_PRINT_EVENT_H_

#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/event_type_names.h"

namespace blink {

class AfterPrintEvent final : public Event {
 public:
  AfterPrintEvent()
      : Event(event_type_names::kAfterprint, Bubbles::kNo, Cancelable::kNo) {}
  ~AfterPrintEvent() override = default;

  // beforeprint/afterprint events need to be dispatched while the execution
  // context is paused.  When printing, window.print() invoked by beforeprint/
  // afterprint event listeners should have no effect, hence the event dispatch
  // needs to be done during the pause.
  bool ShouldDispatchEvenWhenExecutionContextIsPaused() const override {
    return true;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_AFTER_PRINT_EVENT_H_
