// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_COMPOSITOR_THREAD_EVENT_QUEUE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_COMPOSITOR_THREAD_EVENT_QUEUE_H_

#include <memory>

#include "base/containers/circular_deque.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/widget/input/event_with_callback.h"

namespace blink {

namespace test {
class InputHandlerProxyEventQueueTest;
}

// CompositorThreadEventQueue is a coalescing queue. It will examine
// the current events in the queue and will attempt to coalesce with
// the last event.
class PLATFORM_EXPORT CompositorThreadEventQueue {
 public:
  CompositorThreadEventQueue();
  CompositorThreadEventQueue(const CompositorThreadEventQueue&) = delete;
  CompositorThreadEventQueue& operator=(const CompositorThreadEventQueue&) =
      delete;
  ~CompositorThreadEventQueue();

  // Adds an event to the queue. The event may be coalesced with the last event.
  void Queue(std::unique_ptr<EventWithCallback> event);

  std::unique_ptr<EventWithCallback> Pop();

  WebInputEvent::Type PeekType() const;

  bool empty() const { return queue_.empty(); }

  size_t size() const { return queue_.size(); }

 private:
  friend class test::InputHandlerProxyEventQueueTest;
  using EventQueue = base::circular_deque<std::unique_ptr<EventWithCallback>>;
  EventQueue queue_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_COMPOSITOR_THREAD_EVENT_QUEUE_H_
