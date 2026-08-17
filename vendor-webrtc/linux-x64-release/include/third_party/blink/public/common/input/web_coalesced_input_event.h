// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_COALESCED_INPUT_EVENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_COALESCED_INPUT_EVENT_H_

#include <memory>
#include <vector>

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/common/input/web_pointer_event.h"
#include "ui/latency/latency_info.h"

namespace blink {

// This class represents a polymorphic WebInputEvent structure with its
// coalesced events. The event could be any event defined in web_input_event.h,
// including those that cannot be coalesced.
class BLINK_COMMON_EXPORT WebCoalescedInputEvent {
 public:
  WebCoalescedInputEvent(const WebInputEvent&, const ui::LatencyInfo&);
  WebCoalescedInputEvent(std::unique_ptr<WebInputEvent>,
                         const ui::LatencyInfo&);
  WebCoalescedInputEvent(std::unique_ptr<WebInputEvent>,
                         std::vector<std::unique_ptr<WebInputEvent>>,
                         std::vector<std::unique_ptr<WebInputEvent>>,
                         const ui::LatencyInfo&);
  // Copy constructor to deep copy the event.
  WebCoalescedInputEvent(const WebCoalescedInputEvent&);
  ~WebCoalescedInputEvent();

  WebInputEvent* EventPointer();
  void AddCoalescedEvent(const blink::WebInputEvent&);
  const WebInputEvent& Event() const;
  size_t CoalescedEventSize() const;
  const WebInputEvent& CoalescedEvent(size_t index) const;
  const std::vector<std::unique_ptr<WebInputEvent>>&
  GetCoalescedEventsPointers() const;

  void AddPredictedEvent(const blink::WebInputEvent&);
  size_t PredictedEventSize() const;
  const WebInputEvent& PredictedEvent(size_t index) const;
  const std::vector<std::unique_ptr<WebInputEvent>>&
  GetPredictedEventsPointers() const;

  const ui::LatencyInfo& latency_info() const { return latency_; }
  ui::LatencyInfo& latency_info() { return latency_; }

  [[nodiscard]] bool CanCoalesceWith(const WebCoalescedInputEvent& other) const;

  // Coalesce with the |newer_event|. This object will be updated to take into
  // account |newer_event|'s fields.
  void CoalesceWith(const WebCoalescedInputEvent& newer_event);

 private:
  std::unique_ptr<WebInputEvent> event_;
  std::vector<std::unique_ptr<WebInputEvent>> coalesced_events_;
  std::vector<std::unique_ptr<WebInputEvent>> predicted_events_;
  ui::LatencyInfo latency_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_COALESCED_INPUT_EVENT_H_
