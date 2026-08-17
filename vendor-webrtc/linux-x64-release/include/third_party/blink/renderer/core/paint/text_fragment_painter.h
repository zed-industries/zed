// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TEXT_FRAGMENT_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TEXT_FRAGMENT_PAINTER_H_

#include <optional>

#include "third_party/blink/renderer/core/layout/inline/inline_cursor.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ComputedStyle;
class DisplayItemClient;
class InlineCursor;
class InlinePaintContext;
class LayoutObject;
struct PaintInfo;
struct PhysicalRect;
struct PhysicalSize;
struct TextFragmentPaintInfo;

// Text fragment painter for LayoutNG. Operates on FragmentItem that IsText()
// and handles clipping, selection, etc. Delegates to TextPainter to paint the
// text itself.
class TextFragmentPainter {
  STACK_ALLOCATED();

 public:
  explicit TextFragmentPainter(const InlineCursor& cursor) : cursor_(cursor) {}
  TextFragmentPainter(const InlineCursor& cursor,
                      const PhysicalOffset& parent_offset,
                      InlinePaintContext* inline_context)
      : cursor_(cursor),
        parent_offset_(parent_offset),
        inline_context_(inline_context) {}

  void Paint(const PaintInfo&, const PhysicalOffset& paint_offset);

 private:
  void Paint(const TextFragmentPaintInfo& fragment_paint_info,
             const LayoutObject* layout_object,
             const DisplayItemClient& display_item_client,
             const ComputedStyle& style,
             PhysicalRect box_rect,
             const gfx::Rect& visual_rect,
             bool is_ellipsis,
             bool is_symbol_marker,
             const PaintInfo& paint_info,
             const PhysicalOffset& paint_offset);

  static void PaintSymbol(const LayoutObject* layout_object,
                          const ComputedStyle& style,
                          const PhysicalSize box_size,
                          const PaintInfo& paint_info,
                          const PhysicalOffset& paint_offset);

  const InlineCursor& cursor_;
  PhysicalOffset parent_offset_;
  std::optional<InlineCursor> inline_cursor_for_block_flow_;
  InlinePaintContext* inline_context_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TEXT_FRAGMENT_PAINTER_H_
