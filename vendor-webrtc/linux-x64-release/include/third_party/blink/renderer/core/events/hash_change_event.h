/*
 * Copyright (C) 2010 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_HASH_CHANGE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_HASH_CHANGE_EVENT_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_hash_change_event_init.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/event_interface_names.h"
#include "third_party/blink/renderer/core/event_type_names.h"

namespace blink {

class HashChangeEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static HashChangeEvent* Create() {
    return MakeGarbageCollected<HashChangeEvent>();
  }

  static HashChangeEvent* Create(const String& old_url, const String& new_url) {
    return MakeGarbageCollected<HashChangeEvent>(old_url, new_url);
  }

  static HashChangeEvent* Create(const AtomicString& type,
                                 const HashChangeEventInit* initializer) {
    return MakeGarbageCollected<HashChangeEvent>(type, initializer);
  }

  HashChangeEvent() = default;
  HashChangeEvent(const String& old_url, const String& new_url)
      : Event(event_type_names::kHashchange, Bubbles::kNo, Cancelable::kNo),
        old_url_(old_url),
        new_url_(new_url) {}
  HashChangeEvent(const AtomicString& type,
                  const HashChangeEventInit* initializer)
      : Event(type, initializer) {
    if (initializer->hasOldURL())
      old_url_ = initializer->oldURL();
    if (initializer->hasNewURL())
      new_url_ = initializer->newURL();
  }

  const String& oldURL() const { return old_url_; }
  const String& newURL() const { return new_url_; }

  const AtomicString& InterfaceName() const override {
    return event_interface_names::kHashChangeEvent;
  }

  void Trace(Visitor* visitor) const override { Event::Trace(visitor); }

 private:
  String old_url_;
  String new_url_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_HASH_CHANGE_EVENT_H_
