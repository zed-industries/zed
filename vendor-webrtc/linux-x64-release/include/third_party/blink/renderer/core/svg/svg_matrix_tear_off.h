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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_MATRIX_TEAR_OFF_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_MATRIX_TEAR_OFF_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/transforms/affine_transform.h"

namespace blink {

class ExceptionState;
class SVGTransformTearOff;

// SVGMatrixTearOff wraps a AffineTransform for Javascript.
// Its instance can either hold a static value, or this can be teared off from
// |SVGTransform.matrix|.  This does not derive from SVGPropertyTearOff, as its
// instances are never tied to an animated property nor an XML attribute.
class CORE_EXPORT SVGMatrixTearOff final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SVGMatrixTearOff(const AffineTransform&);
  explicit SVGMatrixTearOff(SVGTransformTearOff*);

  double a() { return Value().A(); }
  double b() { return Value().B(); }
  double c() { return Value().C(); }
  double d() { return Value().D(); }
  double e() { return Value().E(); }
  double f() { return Value().F(); }

  void setA(double, ExceptionState&);
  void setB(double, ExceptionState&);
  void setC(double, ExceptionState&);
  void setD(double, ExceptionState&);
  void setE(double, ExceptionState&);
  void setF(double, ExceptionState&);

  SVGMatrixTearOff* translate(double tx, double ty);
  SVGMatrixTearOff* scale(double);
  SVGMatrixTearOff* scaleNonUniform(double sx, double sy);
  SVGMatrixTearOff* rotate(double);
  SVGMatrixTearOff* flipX();
  SVGMatrixTearOff* flipY();
  SVGMatrixTearOff* skewX(double);
  SVGMatrixTearOff* skewY(double);
  SVGMatrixTearOff* multiply(SVGMatrixTearOff*);
  SVGMatrixTearOff* inverse(ExceptionState&);
  SVGMatrixTearOff* rotateFromVector(double x, double y, ExceptionState&);

  SVGTransformTearOff* ContextTransform() { return context_transform_.Get(); }

  const AffineTransform& Value() const;

  void Trace(Visitor*) const override;

 private:
  AffineTransform* MutableValue();
  void CommitChange();

  AffineTransform static_value_;

  Member<SVGTransformTearOff> context_transform_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_MATRIX_TEAR_OFF_H_
