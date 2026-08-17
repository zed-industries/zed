// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_POINTER_PROPERTIES_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_POINTER_PROPERTIES_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/input/pointer_id.h"
#include "third_party/blink/public/mojom/input/input_event.mojom-shared.h"
#include "ui/events/event_constants.h"
#include "ui/gfx/geometry/point_f.h"

#include <limits>

namespace blink {

// This class encapsulates the properties that are common between mouse and
// pointer events and touch points as we transition towards the unified pointer
// event model.
// TODO(mustaq): Unify WebTouchPoint & WebMouseEvent into WebPointerEvent.
// crbug.com/508283
class WebPointerProperties {
 public:
  using Button = mojom::Button;

  using PointerType = ui::EventPointerType;

  enum class Buttons : unsigned {
    kNoButton = 0,
    kLeft = 1 << 0,
    kRight = 1 << 1,
    kMiddle = 1 << 2,
    kBack = 1 << 3,
    kForward = 1 << 4,
    kEraser = 1 << 5
  };

  explicit WebPointerProperties(
      PointerId id_param,
      PointerType pointer_type_param = PointerType::kUnknown,
      Button button_param = Button::kNoButton,
      gfx::PointF position_in_widget = gfx::PointF(),
      gfx::PointF position_in_screen = gfx::PointF(),
      int movement_x = 0,
      int movement_y = 0)
      : id(id_param),
        button(button_param),
        pointer_type(pointer_type_param),
        movement_x(movement_x),
        movement_y(movement_y),
        position_in_widget_(position_in_widget),
        position_in_screen_(position_in_screen) {}

  const gfx::PointF& PositionInWidget() const { return position_in_widget_; }
  const gfx::PointF& PositionInScreen() const { return position_in_screen_; }

  void SetPositionInWidget(float x, float y) {
    position_in_widget_ = gfx::PointF(x, y);
  }

  void SetPositionInScreen(float x, float y) {
    position_in_screen_ = gfx::PointF(x, y);
  }

  void SetPositionInWidget(const gfx::PointF& point) {
    position_in_widget_ = point;
  }

  void SetPositionInScreen(const gfx::PointF& point) {
    position_in_screen_ = point;
  }

  PointerId id;

  // The valid range is [0,1], with NaN meaning pressure is not supported by
  // the input device.
  float force = std::numeric_limits<float>::quiet_NaN();

  // Tilt of a pen stylus from surface normal as plane angles in degrees,
  // Values lie in [-90,90]. A positive tiltX is to the right and a positive
  // tiltY is towards the user.
  double tilt_x = 0;
  double tilt_y = 0;

  // The normalized tangential pressure (or barrel pressure), typically set by
  // an additional control of the stylus, which has a range of [-1,1], where 0
  // is the neutral position of the control. Always 0 if the device does not
  // support it.
  float tangential_pressure = 0.0f;

  // The clockwise rotation of a pen stylus around its own major axis, in
  // degrees in the range [0,359]. Always 0 if the device does not support it.
  int twist = 0;

  // - For pointerup/down events, the button of pointing device that triggered
  // the event.
  // - For other events, the button that was depressed during the move event. If
  // multiple buttons were depressed, one of the depressed buttons (platform
  // dependent).
  Button button;

  PointerType pointer_type;

  int movement_x;
  int movement_y;

  // True if this event has raw movement value from OS.
  // TODO(crbug.com/982379): Figure out how to avoid using this boolean.
  bool is_raw_movement_event = false;

  // Contains unique device id for pen on supported devices.
  int32_t device_id = -1;

 protected:
  // Widget coordinate, which is relative to the bound of current RenderWidget
  // (e.g. a plugin or OOPIF inside a RenderView). Similar to viewport
  // coordinates but without DevTools emulation transform or overscroll applied.
  gfx::PointF position_in_widget_;

  // Screen coordinate
  gfx::PointF position_in_screen_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_POINTER_PROPERTIES_H_
