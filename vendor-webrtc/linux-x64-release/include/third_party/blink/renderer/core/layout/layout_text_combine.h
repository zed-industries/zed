// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_TEXT_COMBINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_TEXT_COMBINE_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_block_flow.h"
#include "third_party/blink/renderer/core/paint/line_relative_rect.h"

namespace blink {

class AffineTransform;
class FragmentItem;
class InlineCursor;
class LayoutText;

// The layout object for the element having "text-combine-upright:all" in
// vertical writing mode, e.g. <i style="text-upright:all"><b>12</b>34<i>.
// Note: When the element is in horizontal writing mode, we don't use this.
// Note: Children of this class must be |LayoutText| associated to |Text| node.
class CORE_EXPORT LayoutTextCombine final : public LayoutBlockFlow {
 public:
  // Note: Mark constructor public for |MakeGarbageCollected|. We should not
  // call this directly.
  LayoutTextCombine();
  ~LayoutTextCombine() override;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(compressed_font_);
    LayoutBlockFlow::Trace(visitor);
  }

  float DesiredWidth() const;
  String GetTextContent() const;

  const Font* CompressedFont() const {
    NOT_DESTROYED();
    return compressed_font_;
  }
  void SetCompressedFont(const Font* font);

  // Scaling

  // Map scaled |offset_in_container| to non-scaled offset if |this| uses
  // scale x, otherwise return |offset_in_container|.
  PhysicalOffset AdjustOffsetForHitTest(
      const PhysicalOffset& offset_in_container) const;

  // Map non-scaled |offset_in_container| to scaled offset if |this| uses
  // scale x, otherwise return |offset_in_container|.
  PhysicalOffset AdjustOffsetForLocalCaretRect(
      const PhysicalOffset& offset_in_container) const;

  // Maps non-scaled |rect| to scaled rect for
  //  * |LayoutText::PhysicalLinesBoundingBox()| used by
  //    |LayoutObject::DebugRect()|, intersection observer, and scroll anchor.
  //  * |FragmentItem::RecalcInkOverflow()| for line box
  //  * |ScrollableOverflowCalculator::AddItemsInternal()| for line box.
  //  * |PhysicalFragment::AddOutlineRectsForCursor()|
  //  * |PhysicalFragment::AddScrollableOverflowForInlineChild()|
  PhysicalRect AdjustRectForBoundingBox(const PhysicalRect& rect) const;

  PhysicalRect ComputeTextBoundsRectForHitTest(
      const FragmentItem& text_item,
      const PhysicalOffset& inline_root_offset) const;

  // Returns ink overflow for text decorations and emphasis mark.
  PhysicalRect RecalcContentsInkOverflow(const InlineCursor&) const;

  void ResetLayout();
  void SetScaleX(float new_scale_x);
  bool UsesScaleX() const {
    NOT_DESTROYED();
    return scale_x_.has_value();
  }

  // Painting
  // |AdjustText{Left,Top}()| are called within affine transformed
  // |GraphicsContext|, e.g. |NGTextFragmentPainter::Paint()|.
  LayoutUnit AdjustTextLeftForPaint(LayoutUnit text_left) const;
  LayoutUnit AdjustTextTopForPaint(LayoutUnit text_top) const;

  AffineTransform ComputeAffineTransformForPaint(
      const PhysicalOffset& paint_offset) const;
  bool NeedsAffineTransformInPaint() const;

  // Returns text frame rect, in logical direction, used with text painters.
  LineRelativeRect ComputeTextFrameRect(
      const PhysicalOffset paint_offset) const;

  // Returns visual rect for painting emphasis mark and text decoration for
  // |BoxFragmentPainter|.
  gfx::Rect VisualRectForPaint(const PhysicalOffset& paint_offset) const;

  static void AssertStyleIsValid(const ComputedStyle& style);

  // Create anonymous wrapper having |text_child|.
  static LayoutTextCombine* CreateAnonymous(LayoutText* text_child);

  // Returns true if |layout_object| is a child of |LayoutTextCombine|.
  static bool ShouldBeParentOf(const LayoutObject& layout_object);

 private:
  bool IsLayoutTextCombine() const final {
    NOT_DESTROYED();
    return true;
  }
  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutTextCombine";
  }

  // Helper functions for scaling.
  PhysicalOffset ApplyScaleX(const PhysicalOffset& offset) const;
  PhysicalRect ApplyScaleX(const PhysicalRect& rect) const;
  PhysicalSize ApplyScaleX(const PhysicalSize& offset) const;
  PhysicalOffset UnapplyScaleX(const PhysicalOffset& offset) const;

  float ComputeInlineSpacing() const;
  bool UsingSyntheticOblique() const;

  // |scale_x_| holds scale factor to width of text content to 1em. When we
  // use |scale_x_|, we use |StyleRef().GetFont()| instead of compressed font.
  std::optional<float> scale_x_;

  // |compressed_font_| hold width variant of |StyleRef().GetFont()|.
  Member<const Font> compressed_font_;
};

// static
inline bool LayoutTextCombine::ShouldBeParentOf(
    const LayoutObject& layout_object) {
  if (layout_object.IsHorizontalWritingMode() || !layout_object.IsText() ||
      layout_object.IsSVGInlineText()) [[likely]] {
    return false;
  }
  if (layout_object.StyleRef().HasTextCombine() &&
      layout_object.IsLayoutNGObject()) [[unlikely]] {
    return true;
  }
  return false;
}

template <>
struct DowncastTraits<LayoutTextCombine> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutTextCombine();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_TEXT_COMBINE_H_
