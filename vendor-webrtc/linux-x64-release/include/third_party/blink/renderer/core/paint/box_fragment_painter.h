// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_FRAGMENT_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_FRAGMENT_PAINTER_H_

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/background_bleed_avoidance.h"
#include "third_party/blink/renderer/core/layout/hit_test_phase.h"
#include "third_party/blink/renderer/core/layout/inline/inline_cursor.h"
#include "third_party/blink/renderer/core/layout/physical_box_fragment.h"
#include "third_party/blink/renderer/core/paint/box_painter_base.h"
#include "third_party/blink/renderer/core/paint/inline_paint_context.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class BoxDecorationData;
class FillLayer;
class FragmentItems;
class HitTestLocation;
class HitTestResult;
class InlineBackwardCursor;
class InlineBoxFragmentPainter;
class InlineCursor;
class PhysicalFragment;
class ScopedPaintState;
struct PaintInfo;

// Painter for LayoutNG box fragments, paints borders and background. Delegates
// to TextFragmentPainter to paint line box fragments.
class CORE_EXPORT BoxFragmentPainter : public BoxPainterBase {
  STACK_ALLOCATED();

 public:
  explicit BoxFragmentPainter(const PhysicalBoxFragment&);
  // Construct for an inline box.
  BoxFragmentPainter(const InlineCursor& inline_box_cursor,
                     const FragmentItem& item,
                     const PhysicalBoxFragment& fragment,
                     InlinePaintContext* inline_context);

  void Paint(const PaintInfo&);
  // Routes single PaintPhase to actual painters, and traverses children.
  void PaintObject(const PaintInfo&,
                   const PhysicalOffset&,
                   bool suppress_box_decoration_background = false);

  // Hit tests this box fragment.
  // @param physical_offset Physical offset of this box fragment in the
  // coordinate space of |hit_test_location|.
  // TODO(eae): Change to take a HitTestResult pointer instead as it mutates.
  bool NodeAtPoint(HitTestResult&,
                   const HitTestLocation& hit_test_location,
                   const PhysicalOffset& physical_offset,
                   HitTestPhase);
  bool NodeAtPoint(HitTestResult&,
                   const HitTestLocation& hit_test_location,
                   const PhysicalOffset& physical_offset,
                   const PhysicalOffset& inline_root_offset,
                   HitTestPhase);

  bool HitTestAllPhases(HitTestResult&,
                        const HitTestLocation&,
                        const PhysicalOffset& accumulated_offset);

  void PaintBoxDecorationBackgroundWithRectImpl(const PaintInfo&,
                                                const PhysicalRect&,
                                                const BoxDecorationData&);

  gfx::Rect VisualRect(const PhysicalOffset& paint_offset);

 protected:
  BoxPainterBase::FillLayerInfo GetFillLayerInfo(
      const Color&,
      const FillLayer&,
      BackgroundBleedAvoidance,
      bool is_painting_background_in_contents_space,
      PaintFlags paint_flags) const override;

  void PaintTextClipMask(const PaintInfo&,
                         const gfx::Rect& mask_rect,
                         const PhysicalOffset& paint_offset,
                         bool object_has_multiple_boxes) override;
  void PaintTextClipMask(const PaintInfo& paint_info,
                         PhysicalOffset paint_offset,
                         InlineBoxFragmentPainter* inline_box_painter);
  PhysicalRect AdjustRectForScrolledContent(GraphicsContext&,
                                            const PhysicalBoxStrut& borders,
                                            const PhysicalRect&) const override;

 private:
  BoxFragmentPainter(const PhysicalBoxFragment&,
                     const DisplayItemClient& display_item_client,
                     const InlineCursor* inline_box_cursor = nullptr,
                     const FragmentItem* box_item = nullptr,
                     InlinePaintContext* inline_context = nullptr);

  enum MoveTo { kDontSkipChildren, kSkipChildren };
  bool ShouldPaint(const ScopedPaintState&) const;

  void PaintBoxDecorationBackground(const PaintInfo&,
                                    const PhysicalOffset& paint_offset,
                                    bool suppress_box_decoration_background);

  // |visual_rect| is for the drawing display item, covering overflowing box
  // shadows and border image outsets. |paint_rect| is the border box rect in
  // paint coordinates.
  void PaintBoxDecorationBackgroundWithRect(const PaintInfo&,
                                            const gfx::Rect& visual_rect,
                                            const PhysicalRect& paint_rect,
                                            const DisplayItemClient&);
  void PaintCompositeBackgroundAttachmentFixed(const PaintInfo&,
                                               const DisplayItemClient&,
                                               const BoxDecorationData&);
  void PaintBoxDecorationBackgroundWithDecorationData(
      const PaintInfo&,
      const gfx::Rect& visual_rect,
      const PhysicalRect& paint_rect,
      const DisplayItemClient&,
      DisplayItem::Type,
      const BoxDecorationData&);

  void PaintBoxDecorationBackgroundForBlockInInline(
      InlineCursor* children,
      const PaintInfo&,
      const PhysicalOffset& paint_offset);

  void PaintColumnRules(const PaintInfo&, const PhysicalOffset& paint_offset);

  void PaintInternal(const PaintInfo&);
  void PaintAllPhasesAtomically(const PaintInfo&);
  void PaintCurrentPageContainer(const PaintInfo&);
  void PaintBlockChildren(const PaintInfo&, PhysicalOffset);
  void PaintBlockChild(const PhysicalFragmentLink& child,
                       const PaintInfo& paint_info,
                       const PaintInfo& paint_info_for_descendants,
                       PhysicalOffset paint_offset);
  void PaintInlineItems(const PaintInfo&,
                        const PhysicalOffset& paint_offset,
                        const PhysicalOffset& parent_offset,
                        InlineCursor* cursor);
  void PaintLineBoxes(const PaintInfo&, const PhysicalOffset& paint_offset);
  void PaintLineBoxChildItems(InlineCursor* children,
                              const PaintInfo&,
                              const PhysicalOffset& paint_offset);
  void PaintLineBox(const PhysicalFragment& line_box_fragment,
                    const DisplayItemClient& display_item_client,
                    const FragmentItem& line_box_item,
                    const PaintInfo&,
                    const PhysicalOffset& paint_offset);
  void PaintBackplate(InlineCursor* descendants,
                      const PaintInfo&,
                      const PhysicalOffset& paint_offset);
  void PaintTextItem(const InlineCursor& cursor,
                     const PaintInfo&,
                     const PhysicalOffset& paint_offset,
                     const PhysicalOffset& parent_offset);
  void PaintBoxItem(const FragmentItem& item,
                    const PhysicalBoxFragment& child_fragment,
                    const InlineCursor& cursor,
                    const PaintInfo& paint_info,
                    const PhysicalOffset& paint_offset);
  void PaintBoxItem(const FragmentItem& item,
                    const InlineCursor& cursor,
                    const PaintInfo& paint_info,
                    const PhysicalOffset& paint_offset,
                    const PhysicalOffset& parent_offset);
  void PaintFloatingItems(const PaintInfo& paint_info, InlineCursor* cursor);
  void PaintFloatingChildren(const PhysicalFragment&,
                             const PaintInfo& paint_info);
  void PaintFloats(const PaintInfo&);
  void PaintMask(const PaintInfo&, const PhysicalOffset& paint_offset);
  void PaintBackground(const PaintInfo&,
                       const PhysicalRect&,
                       const Color& background_color,
                       BackgroundBleedAvoidance = kBackgroundBleedNone);
  void PaintCaretsIfNeeded(const ScopedPaintState&,
                           const PaintInfo&,
                           const PhysicalOffset& paint_offset);
  bool PaintOverflowControls(const PaintInfo&,
                             const PhysicalOffset& paint_offset);
  void PaintGapDecorations(const PaintInfo&, const PhysicalRect& paint_rect);
  void PaintGaps(GridTrackSizingDirection track_direction,
                 const PaintInfo& paint_info,
                 const PhysicalRect& paint_rect,
                 const GapGeometry& gap_geometry);

  InlinePaintContext& EnsureInlineContext();

  // This should be called in the background paint phase even if there is no
  // other painted content.
  void RecordScrollHitTestData(const PaintInfo&,
                               const DisplayItemClient& background_client);

  bool ShouldRecordHitTestData(const PaintInfo&);

  // This struct has common data needed while traversing trees for the hit
  // testing.
  struct HitTestContext {
    STACK_ALLOCATED();

   public:
    // Add |node| to |HitTestResult|. Returns true if the hit-testing should
    // stop.
    // T is PhysicalRect or gfx::QuadF.
    template <typename T>
    bool AddNodeToResult(Node* node,
                         const PhysicalBoxFragment* box_fragment,
                         const T& bounds_rect,
                         const PhysicalOffset& offset) const;
    // Same as |AddNodeToResult|, except that |offset| is in the content
    // coordinate system rather than the container coordinate system. They
    // differ when |container| is a scroll container.
    // T is PhysicalRect or gfx::QuadF.
    template <typename T>
    bool AddNodeToResultWithContentOffset(Node* node,
                                          const PhysicalBoxFragment& container,
                                          const T& bounds_rect,
                                          PhysicalOffset offset) const;

    HitTestPhase phase;
    const HitTestLocation& location;
    // When traversing within an inline formatting context, this member
    // represents the offset of the root of the inline formatting context.
    PhysicalOffset inline_root_offset;
    // The result is set to this member, but its address does not change during
    // the traversal.
    HitTestResult* result;
  };

  // Hit tests the children of a container fragment, which is either
  // |box_fragment_|, or one of its child line box fragments.
  // @param physical_offset Physical offset of the container fragment's content
  // box in paint layer. Note that this includes scrolling offset when the
  // container has 'overflow: scroll'.
  bool NodeAtPoint(const HitTestContext& hit_test,
                   const PhysicalOffset& physical_offset);
  bool HitTestChildren(const HitTestContext& hit_test,
                       const PhysicalOffset& physical_offset);
  bool HitTestChildren(const HitTestContext& hit_test,
                       const PhysicalBoxFragment& container,
                       const InlineCursor& children,
                       const PhysicalOffset& physical_offset);
  bool HitTestBlockChildren(HitTestResult&,
                            const HitTestLocation&,
                            PhysicalOffset,
                            HitTestPhase);
  bool HitTestItemsChildren(const HitTestContext& hit_test,
                            const PhysicalBoxFragment& container,
                            const InlineCursor& children);
  bool HitTestFloatingChildren(const HitTestContext& hit_test,
                               const PhysicalFragment& container,
                               const PhysicalOffset& accumulated_offset);
  bool HitTestFloatingChildItems(const HitTestContext& hit_test,
                                 const InlineCursor& children,
                                 const PhysicalOffset& accumulated_offset);

  // Hit tests a child inline box fragment.
  // @param physical_offset Physical offset of the given box fragment in the
  // paint layer.
  bool HitTestInlineChildBoxFragment(const HitTestContext& hit_test,
                                     const PhysicalBoxFragment& fragment,
                                     const InlineBackwardCursor& cursor,
                                     const PhysicalOffset& physical_offset);
  bool HitTestChildBoxItem(const HitTestContext& hit_test,
                           const PhysicalBoxFragment& container,
                           const FragmentItem& item,
                           const InlineBackwardCursor& cursor);

  // Hit tests the given text fragment.
  // @param physical_offset Physical offset of the text fragment in paint layer.
  bool HitTestTextFragment(const HitTestContext& hit_test,
                           const InlineBackwardCursor& cursor,
                           const PhysicalOffset& physical_offset);
  bool HitTestTextItem(const HitTestContext& hit_test,
                       const FragmentItem& text_item,
                       const InlineBackwardCursor& cursor);

  // Hit tests the given line box fragment.
  // @param physical_offset Physical offset of the line box fragment in paint
  // layer.
  bool HitTestLineBoxFragment(const HitTestContext& hit_test,
                              const PhysicalLineBoxFragment& fragment,
                              const InlineBackwardCursor& cursor,
                              const PhysicalOffset& physical_offset);

  // Returns whether the hit test location is completely outside the border box,
  // which possibly has rounded corners.
  bool HitTestClippedOutByBorder(
      const HitTestLocation&,
      const PhysicalOffset& border_box_location) const;

  bool HitTestOverflowControl(const HitTestContext&,
                              PhysicalOffset accumulated_offset);

  bool UpdateHitTestResultForView(const PhysicalRect& bounds_rect,
                                  const HitTestContext& hit_test) const;

  const PhysicalBoxFragment& GetPhysicalFragment() const {
    return box_fragment_;
  }
  const DisplayItemClient& GetDisplayItemClient() const {
    return display_item_client_;
  }
  PhysicalRect InkOverflowIncludingFilters() const;

  static bool ShouldHitTestCulledInlineAncestors(const HitTestContext& hit_test,
                                                 const FragmentItem& item);

  const PhysicalBoxFragment& box_fragment_;
  const DisplayItemClient& display_item_client_;
  const FragmentItems* items_ = nullptr;
  const FragmentItem* box_item_ = nullptr;
  const InlineCursor* inline_box_cursor_ = nullptr;
  InlinePaintContext* inline_context_ = nullptr;
  std::optional<InlinePaintContext> inline_context_storage_;
};

inline BoxFragmentPainter::BoxFragmentPainter(
    const PhysicalBoxFragment& box,
    const DisplayItemClient& display_item_client,
    const InlineCursor* inline_box_cursor,
    const FragmentItem* box_item,
    InlinePaintContext* inline_context)
    : BoxPainterBase(box.GetDocument(), box.Style(), box.GetNode()),
      box_fragment_(box),
      display_item_client_(display_item_client),
      items_(box.Items()),
      box_item_(box_item),
      inline_box_cursor_(inline_box_cursor),
      inline_context_(inline_context) {
  DCHECK(box.IsBox() || box.IsRenderedLegend());
  DCHECK_EQ(box.PostLayout(), &box);
#if DCHECK_IS_ON()
  if (inline_box_cursor_)
    DCHECK_EQ(inline_box_cursor_->Current().Item(), box_item_);
  if (box_item_)
    DCHECK_EQ(box_item_->BoxFragment(), &box);
#endif
}

inline BoxFragmentPainter::BoxFragmentPainter(
    const PhysicalBoxFragment& fragment)
    : BoxFragmentPainter(fragment,
                         *fragment.GetLayoutObject(),
                         /* inline_box_cursor */ nullptr,
                         /* box_item */ nullptr,
                         /* inline_context */ nullptr) {}

inline BoxFragmentPainter::BoxFragmentPainter(
    const InlineCursor& inline_box_cursor,
    const FragmentItem& item,
    const PhysicalBoxFragment& fragment,
    InlinePaintContext* inline_context)
    : BoxFragmentPainter(fragment,
                         *item.GetDisplayItemClient(),
                         &inline_box_cursor,
                         &item,
                         inline_context) {
  DCHECK_EQ(item.BoxFragment(), &fragment);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_FRAGMENT_PAINTER_H_
