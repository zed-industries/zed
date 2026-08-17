// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_DISPLAY_SIZE_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_DISPLAY_SIZE_MOJOM_TRAITS_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/interest_group/ad_display_size.h"
#include "third_party/blink/public/mojom/interest_group/ad_display_size.mojom-shared.h"
#include "url/gurl.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::AdSizeDataView, blink::AdSize> {
  static double width(const blink::AdSize& size) { return size.width; }

  static blink::AdSize::LengthUnit width_units(const blink::AdSize& size) {
    return size.width_units;
  }

  static double height(const blink::AdSize& size) { return size.height; }

  static blink::AdSize::LengthUnit height_units(const blink::AdSize& size) {
    return size.height_units;
  }

  static bool Read(blink::mojom::AdSizeDataView data, blink::AdSize* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::AdDescriptorDataView, blink::AdDescriptor> {
  static GURL url(const blink::AdDescriptor& ad_descriptor) {
    return ad_descriptor.url;
  }

  static std::optional<blink::AdSize> size(
      const blink::AdDescriptor& ad_descriptor) {
    return ad_descriptor.size;
  }

  static bool Read(blink::mojom::AdDescriptorDataView data,
                   blink::AdDescriptor* ad_descriptor);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_DISPLAY_SIZE_MOJOM_TRAITS_H_
