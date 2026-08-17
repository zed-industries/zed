// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DEV_TOOLS_EMULATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DEV_TOOLS_EMULATOR_H_

#include <optional>

#include "third_party/blink/public/common/widget/device_emulation_params.h"
#include "third_party/blink/public/mojom/webpreferences/web_preferences.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/lcd_text_preference.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "ui/gfx/geometry/transform.h"

namespace gfx {
class PointF;
}  // namespace gfx

namespace blink {

class WebViewImpl;

class CORE_EXPORT DevToolsEmulator final
    : public GarbageCollected<DevToolsEmulator> {
 public:
  explicit DevToolsEmulator(WebViewImpl*);
  ~DevToolsEmulator();
  void Shutdown();

  void Trace(Visitor*) const;

  // Settings overrides.
  void SetTextAutosizingEnabled(bool);
  void SetDeviceScaleAdjustment(float);
  void SetLCDTextPreference(LCDTextPreference);
  void SetViewportStyle(mojom::blink::ViewportStyle);
  void SetPluginsEnabled(bool);
  void SetScriptEnabled(bool);
  void SetHideScrollbars(bool);
  void SetCookieEnabled(bool);
  void SetDoubleTapToZoomEnabled(bool);
  bool DoubleTapToZoomEnabled() const;
  void SetAvailablePointerTypes(int);
  void SetPrimaryPointerType(mojom::blink::PointerType);
  void SetAvailableHoverTypes(int);
  void SetPrimaryHoverType(mojom::blink::HoverType);
  void SetOutputDeviceUpdateAbilityType(
      mojom::blink::OutputDeviceUpdateAbilityType);
  void SetMainFrameResizesAreOrientationChanges(bool);
  void SetDefaultPageScaleLimits(float min_scale, float max_scale);
  void SetShrinksViewportContentToFit(bool shrink_viewport_content);
  void SetViewportEnabled(bool);
  void SetViewportMetaEnabled(bool);

  // Enables and/or sets the parameters for emulation. Returns the emulation
  // transform to be used as a result.
  gfx::Transform EnableDeviceEmulation(const DeviceEmulationParams&);
  // Disables emulation.
  void DisableDeviceEmulation();

  bool ResizeIsDeviceSizeChange();
  void SetTouchEventEmulationEnabled(bool, int max_touch_points);
  void SetScriptExecutionDisabled(bool);
  void SetScrollbarsHidden(bool);
  void SetDocumentCookieDisabled(bool);
  void SetAutoDarkModeOverride(bool);
  void ResetAutoDarkModeOverride();

  bool HasViewportOverride() const { return !!viewport_override_; }

  // Notify the DevToolsEmulator about a scroll or scale change of the
  // outermost main frame. Returns an updated emulation transform for a
  // viewport override, and should only be called when HasViewportOverride() is
  // true.
  gfx::Transform OutermostMainFrameScrollOrScaleChanged();

  // Returns the scale used to convert incoming input events while emulating
  // device metics.
  float InputEventsScaleForEmulation();

  gfx::Transform ForceViewportForTesting(const gfx::PointF& position,
                                         float scale) {
    return ForceViewport(position, scale);
  }
  gfx::Transform ResetViewportForTesting() { return ResetViewport(); }

 private:
  class ScopedGlobalOverrides;

  void EnableMobileEmulation();
  void DisableMobileEmulation();

  // Enables viewport override and returns the emulation transform to be used.
  // The |position| is in CSS pixels, and |scale| is relative to a page scale of
  // 1.0.
  gfx::Transform ForceViewport(const gfx::PointF& position, float scale);
  // Disables viewport override and returns the emulation transform to be used.
  gfx::Transform ResetViewport();

  // Returns the original device scale factor when overridden by DevTools, or
  // deviceScaleFactor() otherwise.
  float CompositorDeviceScaleFactor() const;

  void ApplyViewportOverride(gfx::Transform*);
  gfx::Transform ComputeRootLayerTransform();
  bool emulate_mobile_enabled() const {
    CHECK(!global_overrides_ || device_metrics_enabled_);
    return !!global_overrides_;
  }

  WebViewImpl* web_view_;

  bool is_shutdown_ = false;
  bool device_metrics_enabled_;
  scoped_refptr<ScopedGlobalOverrides> global_overrides_;
  DeviceEmulationParams emulation_params_;

  struct ViewportOverride {
    gfx::PointF position;
    double scale;
  };
  std::optional<ViewportOverride> viewport_override_;

  bool is_overlay_scrollbars_enabled_;
  bool is_orientation_event_enabled_;
  bool is_mobile_layout_theme_enabled_;
  bool embedder_text_autosizing_enabled_;
  float embedder_device_scale_adjustment_;
  LCDTextPreference embedder_lcd_text_preference_;
  mojom::blink::ViewportStyle embedder_viewport_style_;
  bool embedder_plugins_enabled_;
  int embedder_available_pointer_types_;
  mojom::blink::PointerType embedder_primary_pointer_type_;
  int embedder_available_hover_types_;
  mojom::blink::HoverType embedder_primary_hover_type_;
  mojom::blink::OutputDeviceUpdateAbilityType
      embedder_output_device_update_ability_type_;
  bool embedder_main_frame_resizes_are_orientation_changes_;
  float embedder_min_page_scale_;
  float embedder_max_page_scale_;
  bool embedder_shrink_viewport_content_;
  bool embedder_viewport_enabled_;
  bool embedder_viewport_meta_enabled_;

  bool touch_event_emulation_enabled_;
  bool double_tap_to_zoom_enabled_;
  int original_max_touch_points_;

  bool embedder_script_enabled_;
  bool script_execution_disabled_;

  bool embedder_hide_scrollbars_;
  bool scrollbars_hidden_;

  bool embedder_cookie_enabled_;
  bool document_cookie_disabled_;

  bool embedder_force_dark_mode_enabled_;
  bool auto_dark_overriden_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DEV_TOOLS_EMULATOR_H_
