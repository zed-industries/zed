// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_SIMULATED_EVENT_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_SIMULATED_EVENT_UTIL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/dom/events/simulated_click_options.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class CORE_EXPORT SimulatedEventUtil {
 public:
  SimulatedEventUtil() = delete;

  static Event* CreateEvent(const AtomicString& event_type,
                            Node& node,
                            const Event* underlying_event,
                            SimulatedClickCreationScope creation_scope);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_SIMULATED_EVENT_UTIL_H_
