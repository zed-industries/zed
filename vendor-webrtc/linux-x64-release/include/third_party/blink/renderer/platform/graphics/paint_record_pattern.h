// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_RECORD_PATTERN_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_RECORD_PATTERN_H_

#include "third_party/blink/renderer/platform/graphics/pattern.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

// TODO(enne): rename this
class PLATFORM_EXPORT PaintRecordPattern final : public Pattern {
 public:
  static scoped_refptr<PaintRecordPattern>
  Create(PaintRecord, const gfx::RectF& record_bounds, RepeatMode);

  ~PaintRecordPattern() override;

 protected:
  sk_sp<PaintShader> CreateShader(const SkMatrix&) const override;

 private:
  PaintRecordPattern(PaintRecord, const gfx::RectF& record_bounds, RepeatMode);

  PaintRecord tile_record_;
  gfx::RectF tile_record_bounds_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_RECORD_PATTERN_H_
