// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_MATHML_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_MATHML_PAINTER_H_

#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class PhysicalBoxFragment;
struct PaintInfo;
struct PhysicalRect;

class MathMLPainter {
  STACK_ALLOCATED();

 public:
  explicit MathMLPainter(const PhysicalBoxFragment& box_fragment)
      : box_fragment_(box_fragment) {}
  void Paint(const PaintInfo&, PhysicalOffset);

 private:
  void PaintBar(const PaintInfo&, const PhysicalRect&);
  void PaintFractionBar(const PaintInfo&, PhysicalOffset);
  void PaintOperator(const PaintInfo&, PhysicalOffset);
  void PaintRadicalSymbol(const PaintInfo&, PhysicalOffset);
  void PaintStretchyOrLargeOperator(const PaintInfo&, PhysicalOffset);

  const PhysicalBoxFragment& box_fragment_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_MATHML_PAINTER_H_
