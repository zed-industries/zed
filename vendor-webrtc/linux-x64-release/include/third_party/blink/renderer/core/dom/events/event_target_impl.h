// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_TARGET_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_TARGET_IMPL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ScriptState;

// Constructible version of EventTarget. Calls to EventTarget
// constructor in JavaScript will return an instance of this class.
// We don't use EventTarget directly because EventTarget is an abstract
// class and and making it non-abstract is unfavorable  because it will
// increase the size of EventTarget and all of its subclasses with code
// that are mostly unnecessary for them, resulting in a performance
// decrease.
class CORE_EXPORT EventTargetImpl final : public EventTarget,
                                          public ExecutionContextClient {
 public:
  explicit EventTargetImpl(ScriptState*);
  ~EventTargetImpl() override = default;

  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;
  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_TARGET_IMPL_H_
