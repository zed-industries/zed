// Copyright 2023 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_COLOR_PROVIDER_COLOR_MAPS_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_COLOR_PROVIDER_COLOR_MAPS_MOJOM_TRAITS_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/page/color_provider_color_maps.h"
#include "third_party/blink/public/mojom/page/page.mojom.h"
#include "ui/color/color_provider_utils.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::ColorProviderColorMapsDataView,
                 blink::ColorProviderColorMaps> {
  static const ui::RendererColorMap& light_colors_map(
      const blink::ColorProviderColorMaps& colors) {
    return colors.light_colors_map;
  }

  static const ui::RendererColorMap& dark_colors_map(
      const blink::ColorProviderColorMaps& colors) {
    return colors.dark_colors_map;
  }

  static const ui::RendererColorMap& forced_colors_map(
      const blink::ColorProviderColorMaps& colors) {
    return colors.forced_colors_map;
  }

  static bool Read(blink::mojom::ColorProviderColorMapsDataView data,
                   blink::ColorProviderColorMaps* out_colors);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_COLOR_PROVIDER_COLOR_MAPS_MOJOM_TRAITS_H_
