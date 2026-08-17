/*
 * Copyright (C) 2007, 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2013 Intel Corporation. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_TRANSITION_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_TRANSITION_EVENT_H_

#include "third_party/blink/renderer/core/animation/animation_time_delta.h"
#include "third_party/blink/renderer/core/dom/events/event.h"

namespace blink {

class TransitionEventInit;

class TransitionEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static TransitionEvent* Create() {
    return MakeGarbageCollected<TransitionEvent>();
  }
  static TransitionEvent* Create(const AtomicString& type,
                                 const String& property_name,
                                 const AnimationTimeDelta& elapsed_time,
                                 const String& pseudo_element) {
    return MakeGarbageCollected<TransitionEvent>(type, property_name,
                                                 elapsed_time, pseudo_element);
  }
  static TransitionEvent* Create(const AtomicString& type,
                                 const TransitionEventInit* initializer) {
    return MakeGarbageCollected<TransitionEvent>(type, initializer);
  }

  TransitionEvent();
  TransitionEvent(const AtomicString& type,
                  const String& property_name,
                  const AnimationTimeDelta& elapsed_time,
                  const String& pseudo_element);
  TransitionEvent(const AtomicString& type,
                  const TransitionEventInit* initializer);
  ~TransitionEvent() override;

  const String& propertyName() const;
  double elapsedTime() const;
  const String& pseudoElement() const;

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  String property_name_;
  AnimationTimeDelta elapsed_time_;
  String pseudo_element_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_TRANSITION_EVENT_H_
