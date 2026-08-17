// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_IMAGE_SLICE_PROPERTY_FUNCTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_IMAGE_SLICE_PROPERTY_FUNCTIONS_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// This struct doesn't retain ownership of the slices, treat it like a
// reference.
struct ImageSlice {
  ImageSlice(const LengthBox& slices, bool fill) : slices(slices), fill(fill) {}

  const LengthBox& slices;
  bool fill;
};

class ImageSlicePropertyFunctions {
  STATIC_ONLY(ImageSlicePropertyFunctions);

 public:
  static ImageSlice GetInitialImageSlice(const CSSProperty& property,
                                         const ComputedStyle& initial_style) {
    return GetImageSlice(property, initial_style);
  }

  static ImageSlice GetImageSlice(const CSSProperty& property,
                                  const ComputedStyle& style) {
    switch (property.PropertyID()) {
      case CSSPropertyID::kBorderImageSlice:
        return ImageSlice(style.BorderImageSlices(),
                          style.BorderImageSlicesFill());
      case CSSPropertyID::kWebkitMaskBoxImageSlice:
        return ImageSlice(style.MaskBoxImageSlices(),
                          style.MaskBoxImageSlicesFill());
      default:
        NOTREACHED();
    }
  }

  static void SetImageSlice(const CSSProperty& property,
                            ComputedStyleBuilder& builder,
                            const ImageSlice& slice) {
    switch (property.PropertyID()) {
      case CSSPropertyID::kBorderImageSlice:
        builder.SetBorderImageSlices(slice.slices);
        builder.SetBorderImageSlicesFill(slice.fill);
        break;
      case CSSPropertyID::kWebkitMaskBoxImageSlice:
        builder.SetMaskBoxImageSlices(slice.slices);
        builder.SetMaskBoxImageSlicesFill(slice.fill);
        break;
      default:
        NOTREACHED();
    }
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_IMAGE_SLICE_PROPERTY_FUNCTIONS_H_
