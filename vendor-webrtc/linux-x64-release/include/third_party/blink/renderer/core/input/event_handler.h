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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_EVENT_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_EVENT_HANDLER_H_

#include <optional>

#include "base/debug/crash_logging.h"
#include "base/gtest_prod_util.h"
#include "base/time/time.h"
#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/common/input/web_menu_source_type.h"
#include "third_party/blink/public/platform/web_input_event_result.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/events/text_event_input_type.h"
#include "third_party/blink/renderer/core/input/gesture_manager.h"
#include "third_party/blink/renderer/core/input/keyboard_event_manager.h"
#include "third_party/blink/renderer/core/input/mouse_event_manager.h"
#include "third_party/blink/renderer/core/input/mouse_wheel_event_manager.h"
#include "third_party/blink/renderer/core/input/pointer_event_manager.h"
#include "third_party/blink/renderer/core/input/scroll_manager.h"
#include "third_party/blink/renderer/core/layout/hit_test_request.h"
#include "third_party/blink/renderer/core/page/event_with_hit_test_results.h"
#include "third_party/blink/renderer/core/page/touch_adjustment.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace ui {
class Cursor;
}

namespace blink {

class DataTransfer;
class PaintLayer;
class Element;
class Event;
class EventHandlerRegistry;
template <typename EventType>
class EventWithHitTestResults;
class HTMLFrameSetElement;
class HitTestRequest;
class HitTestResult;
class LayoutObject;
class LocalFrame;
class Node;
class ScrollableArea;
class Scrollbar;
class SelectionController;
class TextEvent;
class WebGestureEvent;
class WebMouseEvent;
class WebMouseWheelEvent;

// Handles events for Pointers (Mouse/Touch), HitTests, DragAndDrop, etc.
class CORE_EXPORT EventHandler final : public GarbageCollected<EventHandler> {
 public:
  explicit EventHandler(LocalFrame&);
  EventHandler(const EventHandler&) = delete;
  EventHandler& operator=(const EventHandler&) = delete;
  void Trace(Visitor*) const;

  void Clear();

  void NodeChildrenWillBeRemoved(ContainerNode& container) {
    mouse_event_manager_->NodeChildrenWillBeRemoved(container);
  }
  void NodeWillBeRemoved(Node& node) {
    mouse_event_manager_->NodeWillBeRemoved(node);
    pointer_event_manager_->NodeWillBeRemoved(node);
  }

  void UpdateSelectionForMouseDrag();
  void StartMiddleClickAutoscroll(LayoutObject*);

  // TODO(nzolghadr): Some of the APIs in this class only forward the action
  // to the corresponding Manager class. We need to investigate whether it is
  // better to expose the manager instance itself later or can the access to
  // those APIs be more limited or removed.

  void StopAutoscroll();

  HitTestResult HitTestResultAtLocation(
      const HitTestLocation&,
      HitTestRequest::HitTestRequestType hit_type = HitTestRequest::kReadOnly |
                                                    HitTestRequest::kActive,
      const LayoutObject* stop_node = nullptr,
      bool no_lifecycle_update = false,
      std::optional<HitTestRequest::HitNodeCb> hit_node_cb = std::nullopt);

  bool MousePressed() const { return mouse_event_manager_->MousePressed(); }
  bool IsMousePositionUnknown() const {
    return mouse_event_manager_->IsMousePositionUnknown();
  }
  void ResetLastMousePositionForWebTest();
  void ClearMouseEventManager() const { mouse_event_manager_->Clear(); }

  WebInputEventResult UpdateDragAndDrop(const WebMouseEvent&, DataTransfer*);
  void CancelDragAndDrop(const WebMouseEvent&, DataTransfer*);
  WebInputEventResult PerformDragAndDrop(const WebMouseEvent&, DataTransfer*);
  void UpdateDragStateAfterEditDragIfNeeded(Element* root_editable_element);

  void ScheduleHoverStateUpdate();
  void ScheduleCursorUpdate();

  // Return whether a mouse cursor update is currently pending.  Used for
  // testing.
  bool CursorUpdatePending();

  bool IsHandlingKeyEvent() const;

  void SetResizingFrameSet(HTMLFrameSetElement*);

  void ResizeScrollableAreaDestroyed();

  gfx::PointF LastKnownMousePositionInRootFrame() const;
  gfx::PointF LastKnownMouseScreenPosition() const;

  gfx::Point DragDataTransferLocationForTesting();

  // Performs a logical scroll that chains, crossing frames, starting from
  // the given node or a reasonable default (focus/last clicked).
  bool BubblingScroll(mojom::blink::ScrollDirection,
                      ui::ScrollGranularity,
                      Node* starting_node = nullptr);

  WebInputEventResult HandleMouseMoveEvent(
      const WebMouseEvent&,
      const Vector<WebMouseEvent>& coalesced_events,
      const Vector<WebMouseEvent>& predicted_events);
  void HandleMouseLeaveEvent(const WebMouseEvent&);

  WebInputEventResult HandlePointerEvent(
      const WebPointerEvent&,
      const Vector<WebPointerEvent>& coalesced_events,
      const Vector<WebPointerEvent>& predicted_events);

  WebInputEventResult DispatchBufferedTouchEvents();

  WebInputEventResult HandleMousePressEvent(const WebMouseEvent&);
  WebInputEventResult HandleMouseReleaseEvent(const WebMouseEvent&);
  WebInputEventResult HandleWheelEvent(const WebMouseWheelEvent&);

  WebInputEventResult HandleTargetedMouseEvent(
      Element* target,
      const WebMouseEvent&,
      const AtomicString& event_type,
      const Vector<WebMouseEvent>& coalesced_events,
      const Vector<WebMouseEvent>& predicted_events);

  // Called on the local root frame exactly once per gesture event.
  WebInputEventResult HandleGestureEvent(const WebGestureEvent&);
  WebInputEventResult HandleGestureEvent(const GestureEventWithHitTestResults&);

  // Clear the old hover/active state within frames before moving the hover
  // state to the another frame. |is_active| specifies whether the active state
  // is being applied to or removed from the given element. This method should
  // be initially called on the root document, it will recurse into child
  // frames as needed.
  void UpdateCrossFrameHoverActiveState(bool is_active, Element*);

  // Hit-test the provided (non-scroll) gesture event, applying touch-adjustment
  // and updating hover/active state across all frames if necessary. This should
  // be called at most once per gesture event, and called on the local root
  // frame.
  // Note: This is similar to (the less clearly named) prepareMouseEvent.
  // FIXME: Remove readOnly param when there is only ever a single call to this.
  GestureEventWithHitTestResults TargetGestureEvent(const WebGestureEvent&,
                                                    bool read_only = false);
  GestureEventWithHitTestResults HitTestResultForGestureEvent(
      const WebGestureEvent&,
      HitTestRequest::HitTestRequestType);

  bool BestNodeForHitTestResult(TouchAdjustmentCandidateType candidate_type,
                                const HitTestLocation& location,
                                const HitTestResult&,
                                gfx::Point& adjusted_point,
                                Node*& adjusted_node);
  void CacheTouchAdjustmentResult(uint32_t, gfx::PointF);

  // Dispatch a context menu event. If |override_target_element| is provided,
  // the context menu event will use that, so that the browser-generated context
  // menu will be filled with options relevant to it, rather than the element
  // found via hit testing the event's screen point. This is used so that a
  // context menu generated via the keyboard reliably uses the correct target.
  WebInputEventResult SendContextMenuEvent(
      const WebMouseEvent&,
      Element* override_target_element = nullptr);
  WebInputEventResult ShowNonLocatedContextMenu(
      Element* override_target_element = nullptr,
      WebMenuSourceType = kMenuSourceNone);

  // Returns whether pointerId is active or not
  bool IsPointerEventActive(PointerId);

  void SetPointerCapture(PointerId, Element*);
  void ReleasePointerCapture(PointerId, Element*);
  void ReleaseMousePointerCapture();
  bool HasPointerCapture(PointerId, const Element*) const;

  void ElementRemoved(Element*);

  void SetMouseDownMayStartAutoscroll();

  bool HandleAccessKey(const WebKeyboardEvent&);
  WebInputEventResult KeyEvent(const WebKeyboardEvent&);
  void DefaultKeyboardEventHandler(KeyboardEvent*);

  bool HandleTextInputEvent(const String& text,
                            Event* underlying_event = nullptr,
                            TextEventInputType = kTextEventInputKeyboard);
  void DefaultTextInputEventHandler(TextEvent*);

  void DragSourceEndedAt(const WebMouseEvent&, ui::mojom::blink::DragOperation);

  void CapsLockStateMayHaveChanged();  // Only called by FrameSelection

  static bool UsesHandCursor(const Node*);

  void NotifyElementActivated();

  SelectionController& GetSelectionController() const {
    return *selection_controller_;
  }

  bool IsPointerIdActiveOnFrame(PointerId, LocalFrame*) const;

  LocalFrame* DetermineActivePointerTrackerFrame(PointerId pointer_id) const;

  // Clears drag target and related states. It is called when drag is done or
  // canceled.
  void ClearDragState();

  EventHandlerRegistry& GetEventHandlerRegistry() const {
    return *event_handler_registry_;
  }

  GestureManager& GetGestureManager() const { return *gesture_manager_; }

  KeyboardEventManager& GetKeyboardEventManager() const {
    return *keyboard_event_manager_;
  }

  void RecomputeMouseHoverStateIfNeeded();

  void MarkHoverStateDirty();

  // Reset the last mouse position so that movement after unlock will be
  // restart from the lock position.
  void ResetMousePositionForPointerUnlock();

  bool LongTapShouldInvokeContextMenu();

  void UpdateCursor();

  float cursor_accessibility_scale_factor() const {
    return cursor_accessibility_scale_factor_;
  }
  void set_cursor_accessibility_scale_factor(float scale) {
    cursor_accessibility_scale_factor_ = scale;
  }

  void OnScrollbarDestroyed(const Scrollbar& scrollbar);

  Element* GetElementUnderMouse();

  Element* CurrentTouchDownElement();

  void SetDelayedNavigationTaskHandle(TaskHandle task_handle);

  TaskHandle& GetDelayedNavigationTaskHandle();

  base::debug::CrashKeyString* CrashKeyForBug1519197() const;

 private:
  WebInputEventResult HandleMouseMoveOrLeaveEvent(
      const WebMouseEvent&,
      const Vector<WebMouseEvent>& coalesced_events,
      const Vector<WebMouseEvent>& predicted_events,
      HitTestResult* hovered_node = nullptr,
      HitTestLocation* hit_test_location = nullptr);

  // Updates the event, location and result to the adjusted target.
  void ApplyTouchAdjustment(WebGestureEvent*, HitTestLocation&, HitTestResult&);

  void PerformHitTest(const HitTestLocation& location,
                      HitTestResult&,
                      bool no_lifecycle_update) const;

  void UpdateGestureTargetNodeForMouseEvent(
      const GestureEventWithHitTestResults&);

  // Handle the provided non-scroll gesture event. Should be called only on the
  // inner frame.
  WebInputEventResult HandleGestureEventInFrame(
      const GestureEventWithHitTestResults&);

  bool ShouldApplyTouchAdjustment(const WebGestureEvent&) const;
  bool GestureCorrespondsToAdjustedTouch(const WebGestureEvent&);
  bool IsSelectingLink(const HitTestResult&);
  bool ShouldShowIBeamForNode(const Node*, const HitTestResult&);
  bool ShouldShowResizeForNode(const LayoutObject&, const HitTestLocation&);
  std::optional<ui::Cursor> SelectCursor(const HitTestLocation& location,
                                         const HitTestResult&);
  std::optional<ui::Cursor> SelectAutoCursor(const HitTestResult&,
                                             Node*,
                                             const ui::Cursor& i_beam);

  void HoverTimerFired(TimerBase*);
  void CursorUpdateTimerFired(TimerBase*);
  void ActiveIntervalTimerFired(TimerBase*);

  ScrollableArea* AssociatedScrollableArea(const PaintLayer*) const;

  Element* EffectiveMouseEventTargetElement(Element*);

  // Task handle used to distinguish single/double click with some modifiers.
  //
  // When single click with some modifiers occurred, this task handle is set.
  // If double click follows, this is cancelled and renderer emit double click
  // event. (By default, it is handled by renderer as text selection.) If not,
  // the delayed navigation is emitted.
  //
  // Currently, the target navigations are the followings:
  //
  // - Download (Alt-click with/without some other modifiers.)
  // - Link Preview (Alt-click)
  //
  // For more details, see https://crbug.com/1428816.
  TaskHandle delayed_navigation_task_handle_;

  // Dispatches ME after corresponding PE provided the PE has not been
  // canceled. The |mouse_event_type| arg must be one of {mousedown,
  // mousemove, mouseup}.
  WebInputEventResult DispatchMousePointerEvent(
      const WebInputEvent::Type,
      Element* target,
      const WebMouseEvent&,
      const Vector<WebMouseEvent>& coalesced_events,
      const Vector<WebMouseEvent>& predicted_events,
      bool skip_click_dispatch = false);

  WebInputEventResult PassMousePressEventToSubframe(
      MouseEventWithHitTestResults&,
      LocalFrame* subframe);
  WebInputEventResult PassMouseMoveEventToSubframe(
      MouseEventWithHitTestResults&,
      const Vector<WebMouseEvent>& coalesced_events,
      const Vector<WebMouseEvent>& predicted_events,
      LocalFrame* subframe,
      HitTestResult* hovered_node = nullptr,
      HitTestLocation* hit_test_location = nullptr);
  WebInputEventResult PassMouseReleaseEventToSubframe(
      MouseEventWithHitTestResults&,
      LocalFrame* subframe);

  bool PassMousePressEventToScrollbar(MouseEventWithHitTestResults&);

  // |last_scrollbar_under_mouse_| is set when the mouse moves off of a
  // scrollbar, and used to notify it of MouseUp events to release mouse
  // capture.
  void UpdateLastScrollbarUnderMouse(Scrollbar*, bool);

  WebInputEventResult HandleGestureShowPress();

  bool RootFrameTrackedActivePointerInCurrentFrame(PointerId pointer_id) const;

  void CaptureMouseEventsToWidget(bool);

  void ReleaseMouseCaptureFromLocalRoot();
  void ReleaseMouseCaptureFromCurrentFrame();

  MouseEventWithHitTestResults GetMouseEventTarget(
      const HitTestRequest& request,
      const WebMouseEvent& mev);

  // Returned rect is in local root frame coordinates.
  gfx::Rect GetFocusedElementRectForNonLocatedContextMenu(
      Element* focused_element);

  // NOTE: If adding a new field to this class please ensure that it is
  // cleared in |EventHandler::clear()|.

  const Member<LocalFrame> frame_;

  const Member<SelectionController> selection_controller_;

  // TODO(lanwei): Remove the below timers for updating hover and cursor.
  HeapTaskRunnerTimer<EventHandler> hover_timer_;

  // TODO(rbyers): Mouse cursor update is page-wide, not per-frame.  Page-wide
  // state should move out of EventHandler to a new PageEventHandler class.
  // crbug.com/449649
  HeapTaskRunnerTimer<EventHandler> cursor_update_timer_;

  Member<Element> capturing_mouse_events_element_;
  // |capturing_subframe_element_| has similar functionality as
  // |capturing_mouse_events_element_|. It replaces |capturing_..| when
  // UnifiedPointerCapture enabled.
  Member<Element> capturing_subframe_element_;

  // Indicates whether the current widget is capturing mouse input.
  // Only used for local frame root EventHandlers.
  bool is_widget_capturing_mouse_events_ = false;

  Member<LocalFrame> last_mouse_move_event_subframe_;
  Member<Scrollbar> last_scrollbar_under_mouse_;

  Member<Node> drag_target_;
  bool should_only_fire_drag_over_event_;

  Member<HTMLFrameSetElement> frame_set_being_resized_;

  // Local frames in the same local root share the same EventHandlerRegistry.
  Member<EventHandlerRegistry> event_handler_registry_;
  Member<ScrollManager> scroll_manager_;
  Member<MouseEventManager> mouse_event_manager_;
  Member<MouseWheelEventManager> mouse_wheel_event_manager_;
  Member<KeyboardEventManager> keyboard_event_manager_;
  Member<PointerEventManager> pointer_event_manager_;
  Member<GestureManager> gesture_manager_;

  double max_mouse_moved_duration_;

  float cursor_accessibility_scale_factor_ = 1.f;

  HeapTaskRunnerTimer<EventHandler> active_interval_timer_;

  // last_show_press_timestamp_ prevents the active state rewrited by
  // following events too soon (less than 0.15s). It is ok we only record
  // last_show_press_timestamp_ in root frame since root frame will have
  // subframe as active element if subframe has active element.
  std::optional<base::TimeTicks> last_show_press_timestamp_;
  Member<Element> last_deferred_tap_element_;

  // Set on GestureTapDown if unique_touch_event_id_ matches cached adjusted
  // touchstart event id.
  bool should_use_touch_event_adjusted_point_;

  // Stored the last touch type primary pointer down adjustment result.
  // This is used in gesture event hit test.
  TouchAdjustmentResult touch_adjustment_result_;

  struct {
    DOMNodeId mouse_down_target = kInvalidDOMNodeId;
    DOMNodeId tap_target = kInvalidDOMNodeId;
    base::TimeTicks mouse_down_time;
    base::TimeTicks tap_time;
  } discarded_events_;

  // ShouldShowIBeamForNode's unit tests:
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest, HitOnNothingDoesNotShowIBeam);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest, HitOnTextShowsIBeam);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest,
                           HitOnUserSelectNoneDoesNotShowIBeam);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest,
                           ShadowChildCanOverrideUserSelectNone);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest,
                           UserSelectAllCanOverrideUserSelectNone);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest,
                           UserSelectNoneCanOverrideUserSelectAll);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest,
                           UserSelectTextCanOverrideUserSelectNone);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest,
                           UserSelectNoneCanOverrideUserSelectText);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest,
                           ShadowChildCanOverrideUserSelectText);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest, InputFieldsCanStartSelection);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest, ImagesCannotStartSelection);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest, AnchorTextCannotStartSelection);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest,
                           EditableAnchorTextCanStartSelection);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest,
                           ReadOnlyInputDoesNotInheritUserSelect);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest,
                           CursorForVerticalResizableTextArea);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest,
                           CursorForHorizontalResizableTextArea);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest, CursorForResizableTextArea);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest, CursorForRtlResizableTextArea);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest,
                           CursorForInlineVerticalWritingMode);
  FRIEND_TEST_ALL_PREFIXES(EventHandlerTest, CursorForBlockVerticalWritingMode);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_EVENT_HANDLER_H_
