/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2012 Apple Inc. All rights reserved.
 * Copyright (C) 2006 Alexey Proskuryakov (ap@webkit.org)
 *           (C) 2007, 2008 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2011 Andreas Kling (kling@webkit.org)
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_LISTENER_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_LISTENER_MAP_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/registered_event_listener.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class EventListenerOptions;
class EventTarget;

using EventListenerVector = GCedHeapVector<Member<RegisteredEventListener>, 1>;
using EventListenerVectorSnapshot =
    HeapVector<Member<RegisteredEventListener>, 1>;

class CORE_EXPORT EventListenerMap final {
  DISALLOW_NEW();

 public:
  EventListenerMap();
  EventListenerMap(const EventListenerMap&) = delete;
  EventListenerMap& operator=(const EventListenerMap&) = delete;

  void Clear();
  bool IsEmpty() const { return entries_.empty(); }
  bool Contains(const AtomicString& event_type) const;
  bool ContainsCapturing(const AtomicString& event_type) const;
  bool ContainsJSBasedEventListeners(const AtomicString& event_type) const;

  // Add an event listener. If the listener was indeed added (duplicates will
  // return false) `registered_listener` will be updated to the
  // `RegisteredEventListener` stored in the map.
  bool Add(const AtomicString& event_type,
           EventListener*,
           const AddEventListenerOptionsResolved*,
           RegisteredEventListener** registered_listener);
  // Remove an event listener. If the listener is found the result will be
  // true and `registered_listener` will be updated to the
  // `RegisteredEventListener` that was removed from the map.
  bool Remove(const AtomicString& event_type,
              const EventListener*,
              const EventListenerOptions*,
              RegisteredEventListener** registered_listener);
  EventListenerVector* Find(const AtomicString& event_type);
  Vector<AtomicString> EventTypes() const;

  template <typename CallbackType>
  void ForAllEventListenerTypes(CallbackType callback) const {
    for (const auto& entry : entries_) {
      callback(entry.first, entry.second->size());
    }
  }

  void CopyEventListenersNotCreatedFromMarkupToTarget(EventTarget*);

  void Trace(Visitor*) const;

 private:
  friend class EventListenerIterator;

  // We use HeapVector instead of HeapHashMap because
  //  - HeapVector is much more space efficient than HeapHashMap.
  //  - An EventTarget rarely has event listeners for many event types, and
  //    HeapVector is faster in such cases.
  HeapVector<std::pair<AtomicString, Member<EventListenerVector>>, 2> entries_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_LISTENER_MAP_H_
