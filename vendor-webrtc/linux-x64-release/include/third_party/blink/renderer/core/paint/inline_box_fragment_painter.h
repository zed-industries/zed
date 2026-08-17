// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_INLINE_BOX_FRAGMENT_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_INLINE_BOX_FRAGMENT_PAINTER_H_

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/layout/inline/inline_cursor.h"
#include "third_party/blink/renderer/core/layout/inline/physical_line_box_fragment.h"
#include "third_party/blink/renderer/core/layout/layout_object_inlines.h"
#include "third_party/blink/renderer/core/paint/box_fragment_painter.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class InlinePaintContext;
struct PaintInfo;
struct PhysicalRect;

// Common base class for InlineBoxFragmentPainter and
// LineBoxFragmentPainter.
class InlineBoxFragmentPainterBase {
  STACK_ALLOCATED();

 public:
  void ComputeFragmentOffsetOnLine(TextDirection,
                                   LayoutUnit* offset_on_line,
                                   LayoutUnit* total_width) const;

 protected:
  InlineBoxFragmentPainterBase(const PhysicalFragment& inline_box_fragment,
                               const InlineCursor* inline_box_cursor,
                               const FragmentItem& inline_box_item,
                               const LayoutObject& layout_object,
                               const ComputedStyle& style,
                               const ComputedStyle& line_style,
                               InlinePaintContext* inline_context)
      : image_observer_(layout_object),
        document_(&layout_object.GetDocument()),
        node_(layout_object.GeneratingNode()),
        style_(style),
        line_style_(line_style),
        inline_box_fragment_(inline_box_fragment),
        inline_box_item_(inline_box_item),
        inline_box_cursor_(inline_box_cursor),
        inline_context_(inline_context) {
#if DCHECK_IS_ON()
    if (inline_box_cursor)
      DCHECK_EQ(inline_box_cursor->Current().Item(), &inline_box_item);
    if (inline_box_item.BoxFragment())
      DCHECK_EQ(inline_box_item.BoxFragment(), &inline_box_fragment);
    else
      DCHECK_EQ(inline_box_item.LineBoxFragment(), &inline_box_fragment);
#endif
  }

  // Constructor for |FragmentItem|.
  InlineBoxFragmentPainterBase(const InlineCursor& inline_box_cursor,
                               const FragmentItem& inline_box_item,
                               const PhysicalBoxFragment& inline_box_fragment,
                               const LayoutObject& layout_object,
                               const ComputedStyle& style,
                               const ComputedStyle& line_style,
                               InlinePaintContext* inline_context)
      : InlineBoxFragmentPainterBase(inline_box_fragment,
                                     &inline_box_cursor,
                                     inline_box_item,
                                     layout_object,
                                     style,
                                     line_style,
                                     inline_context) {}

  const DisplayItemClient& GetDisplayItemClient() const {
    DCHECK(inline_box_item_.GetDisplayItemClient());
    return *inline_box_item_.GetDisplayItemClient();
  }

  virtual PhysicalBoxSides SidesToInclude() const = 0;

  PhysicalRect PaintRectForImageStrip(const PhysicalRect&,
                                      TextDirection direction) const;
  static PhysicalRect ClipRectForNinePieceImageStrip(
      const ComputedStyle& style,
      PhysicalBoxSides sides_to_include,
      const NinePieceImage& image,
      const PhysicalRect& paint_rect);

  enum SlicePaintingType {
    kDontPaint,
    kPaintWithoutClip,
    kPaintWithClip,
  };
  SlicePaintingType GetBorderPaintType(const PhysicalRect& adjusted_frame_rect,
                                       gfx::Rect& adjusted_clip_rect,
                                       bool object_has_multiple_boxes) const;
  SlicePaintingType GetSlicePaintType(const NinePieceImage&,
                                      const PhysicalRect& adjusted_frame_rect,
                                      gfx::Rect& adjusted_clip_rect,
                                      bool object_has_multiple_boxes) const;

  void PaintNormalBoxShadow(const PaintInfo&,
                            const ComputedStyle&,
                            const PhysicalRect& paint_rect);
  void PaintInsetBoxShadow(const PaintInfo&,
                           const ComputedStyle&,
                           const PhysicalRect& paint_rect);

  void PaintBackgroundBorderShadow(const PaintInfo&,
                                   const PhysicalOffset& paint_offset);

  void PaintBoxDecorationBackground(BoxPainterBase&,
                                    const PaintInfo&,
                                    const PhysicalOffset& paint_offset,
                                    const PhysicalRect& adjusted_frame_rect,
                                    const BoxBackgroundPaintContext&,
                                    bool object_has_multiple_boxes,
                                    PhysicalBoxSides sides_to_include);

  void PaintFillLayers(BoxPainterBase&,
                       const PaintInfo&,
                       const Color&,
                       const FillLayer&,
                       const PhysicalRect&,
                       const BoxBackgroundPaintContext&,
                       bool object_has_multiple_boxes);
  void PaintFillLayer(BoxPainterBase&,
                      const PaintInfo&,
                      const Color&,
                      const FillLayer&,
                      const PhysicalRect&,
                      const BoxBackgroundPaintContext&,
                      bool object_has_multiple_boxes);

  gfx::Rect VisualRect(const PhysicalOffset& paint_offset);

  const ImageResourceObserver& image_observer_;
  const Document* document_;
  Node* node_;

  // Style for the corresponding node.
  const ComputedStyle& style_;

  // Style taking ::first-line into account.
  const ComputedStyle& line_style_;

  const PhysicalFragment& inline_box_fragment_;
  const FragmentItem& inline_box_item_;
  const InlineCursor* inline_box_cursor_ = nullptr;
  InlinePaintContext* inline_context_ = nullptr;
};

// Painter for LayoutNG inline box fragments. Delegates to BoxFragmentPainter
// for all box painting logic that isn't specific to inline boxes.
class InlineBoxFragmentPainter : public InlineBoxFragmentPainterBase {
  STACK_ALLOCATED();

 public:
  // Constructor for |FragmentItem|.
  InlineBoxFragmentPainter(const InlineCursor& inline_box_cursor,
                           const FragmentItem& inline_box_item,
                           const PhysicalBoxFragment& inline_box_fragment,
                           InlinePaintContext* inline_context)
      : InlineBoxFragmentPainterBase(inline_box_cursor,
                                     inline_box_item,
                                     inline_box_fragment,
                                     *inline_box_fragment.GetLayoutObject(),
                                     inline_box_fragment.Style(),
                                     inline_box_fragment.Style(),
                                     inline_context) {
    CheckValid();
  }
  InlineBoxFragmentPainter(const InlineCursor& inline_box_cursor,
                           const FragmentItem& inline_box_item,
                           InlinePaintContext* inline_context)
      : InlineBoxFragmentPainter(inline_box_cursor,
                                 inline_box_item,
                                 *inline_box_item.BoxFragment(),
                                 inline_context) {
    DCHECK(inline_box_item.BoxFragment());
  }

  void Paint(const PaintInfo&, const PhysicalOffset& paint_offset);

  // Paint all fragments generated for the inline within the given block
  // container, specified as a fragment data index. The index is relative to the
  // first block fragment where the inline occurs.
  //
  // TODO(crbug.com/1478119): If looking up a FragmentData were O(1) instead of
  // O(n), there should be no need to pass both FragmentData and the index.
  static void PaintAllFragments(const LayoutInline& layout_inline,
                                const FragmentData& fragment_data,
                                wtf_size_t fragment_data_idx,
                                const PaintInfo&);

 private:
  void PaintMask(const PaintInfo&, const PhysicalOffset& paint_offset);

  const PhysicalBoxFragment& BoxFragment() const {
    return static_cast<const PhysicalBoxFragment&>(inline_box_fragment_);
  }

  PhysicalBoxSides SidesToInclude() const final;

#if DCHECK_IS_ON()
  void CheckValid() const;
#else
  void CheckValid() const {}
#endif
};

// Painter for LayoutNG line box fragments. Line boxes don't paint anything,
// except when ::first-line style has background properties specified.
// https://drafts.csswg.org/css-pseudo-4/#first-line-styling
class LineBoxFragmentPainter : public InlineBoxFragmentPainterBase {
  STACK_ALLOCATED();

 public:
  LineBoxFragmentPainter(const PhysicalFragment& line_box_fragment,
                         const FragmentItem& line_box_item,
                         const PhysicalBoxFragment& block_fragment)
      : LineBoxFragmentPainter(line_box_fragment,
                               line_box_item,
                               block_fragment,
                               *block_fragment.GetLayoutObject()) {}

  static bool NeedsPaint(const PhysicalFragment& line_fragment) {
    DCHECK(line_fragment.IsLineBox());
    return line_fragment.UsesFirstLineStyle();
  }

  // Borders are not part of ::first-line style and therefore not painted, but
  // the function name is kept consistent with other classes.
  void PaintBackgroundBorderShadow(const PaintInfo&,
                                   const PhysicalOffset& paint_offset);

 private:
  LineBoxFragmentPainter(const PhysicalFragment& line_box_fragment,
                         const FragmentItem& line_box_item,
                         const PhysicalBoxFragment& block_fragment,
                         const LayoutObject& layout_block_flow)
      : InlineBoxFragmentPainterBase(
            line_box_fragment,
            /* inline_box_cursor */ nullptr,
            line_box_item,
            layout_block_flow,
            // Use the style from the containing block. |line_fragment.Style()|
            // is a copy at the time of the last layout to reflect the line
            // direction, and its paint properties may have been changed.
            // TODO(kojii): Reconsider |line_fragment.Style()|.
            layout_block_flow.StyleRef(),
            layout_block_flow.FirstLineStyleRef(),
            /* inline_context */ nullptr),
        block_fragment_(block_fragment) {
    DCHECK(line_box_fragment.IsLineBox());
    DCHECK(NeedsPaint(line_box_fragment));
    DCHECK(layout_block_flow.IsLayoutNGObject());
  }

  const PhysicalLineBoxFragment& LineBoxFragment() const {
    return static_cast<const PhysicalLineBoxFragment&>(inline_box_fragment_);
  }

  PhysicalBoxSides SidesToInclude() const final { return PhysicalBoxSides(); }

  const PhysicalBoxFragment& block_fragment_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_INLINE_BOX_FRAGMENT_PAINTER_H_
