/*
 * Copyright (C) 2009 Apple Inc. All Rights Reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE, INC. ``AS IS'' AND ANY
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_POP_STATE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_POP_STATE_EVENT_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_pop_state_event_init.h"
#include "third_party/blink/renderer/bindings/core/v8/world_safe_v8_reference.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExceptionState;
class History;
class SerializedScriptValue;

class CORE_EXPORT PopStateEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static PopStateEvent* Create();
  static PopStateEvent* Create(ScriptState* script_state,
                               const AtomicString& type,
                               const PopStateEventInit* initializer);
  static PopStateEvent* Create(
      scoped_refptr<SerializedScriptValue> serialized_state,
      History* history,
      bool has_ua_visual_transition);

  PopStateEvent() = default;
  PopStateEvent(ScriptState* script_state,
                const AtomicString& type,
                const PopStateEventInit* initializer);
  PopStateEvent(scoped_refptr<SerializedScriptValue> serialized_state,
                History* history,
                bool has_ua_visual_transition);
  ~PopStateEvent() override = default;

  ScriptValue state(ScriptState* script_state, ExceptionState& exception_state);
  bool hasUAVisualTransition() const { return has_ua_visual_transition_; }
  bool IsStateDirty() const { return false; }

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  WorldSafeV8Reference<v8::Value> state_;
  scoped_refptr<SerializedScriptValue> serialized_state_;
  Member<History> history_;
  const bool has_ua_visual_transition_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_POP_STATE_EVENT_H_
