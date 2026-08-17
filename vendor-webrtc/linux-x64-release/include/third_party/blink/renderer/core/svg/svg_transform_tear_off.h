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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TRANSFORM_TEAR_OFF_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TRANSFORM_TEAR_OFF_H_

#include "third_party/blink/renderer/core/svg/properties/svg_property_tear_off.h"
#include "third_party/blink/renderer/core/svg/svg_transform.h"

namespace blink {

class SVGMatrixTearOff;

class SVGTransformTearOff final : public SVGPropertyTearOff<SVGTransform> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum SVGTransformType {
    kSvgTransformUnknown = static_cast<int>(blink::SVGTransformType::kUnknown),
    kSvgTransformMatrix = static_cast<int>(blink::SVGTransformType::kMatrix),
    kSvgTransformTranslate =
        static_cast<int>(blink::SVGTransformType::kTranslate),
    kSvgTransformScale = static_cast<int>(blink::SVGTransformType::kScale),
    kSvgTransformRotate = static_cast<int>(blink::SVGTransformType::kRotate),
    kSvgTransformSkewx = static_cast<int>(blink::SVGTransformType::kSkewx),
    kSvgTransformSkewy = static_cast<int>(blink::SVGTransformType::kSkewy),
  };

  static SVGTransformTearOff* CreateDetached();

  SVGTransformTearOff(SVGMatrixTearOff*);
  SVGTransformTearOff(SVGTransform*,
                      SVGAnimatedPropertyBase* binding,
                      PropertyIsAnimValType);
  ~SVGTransformTearOff() override;

  uint16_t transformType() {
    return static_cast<uint16_t>(Target()->TransformType());
  }
  SVGMatrixTearOff* matrix();
  float angle() { return Target()->Angle(); }

  void setMatrix(SVGMatrixTearOff*, ExceptionState&);
  void setTranslate(float tx, float ty, ExceptionState&);
  void setScale(float sx, float sy, ExceptionState&);
  void setRotate(float angle, float cx, float cy, ExceptionState&);
  void setSkewX(float, ExceptionState&);
  void setSkewY(float, ExceptionState&);

  void Trace(Visitor*) const override;

 private:
  Member<SVGMatrixTearOff> matrix_tearoff_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TRANSFORM_TEAR_OFF_H_
