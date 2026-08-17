/*
 * Copyright (C) 2006, 2007, 2009, 2010, 2011 Apple Inc. All rights reserved.
 * Copyright (C) 2015 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SELECTION_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SELECTION_CONTROLLER_H_

#include "third_party/blink/public/platform/web_input_event_result.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/editing/frame_selection.h"
#include "third_party/blink/renderer/core/editing/position_with_affinity.h"
#include "third_party/blink/renderer/core/editing/text_granularity.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/page/event_with_hit_test_results.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class HitTestResult;
class LocalFrame;

class CORE_EXPORT SelectionController final
    : public GarbageCollected<SelectionController>,
      public ExecutionContextLifecycleObserver {
 public:
  explicit SelectionController(LocalFrame&);
  SelectionController(const SelectionController&) = delete;
  SelectionController& operator=(const SelectionController&) = delete;
  ~SelectionController() override;
  void Trace(Visitor*) const override;

  bool HandleMousePressEvent(const MouseEventWithHitTestResults&);
  WebInputEventResult HandleMouseDraggedEvent(
      const MouseEventWithHitTestResults&,
      const gfx::Point&,
      const PhysicalOffset&);
  bool HandleMouseReleaseEvent(const MouseEventWithHitTestResults&,
                               const PhysicalOffset&);
  bool HandlePasteGlobalSelection(const WebMouseEvent&);
  bool HandleGestureLongPress(const HitTestResult&);
  void HandleGestureTwoFingerTap(const GestureEventWithHitTestResults&);

  void UpdateSelectionForMouseDrag(const PhysicalOffset&,
                                   const PhysicalOffset&);
  template <typename MouseEventObject>
  void UpdateSelectionForContextMenuEvent(const MouseEventObject* mouse_event,
                                          const HitTestResult& hit_test_result,
                                          const PhysicalOffset& position);
  void PassMousePressEventToSubframe(const MouseEventWithHitTestResults&);

  void InitializeSelectionState();
  void SetMouseDownMayStartSelect(bool);
  bool MouseDownMayStartSelect() const;
  bool MouseDownWasSingleClickInSelection() const;
  void NotifySelectionChanged();
  bool HasExtendedSelection() const {
    return selection_state_ == SelectionState::kExtendedSelection;
  }

 private:
  friend class SelectionControllerTest;

  enum class AppendTrailingWhitespace { kShouldAppend, kDontAppend };
  enum class SelectInputEventType { kTouch, kMouse };
  enum EndPointsAdjustmentMode {
    kAdjustEndpointsAtBidiBoundary,
    kDoNotAdjustEndpoints
  };

  Document& GetDocument() const;

  WebInputEventResult UpdateSelectionForMouseDrag(const HitTestResult&,
                                                  const PhysicalOffset&);

  // Returns |true| if a word was selected.
  bool SelectClosestWordFromHitTestResult(const HitTestResult&,
                                          AppendTrailingWhitespace,
                                          SelectInputEventType);
  void SelectClosestMisspellingFromHitTestResult(const HitTestResult&,
                                                 AppendTrailingWhitespace);
  // Returns |true| if a word was selected.
  template <typename MouseEventObject>
  bool SelectClosestWordFromMouseEvent(const MouseEventObject* mouse_event,
                                       const HitTestResult& result);
  template <typename MouseEventObject>
  void SelectClosestMisspellingFromMouseEvent(
      const MouseEventObject* mouse_event,
      const HitTestResult& hit_test_result);
  template <typename MouseEventObject>
  void SelectClosestWordOrLinkFromMouseEvent(
      const MouseEventObject* mouse_event,
      const HitTestResult& hit_test_result);
  void SetNonDirectionalSelectionIfNeeded(const SelectionInFlatTree&,
                                          const SetSelectionOptions&,
                                          EndPointsAdjustmentMode);
  void SetCaretAtHitTestResult(const HitTestResult&);
  bool UpdateSelectionForMouseDownDispatchingSelectStart(
      Node*,
      const SelectionInFlatTree&,
      const SetSelectionOptions&);

  FrameSelection& Selection() const;

  // Implements |ExecutionContextLifecycleObserver|.
  // TODO(yosin): We should relocate |original_anchor_in_flat_tree_| when DOM
  // tree changed.
  void ContextDestroyed() final;

  bool HandleSingleClick(const MouseEventWithHitTestResults&);
  bool HandleDoubleClick(const MouseEventWithHitTestResults&);
  bool HandleTripleClick(const MouseEventWithHitTestResults&);

  void HandleTapOnCaret(const MouseEventWithHitTestResults&,
                        const SelectionInFlatTree&);
  bool HandleTapInsideSelection(const MouseEventWithHitTestResults&,
                                const SelectionInFlatTree&);

  Member<LocalFrame> const frame_;
  // Used to store anchor before the adjustment at bidi boundary
  PositionInFlatTreeWithAffinity original_anchor_in_flat_tree_;
  bool mouse_down_may_start_select_;
  bool mouse_down_was_single_click_on_caret_ = false;
  bool mouse_down_was_single_click_in_selection_;
  bool mouse_down_allows_multi_click_;
  enum class SelectionState {
    kHaveNotStartedSelection,
    kPlacedCaret,
    kExtendedSelection
  };
  SelectionState selection_state_;
};

bool IsSelectionOverLink(const MouseEventWithHitTestResults&);
bool IsExtendingSelection(const MouseEventWithHitTestResults&);
CORE_EXPORT SelectionInFlatTree
AdjustSelectionWithTrailingWhitespace(const SelectionInFlatTree&);
CORE_EXPORT SelectionInFlatTree
AdjustSelectionByUserSelect(Node*, const SelectionInFlatTree&);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SELECTION_CONTROLLER_H_
