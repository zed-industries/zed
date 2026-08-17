// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_IMAGE_COMPUTED_CSS_VALUE_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_IMAGE_COMPUTED_CSS_VALUE_BUILDER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CSSImageSetValue;
class CSSValue;
class ComputedStyle;
enum class CSSValuePhase;

// A helper class that, from a CSS <image> value, produces a CSSValue that
// somewhat resembles a computed value equivalent [1]. Primarily for
// serializing StylePendingImages. Prefer `StyleImage::ComputedCSSValue()` if
// at all possible.
//
// [1] "A computed <image> value is the specified value with any <url>s,
//      <color>s, and <length>s computed."
//     (https://drafts.csswg.org/css-images-4/#image-values)
class StyleImageComputedCSSValueBuilder {
  STACK_ALLOCATED();

 public:
  StyleImageComputedCSSValueBuilder(const ComputedStyle& style,
                                    bool allow_visited_style,
                                    CSSValuePhase value_phase)
      : style_(style),
        allow_visited_style_(allow_visited_style),
        value_phase_(value_phase) {}

  CSSValue* Build(CSSValue* value) const;

 private:
  CSSValue* CrossfadeArgument(CSSValue*) const;
  CSSValue* BuildImageSet(const CSSImageSetValue&) const;

  const ComputedStyle& style_;
  const bool allow_visited_style_;
  CSSValuePhase value_phase_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_IMAGE_COMPUTED_CSS_VALUE_BUILDER_H_
