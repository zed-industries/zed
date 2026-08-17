/*
 * Copyright (C) 2010 Google Inc. All Rights Reserved.
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_WINDOW_EVENT_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_WINDOW_EVENT_CONTEXT_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class EventTarget;
class Event;
class LocalDOMWindow;
class NodeEventContext;

class WindowEventContext : public GarbageCollected<WindowEventContext> {
 public:
  WindowEventContext(Event&, const NodeEventContext& top_node_event_context);
  WindowEventContext(const WindowEventContext&) = delete;
  WindowEventContext& operator=(const WindowEventContext&) = delete;

  LocalDOMWindow* Window() const;
  EventTarget* Target() const;
  EventTarget* RelatedTarget() const;
  bool HandleLocalEvents(Event&);

  void Trace(Visitor*) const;

 private:
  Member<LocalDOMWindow> window_;
  Member<EventTarget> target_;
  Member<EventTarget> related_target_;
};

inline LocalDOMWindow* WindowEventContext::Window() const {
  return window_.Get();
}

inline EventTarget* WindowEventContext::Target() const {
  return target_.Get();
}

inline EventTarget* WindowEventContext::RelatedTarget() const {
  return related_target_.Get();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_WINDOW_EVENT_CONTEXT_H_
