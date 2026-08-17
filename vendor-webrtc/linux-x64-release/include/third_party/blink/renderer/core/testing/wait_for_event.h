// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_WAIT_FOR_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_WAIT_FOR_EVENT_H_

#include "base/run_loop.h"
#include "third_party/blink/renderer/core/dom/events/native_event_listener.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class Event;
class EventTarget;

// Helper class that will block running the test until the given event is fired
// on the given element.
class WaitForEvent : public NativeEventListener {
 public:
  // Use this when you want to manually configure first.
  WaitForEvent();

  // Convenient shorthand for waiting for a single event on a single target,
  // immediately.
  WaitForEvent(EventTarget*, const AtomicString&);

  void AddEventListener(EventTarget*, const AtomicString& name);
  void AddCompletionClosure(base::OnceClosure);
  Event* GetLastEvent() const { return event_.Get(); }

  void Invoke(ExecutionContext*, Event*) final;
  void Trace(Visitor*) const final;

 private:
  base::RunLoop run_loop_;
  HeapVector<std::pair<Member<EventTarget>, AtomicString>> listeners_;
  Vector<base::OnceClosure> closures_;
  Member<Event> event_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_WAIT_FOR_EVENT_H_
