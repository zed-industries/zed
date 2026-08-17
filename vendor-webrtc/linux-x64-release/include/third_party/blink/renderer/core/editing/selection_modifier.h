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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SELECTION_MODIFIER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SELECTION_MODIFIER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/selection_template.h"
#include "third_party/blink/renderer/core/editing/visible_selection.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class LocalFrame;

enum class SelectionModifyAlteration { kMove, kExtend };
enum class SelectionModifyVerticalDirection { kUp, kDown };
enum class SelectionModifyDirection { kBackward, kForward, kLeft, kRight };

class CORE_EXPORT SelectionModifier {
  STACK_ALLOCATED();

 public:
  // |frame| is used for providing settings.
  SelectionModifier(const LocalFrame& /* frame */,
                    const SelectionInDOMTree&,
                    LayoutUnit);
  SelectionModifier(const LocalFrame&, const SelectionInDOMTree&);
  SelectionModifier(const SelectionModifier&) = delete;
  SelectionModifier& operator=(const SelectionModifier&) = delete;

  LayoutUnit XPosForVerticalArrowNavigation() const {
    return x_pos_for_vertical_arrow_navigation_;
  }

  // TODO(editing-dev): We should rename |Selection()| to
  // |ComputeVisibleSelectionDeprecated()| and introduce |GetSelection()|
  // to return |current_selection_|.
  VisibleSelection Selection() const;

  TextDirection DirectionOfSelection() const;

  bool Modify(SelectionModifyAlteration,
              SelectionModifyDirection,
              TextGranularity);
  bool ModifyWithPageGranularity(SelectionModifyAlteration,
                                 unsigned vertical_distance,
                                 SelectionModifyVerticalDirection);
  void SetSelectionIsDirectional(bool selection_is_directional) {
    selection_is_directional_ = selection_is_directional;
  }

 private:
  const LocalFrame& GetFrame() const { return *frame_; }

  static bool ShouldAlwaysUseDirectionalSelection(const LocalFrame&);
  VisibleSelectionInFlatTree PrepareToModifySelection(
      SelectionModifyAlteration,
      SelectionModifyDirection) const;
  TextDirection DirectionOfEnclosingBlock() const;
  TextDirection LineDirectionOfFocus() const;
  VisiblePositionInFlatTree PositionForPlatform(bool is_get_start) const;
  VisiblePositionInFlatTree StartForPlatform() const;
  VisiblePositionInFlatTree EndForPlatform() const;
  LayoutUnit LineDirectionPointForBlockDirectionNavigation(
      const PositionInFlatTree&);
  VisiblePositionInFlatTree ComputeModifyPosition(SelectionModifyAlteration,
                                                  SelectionModifyDirection,
                                                  TextGranularity);
  VisiblePositionInFlatTree ModifyExtendingRight(TextGranularity);
  VisiblePositionInFlatTree ModifyExtendingRightInternal(TextGranularity);
  VisiblePositionInFlatTree ModifyExtendingForward(TextGranularity);
  VisiblePositionInFlatTree ModifyExtendingForwardInternal(TextGranularity);
  VisiblePositionInFlatTree ModifyMovingRight(TextGranularity);
  VisiblePositionInFlatTree ModifyMovingForward(TextGranularity);
  VisiblePositionInFlatTree ModifyExtendingLeft(TextGranularity);
  VisiblePositionInFlatTree ModifyExtendingLeftInternal(TextGranularity);
  VisiblePositionInFlatTree ModifyExtendingBackward(TextGranularity);
  VisiblePositionInFlatTree ModifyExtendingBackwardInternal(TextGranularity);
  VisiblePositionInFlatTree ModifyMovingLeft(TextGranularity);
  VisiblePositionInFlatTree ModifyMovingBackward(TextGranularity);
  PositionInFlatTree NextWordPositionForPlatform(const PositionInFlatTree&);

  void UpdateAllLifecyclePhasesExceptPaint();

  static PositionInFlatTreeWithAffinity PreviousLinePosition(
      const PositionInFlatTreeWithAffinity&,
      LayoutUnit line_direction_point);
  static PositionInFlatTreeWithAffinity NextLinePosition(
      const PositionInFlatTreeWithAffinity&,
      LayoutUnit line_direction_point);
  static VisiblePositionInFlatTree PreviousParagraphPosition(
      const VisiblePositionInFlatTree&,
      LayoutUnit line_direction_point);
  static VisiblePositionInFlatTree NextParagraphPosition(
      const VisiblePositionInFlatTree&,
      LayoutUnit line_direction_point);

  const LocalFrame* frame_;
  // TODO(editing-dev): We should get rid of |selection_| once we change
  // all member functions not to use |selection_|.
  // |selection_| is used as implicit parameter or a cache instead of pass it.
  VisibleSelectionInFlatTree selection_;
  // TODO(editing-dev): We should introduce |GetSelection()| to return
  // |result_| to replace |Selection().AsSelection()|.
  // |current_selection_| holds initial value and result of |Modify()|.
  SelectionInFlatTree current_selection_;
  LayoutUnit x_pos_for_vertical_arrow_navigation_;
  bool selection_is_directional_ = false;
};

LayoutUnit NoXPosForVerticalArrowNavigation();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SELECTION_MODIFIER_H_
