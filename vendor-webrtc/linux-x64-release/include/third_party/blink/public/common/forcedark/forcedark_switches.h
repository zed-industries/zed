// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FORCEDARK_FORCEDARK_SWITCHES_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FORCEDARK_FORCEDARK_SWITCHES_H_

namespace blink {

// Specifies algorithm for modifying how colors are rendered in Force Dark.
enum class ForceDarkInversionMethod {
  // Use the value provided via command line with --blink_settings or, if that
  // flag is absent, get the value from the defaults in
  // renderer/core/frame/settings.json5.
  kUseBlinkSettings,

  // Modify colors by converting them to the HSL color space and inverting the
  // lightness (i.e. the "L" in HSL).
  kHslBased,

  // Modify colors by converting them to CIE L*a*b color space and inverting the
  // L value.
  kCielabBased,

  // Modify colors by subtracting each of r, g, and b from their maximum value.
  kRgbBased
};

// Specifies algorithm for determining which images to invert in Force Dark.
enum class ForceDarkImageBehavior {
  // Same as ForceDarkInversionMethod::kUseBlinkSettings above.
  kUseBlinkSettings,

  // Do not invert any images.
  kInvertNone,

  // Invert only some images. Images that act as icons or text should be
  // inverted, but photos, avatars, etc. should not be.
  kInvertSelectively
};

// Specifies the classifier used to determine which images to invert, when
// ForceDarkImageBehavior is |kInvertSelectively|
enum class ForceDarkImageClassifier {
  // Same as ForceDarkInversionMethod::kUseBlinkSettings above.
  kUseBlinkSettings,

  // See DarkModeImageClassifierPolicy::kNumColorsWithMlFallback.
  kNumColorsWithMlFallback,

  // See DarkModeImageClassifierPolicy::kTransparencyAndNumColors.
  kTransparencyAndNumColors
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FORCEDARK_FORCEDARK_SWITCHES_H_
