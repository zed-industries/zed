/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2006 Alexey Proskuryakov (ap@webkit.org)
 *           (C) 2007, 2008 Nikolas Zimmermann <zimmermann@kde.org>
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_TARGET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_TARGET_H_

#include "third_party/blink/public/platform/task_type.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_dispatch_result.h"
#include "third_party/blink/renderer/core/dom/events/event_listener_map.h"
#include "third_party/blink/renderer/core/event_type_names.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class AddEventListenerOptionsResolved;
class DOMWindow;
class Event;
class ExceptionState;
class ExecutionContext;
class LocalDOMWindow;
class MessagePort;
class Node;
class Observable;
class ObservableEventListenerOptions;
class ScriptState;
class ServiceWorker;
class V8EventListener;
class V8UnionAddEventListenerOptionsOrBoolean;
class V8UnionBooleanOrEventListenerOptions;

// Macros to define an attribute event listener.
//  |lower_name| - Lower-cased event type name.  e.g. |focus|
//  |symbol_name| - C++ symbol name in event_type_names namespace. e.g. |kFocus|
// FIXME: These macros should be split into separate DEFINE and DECLARE
// macros to avoid causing so many header includes.

#define DEFINE_ATTRIBUTE_EVENT_LISTENER(lower_name, symbol_name)        \
  EventListener* on##lower_name() {                                     \
    return GetAttributeEventListener(event_type_names::symbol_name);    \
  }                                                                     \
  void setOn##lower_name(EventListener* listener) {                     \
    SetAttributeEventListener(event_type_names::symbol_name, listener); \
  }

#define DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(lower_name, symbol_name)  \
  static EventListener* on##lower_name(EventTarget& eventTarget) {       \
    return eventTarget.GetAttributeEventListener(                        \
        event_type_names::symbol_name);                                  \
  }                                                                      \
  static void setOn##lower_name(EventTarget& eventTarget,                \
                                EventListener* listener) {               \
    eventTarget.SetAttributeEventListener(event_type_names::symbol_name, \
                                          listener);                     \
  }

#define DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(lower_name, symbol_name)        \
  EventListener* on##lower_name() {                                            \
    return GetDocumentForWindowEventHandler().GetWindowAttributeEventListener( \
        event_type_names::symbol_name);                                        \
  }                                                                            \
  void setOn##lower_name(EventListener* listener) {                            \
    GetDocumentForWindowEventHandler().SetWindowAttributeEventListener(        \
        event_type_names::symbol_name, listener);                              \
  }

class CORE_EXPORT EventTargetData final
    : public GarbageCollected<EventTargetData> {
 public:
  EventTargetData();
  EventTargetData(const EventTargetData&) = delete;
  EventTargetData& operator=(const EventTargetData&) = delete;
  ~EventTargetData();

  void Trace(Visitor*) const;

  EventListenerMap event_listener_map;
};

// All DOM event targets extend EventTarget. The spec is defined here:
// https://dom.spec.whatwg.org/#interface-eventtarget
// EventTarget objects allow us to add and remove an event
// listeners of a specific event type. Each EventTarget object also represents
// the target to which an event is dispatched when something has occurred.
// All nodes are EventTargets, some other event targets include: XMLHttpRequest,
// AudioNode and AudioContext.

// To make your class an EventTarget, follow these steps:
// - Make your IDL interface inherit from EventTarget.
// - In your class declaration, EventTarget must come first in the base class
//   list. If your class is non-final, your class must be the first base class
//   for any derived classes as well.
// - If you added an onfoo attribute, use DEFINE_ATTRIBUTE_EVENT_LISTENER(foo)
//   in your class declaration. Add "attribute EventHandler onfoo;" to the IDL
//   file.
// - Override EventTarget::interfaceName() and getExecutionContext(). The former
//   will typically return EventTargetNames::YourClassName. The latter will
//   return ExecutionContextLifecycleObserver::executionContext (if you are an
//   ExecutionContextLifecycleObserver)
//   or the document you're in.
class CORE_EXPORT EventTarget : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~EventTarget() override;

  virtual const AtomicString& InterfaceName() const = 0;
  virtual ExecutionContext* GetExecutionContext() const = 0;

  virtual Node* ToNode();
  virtual const DOMWindow* ToDOMWindow() const;
  virtual const LocalDOMWindow* ToLocalDOMWindow() const;
  virtual LocalDOMWindow* ToLocalDOMWindow();
  virtual MessagePort* ToMessagePort();
  virtual ServiceWorker* ToServiceWorker();

  // This method is called when the enqueued event is dispatched.
  // The input is the event type of the current dispatched event.
  virtual void ResetEventQueueStatus(const AtomicString& event_type);

  static EventTarget* Create(ScriptState*);

  // Returns an Observable whose native subscription algorithm adds an event
  // listener of type `event_type` to `this`. See
  // https://wicg.github.io/observable/.
  Observable* when(const AtomicString& event_type,
                   const ObservableEventListenerOptions*);

  bool addEventListener(const AtomicString& event_type, V8EventListener*);
  bool addEventListener(
      const AtomicString& event_type,
      V8EventListener* listener,
      const V8UnionAddEventListenerOptionsOrBoolean* bool_or_options);
  bool addEventListener(const AtomicString& event_type,
                        EventListener*,
                        bool use_capture = false);
  bool addEventListener(const AtomicString& event_type,
                        EventListener*,
                        AddEventListenerOptionsResolved*);

  bool removeEventListener(const AtomicString& event_type, V8EventListener*);
  bool removeEventListener(
      const AtomicString& event_type,
      V8EventListener* listener,
      const V8UnionBooleanOrEventListenerOptions* bool_or_options);
  bool removeEventListener(const AtomicString& event_type,
                           const EventListener*,
                           bool use_capture);
  bool removeEventListener(const AtomicString& event_type,
                           const EventListener*,
                           EventListenerOptions*);
  virtual void RemoveAllEventListeners();

  DispatchEventResult DispatchEvent(Event&);

  void EnqueueEvent(Event&, TaskType);

  // dispatchEventForBindings is intended to only be called from
  // javascript originated calls. This method will validate and may adjust
  // the Event object before dispatching.
  bool dispatchEventForBindings(Event*, ExceptionState&);

  // Used for legacy "onEvent" attribute APIs.
  virtual bool SetAttributeEventListener(const AtomicString& event_type,
                                         EventListener*);
  EventListener* GetAttributeEventListener(const AtomicString& event_type);

  bool HasEventListeners() const;
  bool HasEventListeners(const AtomicString& event_type) const;
  bool HasAnyEventListeners(const Vector<AtomicString>& event_types) const;
  bool HasCapturingEventListeners(const AtomicString& event_type);
  bool HasJSBasedEventListeners(const AtomicString& event_type) const;
  EventListenerVector* GetEventListeners(const AtomicString& event_type);
  // Number of event listeners for |event_type| registered at this event target.
  int NumberOfEventListeners(const AtomicString& event_type) const;

  Vector<AtomicString> EventTypes();

  DispatchEventResult FireEventListeners(Event&);

  static DispatchEventResult GetDispatchEventResult(const Event&);

  virtual bool KeepEventInNode(const Event&) const { return false; }

  virtual bool IsWindowOrWorkerGlobalScope() const { return false; }

  // Returns true if the target is window, window.document, or
  // window.document.body.
  bool IsTopLevelNode();

  EventTargetData* GetEventTargetData();

  // GlobalEventHandlers:
  // These event listener helpers are defined internally for all EventTargets,
  // but they will only actually be web-exposed for interfaces that include
  // GlobalEventHandlers as a mixin in the idl.
  DEFINE_ATTRIBUTE_EVENT_LISTENER(abort, kAbort)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(animationend, kAnimationend)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(animationiteration, kAnimationiteration)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(animationstart, kAnimationstart)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(auxclick, kAuxclick)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(beforeinput, kBeforeinput)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(beforematch, kBeforematch)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(beforetoggle, kBeforetoggle)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(blur, kBlur)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(cancel, kCancel)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(canplay, kCanplay)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(canplaythrough, kCanplaythrough)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(change, kChange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(click, kClick)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(close, kClose)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(command, kCommand)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(contentvisibilityautostatechange,
                                  kContentvisibilityautostatechange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(contextmenu, kContextmenu)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(contextlost, kContextlost)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(contextrestored, kContextrestored)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(cuechange, kCuechange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(dblclick, kDblclick)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(drag, kDrag)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(dragend, kDragend)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(dragenter, kDragenter)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(dragleave, kDragleave)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(dragover, kDragover)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(dragstart, kDragstart)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(drop, kDrop)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(durationchange, kDurationchange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(emptied, kEmptied)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(ended, kEnded)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(error, kError)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(fencedtreeclick, kFencedtreeclick)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(focus, kFocus)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(formdata, kFormdata)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(gotpointercapture, kGotpointercapture)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(input, kInput)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(invalid, kInvalid)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(keydown, kKeydown)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(keypress, kKeypress)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(keyup, kKeyup)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(load, kLoad)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(loadeddata, kLoadeddata)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(loadedmetadata, kLoadedmetadata)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(loadstart, kLoadstart)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(lostpointercapture, kLostpointercapture)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(mousedown, kMousedown)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(mouseenter, kMouseenter)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(mouseleave, kMouseleave)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(mousemove, kMousemove)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(mouseout, kMouseout)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(mouseover, kMouseover)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(mouseup, kMouseup)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(mousewheel, kMousewheel)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(overscroll, kOverscroll)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(pause, kPause)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(play, kPlay)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(playing, kPlaying)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(pointercancel, kPointercancel)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(pointerdown, kPointerdown)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(pointerenter, kPointerenter)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(pointerleave, kPointerleave)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(pointermove, kPointermove)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(pointerout, kPointerout)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(pointerover, kPointerover)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(pointerrawupdate, kPointerrawupdate)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(pointerup, kPointerup)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(popoverhide, kPopoverhide)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(popovershow, kPopovershow)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(progress, kProgress)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(ratechange, kRatechange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(reset, kReset)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(resize, kResize)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(scroll, kScroll)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(scrollend, kScrollend)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(securitypolicyviolation,
                                  kSecuritypolicyviolation)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(seeked, kSeeked)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(seeking, kSeeking)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(select, kSelect)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(selectionchange, kSelectionchange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(selectstart, kSelectstart)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(slotchange, kSlotchange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(scrollsnapchange, kScrollsnapchange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(scrollsnapchanging, kScrollsnapchanging)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(stalled, kStalled)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(submit, kSubmit)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(suspend, kSuspend)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(timeupdate, kTimeupdate)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(toggle, kToggle)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(touchcancel, kTouchcancel)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(touchend, kTouchend)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(touchmove, kTouchmove)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(touchstart, kTouchstart)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(transitioncancel, kTransitioncancel)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(transitionend, kTransitionend)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(transitionrun, kTransitionrun)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(transitionstart, kTransitionstart)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(volumechange, kVolumechange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(waiting, kWaiting)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(webkitanimationend, kWebkitAnimationEnd)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(webkitanimationiteration,
                                  kWebkitAnimationIteration)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(webkitanimationstart, kWebkitAnimationStart)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(webkittransitionend, kWebkitTransitionEnd)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(wheel, kWheel)

  void Trace(Visitor*) const override;

 protected:
  EventTarget();

  virtual bool AddEventListenerInternal(const AtomicString& event_type,
                                        EventListener*,
                                        const AddEventListenerOptionsResolved*);
  bool RemoveEventListenerInternal(const AtomicString& event_type,
                                   const EventListener*,
                                   const EventListenerOptions*);

  // Called when an event listener has been successfully added.
  virtual void AddedEventListener(const AtomicString& event_type,
                                  RegisteredEventListener&);

  // Called when an event listener is removed. The original registration
  // parameters of this event listener are available to be queried.
  virtual void RemovedEventListener(const AtomicString& event_type,
                                    const RegisteredEventListener&);

  virtual DispatchEventResult DispatchEventInternal(Event&);

  EventTargetData& EnsureEventTargetData();

 private:
  LocalDOMWindow* ExecutingWindow();
  void SetDefaultAddEventListenerOptions(const AtomicString& event_type,
                                         EventListener*,
                                         AddEventListenerOptionsResolved*);

  RegisteredEventListener* GetAttributeRegisteredEventListener(
      const AtomicString& event_type);

  // Fire event listeners. This method makes a copy of the `EventListenerVector`
  // on invocation to match the HTML spec. Do not try to optimize it away.
  // The spec snapshots the array at the beginning of a dispatch so that
  // listeners adding or removing other event listeners during dispatch is
  // done in a consistent way.
  using EventListenerVectorSnapshot =
      HeapVector<Member<RegisteredEventListener>, 1>;
  bool FireEventListeners(Event&,
                          EventTargetData*,
                          EventListenerVectorSnapshot);
  void CountLegacyEvents(const AtomicString& legacy_type_name,
                         EventListenerVector*,
                         EventListenerVector*);

  void DispatchEnqueuedEvent(Event*, ExecutionContext*);

  Member<EventTargetData> data_;

  friend class EventListenerIterator;
};

DISABLE_CFI_PERF
inline bool EventTarget::HasEventListeners() const {
  // FIXME: We should have a const version of eventTargetData.
  if (const EventTargetData* d =
          const_cast<EventTarget*>(this)->GetEventTargetData())
    return !d->event_listener_map.IsEmpty();
  return false;
}

DISABLE_CFI_PERF
inline bool EventTarget::HasEventListeners(
    const AtomicString& event_type) const {
  // FIXME: We should have const version of eventTargetData.
  if (const EventTargetData* d =
          const_cast<EventTarget*>(this)->GetEventTargetData())
    return d->event_listener_map.Contains(event_type);
  return false;
}

DISABLE_CFI_PERF
inline bool EventTarget::HasAnyEventListeners(
    const Vector<AtomicString>& event_types) const {
  for (const AtomicString& event_type : event_types) {
    if (HasEventListeners(event_type))
      return true;
  }
  return false;
}

inline bool EventTarget::HasCapturingEventListeners(
    const AtomicString& event_type) {
  EventTargetData* d = GetEventTargetData();
  if (!d)
    return false;
  return d->event_listener_map.ContainsCapturing(event_type);
}

inline bool EventTarget::HasJSBasedEventListeners(
    const AtomicString& event_type) const {
  // TODO(rogerj): We should have const version of eventTargetData.
  if (const EventTargetData* d =
          const_cast<EventTarget*>(this)->GetEventTargetData())
    return d->event_listener_map.ContainsJSBasedEventListeners(event_type);
  return false;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_EVENT_TARGET_H_
