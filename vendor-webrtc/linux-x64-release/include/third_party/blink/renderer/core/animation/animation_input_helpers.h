// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATION_INPUT_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATION_INPUT_HELPERS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class Document;
class ExceptionState;
class PropertyHandle;
class TimingFunction;

class CORE_EXPORT AnimationInputHelpers {
  STATIC_ONLY(AnimationInputHelpers);

 public:
  static CSSPropertyID KeyframeAttributeToCSSProperty(const WTF::String&,
                                                      const Document&);
  static scoped_refptr<TimingFunction> ParseTimingFunction(const WTF::String&,
                                                           Document*,
                                                           ExceptionState&);

  static WTF::String PropertyHandleToKeyframeAttribute(PropertyHandle);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATION_INPUT_HELPERS_H_
