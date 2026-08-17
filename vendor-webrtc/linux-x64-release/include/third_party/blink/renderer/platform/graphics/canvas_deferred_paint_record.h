// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CANVAS_DEFERRED_PAINT_RECORD_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CANVAS_DEFERRED_PAINT_RECORD_H_

#include "cc/paint/deferred_paint_record.h"
#include "cc/paint/paint_record.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/gfx/geometry/size_f.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

class PLATFORM_EXPORT CanvasDeferredPaintRecord
    : public cc::DeferredPaintRecord {
 public:
  CanvasDeferredPaintRecord();
  void SetPaintRecord(cc::PaintRecord, gfx::SizeF);
  void Clear();
  cc::PaintRecord GetPaintRecord() const { return paint_record_; }
  gfx::Transform GetTransform() const { return transform_; }
  void SetTransform(gfx::Transform transform) { transform_ = transform; }
  void SetIsDirty(bool is_dirty) { is_dirty_ = is_dirty; }
  bool IsDirty() const { return is_dirty_; }

  gfx::SizeF GetSize() const override;

 protected:
  ~CanvasDeferredPaintRecord() override;

 private:
  gfx::Transform transform_;
  gfx::SizeF size_{0, 0};
  cc::PaintRecord paint_record_;
  bool is_dirty_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CANVAS_DEFERRED_PAINT_RECORD_H_
