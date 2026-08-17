// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_MOUSE_EVENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_MOUSE_EVENT_H_

#include "base/check_op.h"
#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/common/input/web_menu_source_type.h"
#include "third_party/blink/public/common/input/web_pointer_properties.h"

namespace blink {

class WebGestureEvent;

// WebMouseEvent --------------------------------------------------------------

class BLINK_COMMON_EXPORT WebMouseEvent : public WebInputEvent,
                                          public WebPointerProperties {
 public:
  static constexpr PointerId kMousePointerId = std::numeric_limits<int>::max();

  int click_count = {};

  // Only used for contextmenu events.
  WebMenuSourceType menu_source_type = kMenuSourceNone;

  WebMouseEvent(Type type_param,
                gfx::PointF position,
                gfx::PointF global_position,
                Button button_param,
                int click_count_param,
                int modifiers_param,
                base::TimeTicks time_stamp_param,
                WebMenuSourceType menu_source_type_param = kMenuSourceNone,
                PointerId id_param = kMousePointerId)
      : WebInputEvent(type_param, modifiers_param, time_stamp_param),
        WebPointerProperties(id_param,
                             PointerType::kMouse,
                             button_param,
                             position,
                             global_position),
        click_count(click_count_param),
        menu_source_type(menu_source_type_param) {
    DCHECK_GE(type_param, Type::kMouseTypeFirst);
    DCHECK_LE(type_param, Type::kMouseTypeLast);
  }

  WebMouseEvent(Type type_param,
                int modifiers_param,
                base::TimeTicks time_stamp_param,
                PointerId id_param = kMousePointerId)
      : WebInputEvent(type_param, modifiers_param, time_stamp_param),
        WebPointerProperties(id_param, PointerType::kMouse) {}

  WebMouseEvent() : WebMouseEvent(kMousePointerId) {}

  bool FromTouch() const {
    return (GetModifiers() & kIsCompatibilityEventForTouch) != 0;
  }

  int ClickCount() const { return click_count; }

  WebMenuSourceType GetMenuSourceType() const { return menu_source_type; }

  WebMouseEvent(Type type_param,
                const WebGestureEvent&,
                Button button_param,
                int click_count_param,
                int modifiers_param,
                base::TimeTicks time_stamp_param,
                PointerId id_param = kMousePointerId);

  std::unique_ptr<WebInputEvent> Clone() const override;
  bool CanCoalesce(const WebInputEvent& event) const override;
  void Coalesce(const WebInputEvent& event) override;

  gfx::PointF PositionInRootFrame() const;

  // Sets any scaled values to be their computed values and sets |frame_scale_|
  // back to 1 and |frame_translate_| X and Y coordinates back to 0.
  WebMouseEvent FlattenTransform() const;

  // Makes the event modifier bit corresponding to the `button` field match the
  // implied button state. More precisely, at mousedown it sets the modifier bit
  // and at mouseup it resets the bit.  Low-level events from the system may not
  // set/reset the bit correctly as per the spec:
  // https://www.w3.org/TR/uievents/#dom-mouseevent-buttons
  //
  // Other modifier bits remain unchanged for these two events, and no change is
  // made for other events.
  void UpdateEventModifiersToMatchButton();

 protected:
  WebMouseEvent(PointerId id_param)
      : WebPointerProperties(id_param, PointerType::kMouse) {}

  void FlattenTransformSelf();

 private:
  void SetMenuSourceType(WebInputEvent::Type);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_MOUSE_EVENT_H_
