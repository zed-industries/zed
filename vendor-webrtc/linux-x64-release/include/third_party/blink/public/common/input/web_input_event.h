/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_INPUT_EVENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_INPUT_EVENT_H_

#include <string.h>

#include <memory>
#include <optional>

#include "base/notreached.h"
#include "base/time/time.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/input/input_event.mojom-shared.h"
#include "ui/events/event_latency_metadata.h"
#include "ui/events/types/event_type.h"
#include "ui/gfx/geometry/point_f.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace blink {

// WebInputEvent --------------------------------------------------------------

class BLINK_COMMON_EXPORT WebInputEvent {
 public:
  // When we use an input method (or an input method editor), we receive
  // two events for a keypress. The former event is a keydown, which
  // provides a keycode, and the latter is a textinput, which provides
  // a character processed by an input method. (The mapping from a
  // keycode to a character code is not trivial for non-English
  // keyboards.)
  // To support input methods, Safari sends keydown events to WebKit for
  // filtering. WebKit sends filtered keydown events back to Safari,
  // which sends them to input methods.
  // Unfortunately, it is hard to apply this design to Chrome because of
  // our multiprocess architecture. An input method is running in a
  // browser process. On the other hand, WebKit is running in a renderer
  // process. So, this design results in increasing IPC messages.
  // To support input methods without increasing IPC messages, Chrome
  // handles keyboard events in a browser process and send asynchronous
  // input events (to be translated to DOM events) to a renderer
  // process.
  // This design is mostly the same as the one of Windows and Mac Carbon.
  // So, for what it's worth, our Linux and Mac front-ends emulate our
  // Windows front-end. To emulate our Windows front-end, we can share
  // our back-end code among Windows, Linux, and Mac.
  // TODO(hbono): Issue 18064: remove the KeyDown type since it isn't
  // used in Chrome any longer.

  using Type = blink::mojom::EventType;

  // The modifier constants cannot change their values since pepper
  // does a 1-1 mapping of its values; see
  // content/renderer/pepper/event_conversion.cc
  //
  // A Java counterpart will be generated for this enum.
  // GENERATED_JAVA_ENUM_PACKAGE: org.chromium.blink_public.web
  // GENERATED_JAVA_CLASS_NAME_OVERRIDE: WebInputEventModifier
  enum Modifiers {
    // modifiers for all events:
    kShiftKey = 1 << 0,
    kControlKey = 1 << 1,
    kAltKey = 1 << 2,
    kMetaKey = 1 << 3,

    // modifiers for keyboard events:
    kIsKeyPad = 1 << 4,
    kIsAutoRepeat = 1 << 5,

    // modifiers for mouse events:
    kLeftButtonDown = 1 << 6,
    kMiddleButtonDown = 1 << 7,
    kRightButtonDown = 1 << 8,

    // Toggle modifers for all events.
    kCapsLockOn = 1 << 9,
    kNumLockOn = 1 << 10,

    kIsLeft = 1 << 11,
    kIsRight = 1 << 12,

    // Indicates that an event was generated on the touch screen while
    // touch accessibility is enabled, so the event should be handled
    // by accessibility code first before normal input event processing.
    kIsTouchAccessibility = 1 << 13,

    kIsComposing = 1 << 14,

    kAltGrKey = 1 << 15,
    kFnKey = 1 << 16,
    kSymbolKey = 1 << 17,

    kScrollLockOn = 1 << 18,

    // Whether this is a compatibility event generated due to a
    // native touch event. Mouse events generated from touch
    // events will set this.
    kIsCompatibilityEventForTouch = 1 << 19,

    kBackButtonDown = 1 << 20,
    kForwardButtonDown = 1 << 21,

    // Represents movement as a result of content changing under the cursor,
    // not actual physical movement of the pointer
    kRelativeMotionEvent = 1 << 22,

    // Indication this event was injected by the devtools.
    // TODO(dtapuska): Remove this flag once we are able to bind callbacks
    // in event sending.
    kFromDebugger = 1 << 23,

    // Indicates this event is targeting an OOPIF, and the iframe or one of its
    // ancestor frames moved within its embedding page's viewport recently.
    kTargetFrameMovedRecently = 1 << 24,

    // TODO(szager): This is the same as kTargetFrameMovedRecently, but it
    // overrides that value for iframes that are using IntersectionObserver V2
    // features (i.e. occlusion detection). The purpose of this distinction is
    // to preserve existing behavior for IOv2-using iframes while dialing in the
    // feature parameters for kDiscardInputEventsToRecentlyMovedFrames, which is
    // broader in scope. At the end of that process, this flag should be removed
    // in favor of kTargetFrameMovedRecently.
    kTargetFrameMovedRecentlyForIOv2 = 1 << 25,

    // When an event is forwarded to the main thread, this modifier will tell if
    // the event was already handled by the compositor thread or not. Based on
    // this, the decision of whether or not the main thread should handle this
    // event for the scrollbar can then be made.
    kScrollbarManipulationHandledOnCompositorThread = 1 << 26,

    // The set of non-stateful modifiers that specifically change the
    // interpretation of the key being pressed. For example; IsLeft,
    // IsRight, IsComposing don't change the meaning of the key
    // being pressed. NumLockOn, ScrollLockOn, CapsLockOn are stateful
    // and don't indicate explicit depressed state.
    kKeyModifiers = kSymbolKey | kFnKey | kAltGrKey | kMetaKey | kAltKey |
                    kControlKey | kShiftKey,

    kNoModifiers = 0,
  };

  using DispatchType = mojom::DispatchType;

  // The rail mode for a wheel event specifies the axis on which scrolling is
  // expected to stick. If this axis is set to Free, then scrolling is not
  // stuck to any axis.
  enum RailsMode {
    kRailsModeFree = 0,
    kRailsModeHorizontal = 1,
    kRailsModeVertical = 2,
  };

  static const int kInputModifiers =
      kShiftKey | kControlKey | kAltKey | kMetaKey;

  static constexpr base::TimeTicks GetStaticTimeStampForTests() {
    // Note: intentionally use a relatively large delta from base::TimeTicks ==
    // 0. Otherwise, code that tracks the time ticks of the last event that
    // happened and computes a delta might get confused when the testing
    // timestamp is near 0, as the computed delta may very well be under the
    // delta threshhold.
    //
    // TODO(dcheng): This really shouldn't use FromInternalValue(), but
    // constexpr support for time operations is a bit busted...
    return base::TimeTicks::FromInternalValue(123'000'000);
  }

  // Returns true if the WebInputEvent |type| is a mouse event.
  static bool IsMouseEventType(WebInputEvent::Type type) {
    return Type::kMouseTypeFirst <= type && type <= Type::kMouseTypeLast;
  }

  // Returns true if the WebInputEvent |type| is a keyboard event.
  static bool IsKeyboardEventType(WebInputEvent::Type type) {
    return Type::kKeyboardTypeFirst <= type && type <= Type::kKeyboardTypeLast;
  }

  // Returns true if the WebInputEvent |type| is a touch event.
  static bool IsTouchEventType(WebInputEvent::Type type) {
    return Type::kTouchTypeFirst <= type && type <= Type::kTouchTypeLast;
  }

  // Returns true if the WebInputEvent is a gesture event.
  static bool IsGestureEventType(WebInputEvent::Type type) {
    return Type::kGestureTypeFirst <= type && type <= Type::kGestureTypeLast;
  }

  // Returns true if the WebInputEvent |type| is a pointer event.
  static bool IsPointerEventType(WebInputEvent::Type type) {
    return Type::kPointerTypeFirst <= type && type <= Type::kPointerTypeLast;
  }

  // Returns true if the WebInputEvent |type| will potentially be considered
  // part of a "web interaction" for responsiveness metrics, e.g.
  // Interaction-to-Next-Paint (INP). This, for example, includes clicks and
  // key presses, but excludes continuous input like scrolling.
  //
  // This list includes `WebInputEvent::Type`s that can result in dispatching
  // relevant events [1] considered by blink::ResponsivenessMetrics (see
  // IsEventTypeForInteractionId() in window_performance.cc). For example the
  // handling of kPointerUp, kMouseUp, and kTouchEnd `WebInputEvent::Type` raw
  // events can all lead to dispatching a "pointerup" event, which is used in
  // computing responsiveness metrics.
  //
  // [1] Note this excludes some events that are used for responsiveness metrics
  // state tracking, e.g. "pointercancel".
  static bool IsWebInteractionEvent(WebInputEvent::Type type) {
    return type == WebInputEvent::Type::kMouseDown ||
           type == WebInputEvent::Type::kMouseUp ||
           type == WebInputEvent::Type::kKeyDown ||
           type == WebInputEvent::Type::kRawKeyDown ||
           type == WebInputEvent::Type::kKeyUp ||
           type == WebInputEvent::Type::kChar ||
           type == WebInputEvent::Type::kGestureTapDown ||
           type == WebInputEvent::Type::kGestureTap ||
           type == WebInputEvent::Type::kPointerDown ||
           type == WebInputEvent::Type::kPointerUp ||
           type == WebInputEvent::Type::kTouchStart ||
           type == WebInputEvent::Type::kTouchEnd;
  }

  bool IsSameEventClass(const WebInputEvent& other) const {
    if (IsMouseEventType(type_))
      return IsMouseEventType(other.type_);
    if (IsGestureEventType(type_))
      return IsGestureEventType(other.type_);
    if (IsTouchEventType(type_))
      return IsTouchEventType(other.type_);
    if (IsKeyboardEventType(type_))
      return IsKeyboardEventType(other.type_);
    if (IsPointerEventType(type_))
      return IsPointerEventType(other.type_);
    return type_ == other.type_;
  }

  bool IsGestureScroll() const {
    switch (type_) {
      case Type::kGestureScrollBegin:
      case Type::kGestureScrollUpdate:
      case Type::kGestureScrollEnd:
        return true;
      default:
        return false;
    }
  }

  // Returns true if the WebInputEvent |type| is a pinch gesture event.
  static bool IsPinchGestureEventType(WebInputEvent::Type type) {
    return Type::kGesturePinchTypeFirst <= type &&
           type <= Type::kGesturePinchTypeLast;
  }

  // Returns true if the WebInputEvent |type| is a fling gesture event.
  static bool IsFlingGestureEventType(WebInputEvent::Type type) {
    return Type::kGestureFlingStart <= type &&
           type <= Type::kGestureFlingCancel;
  }

  static const char* GetName(WebInputEvent::Type type) {
#define CASE_TYPE(t)              \
  case WebInputEvent::Type::k##t: \
    return #t
    switch (type) {
      CASE_TYPE(Undefined);
      CASE_TYPE(MouseDown);
      CASE_TYPE(MouseUp);
      CASE_TYPE(MouseMove);
      CASE_TYPE(MouseEnter);
      CASE_TYPE(MouseLeave);
      CASE_TYPE(ContextMenu);
      CASE_TYPE(MouseWheel);
      CASE_TYPE(RawKeyDown);
      CASE_TYPE(KeyDown);
      CASE_TYPE(KeyUp);
      CASE_TYPE(Char);
      CASE_TYPE(GestureScrollBegin);
      CASE_TYPE(GestureScrollEnd);
      CASE_TYPE(GestureScrollUpdate);
      CASE_TYPE(GestureFlingStart);
      CASE_TYPE(GestureFlingCancel);
      CASE_TYPE(GestureShowPress);
      CASE_TYPE(GestureTap);
      CASE_TYPE(GestureTapUnconfirmed);
      CASE_TYPE(GestureTapDown);
      CASE_TYPE(GestureTapCancel);
      CASE_TYPE(GestureDoubleTap);
      CASE_TYPE(GestureTwoFingerTap);
      CASE_TYPE(GestureShortPress);
      CASE_TYPE(GestureLongPress);
      CASE_TYPE(GestureLongTap);
      CASE_TYPE(GestureBegin);
      CASE_TYPE(GestureEnd);
      CASE_TYPE(GesturePinchBegin);
      CASE_TYPE(GesturePinchEnd);
      CASE_TYPE(GesturePinchUpdate);
      CASE_TYPE(TouchStart);
      CASE_TYPE(TouchMove);
      CASE_TYPE(TouchEnd);
      CASE_TYPE(TouchCancel);
      CASE_TYPE(TouchScrollStarted);
      CASE_TYPE(PointerDown);
      CASE_TYPE(PointerUp);
      CASE_TYPE(PointerMove);
      CASE_TYPE(PointerRawUpdate);
      CASE_TYPE(PointerCancel);
      CASE_TYPE(PointerCausedUaAction);
    }
#undef CASE_TYPE
    NOTREACHED();
  }

  float FrameScale() const { return frame_scale_; }
  void SetFrameScale(float scale) { frame_scale_ = scale; }

  gfx::Vector2dF FrameTranslate() const { return frame_translate_; }
  void SetFrameTranslate(const gfx::Vector2dF& translate) {
    frame_translate_ = translate;
  }

  Type GetType() const { return type_; }
  void SetType(Type type_param) { type_ = type_param; }

  int GetModifiers() const { return modifiers_; }
  void SetModifiers(int modifiers_param) { modifiers_ = modifiers_param; }

  // Event time since platform start with microsecond resolution.
  base::TimeTicks TimeStamp() const { return time_stamp_; }
  void SetTimeStamp(base::TimeTicks time_stamp) { time_stamp_ = time_stamp; }
  // Event time when queued by compositor on main thread.
  base::TimeTicks QueuedTimeStamp() const { return queued_time_stamp_; }
  void SetQueuedTimeStamp(base::TimeTicks time_stamp) {
    queued_time_stamp_ = time_stamp;
  }

  const ui::EventLatencyMetadata& GetEventLatencyMetadata() const {
    return event_latency_metadata_;
  }
  ui::EventLatencyMetadata& GetModifiableEventLatencyMetadata() {
    return event_latency_metadata_;
  }

  void SetTargetFrameMovedRecently() {
    modifiers_ |= kTargetFrameMovedRecently;
  }

  void SetTargetFrameMovedRecentlyForIOv2() {
    modifiers_ |= kTargetFrameMovedRecentlyForIOv2;
  }

  void SetScrollbarManipulationHandledOnCompositorThread() {
    modifiers_ |= kScrollbarManipulationHandledOnCompositorThread;
  }

  virtual ~WebInputEvent() = default;

  virtual std::unique_ptr<WebInputEvent> Clone() const = 0;

  // Returns whether the current event can be merged with the provided
  // |event|.
  virtual bool CanCoalesce(const blink::WebInputEvent& event) const = 0;

  // Merge the current event with attributes from |event|.
  virtual void Coalesce(const WebInputEvent& event) = 0;

  // Convert this WebInputEvent::Type to a ui::EventType. Note that this is
  // not a 1:1 relationship. Multiple blink types convert to the same
  // ui::EventType and not all types do convert.
  ui::EventType GetTypeAsUiEventType() const;

  // For the events that are received during an active scroll/fling, we don't
  // count them into the INP metrics. Set by the renderer compositor based on
  // its current gesture-handling state.
  bool GetPreventCountingAsInteraction() const {
    return prevent_counting_as_interaction_;
  }
  void SetPreventCountingAsInteractionTrue() {
    prevent_counting_as_interaction_ = true;
  }

 protected:
  // The root frame scale.
  float frame_scale_ = 1;

  // The root frame translation (applied post scale).
  gfx::Vector2dF frame_translate_;

  WebInputEvent(Type type, int modifiers, base::TimeTicks time_stamp)
      : time_stamp_(time_stamp), type_(type), modifiers_(modifiers) {}

  WebInputEvent() { time_stamp_ = base::TimeTicks(); }

  static DispatchType MergeDispatchTypes(DispatchType type_1,
                                         DispatchType type_2);

  base::TimeTicks time_stamp_;
  base::TimeTicks queued_time_stamp_;
  Type type_ = Type::kUndefined;
  int modifiers_ = kNoModifiers;

  ui::EventLatencyMetadata event_latency_metadata_;

  bool prevent_counting_as_interaction_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_INPUT_EVENT_H_
