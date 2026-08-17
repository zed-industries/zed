// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_PAGE_ZOOM_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_PAGE_ZOOM_H_

#include "base/containers/span.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

// Note on terminology: "zoom level" is a logarithmic scale with a hard-coded
// log base (see kTextSizeMultiplierRatio). "zoom factor" is a straight
// multiplier which is used for rendering.

// Default user-selectable browser zoom factors.
BLINK_COMMON_EXPORT extern const base::span<const double>
    kPresetBrowserZoomFactors;

// The minimum and maximum browser zoom factors that are allowed.
BLINK_COMMON_EXPORT extern const double kMinimumBrowserZoomFactor;
BLINK_COMMON_EXPORT extern const double kMaximumBrowserZoomFactor;

// Convert between zoom factors and levels.
BLINK_COMMON_EXPORT double ZoomLevelToZoomFactor(double zoom_level);
BLINK_COMMON_EXPORT double ZoomFactorToZoomLevel(double zoom_factor);

// Use this to compare page zoom factors and levels. It accounts for precision
// loss due to conversions back and forth.
BLINK_COMMON_EXPORT bool ZoomValuesEqual(double value_a, double value_b);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_PAGE_ZOOM_H_
