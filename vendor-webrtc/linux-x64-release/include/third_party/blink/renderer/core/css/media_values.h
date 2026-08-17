// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_VALUES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_VALUES_H_

#include <optional>

#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/css/preferred_contrast.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/device_posture/device_posture_provider.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/manifest/display_mode.mojom-shared.h"
#include "third_party/blink/public/mojom/webpreferences/web_preferences.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/container_state.h"
#include "third_party/blink/renderer/core/css/css_length_resolver.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"
#include "ui/base/mojom/window_show_state.mojom-blink-forward.h"
#include "ui/base/pointer/pointer_device.h"

namespace blink {

class CSSPrimitiveValue;
class Document;
class Element;
class LocalFrame;
enum class CSSValueID;
enum class ColorSpaceGamut;
enum class ForcedColors;
enum class NavigationControls;
enum class Scripting;

mojom::blink::PreferredColorScheme CSSValueIDToPreferredColorScheme(
    CSSValueID id);
mojom::blink::PreferredContrast CSSValueIDToPreferredContrast(CSSValueID);
ForcedColors CSSValueIDToForcedColors(CSSValueID id);

class CORE_EXPORT MediaValues : public GarbageCollected<MediaValues>,
                                public CSSLengthResolver {
 public:
  MediaValues() : CSSLengthResolver(1.0f /* zoom */) {}
  virtual ~MediaValues() = default;
  virtual void Trace(Visitor* visitor) const {}

  static MediaValues* CreateDynamicIfFrameExists(LocalFrame*);

  template <typename T>
  bool ComputeLength(double value,
                     CSSPrimitiveValue::UnitType type,
                     T& result) const {
    double temp_result;
    if (!ComputeLengthImpl(value, type, temp_result)) {
      return false;
    }
    result = ClampTo<T>(temp_result);
    return true;
  }

  std::optional<double> InlineSize() const;
  std::optional<double> BlockSize() const;
  virtual std::optional<double> Width() const { return ViewportWidth(); }
  virtual std::optional<double> Height() const { return ViewportHeight(); }
  virtual int DeviceWidth() const = 0;
  virtual int DeviceHeight() const = 0;
  virtual float DevicePixelRatio() const = 0;
  virtual bool DeviceSupportsHDR() const = 0;
  virtual int ColorBitsPerComponent() const = 0;
  virtual int MonochromeBitsPerComponent() const = 0;
  virtual bool InvertedColors() const = 0;
  virtual mojom::blink::PointerType PrimaryPointerType() const = 0;
  virtual int AvailablePointerTypes() const = 0;
  virtual mojom::blink::HoverType PrimaryHoverType() const = 0;
  virtual mojom::blink::OutputDeviceUpdateAbilityType
  OutputDeviceUpdateAbilityType() const = 0;
  virtual int AvailableHoverTypes() const = 0;
  virtual bool ThreeDEnabled() const = 0;
  virtual const String MediaType() const = 0;
  virtual blink::mojom::DisplayMode DisplayMode() const = 0;
  virtual ui::mojom::blink::WindowShowState WindowShowState() const = 0;
  virtual bool Resizable() const = 0;
  virtual bool StrictMode() const = 0;
  virtual Document* GetDocument() const = 0;
  virtual bool HasValues() const = 0;

  virtual ColorSpaceGamut ColorGamut() const = 0;
  virtual mojom::blink::PreferredColorScheme GetPreferredColorScheme()
      const = 0;
  virtual mojom::blink::PreferredContrast GetPreferredContrast() const = 0;
  virtual bool PrefersReducedMotion() const = 0;
  virtual bool PrefersReducedData() const = 0;
  virtual bool PrefersReducedTransparency() const = 0;
  virtual ForcedColors GetForcedColors() const = 0;
  virtual NavigationControls GetNavigationControls() const = 0;
  virtual int GetHorizontalViewportSegments() const = 0;
  virtual int GetVerticalViewportSegments() const = 0;
  virtual mojom::blink::DevicePostureType GetDevicePosture() const = 0;
  // For evaluating scroll-state(stuck: left), scroll-state(stuck: right)
  virtual ContainerStuckPhysical StuckHorizontal() const {
    return ContainerStuckPhysical::kNo;
  }
  // For evaluating scroll-state(stuck: top), scroll-state(stuck: bottom)
  virtual ContainerStuckPhysical StuckVertical() const {
    return ContainerStuckPhysical::kNo;
  }
  // For evaluating scroll-state(stuck: inline-start),
  // scroll-state(stuck: inline-end)
  virtual ContainerStuckLogical StuckInline() const {
    return ContainerStuckLogical::kNo;
  }
  // For evaluating scroll-state(stuck: block-start),
  // scroll-state(stuck: block-end)
  virtual ContainerStuckLogical StuckBlock() const {
    return ContainerStuckLogical::kNo;
  }
  // For boolean context evaluation.
  bool Stuck() const {
    return StuckHorizontal() != ContainerStuckPhysical::kNo ||
           StuckVertical() != ContainerStuckPhysical::kNo;
  }
  virtual ContainerSnappedFlags SnappedFlags() const {
    return static_cast<ContainerSnappedFlags>(ContainerSnapped::kNone);
  }
  // For evaluating scroll-state(snapped: x/y)
  bool SnappedX() const {
    return SnappedFlags() &
           static_cast<ContainerSnappedFlags>(ContainerSnapped::kX);
  }
  bool SnappedY() const {
    return SnappedFlags() &
           static_cast<ContainerSnappedFlags>(ContainerSnapped::kY);
  }
  // For evaluating scroll-state(snapped: block/inline)
  bool SnappedBlock() const;
  bool SnappedInline() const;
  // For boolean context evaluation.
  bool Snapped() const {
    return SnappedFlags() !=
           static_cast<ContainerSnappedFlags>(ContainerSnapped::kNone);
  }
  // For evaluating scroll-state(scrollable: left/right)
  virtual ContainerScrollableFlags ScrollableHorizontal() const {
    return static_cast<ContainerScrollableFlags>(ContainerScrollable::kNone);
  }
  // For evaluating scroll-state(scrollable: top/bottom)
  virtual ContainerScrollableFlags ScrollableVertical() const {
    return static_cast<ContainerScrollableFlags>(ContainerScrollable::kNone);
  }
  // For evaluating scroll-state(scrollable: inline-start/inline-end)
  virtual ContainerScrollableFlags ScrollableInline() const {
    return static_cast<ContainerScrollableFlags>(ContainerScrollable::kNone);
  }
  // For evaluating scroll-state(scrollable: block-start/block-end)
  virtual ContainerScrollableFlags ScrollableBlock() const {
    return static_cast<ContainerScrollableFlags>(ContainerScrollable::kNone);
  }
  // For boolean context evaluation
  bool Scrollable() const {
    return ScrollableHorizontal() != static_cast<ContainerScrollableFlags>(
                                         ContainerScrollable::kNone) ||
           ScrollableVertical() != static_cast<ContainerScrollableFlags>(
                                       ContainerScrollable::kNone);
  }
  // Returns the container element used to retrieve base style and parent style
  // when computing the computed value of a style() container query.
  virtual Element* ContainerElement() const { return nullptr; }

  virtual Scripting GetScripting() const = 0;

  // CSSLengthResolver override.
  void ReferenceTreeScope() const override {}
  void ReferenceAnchor() const override {}
  void ReferenceSibling() const override {}

  Element* GetElement() const override { NOTREACHED(); }

 protected:
  static double CalculateViewportWidth(LocalFrame*);
  static double CalculateViewportHeight(LocalFrame*);
  static double CalculateSmallViewportWidth(LocalFrame*);
  static double CalculateSmallViewportHeight(LocalFrame*);
  static double CalculateLargeViewportWidth(LocalFrame*);
  static double CalculateLargeViewportHeight(LocalFrame*);
  static double CalculateDynamicViewportWidth(LocalFrame*);
  static double CalculateDynamicViewportHeight(LocalFrame*);
  static float CalculateEmSize(LocalFrame*);
  static float CalculateExSize(LocalFrame*);
  static float CalculateChSize(LocalFrame*);
  static float CalculateIcSize(LocalFrame*);
  static float CalculateCapSize(LocalFrame*);
  static float CalculateLineHeight(LocalFrame*);
  static int CalculateDeviceWidth(LocalFrame*);
  static int CalculateDeviceHeight(LocalFrame*);
  static bool CalculateStrictMode(LocalFrame*);
  static float CalculateDevicePixelRatio(LocalFrame*);
  static bool CalculateDeviceSupportsHDR(LocalFrame*);
  static int CalculateColorBitsPerComponent(LocalFrame*);
  static int CalculateMonochromeBitsPerComponent(LocalFrame*);
  static bool CalculateInvertedColors(LocalFrame*);
  static const String CalculateMediaType(LocalFrame*);
  static blink::mojom::DisplayMode CalculateDisplayMode(LocalFrame*);
  static ui::mojom::blink::WindowShowState CalculateWindowShowState(
      LocalFrame*);
  static bool CalculateResizable(LocalFrame*);
  static bool CalculateThreeDEnabled(LocalFrame*);
  static mojom::blink::PointerType CalculatePrimaryPointerType(LocalFrame*);
  static int CalculateAvailablePointerTypes(LocalFrame*);
  static mojom::blink::HoverType CalculatePrimaryHoverType(LocalFrame*);
  static mojom::blink::OutputDeviceUpdateAbilityType
  CalculateOutputDeviceUpdateAbilityType(LocalFrame*);
  static int CalculateAvailableHoverTypes(LocalFrame*);
  static ColorSpaceGamut CalculateColorGamut(LocalFrame*);
  static mojom::blink::PreferredColorScheme CalculatePreferredColorScheme(
      LocalFrame*);
  static mojom::blink::PreferredContrast CalculatePreferredContrast(
      LocalFrame*);
  static bool CalculatePrefersReducedMotion(LocalFrame*);
  static bool CalculatePrefersReducedData(LocalFrame*);
  static bool CalculatePrefersReducedTransparency(LocalFrame*);
  static ForcedColors CalculateForcedColors(LocalFrame*);
  static NavigationControls CalculateNavigationControls(LocalFrame*);
  static int CalculateHorizontalViewportSegments(LocalFrame*);
  static int CalculateVerticalViewportSegments(LocalFrame*);
  static mojom::blink::DevicePostureType CalculateDevicePosture(LocalFrame*);
  static Scripting CalculateScripting(LocalFrame*);

  bool ComputeLengthImpl(double value,
                         CSSPrimitiveValue::UnitType,
                         double& result) const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_VALUES_H_
