// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_WIDGET_DEVICE_EMULATION_PARAMS_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_WIDGET_DEVICE_EMULATION_PARAMS_MOJOM_TRAITS_H_

#include <optional>

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/widget/device_emulation_params.h"
#include "third_party/blink/public/mojom/widget/device_emulation_params.mojom-shared.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::DeviceEmulationParamsDataView,
                 blink::DeviceEmulationParams> {
  static blink::mojom::EmulatedScreenType screen_type(
      const blink::DeviceEmulationParams& r) {
    return r.screen_type;
  }

  static const gfx::Size& screen_size(const blink::DeviceEmulationParams& r) {
    return r.screen_size;
  }

  static const std::optional<gfx::Point>& view_position(
      const blink::DeviceEmulationParams& r) {
    return r.view_position;
  }

  static const gfx::Size& view_size(const blink::DeviceEmulationParams& r) {
    return r.view_size;
  }

  static float device_scale_factor(const blink::DeviceEmulationParams& r) {
    return r.device_scale_factor;
  }

  static float scale(const blink::DeviceEmulationParams& r) { return r.scale; }

  static const gfx::PointF& viewport_offset(
      const blink::DeviceEmulationParams& r) {
    return r.viewport_offset;
  }

  static float viewport_scale(const blink::DeviceEmulationParams& r) {
    return r.viewport_scale;
  }

  static display::mojom::ScreenOrientation screen_orientation_type(
      const blink::DeviceEmulationParams& r) {
    return r.screen_orientation_type;
  }

  static uint32_t screen_orientation_angle(
      const blink::DeviceEmulationParams& r) {
    return r.screen_orientation_angle;
  }

  static const std::vector<gfx::Rect>& viewport_segments(
      const blink::DeviceEmulationParams& r) {
    return r.viewport_segments;
  }

  static blink::mojom::DevicePostureType device_posture(
      const blink::DeviceEmulationParams& r) {
    return r.device_posture;
  }

  static bool Read(blink::mojom::DeviceEmulationParamsDataView r,
                   blink::DeviceEmulationParams* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_WIDGET_DEVICE_EMULATION_PARAMS_MOJOM_TRAITS_H_
