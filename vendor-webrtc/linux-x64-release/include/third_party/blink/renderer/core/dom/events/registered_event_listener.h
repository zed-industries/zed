/*
 * Copyright (C) 2001 Peter Kelly (pmk@post.com)
 * Copyright (C) 2001 Tobias Anton (anton@stud.fbi.fh-darmstadt.de)
 * Copyright (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
 * Copyright (C) 2003, 2004, 2005, 2006, 2008, 2009 Apple Inc. All rights
 * reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_REGISTERED_EVENT_LISTENER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_REGISTERED_EVENT_LISTENER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class AddEventListenerOptionsResolved;
class Event;
class EventListener;
class EventListenerOptions;

// RegisteredEventListener represents 'event listener' defined in the DOM
// standard. https://dom.spec.whatwg.org/#concept-event-listener
class RegisteredEventListener final
    : public GarbageCollected<RegisteredEventListener> {
 public:
  RegisteredEventListener();
  RegisteredEventListener(EventListener* listener,
                          const AddEventListenerOptionsResolved* options);
  RegisteredEventListener(const RegisteredEventListener& that) = delete;
  RegisteredEventListener& operator=(const RegisteredEventListener& that) =
      delete;

  void Trace(Visitor* visitor) const;

  AddEventListenerOptionsResolved* Options() const;

  const EventListener* Callback() const { return callback_.Get(); }

  EventListener* Callback() { return callback_.Get(); }

  void SetCallback(EventListener* listener);

  bool Passive() const { return passive_; }

  bool Once() const { return once_; }

  bool Capture() const { return use_capture_; }

  bool BlockedEventWarningEmitted() const {
    return blocked_event_warning_emitted_;
  }

  bool PassiveForcedForDocumentTarget() const {
    return passive_forced_for_document_target_;
  }

  bool PassiveSpecified() const { return passive_specified_; }

  void SetBlockedEventWarningEmitted() {
    blocked_event_warning_emitted_ = true;
  }

  bool Matches(const EventListener* listener,
               const EventListenerOptions* options) const;

  bool ShouldFire(const Event&) const;

  bool Removed() const { return removed_; }

  void SetRemoved() { removed_ = true; }

 private:
  Member<EventListener> callback_;
  unsigned use_capture_ : 1;
  unsigned passive_ : 1;
  unsigned once_ : 1;
  unsigned blocked_event_warning_emitted_ : 1;
  unsigned passive_forced_for_document_target_ : 1;
  unsigned passive_specified_ : 1;
  unsigned removed_ : 1;
};

bool operator==(const RegisteredEventListener&, const RegisteredEventListener&);

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::RegisteredEventListener)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_REGISTERED_EVENT_LISTENER_H_
