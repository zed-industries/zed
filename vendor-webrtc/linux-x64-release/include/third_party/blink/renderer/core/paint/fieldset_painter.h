// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FIELDSET_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FIELDSET_PAINTER_H_

#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class BoxDecorationData;
class PhysicalBoxFragment;
struct FieldsetPaintInfo;
struct PaintInfo;
struct PhysicalRect;

class FieldsetPainter {
  STACK_ALLOCATED();

 public:
  explicit FieldsetPainter(const PhysicalBoxFragment& fieldset)
      : fieldset_(fieldset) {}

  void PaintBoxDecorationBackground(const PaintInfo&,
                                    const PhysicalRect&,
                                    const BoxDecorationData&);
  void PaintMask(const PaintInfo&, const PhysicalOffset&);

 private:
  FieldsetPaintInfo CreateFieldsetPaintInfo() const;

  const PhysicalBoxFragment& fieldset_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FIELDSET_PAINTER_H_
