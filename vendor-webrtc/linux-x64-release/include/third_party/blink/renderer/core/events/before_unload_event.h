/*
 * Copyright (C) 2001 Peter Kelly (pmk@post.com)
 * Copyright (C) 2001 Tobias Anton (anton@stud.fbi.fh-darmstadt.de)
 * Copyright (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
 * Copyright (C) 2003, 2004, 2005, 2006 Apple Computer, Inc.
 * Copyright (C) 2013 Samsung Electronics.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_BEFORE_UNLOAD_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_BEFORE_UNLOAD_EVENT_H_

#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/event_interface_names.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class BeforeUnloadEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static BeforeUnloadEvent* Create() {
    return MakeGarbageCollected<BeforeUnloadEvent>();
  }

  BeforeUnloadEvent();
  ~BeforeUnloadEvent() override;

  bool IsBeforeUnloadEvent() const override;

  void setReturnValue(const String& return_value) {
    return_value_ = return_value;
  }
  String returnValue() const { return return_value_; }

  const AtomicString& InterfaceName() const override {
    return event_interface_names::kBeforeUnloadEvent;
  }

  // A confirmation dialog for leaving a page is expected to be shown
  // regardless of the state of the page.  So, beforeunload's event
  // listeners should always run regardless of pause.
  bool ShouldDispatchEvenWhenExecutionContextIsPaused() const override {
    return true;
  }

  void Trace(Visitor*) const override;

 private:
  String return_value_;
};

template <>
struct DowncastTraits<BeforeUnloadEvent> {
  static bool AllowFrom(const Event& event) {
    return event.IsBeforeUnloadEvent();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_BEFORE_UNLOAD_EVENT_H_
