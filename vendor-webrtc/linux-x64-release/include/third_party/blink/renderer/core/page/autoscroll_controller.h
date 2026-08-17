/*
 * Copyright (C) 2006, 2007, 2009, 2010, 2011 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_AUTOSCROLL_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_AUTOSCROLL_CONTROLLER_H_

#include "base/gtest_prod_util.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "ui/gfx/geometry/point_f.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class LocalFrame;
class Node;
class Page;
class LayoutBox;
class LayoutObject;

enum AutoscrollType {
  kNoAutoscroll,
  kAutoscrollForDragAndDrop,
  kAutoscrollForSelection,
  kAutoscrollForMiddleClick,
#if BUILDFLAG(IS_IOS)
  kAutoscrollForSelectionToPoint,
#endif  // BUILDFLAG(IS_IOS)
};

enum MiddleClickMode {
  // Middle button was just pressed but was neither released nor moved out of
  // the deadzone yet.
  kMiddleClickInitial,
  // Mouse was moved out of the deadzone while still holding middle mouse
  // button.  In this mode, we'll stop autoscrolling when it's released.
  kMiddleClickHolding,
  // Middle button was released while still in the deadzone.  In this mode,
  // we'll stop autoscrolling when any button is clicked.
  kMiddleClickToggled,
};

// AutscrollController handels autoscroll and middle click autoscroll for
// EventHandler.
class CORE_EXPORT AutoscrollController final
    : public GarbageCollected<AutoscrollController> {
 public:
  explicit AutoscrollController(Page&);

  void Trace(Visitor*) const;

  // Selection and drag-and-drop autoscroll.
  void Animate();
  bool SelectionAutoscrollInProgress() const;
  bool AutoscrollInProgressFor(const LayoutBox*) const;
  bool AutoscrollInProgress() const;
  void StartAutoscrollForSelection(LayoutObject*);
  void StopAutoscroll();
  void StopAutoscrollIfNeeded(LayoutObject*);
  void UpdateDragAndDrop(Node* target_node,
                         const gfx::PointF& event_position,
                         base::TimeTicks event_time);

#if BUILDFLAG(IS_IOS)
  void StartAutoscrollForSelectionToPoint(LayoutObject* layout_object,
                                          const gfx::PointF& point_in_viewport);
#endif  // BUILDFLAG(IS_IOS)

  // Middle-click autoscroll.
  void StartMiddleClickAutoscroll(LocalFrame*,
                                  LayoutBox* scrollable,
                                  const gfx::PointF& position,
                                  const gfx::PointF& position_global);
  void HandleMouseMoveForMiddleClickAutoscroll(
      LocalFrame*,
      const gfx::PointF& position_global,
      bool is_middle_button);
  void HandleMouseReleaseForMiddleClickAutoscroll(LocalFrame*,
                                                  bool is_middle_button);
  void StopMiddleClickAutoscroll(LocalFrame*);
  bool MiddleClickAutoscrollInProgress() const;

 private:
  // For test.
  bool IsAutoscrolling() const;

  Member<Page> page_;
  AutoscrollType autoscroll_type_ = kNoAutoscroll;

  // Selection and drag-and-drop autoscroll.
  void ScheduleMainThreadAnimation();

  // Notify browser process input event router that main frame started drag
  // selection. Any mouse up event in OOF child frames should also dispatch
  // mouse up event in the main frame when the state is active.
  void UpdateCachedAutoscrollForSelectionState(bool autoscroll_selection);
  Member<LayoutBox> autoscroll_layout_object_ = nullptr;
  Member<LayoutBox> pressed_layout_object_ = nullptr;

  PhysicalOffset drag_and_drop_autoscroll_reference_position_;
  base::TimeTicks drag_and_drop_autoscroll_start_time_;

#if BUILDFLAG(IS_IOS)
  PhysicalOffset autoscroll_to_point_reference_position_;
#endif  // BUILDFLAG(IS_IOS)

  // Middle-click autoscroll.
  Member<LayoutBox> horizontal_autoscroll_layout_box_ = nullptr;
  Member<LayoutBox> vertical_autoscroll_layout_box_ = nullptr;
  gfx::PointF middle_click_autoscroll_start_pos_global_;
  gfx::Vector2dF last_velocity_;
  MiddleClickMode middle_click_mode_ = kMiddleClickInitial;

  FRIEND_TEST_ALL_PREFIXES(AutoscrollControllerTest,
                           CrashWhenLayoutStopAnimationBeforeScheduleAnimation);
  FRIEND_TEST_ALL_PREFIXES(AutoscrollControllerTest,
                           ContinueAutoscrollAfterMouseLeaveEvent);
  FRIEND_TEST_ALL_PREFIXES(AutoscrollControllerTest, StopAutoscrollOnResize);
  FRIEND_TEST_ALL_PREFIXES(AutoscrollControllerTest, AutoscrollIsNotPropagated);
  FRIEND_TEST_ALL_PREFIXES(AutoscrollControllerTest,
                           AutoscrollIsPropagatedInYDirection);
  FRIEND_TEST_ALL_PREFIXES(AutoscrollControllerTest, TextSelectionAutoScroll);
  FRIEND_TEST_ALL_PREFIXES(AutoscrollControllerTest,
                           PageVisibilityChangeCancelsAutoscroll);
  FRIEND_TEST_ALL_PREFIXES(AutoscrollControllerTest, PageLoadCancelsAutoscroll);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_AUTOSCROLL_CONTROLLER_H_
