// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SCREEN_METRICS_EMULATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SCREEN_METRICS_EMULATOR_H_

#include <memory>
#include <vector>

#include "third_party/blink/public/common/widget/device_emulation_params.h"
#include "third_party/blink/public/mojom/widget/device_emulation_params.mojom-blink.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "ui/display/screen_info.h"
#include "ui/display/screen_infos.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

struct VisualProperties;
class WebFrameWidgetImpl;

// ScreenMetricsEmulator class manages screen emulation inside a
// WebFrameWidgetImpl. This includes resizing, placing view on the screen at
// desired position, changing device scale factor, and scaling down the whole
// widget if required to fit into the browser window.
class ScreenMetricsEmulator : public GarbageCollected<ScreenMetricsEmulator> {
 public:
  ScreenMetricsEmulator(WebFrameWidgetImpl* frame_widget,
                        const display::ScreenInfos& screen_infos,
                        const gfx::Size& widget_size_dips,
                        const gfx::Size& visible_viewport_size,
                        const gfx::Rect& view_screen_rect,
                        const gfx::Rect& window_screen_rect);
  ScreenMetricsEmulator(const ScreenMetricsEmulator&) = delete;
  ScreenMetricsEmulator& operator=(const ScreenMetricsEmulator&) = delete;
  virtual ~ScreenMetricsEmulator() = default;

  const display::ScreenInfo& GetOriginalScreenInfo() const;
  const display::ScreenInfos& original_screen_infos() const {
    return original_screen_infos_;
  }

  // This rect is the WidgetScreenRect or ViewRect, which is the main frame
  // widget's bounding box, not including OS window decor, in logical DIP screen
  // coordinates.
  const gfx::Rect& original_view_rect() const {
    return original_view_screen_rect_;
  }
  // This rect is the WindowScreenRect or WindowRect, which is the bounding box
  // of the main frame's top level window, including OS window decor, in logical
  // DIP screen coordinates.
  const gfx::Rect& original_window_rect() const {
    return original_window_screen_rect_;
  }

  float scale() const { return emulation_params_.scale; }

  // Emulated position of the main frame widget (aka view) rect.
  gfx::Point ViewRectOrigin();

  // Disables emulation and applies non-emulated values to the
  // WebFrameWidgetImpl. Call this before destroying the ScreenMetricsEmulator.
  void DisableAndApply();

  // Sets new parameters and applies them to the WebFrameWidgetImpl.
  void ChangeEmulationParams(const DeviceEmulationParams& params);

  void UpdateVisualProperties(const VisualProperties& visual_properties);

  void OnUpdateScreenRects(const gfx::Rect& view_screen_rect,
                           const gfx::Rect& window_screen_rect);

  virtual void Trace(Visitor*) const;

 private:
  bool emulating_desktop() const {
    return emulation_params_.screen_type ==
           mojom::blink::EmulatedScreenType::kDesktop;
  }

  // Applies emulated values to the WidgetBase.
  void Apply();

  Member<WebFrameWidgetImpl> const frame_widget_;

  // Parameters as passed by `WebFrameWidgetImpl::EnableDeviceEmulation()`
  DeviceEmulationParams emulation_params_;

  // Original values to restore back after emulation ends.
  display::ScreenInfos original_screen_infos_;
  gfx::Size original_widget_size_dips_;
  gfx::Size original_visible_viewport_size_;
  gfx::Rect original_view_screen_rect_;
  gfx::Rect original_window_screen_rect_;
  std::vector<gfx::Rect> original_root_viewport_segments_
      ALLOW_DISCOURAGED_TYPE(
          "WebFrameWidgetImpl::SetViewportSegments() uses STL");
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SCREEN_METRICS_EMULATOR_H_
