/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2006 Apple Computer, Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_LAYOUT_SELECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_LAYOUT_SELECTION_H_

#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace gfx {
class Rect;
}

namespace blink {

class FrameSelection;
class InlineCursor;
class InlineCursorPosition;
class LayoutObject;
class LayoutText;
class SelectionPaintRange;
enum class SelectionState;
struct LayoutSelectionStatus;
struct LayoutTextSelectionStatus;
struct TextOffsetRange;

class LayoutSelection final : public GarbageCollected<LayoutSelection> {
 public:
  explicit LayoutSelection(FrameSelection&);

  void SetHasPendingSelection();
  void Commit();

  gfx::Rect AbsoluteSelectionBounds();
  void InvalidateStyleAndPaintForSelection();

  LayoutTextSelectionStatus ComputeSelectionStatus(const LayoutText&) const;
  LayoutSelectionStatus ComputeSelectionStatus(const InlineCursor&) const;

  // Compute the layout selection state relative to the current item of the
  // given InlineCursor. E.g. a state of kStart means that the selection
  // starts within the position (and ends elsewhere), where kStartAndEnd means
  // the selection both starts and ends within the position. This information is
  // used at paint time to determine the edges of the layout selection.
  SelectionState ComputePaintingSelectionStateForCursor(
      const InlineCursorPosition&) const;

  static bool IsSelected(const LayoutObject&);

  void ContextDestroyed();

  void Trace(Visitor*) const;

 private:
  LayoutSelectionStatus ComputeSelectionStatus(const InlineCursor&,
                                               const TextOffsetRange&) const;
  SelectionState ComputeSelectionStateFromOffsets(SelectionState state,
                                                  unsigned start_offset,
                                                  unsigned end_offset) const;

  void AssertIsValid() const;

  Member<FrameSelection> frame_selection_;
  bool has_pending_selection_ : 1;

  Member<SelectionPaintRange> paint_range_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_LAYOUT_SELECTION_H_
