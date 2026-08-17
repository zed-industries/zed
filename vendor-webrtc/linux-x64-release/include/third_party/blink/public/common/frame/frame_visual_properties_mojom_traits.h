// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_VISUAL_PROPERTIES_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_VISUAL_PROPERTIES_MOJOM_TRAITS_H_

#include <optional>

#include "base/check_op.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/frame/frame_visual_properties.h"
#include "third_party/blink/public/mojom/frame/frame_visual_properties.mojom-shared.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::FrameVisualPropertiesDataView,
                 blink::FrameVisualProperties> {
  static const display::ScreenInfos& screen_infos(
      const blink::FrameVisualProperties& r) {
    return r.screen_infos;
  }

  static bool auto_resize_enabled(const blink::FrameVisualProperties& r) {
    return r.auto_resize_enabled;
  }

  static bool is_pinch_gesture_active(const blink::FrameVisualProperties& r) {
    return r.is_pinch_gesture_active;
  }

  static uint32_t capture_sequence_number(
      const blink::FrameVisualProperties& r) {
    return r.capture_sequence_number;
  }

  static double zoom_level(const blink::FrameVisualProperties& r) {
    return r.zoom_level;
  }

  static double css_zoom_factor(const blink::FrameVisualProperties& r) {
    return r.css_zoom_factor;
  }

  static double page_scale_factor(const blink::FrameVisualProperties& r) {
    DCHECK_GT(r.page_scale_factor, 0);
    return r.page_scale_factor;
  }

  static double compositing_scale_factor(
      const blink::FrameVisualProperties& r) {
    DCHECK_GT(r.compositing_scale_factor, 0);
    return r.compositing_scale_factor;
  }

  static float cursor_accessibility_scale_factor(
      const blink::FrameVisualProperties& r) {
    DCHECK_GE(r.cursor_accessibility_scale_factor, 1.f);
    return r.cursor_accessibility_scale_factor;
  }

  static const gfx::Size& visible_viewport_size(
      const blink::FrameVisualProperties& r) {
    return r.visible_viewport_size;
  }

  static const gfx::Size& min_size_for_auto_resize(
      const blink::FrameVisualProperties& r) {
    return r.min_size_for_auto_resize;
  }

  static const gfx::Size& max_size_for_auto_resize(
      const blink::FrameVisualProperties& r) {
    return r.max_size_for_auto_resize;
  }

  static const std::vector<gfx::Rect>& root_widget_viewport_segments(
      const blink::FrameVisualProperties& r) {
    return r.root_widget_viewport_segments;
  }

  static const gfx::Rect& compositor_viewport(
      const blink::FrameVisualProperties& r) {
    return r.compositor_viewport;
  }

  static const gfx::Rect& rect_in_local_root(
      const blink::FrameVisualProperties& r) {
    return r.rect_in_local_root;
  }

  static const gfx::Size& local_frame_size(
      const blink::FrameVisualProperties& r) {
    return r.local_frame_size;
  }

  static const viz::LocalSurfaceId& local_surface_id(
      const blink::FrameVisualProperties& r) {
    return r.local_surface_id;
  }

  static bool Read(blink::mojom::FrameVisualPropertiesDataView r,
                   blink::FrameVisualProperties* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_VISUAL_PROPERTIES_MOJOM_TRAITS_H_
