// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_WIDGET_VISUAL_PROPERTIES_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_WIDGET_VISUAL_PROPERTIES_MOJOM_TRAITS_H_

#include <optional>

#include "base/check_op.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/widget/visual_properties.h"
#include "third_party/blink/public/mojom/widget/visual_properties.mojom-shared.h"
#include "ui/base/mojom/window_show_state.mojom-forward.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT StructTraits<blink::mojom::VisualPropertiesDataView,
                                        blink::VisualProperties> {
  static const display::ScreenInfos& screen_infos(
      const blink::VisualProperties& r) {
    return r.screen_infos;
  }

  static bool auto_resize_enabled(const blink::VisualProperties& r) {
    return r.auto_resize_enabled;
  }

  static bool resizable(const blink::VisualProperties& r) {
    return r.resizable;
  }

  static const gfx::Size& min_size_for_auto_resize(
      const blink::VisualProperties& r) {
    return r.min_size_for_auto_resize;
  }

  static const gfx::Size& max_size_for_auto_resize(
      const blink::VisualProperties& r) {
    return r.max_size_for_auto_resize;
  }

  static const gfx::Size& new_size_device_px(const blink::VisualProperties& r) {
    return r.new_size_device_px;
  }

  static const gfx::Size& visible_viewport_size_device_px(
      const blink::VisualProperties& r) {
    return r.visible_viewport_size_device_px;
  }

  static const gfx::Rect& compositor_viewport_pixel_rect(
      const blink::VisualProperties& r) {
    return r.compositor_viewport_pixel_rect;
  }

  static const cc::BrowserControlsParams& browser_controls_params(
      const blink::VisualProperties& r) {
    return r.browser_controls_params;
  }

  static bool scroll_focused_node_into_view(const blink::VisualProperties& r) {
    return r.scroll_focused_node_into_view;
  }

  static const std::optional<viz::LocalSurfaceId>& local_surface_id(
      const blink::VisualProperties& r) {
    return r.local_surface_id;
  }

  static bool is_fullscreen_granted(const blink::VisualProperties& r) {
    return r.is_fullscreen_granted;
  }

  static blink::mojom::DisplayMode display_mode(
      const blink::VisualProperties& r) {
    return r.display_mode;
  }

  static const ui::mojom::WindowShowState& window_show_state(
      const blink::VisualProperties& r) {
    return r.window_show_state;
  }

  static uint32_t capture_sequence_number(const blink::VisualProperties& r) {
    return r.capture_sequence_number;
  }

  static double zoom_level(const blink::VisualProperties& r) {
    return r.zoom_level;
  }

  static double css_zoom_factor(const blink::VisualProperties& r) {
    return r.css_zoom_factor;
  }

  static int virtual_keyboard_resize_height_device_px(
      const blink::VisualProperties& r) {
    return r.virtual_keyboard_resize_height_device_px;
  }

  static double page_scale_factor(const blink::VisualProperties& r) {
    DCHECK_GT(r.page_scale_factor, 0);
    return r.page_scale_factor;
  }

  static double compositing_scale_factor(const blink::VisualProperties& r) {
    DCHECK_GT(r.compositing_scale_factor, 0);
    return r.compositing_scale_factor;
  }

  static float cursor_accessibility_scale_factor(
      const blink::VisualProperties& r) {
    DCHECK_GE(r.cursor_accessibility_scale_factor, 1.f);
    return r.cursor_accessibility_scale_factor;
  }

  static const std::vector<gfx::Rect>& root_widget_viewport_segments(
      const blink::VisualProperties& r) {
    return r.root_widget_viewport_segments;
  }

  static bool is_pinch_gesture_active(const blink::VisualProperties& r) {
    return r.is_pinch_gesture_active;
  }

  static const gfx::Rect& window_controls_overlay_rect(
      const blink::VisualProperties& r) {
    return r.window_controls_overlay_rect;
  }

  static bool Read(blink::mojom::VisualPropertiesDataView r,
                   blink::VisualProperties* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_WIDGET_VISUAL_PROPERTIES_MOJOM_TRAITS_H_
