/*
 * Copyright (C) 2006, 2008, 2009 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_LISTENER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_LISTENER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Event;
class ExecutionContext;

// EventListener represents 'callback' in 'event listener' in DOM standard.
// https://dom.spec.whatwg.org/#concept-event-listener
//
// While RegisteredEventListener represents 'event listener', which consists of
//   - type
//   - callback
//   - capture
//   - passive
//   - once
//   - removed
// EventListener represents 'callback' part.
class CORE_EXPORT EventListener : public GarbageCollected<EventListener>,
                                  public NameClient {
 public:
  EventListener(const EventListener&) = delete;
  EventListener& operator=(const EventListener&) = delete;
  ~EventListener() override = default;

  // Invokes this event listener.
  virtual void Invoke(ExecutionContext*, Event*) = 0;

  // Returns true if this implements IDL EventHandler family.
  virtual bool IsEventHandler() const { return false; }

  // Returns true if this implements IDL EventHandler family and the value is
  // a content attribute (or compiled from a content attribute).
  virtual bool IsEventHandlerForContentAttribute() const { return false; }

  // Returns an uncompiled script body.
  // https://html.spec.whatwg.org/C/webappapis.html#internal-raw-uncompiled-handler
  virtual const String& ScriptBody() const { return g_empty_string; }

  // Returns true if this event listener was created in the current world.
  virtual bool BelongsToTheCurrentWorld(ExecutionContext*) const {
    return false;
  }

  // Returns true if this event listener is considered as the same with the
  // other event listener (in context of EventTarget.removeEventListener).
  // See also |RegisteredEventListener::Matches|.
  //
  // This function must satisfy the symmetric property; a.Matches(b) must
  // produce the same result as b.Matches(a).
  virtual bool Matches(const EventListener&) const = 0;

  virtual void Trace(Visitor*) const {}

  const char* NameInHeapSnapshot() const override { return "EventListener"; }

  // Helper functions for DowncastTraits.
  virtual bool IsJSBasedEventListener() const { return false; }
  virtual bool IsNativeEventListener() const { return false; }

 private:
  EventListener() = default;

  // Only these two classes are direct subclasses of EventListener.  Other
  // subclasses must inherit from either of them.
  friend class JSBasedEventListener;
  friend class NativeEventListener;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_LISTENER_H_
