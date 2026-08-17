// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_VALUES_CACHED_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_VALUES_CACHED_H_

#include "third_party/blink/public/common/css/forced_colors.h"
#include "third_party/blink/public/common/css/navigation_controls.h"
#include "third_party/blink/public/common/css/scripting.h"
#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-blink.h"
#include "third_party/blink/public/mojom/css/preferred_contrast.mojom-blink.h"
#include "third_party/blink/public/mojom/device_posture/device_posture_provider.mojom-blink.h"
#include "third_party/blink/public/mojom/manifest/display_mode.mojom-blink.h"
#include "third_party/blink/public/mojom/webpreferences/web_preferences.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/media_values.h"
#include "third_party/blink/renderer/platform/graphics/color_space_gamut.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"
#include "ui/base/mojom/window_show_state.mojom-blink.h"
#include "ui/base/pointer/pointer_device.h"

namespace blink {

class CORE_EXPORT MediaValuesCached final : public MediaValues {
 public:
  struct CORE_EXPORT MediaValuesCachedData final {
   public:
    // Members variables must be thread safe, since they're copied to the parser
    // thread
    double viewport_width = 0;
    double viewport_height = 0;
    double small_viewport_width = 0;
    double small_viewport_height = 0;
    double large_viewport_width = 0;
    double large_viewport_height = 0;
    double dynamic_viewport_width = 0;
    double dynamic_viewport_height = 0;
    int device_width = 0;
    int device_height = 0;
    float device_pixel_ratio = 1.0;
    bool device_supports_hdr = false;
    int color_bits_per_component = 24;
    int monochrome_bits_per_component = 0;
    bool inverted_colors = false;
    mojom::blink::PointerType primary_pointer_type =
        mojom::blink::PointerType::kPointerNone;
    // Bitmask of |ui::PointerType|
    int available_pointer_types = ui::POINTER_TYPE_NONE;
    mojom::blink::HoverType primary_hover_type =
        mojom::blink::HoverType::kHoverNone;
    mojom::blink::OutputDeviceUpdateAbilityType
        output_device_update_ability_type =
            mojom::blink::OutputDeviceUpdateAbilityType::kFastType;
    // Bitmask of |ui::HoverType|
    int available_hover_types = ui::HOVER_TYPE_NONE;
    float em_size = 16.f;
    float ex_size = 8.f;
    float ch_size = 8.f;
    float ic_size = 16.f;
    float cap_size = 16.f;
    float line_height = 0;
    bool three_d_enabled = false;
    bool strict_mode = true;
    String media_type;
    mojom::blink::DisplayMode display_mode =
        mojom::blink::DisplayMode::kBrowser;
    ui::mojom::blink::WindowShowState window_show_state =
        ui::mojom::blink::WindowShowState::kDefault;
    bool resizable = true;
    ColorSpaceGamut color_gamut = ColorSpaceGamut::kUnknown;
    mojom::blink::PreferredColorScheme preferred_color_scheme =
        mojom::blink::PreferredColorScheme::kLight;
    mojom::blink::PreferredContrast preferred_contrast =
        mojom::blink::PreferredContrast::kNoPreference;
    bool prefers_reduced_motion = false;
    bool prefers_reduced_data = false;
    bool prefers_reduced_transparency = false;
    ForcedColors forced_colors = ForcedColors::kNone;
    NavigationControls navigation_controls = NavigationControls::kNone;
    int horizontal_viewport_segments = 0;
    int vertical_viewport_segments = 0;
    mojom::blink::DevicePostureType device_posture =
        mojom::blink::DevicePostureType::kContinuous;
    Scripting scripting = Scripting::kNone;

    MediaValuesCachedData();
    explicit MediaValuesCachedData(Document&);
  };

  MediaValuesCached();
  explicit MediaValuesCached(Document&);
  explicit MediaValuesCached(const MediaValuesCachedData&);

  MediaValues* Copy() const;

  int DeviceWidth() const override;
  int DeviceHeight() const override;
  float DevicePixelRatio() const override;
  bool DeviceSupportsHDR() const override;
  int ColorBitsPerComponent() const override;
  int MonochromeBitsPerComponent() const override;
  bool InvertedColors() const override;
  mojom::blink::PointerType PrimaryPointerType() const override;
  int AvailablePointerTypes() const override;
  mojom::blink::HoverType PrimaryHoverType() const override;
  mojom::blink::OutputDeviceUpdateAbilityType OutputDeviceUpdateAbilityType()
      const override;
  int AvailableHoverTypes() const override;
  bool ThreeDEnabled() const override;
  bool StrictMode() const override;
  Document* GetDocument() const override;
  bool HasValues() const override;
  const String MediaType() const override;
  blink::mojom::DisplayMode DisplayMode() const override;
  ui::mojom::blink::WindowShowState WindowShowState() const override;
  bool Resizable() const override;
  ColorSpaceGamut ColorGamut() const override;
  mojom::blink::PreferredColorScheme GetPreferredColorScheme() const override;
  mojom::blink::PreferredContrast GetPreferredContrast() const override;
  bool PrefersReducedMotion() const override;
  bool PrefersReducedData() const override;
  bool PrefersReducedTransparency() const override;
  ForcedColors GetForcedColors() const override;
  NavigationControls GetNavigationControls() const override;
  int GetHorizontalViewportSegments() const override;
  int GetVerticalViewportSegments() const override;
  mojom::blink::DevicePostureType GetDevicePosture() const override;
  Scripting GetScripting() const override;

  void OverrideViewportDimensions(double width, double height);

 protected:
  // CSSLengthResolver
  float EmFontSize(float zoom) const override;
  float RemFontSize(float zoom) const override;
  float ExFontSize(float zoom) const override;
  float RexFontSize(float zoom) const override;
  float ChFontSize(float zoom) const override;
  float RchFontSize(float zoom) const override;
  float IcFontSize(float zoom) const override;
  float RicFontSize(float zoom) const override;
  float LineHeight(float zoom) const override;
  float RootLineHeight(float zoom) const override;
  float CapFontSize(float zoom) const override;
  float RcapFontSize(float zoom) const override;
  double ViewportWidth() const override;
  double ViewportHeight() const override;
  double SmallViewportWidth() const override;
  double SmallViewportHeight() const override;
  double LargeViewportWidth() const override;
  double LargeViewportHeight() const override;
  double DynamicViewportWidth() const override;
  double DynamicViewportHeight() const override;
  double ContainerWidth() const override;
  double ContainerHeight() const override;
  double ContainerWidth(const ScopedCSSName&) const override;
  double ContainerHeight(const ScopedCSSName&) const override;
  WritingMode GetWritingMode() const override {
    return WritingMode::kHorizontalTb;
  }

  MediaValuesCachedData data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_VALUES_CACHED_H_
