/*
 * Copyright (C) 2014 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TRANSFORM_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TRANSFORM_LIST_H_

#include "third_party/blink/renderer/core/svg/properties/svg_list_property_helper.h"
#include "third_party/blink/renderer/core/svg/svg_parsing_error.h"
#include "third_party/blink/renderer/core/svg/svg_transform.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSValue;
class SVGTransformListTearOff;

class SVGTransformList final
    : public SVGListPropertyHelper<SVGTransformList, SVGTransform> {
 public:
  typedef SVGTransformListTearOff TearOffType;

  SVGTransformList();
  SVGTransformList(SVGTransformType, const String&);
  ~SVGTransformList() override;

  AffineTransform Concatenate() const;

  SVGParsingError SetValueAsString(const String&);
  bool Parse(const UChar*& ptr, const UChar* end);
  bool Parse(const LChar*& ptr, const LChar* end);

  // SVGPropertyBase:
  void Add(const SVGPropertyBase*, const SVGElement*) override;
  void CalculateAnimatedValue(
      const SMILAnimationEffectParameters&,
      float percentage,
      unsigned repeat_count,
      const SVGPropertyBase* from_value,
      const SVGPropertyBase* to_value,
      const SVGPropertyBase* to_at_end_of_duration_value,
      const SVGElement*) override;
  float CalculateDistance(const SVGPropertyBase* to,
                          const SVGElement*) const override;

  static AnimatedPropertyType ClassType() { return kAnimatedTransformList; }

  const CSSValue* CssValue() const;

 private:
  template <typename CharType>
  SVGParsingError ParseInternal(const CharType*& ptr, const CharType* end);
};

template <>
struct DowncastTraits<SVGTransformList> {
  static bool AllowFrom(const SVGPropertyBase& value) {
    return value.GetType() == SVGTransformList::ClassType();
  }
};

SVGTransformType ParseTransformType(const String&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TRANSFORM_LIST_H_
