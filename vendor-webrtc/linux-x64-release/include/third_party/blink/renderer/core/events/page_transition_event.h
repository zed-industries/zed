/*
 * Copyright (C) 2009 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_PAGE_TRANSITION_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_PAGE_TRANSITION_EVENT_H_

#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/event_type_names.h"

namespace blink {

class PageTransitionEventInit;

class PageTransitionEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static PageTransitionEvent* Create() {
    return MakeGarbageCollected<PageTransitionEvent>();
  }
  static PageTransitionEvent* Create(const AtomicString& type, bool persisted) {
    // Persisted pageshow events must be created through CreatePersistedPageshow
    // (because it needs |navigation_start|).
    DCHECK(!(persisted && type == event_type_names::kPageshow));
    return MakeGarbageCollected<PageTransitionEvent>(type, persisted);
  }
  static PageTransitionEvent* CreatePersistedPageshow(
      base::TimeTicks navigation_start) {
    return MakeGarbageCollected<PageTransitionEvent>(navigation_start);
  }

  static PageTransitionEvent* Create(
      const AtomicString& type,
      const PageTransitionEventInit* initializer) {
    return MakeGarbageCollected<PageTransitionEvent>(type, initializer);
  }

  PageTransitionEvent();
  PageTransitionEvent(const AtomicString& type, bool persisted);
  PageTransitionEvent(const AtomicString&, const PageTransitionEventInit*);
  explicit PageTransitionEvent(base::TimeTicks navigation_start);
  ~PageTransitionEvent() override;

  const AtomicString& InterfaceName() const override;

  bool persisted() const { return persisted_; }

  void Trace(Visitor*) const override;

 private:
  // TODO(rakina): change to PageTransitionEventPersistence.
  bool persisted_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_PAGE_TRANSITION_EVENT_H_
