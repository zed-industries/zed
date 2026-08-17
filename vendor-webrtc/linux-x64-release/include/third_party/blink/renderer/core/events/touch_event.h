/*
 * Copyright 2008, The Android Open Source Project
 * Copyright (C) 2012 Research In Motion Limited. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *  * Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS ``AS IS'' AND ANY
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_TOUCH_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_TOUCH_EVENT_H_

#include "third_party/blink/public/common/input/web_coalesced_input_event.h"
#include "third_party/blink/public/common/input/web_touch_event.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/events/ui_event_with_key_state.h"
#include "third_party/blink/renderer/core/input/touch_list.h"
#include "third_party/blink/renderer/platform/graphics/touch_action.h"

namespace blink {

class TouchEventInit;

class CORE_EXPORT TouchEvent final : public UIEventWithKeyState {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~TouchEvent() override;

  // We only initialize sourceCapabilities when we create TouchEvent from
  // EventHandler, null if it is from JavaScript.
  static TouchEvent* Create() { return MakeGarbageCollected<TouchEvent>(); }
  static TouchEvent* Create(const WebCoalescedInputEvent& event,
                            TouchList* touches,
                            TouchList* target_touches,
                            TouchList* changed_touches,
                            const AtomicString& type,
                            AbstractView* view,
                            TouchAction current_touch_action) {
    return MakeGarbageCollected<TouchEvent>(event, touches, target_touches,
                                            changed_touches, type, view,
                                            current_touch_action);
  }

  static TouchEvent* Create(const AtomicString& type,
                            const TouchEventInit* initializer) {
    return MakeGarbageCollected<TouchEvent>(type, initializer);
  }

  TouchEvent();
  TouchEvent(const WebCoalescedInputEvent&,
             TouchList* touches,
             TouchList* target_touches,
             TouchList* changed_touches,
             const AtomicString& type,
             AbstractView*,
             TouchAction current_touch_action);
  TouchEvent(const AtomicString&, const TouchEventInit*);

  TouchList* touches() const { return touches_.Get(); }
  TouchList* targetTouches() const { return target_touches_.Get(); }
  TouchList* changedTouches() const { return changed_touches_.Get(); }

  void SetTouches(TouchList* touches) { touches_ = touches; }
  void SetTargetTouches(TouchList* target_touches) {
    target_touches_ = target_touches;
  }
  void SetChangedTouches(TouchList* changed_touches) {
    changed_touches_ = changed_touches;
  }

  bool IsTouchEvent() const override;

  const AtomicString& InterfaceName() const override;

  void preventDefault() override;

  const WebCoalescedInputEvent* NativeEvent() const {
    return native_event_.get();
  }

  DispatchEventResult DispatchEvent(EventDispatcher&) override;

  void Trace(Visitor*) const override;

 private:
  bool IsTouchStartOrFirstTouchMove() const;

  Member<TouchList> touches_;
  Member<TouchList> target_touches_;
  Member<TouchList> changed_touches_;

  // The current effective touch action computed before each
  // touchstart event is generated. It is used for UMA histograms.
  TouchAction current_touch_action_;

  std::unique_ptr<WebCoalescedInputEvent> native_event_;
};

template <>
struct DowncastTraits<TouchEvent> {
  static bool AllowFrom(const Event& event) { return event.IsTouchEvent(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_TOUCH_EVENT_H_
