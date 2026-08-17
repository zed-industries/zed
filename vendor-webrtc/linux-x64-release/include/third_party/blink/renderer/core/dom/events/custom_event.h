/*
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies)
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_CUSTOM_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_CUSTOM_EVENT_H_

#include "third_party/blink/renderer/bindings/core/v8/world_safe_v8_reference.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event.h"

namespace blink {

class CustomEventInit;

class CORE_EXPORT CustomEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~CustomEvent() override;

  static CustomEvent* Create() { return MakeGarbageCollected<CustomEvent>(); }

  static CustomEvent* Create(ScriptState* script_state,
                             const AtomicString& type,
                             const CustomEventInit* initializer) {
    return MakeGarbageCollected<CustomEvent>(script_state, type, initializer);
  }

  CustomEvent();
  CustomEvent(ScriptState*,
              const AtomicString& type,
              const CustomEventInit* initializer);

  void initCustomEvent(ScriptState*,
                       const AtomicString& type,
                       bool bubbles,
                       bool cancelable,
                       const ScriptValue& detail);

  const AtomicString& InterfaceName() const override;

  ScriptValue detail(ScriptState*) const;

  void Trace(Visitor*) const override;

 private:
  WorldSafeV8Reference<v8::Value> detail_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_CUSTOM_EVENT_H_
