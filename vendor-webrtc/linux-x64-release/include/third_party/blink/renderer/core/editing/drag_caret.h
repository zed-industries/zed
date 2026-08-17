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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_DRAG_CARET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_DRAG_CARET_H_

#include "third_party/blink/renderer/core/editing/caret_display_item_client.h"
#include "third_party/blink/renderer/core/editing/position_with_affinity.h"
#include "third_party/blink/renderer/platform/graphics/paint_invalidation_reason.h"

namespace blink {

class LayoutBlock;
class PhysicalBoxFragment;
struct PaintInvalidatorContext;

class DragCaret final : public GarbageCollected<DragCaret> {
 public:
  DragCaret();
  DragCaret(const DragCaret&) = delete;
  DragCaret& operator=(const DragCaret&) = delete;

  // Paint invalidation methods delegating to CaretDisplayItemClient.
  void LayoutBlockWillBeDestroyed(const LayoutBlock&);
  void UpdateStyleAndLayoutIfNeeded();
  void InvalidatePaint(const LayoutBlock&, const PaintInvalidatorContext&);

  bool ShouldPaintCaret(const LayoutBlock&) const;
  bool ShouldPaintCaret(const PhysicalBoxFragment&) const;
  void PaintDragCaret(const LocalFrame*,
                      GraphicsContext&,
                      const PhysicalOffset&) const;

  bool IsContentRichlyEditable() const;

  bool HasCaret() const { return position_.IsNotNull(); }
  const PositionWithAffinity& CaretPosition() const { return position_; }
  void SetCaretPosition(const PositionWithAffinity&);
  void Clear() { SetCaretPosition(PositionWithAffinity()); }

  void Trace(Visitor*) const;

  void NodeChildrenWillBeRemoved(ContainerNode&);
  void NodeWillBeRemoved(Node&);

 private:
  PositionWithAffinity position_;
  const Member<CaretDisplayItemClient> display_item_client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_DRAG_CARET_H_
