// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_WIDGET_DEVICE_EMULATION_PARAMS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_WIDGET_DEVICE_EMULATION_PARAMS_H_

#include <optional>

#include "third_party/blink/public/mojom/device_posture/device_posture_provider.mojom-shared.h"
#include "third_party/blink/public/mojom/widget/device_emulation_params.mojom-shared.h"
#include "ui/display/mojom/screen_orientation.mojom.h"
#include "ui/gfx/geometry/point.h"
#include "ui/gfx/geometry/point_f.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

// All sizes are measured in device independent pixels.
struct DeviceEmulationParams {
  mojom::EmulatedScreenType screen_type = mojom::EmulatedScreenType::kDesktop;

  // Emulated screen size. Typically full / physical size of the device screen
  // in DIP. Empty size means using default value: original one for kDesktop
  // screen position, equal to |view_size| for kMobile.
  gfx::Size screen_size;

  // Position of view on the screen. Missing position means using default value:
  // original one for kDesktop screen position, (0, 0) for kMobile.
  std::optional<gfx::Point> view_position;

  // Emulated view size. A width or height of 0 means no override in that
  // dimension, but the other can still be applied. When both are 0, then the
  // |scale| will be applied to the view instead.
  gfx::Size view_size;

  // If zero, the original device scale factor is preserved.
  float device_scale_factor = 0;

  // Scale the contents of the main frame. The view's size will be scaled by
  // this number when they are not specified in |view_size|.
  float scale = 1;

  // Forced viewport offset for screenshots during emulation, (-1, -1) for
  // disabled.
  gfx::PointF viewport_offset = gfx::PointF(-1, -1);

  // Viewport scale for screenshots during emulation, 0 for current.
  float viewport_scale = 0;

  // Optional screen orientation type, with mojom::ScreenOrientation::kUndefined
  // value meaning no emulation necessary.
  display::mojom::ScreenOrientation screen_orientation_type =
      display::mojom::ScreenOrientation::kUndefined;

  // Screen orientation angle, used together with screenOrientationType.
  uint32_t screen_orientation_angle = 0;

  // Screen viewport segments dimensions.
  std::vector<gfx::Rect> viewport_segments;

  // Device posture, the default is "continuous".
  mojom::DevicePostureType device_posture =
      mojom::DevicePostureType::kContinuous;

  DeviceEmulationParams() = default;
};

inline bool operator==(const DeviceEmulationParams& a,
                       const DeviceEmulationParams& b) {
  return a.screen_type == b.screen_type && a.screen_size == b.screen_size &&
         a.view_position == b.view_position &&
         a.device_scale_factor == b.device_scale_factor &&
         a.view_size == b.view_size && a.scale == b.scale &&
         a.screen_orientation_type == b.screen_orientation_type &&
         a.screen_orientation_angle == b.screen_orientation_angle &&
         a.viewport_offset == b.viewport_offset &&
         a.viewport_scale == b.viewport_scale &&
         a.viewport_segments == b.viewport_segments &&
         a.device_posture == b.device_posture;
}

inline bool operator!=(const DeviceEmulationParams& a,
                       const DeviceEmulationParams& b) {
  return !(a == b);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_WIDGET_DEVICE_EMULATION_PARAMS_H_
