// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_PENDING_USER_INPUT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_PENDING_USER_INPUT_H_

#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/common/input/web_input_event_attribution.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/attribution_group.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
namespace scheduler {

class PLATFORM_EXPORT PendingUserInput {
  DISALLOW_NEW();

 public:
  // Handles the dispatch of WebInputEvents from the main thread scheduler,
  // keeping track of the set of in-flight input events.
  class PLATFORM_EXPORT Monitor {
    DISALLOW_NEW();

   public:
    Monitor() = default;
    Monitor(const Monitor&) = delete;
    Monitor& operator=(const Monitor&) = delete;

    void OnEnqueue(WebInputEvent::Type, const WebInputEventAttribution&);
    void OnDequeue(WebInputEvent::Type, const WebInputEventAttribution&);

    // Returns a list of all unique attributions that are marked for event
    // dispatch. If |include_continuous| is true, include event types from
    // "continuous" sources (see PendingUserInput::IsContinuousEventTypes).
    Vector<WebInputEventAttribution> Info(bool include_continuous) const;

   private:
    struct EventCounter {
      EventCounter() = default;

      size_t num_discrete = 0;
      size_t num_continuous = 0;
    };

    // A mapping between attributions to pending events.
    HashMap<AttributionGroup, EventCounter> pending_events_;
  };

  PendingUserInput() = delete;
  PendingUserInput(const PendingUserInput&) = delete;
  PendingUserInput& operator=(const PendingUserInput&) = delete;

  // Returns true if the given blink event type is considered to be sampled
  // from a continuous source.
  // https://wicg.github.io/is-input-pending/#continuousevents
  static bool IsContinuousEventType(WebInputEvent::Type);
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_PENDING_USER_INPUT_H_
