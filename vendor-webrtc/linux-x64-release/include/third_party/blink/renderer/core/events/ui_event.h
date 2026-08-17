/*
 * Copyright (C) 2001 Peter Kelly (pmk@post.com)
 * Copyright (C) 2001 Tobias Anton (anton@stud.fbi.fh-darmstadt.de)
 * Copyright (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
 * Copyright (C) 2003, 2004, 2005, 2006, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_UI_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_UI_EVENT_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_ui_event_init.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/frame/dom_window.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class InputDeviceCapabilities;

// FIXME: Get rid of this type alias.
using AbstractView = DOMWindow;

class CORE_EXPORT UIEvent : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static UIEvent* Create() { return MakeGarbageCollected<UIEvent>(); }
  static UIEvent* Create(const AtomicString& type,
                         const UIEventInit* initializer) {
    return MakeGarbageCollected<UIEvent>(type, initializer);
  }

  UIEvent();
  UIEvent(const AtomicString& type,
          Bubbles,
          Cancelable,
          ComposedMode,
          base::TimeTicks platform_time_stamp,
          AbstractView*,
          int detail,
          InputDeviceCapabilities* source_capabilities);
  UIEvent(const AtomicString&,
          const UIEventInit*,
          base::TimeTicks platform_time_stamp);
  UIEvent(const AtomicString& type, const UIEventInit* init)
      : UIEvent(type, init, base::TimeTicks::Now()) {}
  ~UIEvent() override;

  void initUIEvent(const AtomicString& type,
                   bool bubbles,
                   bool cancelable,
                   AbstractView*,
                   int detail);
  void InitUIEventInternal(const AtomicString& type,
                           bool bubbles,
                           bool cancelable,
                           EventTarget* related_target,
                           AbstractView*,
                           int detail,
                           InputDeviceCapabilities* source_capabilities);

  AbstractView* view() const { return view_.Get(); }
  int detail() const { return detail_; }
  InputDeviceCapabilities* sourceCapabilities() const {
    return source_capabilities_.Get();
  }

  const AtomicString& InterfaceName() const override;
  bool IsUIEvent() const final;

  virtual unsigned which() const;

  void Trace(Visitor*) const override;

 private:
  Member<AbstractView> view_;
  int detail_;
  Member<InputDeviceCapabilities> source_capabilities_;
};

template <>
struct DowncastTraits<UIEvent> {
  static bool AllowFrom(const Event& event) { return event.IsUIEvent(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_UI_EVENT_H_
