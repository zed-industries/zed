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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATED_COLOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATED_COLOR_H_

#include "third_party/blink/renderer/core/css/style_color.h"
#include "third_party/blink/renderer/core/svg/properties/svg_property.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// StyleColor adaptor to SVGPropertyBase. This is only used for SMIL animations.
// FIXME: WebAnimations: Replacable with AnimatableColor once SMIL animations
// are implemented in WebAnimations.
class SVGColorProperty final : public SVGPropertyBase {
 public:
  explicit SVGColorProperty(const WTF::String&);

  void Trace(Visitor* visitor) const override {
    visitor->Trace(style_color_);
    SVGPropertyBase::Trace(visitor);
  }

  WTF::String ValueAsString() const override;

  void Add(const SVGPropertyBase*, const SVGElement*) override;
  void CalculateAnimatedValue(
      const SMILAnimationEffectParameters&,
      float percentage,
      unsigned repeat_count,
      const SVGPropertyBase* from,
      const SVGPropertyBase* to,
      const SVGPropertyBase* to_at_end_of_duration_value,
      const SVGElement*) override;
  float CalculateDistance(const SVGPropertyBase* to,
                          const SVGElement*) const override;

  static AnimatedPropertyType ClassType() { return kAnimatedColor; }
  AnimatedPropertyType GetType() const override { return ClassType(); }

 private:
  StyleColor style_color_;
};

template <>
struct DowncastTraits<SVGColorProperty> {
  static bool AllowFrom(const SVGPropertyBase& value) {
    return value.GetType() == SVGColorProperty::ClassType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATED_COLOR_H_
