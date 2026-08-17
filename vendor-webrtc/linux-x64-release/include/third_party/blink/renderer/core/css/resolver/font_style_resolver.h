// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_FONT_STYLE_RESOLVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_FONT_STYLE_RESOLVER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_value_set.h"
#include "third_party/blink/renderer/platform/fonts/font_description.h"
#include "third_party/blink/renderer/platform/fonts/font_selector.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// FontStyleResolver is a simpler version of Font-related parts of
// StyleResolver. This is needed because ComputedStyle/StyleResolver can't run
// outside the main thread or without a document. This is a way of minimizing
// duplicate code for when font parsing is needed.
class CORE_EXPORT FontStyleResolver {
  STATIC_ONLY(FontStyleResolver);

 public:
  static FontDescription ComputeFont(const CSSPropertyValueSet&, FontSelector*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_FONT_STYLE_RESOLVER_H_
