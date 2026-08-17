// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_KEYFRAME_SHORTHAND_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_KEYFRAME_SHORTHAND_VALUE_H_

#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

#include "third_party/blink/renderer/core/css/style_property_serializer.h"

namespace blink {

// The special value is used to keep around individual longhand css
// property/values that resulted from parsing a shorthand value. This way we can
// reconstruct the shorthand back from them.
//
// Context:
//
// Web Animation specs requires that we keep around and return a parsed
// shorthand name/value pair if they are present in keyframes. However Blink css
// parser does not keep around shorthands and instead produces longhands.
// Instead of updating the css parser engine to preserve shorthands (which is a
// large undertaking) we are taking a shortcut here that allows us to use
// existing logic that enables serialization of a shorthand given its longhands
// i.e., `StylePropertySerializer`. To this end, this class is be used to wrap
// and store longhands produced by a single shorthand as part of animation
// keyframe logic.
//
// For more information see:
//  - `StringKeyframe::SetCSSPropertyValue()`
//  - https://drafts.csswg.org/web-animations/#process-a-keyframes-argument

class CSSKeyframeShorthandValue : public CSSValue {
 public:
  // Assumes that all property/value pairs that are present in the input set are
  // longhands for the same shorthand property/value pair.
  CSSKeyframeShorthandValue(CSSPropertyID shorthand,
                            ImmutableCSSPropertyValueSet*);

  String CustomCSSText() const;

  bool Equals(const CSSKeyframeShorthandValue& other) const {
    return shorthand_ == other.shorthand_ && properties_ == other.properties_;
  }

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  // The shorthand property that these longhands belonged to.
  // Note that a single longhand property may belong to multiple shorthands
  // (e.g., border-left-style belongs to border-style and border) so we keep
  // this value instead of trying to calculate the common shorthand given the
  // longhands.
  CSSPropertyID shorthand_;
  Member<ImmutableCSSPropertyValueSet> properties_;
};

template <>
struct DowncastTraits<CSSKeyframeShorthandValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsShorthandWrapperValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_KEYFRAME_SHORTHAND_VALUE_H_
