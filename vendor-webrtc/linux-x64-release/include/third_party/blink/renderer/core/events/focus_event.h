/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS''
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 * THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_FOCUS_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_FOCUS_EVENT_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/events/ui_event.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class FocusEventInit;

class FocusEvent final : public UIEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static FocusEvent* Create() { return MakeGarbageCollected<FocusEvent>(); }

  static FocusEvent* Create(const AtomicString& type,
                            Bubbles bubbles,
                            AbstractView* view,
                            int detail,
                            EventTarget* related_target,
                            InputDeviceCapabilities* source_capabilities) {
    return MakeGarbageCollected<FocusEvent>(
        type, bubbles, view, detail, related_target, source_capabilities);
  }

  static FocusEvent* Create(const AtomicString& type,
                            const FocusEventInit* initializer) {
    return MakeGarbageCollected<FocusEvent>(type, initializer);
  }

  FocusEvent();
  FocusEvent(const AtomicString& type,
             Bubbles,
             AbstractView*,
             int,
             EventTarget*,
             InputDeviceCapabilities*);
  FocusEvent(const AtomicString& type, const FocusEventInit*);

  EventTarget* relatedTarget() const { return related_target_.Get(); }
  void SetRelatedTarget(EventTarget* related_target) {
    related_target_ = related_target;
  }

  const AtomicString& InterfaceName() const override;
  bool IsFocusEvent() const override;

  DispatchEventResult DispatchEvent(EventDispatcher&) override;

  void Trace(Visitor*) const override;

 private:
  Member<EventTarget> related_target_;
};

template <>
struct DowncastTraits<FocusEvent> {
  static bool AllowFrom(const Event& event) { return event.IsFocusEvent(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_FOCUS_EVENT_H_
