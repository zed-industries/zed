// Copyright (C) 2013 Google Inc. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_STROKE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_STROKE_DATA_H_

#include "cc/paint/paint_flags.h"
#include "third_party/blink/renderer/platform/geometry/dash_array.h"
#include "third_party/blink/renderer/platform/geometry/path_types.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/skia/include/core/SkRefCnt.h"

namespace blink {

// Stroke geometry information.
//
// Closely matches what the lower layers (cc::PaintFlags) can handle, and adds
// no additional semantics (like StyledStrokeData does by handling StrokeStyle).
class PLATFORM_EXPORT StrokeData final {
  DISALLOW_NEW();

 public:
  float Thickness() const { return thickness_; }
  void SetThickness(float thickness) { thickness_ = thickness; }

  void SetLineCap(LineCap cap) {
    line_cap_ = static_cast<cc::PaintFlags::Cap>(cap);
  }

  void SetLineJoin(LineJoin join) {
    line_join_ = static_cast<cc::PaintFlags::Join>(join);
  }

  float MiterLimit() const { return miter_limit_; }
  void SetMiterLimit(float miter_limit) { miter_limit_ = miter_limit; }

  void SetLineDash(const DashArray&, float);
  void SetDashEffect(sk_sp<cc::PathEffect> dash_effect);

  // Sets everything on the paint except the pattern, gradient and color.
  void SetupPaint(cc::PaintFlags*) const;

 private:
  float thickness_ = 0;
  cc::PaintFlags::Cap line_cap_ = cc::PaintFlags::kDefault_Cap;
  cc::PaintFlags::Join line_join_ = cc::PaintFlags::kDefault_Join;
  float miter_limit_ = 4;
  sk_sp<cc::PathEffect> dash_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_STROKE_DATA_H_
