// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_FEATURE_OVERRIDES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_FEATURE_OVERRIDES_H_

#include <optional>

#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/css/preferred_contrast.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

enum class ColorSpaceGamut;
class Document;
enum class ForcedColors;
class MediaQueryExpValue;

class CORE_EXPORT MediaFeatureOverrides {
 public:
  void SetOverride(const AtomicString& feature,
                   const String& value_string,
                   const Document*);

  std::optional<ColorSpaceGamut> GetColorGamut() const { return color_gamut_; }
  std::optional<mojom::blink::PreferredColorScheme> GetPreferredColorScheme()
      const {
    return preferred_color_scheme_;
  }
  std::optional<mojom::blink::PreferredContrast> GetPreferredContrast() const {
    return preferred_contrast_;
  }
  std::optional<bool> GetPrefersReducedMotion() const {
    return prefers_reduced_motion_;
  }
  std::optional<bool> GetPrefersReducedData() const {
    return prefers_reduced_data_;
  }
  std::optional<bool> GetPrefersReducedTransparency() const {
    return prefers_reduced_transparency_;
  }
  std::optional<ForcedColors> GetForcedColors() const { return forced_colors_; }

  static std::optional<mojom::blink::PreferredColorScheme>
  ConvertPreferredColorScheme(const MediaQueryExpValue&);
  static std::optional<mojom::blink::PreferredContrast>
  ConvertPreferredContrast(const MediaQueryExpValue&);
  static std::optional<bool> ConvertPrefersReducedMotion(
      const MediaQueryExpValue& value);
  static std::optional<bool> ConvertPrefersReducedData(
      const MediaQueryExpValue& value);
  static std::optional<bool> ConvertPrefersReducedTransparency(
      const MediaQueryExpValue& value);

  static MediaQueryExpValue ParseMediaQueryValue(const AtomicString&,
                                                 const String&,
                                                 const Document*);

 private:
  std::optional<ColorSpaceGamut> color_gamut_;
  std::optional<mojom::blink::PreferredColorScheme> preferred_color_scheme_;
  std::optional<mojom::blink::PreferredContrast> preferred_contrast_;
  std::optional<bool> prefers_reduced_motion_;
  std::optional<bool> prefers_reduced_data_;
  std::optional<bool> prefers_reduced_transparency_;
  std::optional<ForcedColors> forced_colors_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_FEATURE_OVERRIDES_H_
