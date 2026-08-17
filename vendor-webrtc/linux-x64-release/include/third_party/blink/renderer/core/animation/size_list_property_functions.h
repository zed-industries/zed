// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SIZE_LIST_PROPERTY_FUNCTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SIZE_LIST_PROPERTY_FUNCTIONS_H_

#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/style/fill_layer.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ComputedStyle;
class ComputedStyleBuilder;
class CSSProperty;

using SizeList = Vector<FillSize, 1>;

class SizeListPropertyFunctions {
  STATIC_ONLY(SizeListPropertyFunctions);

 public:
  static SizeList GetInitialSizeList(const CSSProperty&,
                                     const ComputedStyle& initial_style);
  static SizeList GetSizeList(const CSSProperty&, const ComputedStyle&);
  static void SetSizeList(const CSSProperty&,
                          ComputedStyleBuilder&,
                          const SizeList&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SIZE_LIST_PROPERTY_FUNCTIONS_H_
