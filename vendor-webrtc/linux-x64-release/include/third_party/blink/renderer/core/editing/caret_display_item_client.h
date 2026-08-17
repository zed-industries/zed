/*
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010 Apple Inc. All rights
 * reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_CARET_DISPLAY_ITEM_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_CARET_DISPLAY_ITEM_CLIENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "ui/gfx/selection_bound.h"

namespace blink {

class GraphicsContext;
class LayoutBlock;
class PhysicalBoxFragment;
struct PaintInvalidatorContext;

class CORE_EXPORT CaretDisplayItemClient final
    : public GarbageCollected<CaretDisplayItemClient>,
      public DisplayItemClient {
 public:
  CaretDisplayItemClient();
  CaretDisplayItemClient(const CaretDisplayItemClient&) = delete;
  CaretDisplayItemClient& operator=(const CaretDisplayItemClient&) = delete;
  ~CaretDisplayItemClient() override;
  void Trace(Visitor* visitor) const override;

  // Called indirectly from LayoutBlock::willBeDestroyed().
  void LayoutBlockWillBeDestroyed(const LayoutBlock&);

  // Called when a FrameView finishes layout. Updates style and geometry of the
  // caret for paint invalidation and painting.
  void UpdateStyleAndLayoutIfNeeded(const PositionWithAffinity& caret_position);

  void SetActive(bool active);

  // Called during LayoutBlock paint invalidation.
  void InvalidatePaint(const LayoutBlock&, const PaintInvalidatorContext&);

  // Called during pre-paint tree walk to invalidate |previous_layout_block_|.
  void EnsureInvalidationOfPreviousLayoutBlock();

  bool ShouldPaintCaret(const LayoutBlock& block) const {
    return &block == layout_block_;
  }

  bool ShouldPaintCaret(const PhysicalBoxFragment& box_fragment) const;

  void PaintCaret(GraphicsContext&,
                  const PhysicalOffset& paint_offset,
                  DisplayItem::Type) const;

  void RecordSelection(GraphicsContext&,
                       const PhysicalOffset& paint_offset,
                       gfx::SelectionBound::Type type);

  // DisplayItemClient.
  String DebugName() const final;

 private:
  friend class CaretDisplayItemClientTest;
  friend class ComputeCaretRectTest;

  struct CaretRectAndPainterBlock {
    STACK_ALLOCATED();

   public:
    PhysicalRect caret_rect;  // local to |painter_block|
    LayoutBlock* painter_block = nullptr;
    const PhysicalBoxFragment* box_fragment = nullptr;
  };
  // Creating VisiblePosition causes synchronous layout so we should use the
  // PositionWithAffinity version if possible.
  // A position in HTMLTextFromControlElement is a typical example.
  static CaretRectAndPainterBlock ComputeCaretRectAndPainterBlock(
      const PositionWithAffinity& caret_position);

  void InvalidatePaintInCurrentLayoutBlock(const PaintInvalidatorContext&);
  void InvalidatePaintInPreviousLayoutBlock(const PaintInvalidatorContext&);

  // These are updated by UpdateStyleAndLayoutIfNeeded().
  Color color_;
  PhysicalRect local_rect_;
  Member<LayoutBlock> layout_block_;

  // This is set to the previous value of layout_block_ during
  // UpdateStyleAndLayoutIfNeeded() if it hasn't been set since the last paint
  // invalidation. It is used during InvalidatePaint() to invalidate the caret
  // in the previous layout block.
  Member<const LayoutBlock> previous_layout_block_;

  WeakMember<const PhysicalBoxFragment> box_fragment_;

  bool is_active_ = false;
  bool needs_paint_invalidation_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_CARET_DISPLAY_ITEM_CLIENT_H_
