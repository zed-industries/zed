// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_EVENT_MODULES_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_EVENT_MODULES_FACTORY_H_

#include <memory>
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/events/event_factory.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class Event;

class EventModulesFactory final : public EventFactoryBase {
 public:
  static std::unique_ptr<EventModulesFactory> Create() {
    return std::make_unique<EventModulesFactory>();
  }

  Event* Create(ScriptState*,
                ExecutionContext*,
                const String& event_type) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_EVENT_MODULES_FACTORY_H_
