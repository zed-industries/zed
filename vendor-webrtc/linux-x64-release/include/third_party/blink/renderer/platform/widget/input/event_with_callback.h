// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_EVENT_WITH_CALLBACK_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_EVENT_WITH_CALLBACK_H_

#include <list>

#include "third_party/blink/public/common/input/web_coalesced_input_event.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/widget/input/input_handler_proxy.h"
#include "ui/latency/latency_info.h"

namespace cc {
class EventMetrics;
}

namespace blink {

namespace test {
class InputHandlerProxyEventQueueTest;
}

class PLATFORM_EXPORT EventWithCallback {
 public:
  struct PLATFORM_EXPORT OriginalEventWithCallback {
    OriginalEventWithCallback(
        std::unique_ptr<WebCoalescedInputEvent> event,
        std::unique_ptr<cc::EventMetrics> metrics,
        InputHandlerProxy::EventDispositionCallback callback);
    ~OriginalEventWithCallback();

    std::unique_ptr<WebCoalescedInputEvent> event_;
    std::unique_ptr<cc::EventMetrics> metrics_;
    InputHandlerProxy::EventDispositionCallback callback_;
  };
  using OriginalEventList = std::list<OriginalEventWithCallback>;

  EventWithCallback(std::unique_ptr<WebCoalescedInputEvent> event,
                    InputHandlerProxy::EventDispositionCallback callback,
                    std::unique_ptr<cc::EventMetrics> metrics);
  EventWithCallback(std::unique_ptr<WebCoalescedInputEvent> event,
                    OriginalEventList original_events);
  ~EventWithCallback();

  [[nodiscard]] bool CanCoalesceWith(const EventWithCallback& other) const;
  void CoalesceWith(EventWithCallback* other);

  void RunCallbacks(InputHandlerProxy::EventDisposition,
                    const ui::LatencyInfo& latency,
                    std::unique_ptr<InputHandlerProxy::DidOverscrollParams>,
                    const WebInputEventAttribution&);

  const WebInputEvent& event() const { return event_->Event(); }
  WebInputEvent* event_pointer() { return event_->EventPointer(); }
  const ui::LatencyInfo& latency_info() const { return event_->latency_info(); }
  ui::LatencyInfo& latency_info() { return event_->latency_info(); }
  void set_coalesced_scroll_and_pinch() { coalesced_scroll_and_pinch_ = true; }
  bool coalesced_scroll_and_pinch() const {
    return coalesced_scroll_and_pinch_;
  }
  size_t coalesced_count() const { return original_events_.size(); }
  OriginalEventList& original_events() { return original_events_; }
  // |first_original_event()| is used as ID for tracing.
  WebInputEvent* first_original_event() {
    return original_events_.empty()
               ? nullptr
               : original_events_.front().event_->EventPointer();
  }
  void SetScrollbarManipulationHandledOnCompositorThread();

  cc::EventMetrics* metrics() const {
    return original_events_.empty() ? nullptr
                                    : original_events_.front().metrics_.get();
  }

  // Removes metrics objects from all original events and returns the first one
  // for latency reporting purposes.
  std::unique_ptr<cc::EventMetrics> TakeMetrics();

  // Called when the compositor thread starts/finishes processing the event so
  // that the metrics can be updated with the appropriate timestamp. These are
  // only called if the event has metrics.
  void WillStartProcessingForMetrics();
  void DidCompleteProcessingForMetrics();

 private:
  friend class test::InputHandlerProxyEventQueueTest;

  std::unique_ptr<WebCoalescedInputEvent> event_;
  OriginalEventList original_events_;
  bool coalesced_scroll_and_pinch_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_EVENT_WITH_CALLBACK_H_
