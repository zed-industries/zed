// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_METRICS_OVERRIDE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_METRICS_OVERRIDE_H_

#include <optional>

namespace blink {

struct FontMetricsOverride {
  std::optional<float> ascent_override;
  std::optional<float> descent_override;
  std::optional<float> line_gap_override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_METRICS_OVERRIDE_H_
