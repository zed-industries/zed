// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_ELEMENT_COMPUTED_STYLE_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_ELEMENT_COMPUTED_STYLE_MAP_H_

#include "third_party/blink/renderer/core/css/css_computed_style_declaration.h"
#include "third_party/blink/renderer/core/css/cssom/computed_style_property_map.h"
#include "third_party/blink/renderer/core/dom/element.h"

namespace blink {

class ElementComputedStyleMap {
  STATIC_ONLY(ElementComputedStyleMap);

 public:
  static StylePropertyMapReadOnly* computedStyleMap(Element& element) {
    return element.ComputedStyleMap();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_ELEMENT_COMPUTED_STYLE_MAP_H_
