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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_UI_EVENT_WITH_KEY_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_UI_EVENT_WITH_KEY_STATE_H_

#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/events/ui_event.h"

namespace blink {

class EventModifierInit;

class CORE_EXPORT UIEventWithKeyState : public UIEvent {
 public:
  bool ctrlKey() const { return modifiers_ & WebInputEvent::kControlKey; }
  bool shiftKey() const { return modifiers_ & WebInputEvent::kShiftKey; }
  bool altKey() const { return modifiers_ & WebInputEvent::kAltKey; }
  bool metaKey() const { return modifiers_ & WebInputEvent::kMetaKey; }

  // We ignore the new tab modifiers (ctrl or meta, depending on OS) set by
  // JavaScript when processing events.  However, scripts running in isolated
  // worlds (aka content scripts) are not subject to this restriction. Since it
  // is possible that an event created by a content script is caught and
  // recreated by the web page's script, we resort to a global flag.
  static bool NewTabModifierSetFromIsolatedWorld() {
    return new_tab_modifier_set_from_isolated_world_;
  }
  static void ClearNewTabModifierSetFromIsolatedWorld() {
    new_tab_modifier_set_from_isolated_world_ = false;
  }
  static void DidCreateEventInIsolatedWorld(bool ctrl_key,
                                            bool shift_key,
                                            bool alt_key,
                                            bool meta_key);

  static void SetFromWebInputEventModifiers(EventModifierInit*,
                                            WebInputEvent::Modifiers);

  bool getModifierState(const String& key_identifier) const;

  WebInputEvent::Modifiers GetModifiers() const {
    return static_cast<WebInputEvent::Modifiers>(modifiers_);
  }

 protected:
  UIEventWithKeyState() : modifiers_(0) {}

  UIEventWithKeyState(const AtomicString& type,
                      Bubbles,
                      Cancelable,
                      AbstractView*,
                      int detail,
                      WebInputEvent::Modifiers,
                      base::TimeTicks platform_time_stamp,
                      InputDeviceCapabilities* source_capabilities = nullptr);
  UIEventWithKeyState(const AtomicString& type,
                      const EventModifierInit* initializer,
                      base::TimeTicks platform_time_stamp);
  UIEventWithKeyState(const AtomicString& type,
                      const EventModifierInit* initializer)
      : UIEventWithKeyState(type, initializer, base::TimeTicks::Now()) {}
  void InitModifiers(bool ctrl_key,
                     bool alt_key,
                     bool shift_key,
                     bool meta_key);

  unsigned modifiers_;

 private:
  static bool new_tab_modifier_set_from_isolated_world_;
};

const UIEventWithKeyState* FindEventWithKeyState(const Event*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_UI_EVENT_WITH_KEY_STATE_H_
