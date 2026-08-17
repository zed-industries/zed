/*
 * Copyright (C) 2006, 2007, 2008 Apple Computer, Inc.  All rights reserved.
 * Copyright (C) 2007 Alp Toker <alp@atoker.com>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_GRADIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_GRADIENT_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_token.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_color_interpolation_method.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_hue_interpolation_method.h"
#include "third_party/blink/renderer/modules/canvas/canvas2d/identifiability_study_helper.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/graphics/gradient.h"
#include "third_party/blink/renderer/platform/heap/forward.h"  // IWYU pragma: keep (blink::Visitor)
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

// IWYU pragma: no_include "third_party/blink/renderer/platform/heap/visitor.h"

namespace gfx {
class PointF;
}  // namespace gfx

namespace blink {

class ExceptionState;
class ExecutionContext;

class MODULES_EXPORT CanvasGradient final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Linear Gradient
  CanvasGradient(const gfx::PointF& p0, const gfx::PointF& p1);
  // Radial Gradient
  CanvasGradient(const gfx::PointF& p0,
                 float r0,
                 const gfx::PointF& p1,
                 float r1);
  // Conic Gradient
  CanvasGradient(float startAngle, const gfx::PointF& center);

  Gradient* GetGradient() const { return gradient_.get(); }

  void addColorStop(double value, const String& color, ExceptionState&);

  IdentifiableToken GetIdentifiableToken() const;

  // Sets on internal IdentifiabilityStudyHelper.
  void SetExecutionContext(ExecutionContext*);

  void Trace(Visitor* visitor) const override;

  V8ColorInterpolationMethod colorInterpolationMethod() const {
    return color_interpolation_method_;
  }
  void setColorInterpolationMethod(
      const V8ColorInterpolationMethod& color_interpolation_method);

  V8HueInterpolationMethod hueInterpolationMethod() const {
    return hue_interpolation_method_;
  }
  void setHueInterpolationMethod(
      const V8HueInterpolationMethod& hue_interpolation_method);

  bool premultipliedAlpha() const { return premultiplied_alpha_; }
  void setPremultipliedAlpha(bool premultiplied_alpha) {
    premultiplied_alpha_ = premultiplied_alpha;
    gradient_->SetPremultipliedAlphaForInterpolation(premultiplied_alpha);
  }

 private:
  scoped_refptr<Gradient> gradient_;
  IdentifiabilityStudyHelper identifiability_study_helper_;

  V8ColorInterpolationMethod color_interpolation_method_{
      V8ColorInterpolationMethod::Enum::kSRGB};
  V8HueInterpolationMethod hue_interpolation_method_{
      V8HueInterpolationMethod::Enum::kShorter};
  bool premultiplied_alpha_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_GRADIENT_H_
