// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DRAWING_DISPLAY_ITEM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DRAWING_DISPLAY_ITEM_H_

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_record.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/skia/include/core/SkRefCnt.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

// DrawingDisplayItem contains recorded painting operations which can be
// replayed to produce a rastered output.
class PLATFORM_EXPORT DrawingDisplayItem : public DisplayItem {
 public:
  DISABLE_CFI_PERF
  DrawingDisplayItem(DisplayItemClientId client_id,
                     Type type,
                     const gfx::Rect& visual_rect,
                     PaintRecord record,
                     RasterEffectOutset raster_effect_outset,
                     PaintInvalidationReason paint_invalidation_reason =
                         PaintInvalidationReason::kJustCreated);

  const PaintRecord& GetPaintRecord() const {
    DCHECK(!IsTombstone());
    return record_;
  }

  gfx::Rect RectKnownToBeOpaque() const;

  struct BackgroundColorInfo {
    SkColor4f color = SkColors::kTransparent;
    float area = 0;
    bool is_solid_color = false;
  };
  BackgroundColorInfo BackgroundColor() const;

 private:
  friend class DisplayItem;
  bool EqualsForUnderInvalidationImpl(const DrawingDisplayItem&) const;
#if DCHECK_IS_ON()
  void PropertiesAsJSONImpl(JSONObject&) const {}
#endif

  // Status of RectKnownToBeOpaque(). kOther means recalculation.
  enum class Opaqueness { kOther, kFull, kNone };
  Opaqueness GetOpaqueness() const {
    return static_cast<Opaqueness>(opaqueness_);
  }
  void SetOpaqueness(Opaqueness opaqueness) const {
    opaqueness_ = static_cast<unsigned>(opaqueness);
    DCHECK_EQ(GetOpaqueness(), opaqueness);
  }
  gfx::Rect CalculateRectKnownToBeOpaque() const;
  gfx::Rect CalculateRectKnownToBeOpaqueForRecord(const PaintRecord&) const;

  // Improve the visual rect using the paint record. This can improve solid
  // color analysis in cases when the painted content was snapped but the
  // visual rect was not. Check |ShouldTightenVisualRect| before calling.
  static gfx::Rect TightenVisualRect(const gfx::Rect& visual_rect,
                                     const PaintRecord& record);
  static bool ShouldTightenVisualRect(const PaintRecord& record) {
    // We only have an optimization to tighten the visual rect for a single op.
    return record.size() == 1;
  }

  const PaintRecord record_;
};

// TODO(dcheng): Move this ctor back inline once the clang plugin is fixed.
DISABLE_CFI_PERF
inline DrawingDisplayItem::DrawingDisplayItem(
    DisplayItemClientId client_id,
    Type type,
    const gfx::Rect& visual_rect,
    PaintRecord record,
    RasterEffectOutset raster_effect_outset,
    PaintInvalidationReason paint_invalidation_reason)
    : DisplayItem(
          client_id,
          type,
          [&] {
            if (ShouldTightenVisualRect(record)) [[unlikely]] {
              return TightenVisualRect(visual_rect, record);
            }
            return visual_rect;
          }(),
          raster_effect_outset,
          paint_invalidation_reason,
          /* draws_content*/ !record.empty()),
      record_(std::move(record)) {
  DCHECK(IsDrawing());
  DCHECK_EQ(GetOpaqueness(), Opaqueness::kOther);
}

inline gfx::Rect DrawingDisplayItem::RectKnownToBeOpaque() const {
  if (GetOpaqueness() == Opaqueness::kFull)
    return VisualRect();
  if (GetOpaqueness() == Opaqueness::kNone)
    return gfx::Rect();
  return CalculateRectKnownToBeOpaque();
}

template <>
struct DowncastTraits<DrawingDisplayItem> {
  static bool AllowFrom(const DisplayItem& i) {
    return !i.IsTombstone() && i.IsDrawing();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DRAWING_DISPLAY_ITEM_H_
