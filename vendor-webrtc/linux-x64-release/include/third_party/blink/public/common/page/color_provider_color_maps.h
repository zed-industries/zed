// Copyright 2023 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_COLOR_PROVIDER_COLOR_MAPS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_COLOR_PROVIDER_COLOR_MAPS_H_

#include "third_party/blink/public/common/common_export.h"
#include "ui/color/color_provider_utils.h"

namespace blink {

struct BLINK_COMMON_EXPORT ColorProviderColorMaps {
  ui::RendererColorMap light_colors_map;
  ui::RendererColorMap dark_colors_map;
  ui::RendererColorMap forced_colors_map;

  // Creates a default set of color maps for tests and places where we might not
  // have access to ColorProvider objects.
  static ColorProviderColorMaps CreateDefault() {
    return ColorProviderColorMaps{
        ui::GetDefaultBlinkColorProviderColorMaps(/*dark_mode=*/false,
                                                  /*is_forced_colors=*/false),
        ui::GetDefaultBlinkColorProviderColorMaps(/*dark_mode=*/true,
                                                  /*is_forced_colors=*/false),
        ui::GetDefaultBlinkColorProviderColorMaps(/*dark_mode=*/false,
                                                  /*is_forced_colors=*/true),
    };
  }

  // Returns true if any of the maps are empty.
  bool IsEmpty() const {
    return light_colors_map.empty() || dark_colors_map.empty() ||
           forced_colors_map.empty();
  }

  bool operator==(const ColorProviderColorMaps& other) const {
    return light_colors_map == other.light_colors_map &&
           dark_colors_map == other.dark_colors_map &&
           forced_colors_map == other.forced_colors_map;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_COLOR_PROVIDER_COLOR_MAPS_H_
