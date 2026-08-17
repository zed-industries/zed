/*
 * Copyright (C) 2001 Peter Kelly (pmk@post.com)
 * Copyright (C) 2001 Tobias Anton (anton@stud.fbi.fh-darmstadt.de)
 * Copyright (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008 Apple Inc. All rights
 * reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_H_

#include "base/check_op.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_dispatch_result.h"
#include "third_party/blink/renderer/core/probe/async_task_context.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class DOMWrapperWorld;
class EventDispatcher;
class EventInit;
class EventPath;
class EventTarget;
class Node;
class ScriptState;

class CORE_EXPORT Event : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum class Bubbles {
    kYes,
    kNo,
  };

  enum class Cancelable {
    kYes,
    kNo,
  };

  enum class PhaseType : uint8_t {
    kNone = 0,
    kCapturingPhase = 1,
    kAtTarget = 2,
    kBubblingPhase = 3
  };

  enum class ComposedMode {
    kComposed,
    kScoped,
  };

  enum class PassiveMode : uint8_t {
    // Not passive, default initialized.
    kNotPassiveDefault,
    // Not passive, explicitly specified.
    kNotPassive,
    // Passive, explicitly specified.
    kPassive,
    // Passive, not explicitly specified and forced due to document level
    // listener.
    kPassiveForcedDocumentLevel,
    // Passive, default initialized.
    kPassiveDefault,
  };

  static Event* Create() { return MakeGarbageCollected<Event>(); }

  static Event* Create(const AtomicString& type) {
    return MakeGarbageCollected<Event>(type, Bubbles::kNo, Cancelable::kNo);
  }
  static Event* CreateCancelable(const AtomicString& type) {
    return MakeGarbageCollected<Event>(type, Bubbles::kNo, Cancelable::kYes);
  }
  static Event* CreateBubble(const AtomicString& type) {
    return MakeGarbageCollected<Event>(type, Bubbles::kYes, Cancelable::kNo);
  }
  static Event* CreateCancelableBubble(const AtomicString& type) {
    return MakeGarbageCollected<Event>(type, Bubbles::kYes, Cancelable::kYes);
  }

  static Event* Create(const AtomicString& type, const EventInit* initializer) {
    return MakeGarbageCollected<Event>(type, initializer);
  }

  // Creates event objects for use with fenced frames. Because timestamps are
  // a potential privacy leak from the frame to its embedder, clamp all of them
  // to the epoch.
  static Event* CreateFenced(const AtomicString& type) {
    return MakeGarbageCollected<Event>(type, Bubbles::kYes, Cancelable::kYes,
                                       base::TimeTicks::UnixEpoch());
  }

  Event();
  Event(const AtomicString& type,
        Bubbles,
        Cancelable,
        ComposedMode,
        base::TimeTicks platform_time_stamp);
  Event(const AtomicString& type,
        Bubbles,
        Cancelable,
        base::TimeTicks platform_time_stamp);
  Event(const AtomicString& type,
        Bubbles,
        Cancelable,
        ComposedMode = ComposedMode::kScoped);
  Event(const AtomicString& type,
        const EventInit*,
        base::TimeTicks platform_time_stamp);
  Event(const AtomicString& type, const EventInit* init)
      : Event(type, init, base::TimeTicks::Now()) {}
  ~Event() override;

  void initEvent(const AtomicString& type, bool bubbles, bool cancelable);
  void initEvent(const AtomicString& event_type_arg,
                 bool bubbles_arg,
                 bool cancelable_arg,
                 EventTarget* related_target);

  const AtomicString& type() const { return type_; }
  void SetType(const AtomicString& type) { type_ = type; }

  EventTarget* target() const { return target_.Get(); }
  void SetTarget(EventTarget*);

  EventTarget* currentTarget() const;
  void SetCurrentTarget(EventTarget* current_target) {
    current_target_ = current_target;
  }

  // This callback is invoked when an event listener has been dispatched
  // at the current target. It should only be used to influence UMA metrics
  // and not change functionality since observing the presence of listeners
  // is dangerous.
  virtual void DoneDispatchingEventAtCurrentTarget() {}

  void SetRelatedTargetIfExists(EventTarget* related_target);

  PhaseType eventPhase() const { return event_phase_; }
  void SetEventPhase(PhaseType event_phase) { event_phase_ = event_phase; }

  void SetFireOnlyCaptureListenersAtTarget(
      bool fire_only_capture_listeners_at_target) {
    DCHECK_EQ(event_phase_, PhaseType::kAtTarget);
    fire_only_capture_listeners_at_target_ =
        fire_only_capture_listeners_at_target;
  }

  void SetFireOnlyNonCaptureListenersAtTarget(
      bool fire_only_non_capture_listeners_at_target) {
    DCHECK_EQ(event_phase_, PhaseType::kAtTarget);
    fire_only_non_capture_listeners_at_target_ =
        fire_only_non_capture_listeners_at_target;
  }

  bool FireOnlyCaptureListenersAtTarget() const {
    return fire_only_capture_listeners_at_target_;
  }
  bool FireOnlyNonCaptureListenersAtTarget() const {
    return fire_only_non_capture_listeners_at_target_;
  }

  bool bubbles() const { return bubbles_; }
  bool cancelable() const { return cancelable_; }
  bool composed() const { return composed_; }

  // Event creation timestamp in milliseconds. It returns a DOMHighResTimeStamp
  // using the platform timestamp (see |platform_time_stamp_|).
  // For more info see http://crbug.com/160524
  double timeStamp(ScriptState*) const;
  base::TimeTicks PlatformTimeStamp() const { return platform_time_stamp_; }

  void stopPropagation() { propagation_stopped_ = true; }
  void SetStopPropagation(bool stop_propagation) {
    propagation_stopped_ = stop_propagation;
  }
  void stopImmediatePropagation() { immediate_propagation_stopped_ = true; }
  void SetStopImmediatePropagation(bool stop_immediate_propagation) {
    immediate_propagation_stopped_ = stop_immediate_propagation;
  }

  // IE Extensions
  EventTarget* srcElement() const {
    return target();
  }  // MSIE extension - "the object that fired the event"

  bool legacyReturnValue(ScriptState*) const;
  void setLegacyReturnValue(ScriptState*, bool return_value);

  virtual const AtomicString& InterfaceName() const;
  bool HasInterface(const AtomicString&) const;

  // These events are general classes of events.
  virtual bool IsUIEvent() const;
  virtual bool IsMouseEvent() const;
  virtual bool IsFocusEvent() const;
  virtual bool IsKeyboardEvent() const;
  virtual bool IsTouchEvent() const;
  virtual bool IsGestureEvent() const;
  virtual bool IsWheelEvent() const;
  virtual bool IsPointerEvent() const;
  virtual bool IsHighlightPointerEvent() const;
  virtual bool IsInputEvent() const;
  virtual bool IsCompositionEvent() const;

  // Drag events are a subset of mouse events.
  virtual bool IsDragEvent() const;

  // These events lack a DOM interface.
  virtual bool IsClipboardEvent() const;
  virtual bool IsBeforeTextInsertedEvent() const;

  virtual bool IsBeforeCreatePolicyEvent() const;
  virtual bool IsBeforeUnloadEvent() const;
  virtual bool IsErrorEvent() const;

  bool PropagationStopped() const {
    return propagation_stopped_ || immediate_propagation_stopped_;
  }
  bool ImmediatePropagationStopped() const {
    return immediate_propagation_stopped_;
  }
  bool WasInitialized() { return was_initialized_; }

  bool defaultPrevented() const { return default_prevented_; }
  virtual void preventDefault();

  bool DefaultHandled() const { return default_handled_; }
  void SetDefaultHandled() { default_handled_ = true; }

  bool cancelBubble(ScriptState* = nullptr) const {
    return PropagationStopped();
  }
  void setCancelBubble(ScriptState*, bool);

  const Event* UnderlyingEvent() const { return underlying_event_.Get(); }
  void SetUnderlyingEvent(const Event*);

  bool HasEventPath() const { return static_cast<bool>(event_path_); }
  EventPath& GetEventPath() const {
    DCHECK(event_path_);
    return *event_path_;
  }
  void InitEventPath(Node&);

  HeapVector<Member<EventTarget>> composedPath(ScriptState*) const;

  bool IsBeingDispatched() const { return eventPhase() != PhaseType::kNone; }

  // Events that must not leak across isolated world, similar to how
  // ErrorEvent behaves, can override this method.
  virtual bool CanBeDispatchedInWorld(const DOMWrapperWorld&) const {
    return true;
  }

  bool isTrusted() const { return is_trusted_; }
  void SetTrusted(bool value) { is_trusted_ = value; }

  // The spec (https://www.w3.org/TR/uievents/#legacy-uievent-event-order)
  // says that `click` events and `keydown` events should generated *trusted*
  // `DOMActivate` and `click` synthetic events. That is so support legacy
  // behavior that click events would run default event handler behavior.
  // This function checks whether the provided event is "actually" trusted,
  // in that its underlying events are all trusted, including the originating
  // event.
  bool IsFullyTrusted() const;

  void SetComposed(bool composed) {
    DCHECK(!IsBeingDispatched());
    composed_ = composed;
  }

  void SetHandlingPassive(PassiveMode);

  bool PreventDefaultCalledOnUncancelableEvent() const {
    return prevent_default_called_on_uncancelable_event_;
  }

  bool LegacyDidListenersThrow() const {
    return legacy_did_listeners_throw_flag_;
  }

  void LegacySetDidListenersThrowFlag() {
    legacy_did_listeners_throw_flag_ = true;
  }

  void SetCopyEventPathFromUnderlyingEvent() {
    copy_event_path_from_underlying_event_ = true;
  }

  // In general, event listeners do not run when related execution contexts are
  // paused.  However, when this function returns true, event listeners ignore
  // the pause and run.
  virtual bool ShouldDispatchEvenWhenExecutionContextIsPaused() const {
    return false;
  }

  virtual DispatchEventResult DispatchEvent(EventDispatcher&);

  probe::AsyncTaskContext* async_task_context() { return &async_task_context_; }

  void Trace(Visitor*) const override;

 protected:
  virtual void ReceivedTarget();

  void SetBubbles(bool bubble) { bubbles_ = bubble; }

  PassiveMode HandlingPassive() const { return handling_passive_; }

 private:
  AtomicString type_;
  bool bubbles_ : 1;
  bool cancelable_ : 1;
  bool composed_ : 1;

  bool propagation_stopped_ : 1;
  bool immediate_propagation_stopped_ : 1;
  bool default_prevented_ : 1;
  bool default_handled_ : 1;
  bool was_initialized_ : 1;
  bool is_trusted_ : 1;

  // Whether preventDefault was called on uncancelable event.
  bool prevent_default_called_on_uncancelable_event_ : 1;

  // Whether any of listeners have thrown an exception or not.
  // Corresponds to |legacyOutputDidListenersThrowFlag| in DOM standard.
  // https://dom.spec.whatwg.org/#dispatching-events
  // https://dom.spec.whatwg.org/#concept-event-listener-inner-invoke
  bool legacy_did_listeners_throw_flag_ : 1;

  bool fire_only_capture_listeners_at_target_ : 1;
  bool fire_only_non_capture_listeners_at_target_ : 1;

  bool copy_event_path_from_underlying_event_ : 1;

  PassiveMode handling_passive_;
  PhaseType event_phase_;
  probe::AsyncTaskContext async_task_context_;

  Member<EventTarget> current_target_;
  Member<EventTarget> target_;
  Member<const Event> underlying_event_;
  Member<EventPath> event_path_;
  // The monotonic platform time in seconds, for input events it is the
  // event timestamp provided by the host OS and reported in the original
  // WebInputEvent instance.
  base::TimeTicks platform_time_stamp_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_H_
