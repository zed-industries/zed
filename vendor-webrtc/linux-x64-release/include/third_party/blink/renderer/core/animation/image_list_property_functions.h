// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_IMAGE_LIST_PROPERTY_FUNCTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_IMAGE_LIST_PROPERTY_FUNCTIONS_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

using StyleImageList = GCedHeapVector<Member<StyleImage>, 1>;

class ImageListPropertyFunctions {
 public:
  static void GetInitialImageList(const CSSProperty&, StyleImageList* result) {
    result->clear();
  }

  static void GetImageList(const CSSProperty& property,
                           const ComputedStyle& style,
                           StyleImageList* result) {
    const FillLayer* fill_layer = nullptr;
    switch (property.PropertyID()) {
      case CSSPropertyID::kBackgroundImage:
        fill_layer = &style.BackgroundLayers();
        break;
      case CSSPropertyID::kMaskImage:
        fill_layer = &style.MaskLayers();
        break;
      default:
        NOTREACHED();
    }

    result->clear();
    while (fill_layer) {
      result->push_back(fill_layer->GetImage());
      fill_layer = fill_layer->Next();
    }
  }

  static void SetImageList(const CSSProperty& property,
                           ComputedStyleBuilder& builder,
                           const StyleImageList* image_list) {
    FillLayer* fill_layer = nullptr;
    switch (property.PropertyID()) {
      case CSSPropertyID::kBackgroundImage:
        fill_layer = &builder.AccessBackgroundLayers();
        break;
      case CSSPropertyID::kMaskImage:
        fill_layer = &builder.AccessMaskLayers();
        break;
      default:
        NOTREACHED();
    }

    FillLayer* prev = nullptr;
    for (wtf_size_t i = 0; i < image_list->size(); i++) {
      if (!fill_layer)
        fill_layer = prev->EnsureNext();
      fill_layer->SetImage(image_list->at(i));
      prev = fill_layer;
      fill_layer = fill_layer->Next();
    }
    while (fill_layer) {
      fill_layer->ClearImage();
      fill_layer = fill_layer->Next();
    }
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_IMAGE_LIST_PROPERTY_FUNCTIONS_H_
