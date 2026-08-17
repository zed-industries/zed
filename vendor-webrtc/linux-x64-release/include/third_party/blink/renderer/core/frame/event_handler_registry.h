// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_EVENT_HANDLER_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_EVENT_HANDLER_REGISTRY_H_

#include <array>

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"  // TODO(sashab): Remove this.
#include "third_party/blink/renderer/platform/heap/forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/hash_counted_set.h"

namespace blink {

class AddEventListenerOptions;
class Document;
class EventTarget;
class LocalFrame;
class Page;

// We use UntracedMember<> here to do custom weak processing.
typedef HashCountedSet<UntracedMember<EventTarget>> EventTargetSet;

// Registry for keeping track of event handlers. Note that only handlers on
// documents that can be rendered or can receive input (i.e., are attached to a
// Page) are registered here. Each local root has an EventHandlerRegistry;
// event targets for a frame may only be registered with the
// EventHandlerRegistry of its corresponding local root.
class CORE_EXPORT EventHandlerRegistry final
    : public GarbageCollected<EventHandlerRegistry> {
 public:
  explicit EventHandlerRegistry(LocalFrame&);
  ~EventHandlerRegistry();

  // Supported event handler classes. Note that each one may correspond to
  // multiple event types.
  enum EventHandlerClass {
    kScrollEvent,
    kWheelEventBlocking,
    kWheelEventPassive,
    kTouchAction,
    kTouchStartOrMoveEventBlocking,
    kTouchStartOrMoveEventBlockingLowLatency,
    kTouchStartOrMoveEventPassive,
    kTouchEndOrCancelEventBlocking,
    kTouchEndOrCancelEventPassive,
    kPointerEvent,  // This includes all pointerevents excluding
                    // pointerrawupdate.
    kPointerRawUpdateEvent,
#if DCHECK_IS_ON()
    // Additional event categories for verifying handler tracking logic.
    kEventsForTesting,
#endif
    kEventHandlerClassCount,  // Must be the last entry.
  };

  // Returns true if the Page has event handlers of the specified class.
  bool HasEventHandlers(EventHandlerClass) const;

  // Returns a set of EventTargets which have registered handlers of the given
  // class.
  const EventTargetSet* EventHandlerTargets(EventHandlerClass) const;

  // Registration and management of event handlers attached to EventTargets.
  void DidAddEventHandler(EventTarget&,
                          const AtomicString& event_type,
                          const AddEventListenerOptions*);
  void DidAddEventHandler(EventTarget&, EventHandlerClass);
  void DidRemoveEventHandler(EventTarget&,
                             const AtomicString& event_type,
                             const AddEventListenerOptions*);
  void DidRemoveEventHandler(EventTarget&, EventHandlerClass);
  void DidRemoveAllEventHandlers(EventTarget&);

  void DidMoveIntoPage(EventTarget&);
  void DidMoveOutOfPage(EventTarget&);

  // Either |documentDetached| or |didMove{Into,OutOf,Between}Pages| must
  // be called whenever the Page that is associated with a registered event
  // target changes. This ensures the registry does not end up with stale
  // references to handlers that are no longer related to it.
  void DocumentDetached(Document&);

  void Trace(Visitor*) const;

 private:
  enum ChangeOperation {
    kAdd,       // Add a new event handler.
    kRemove,    // Remove an existing event handler.
    kRemoveAll  // Remove any and all existing event handlers for a given
                // target.
  };

  // Returns true if |eventType| belongs to a class this registry tracks.
  static bool EventTypeToClass(const AtomicString& event_type,
                               const AddEventListenerOptions*,
                               EventHandlerClass* result);

  // Returns true if the operation actually added a new target or completely
  // removed an existing one.
  void UpdateEventHandlerTargets(ChangeOperation,
                                 EventHandlerClass,
                                 EventTarget*);

  // Called on the EventHandlerRegistry of the root Document to notify
  // clients when we have added or remove a handler for a given event class.
  // |hasActiveHandlers| can be used to distinguish between having and not
  // having an active handler.
  void NotifyHandlersChanged(EventTarget*,
                             EventHandlerClass,
                             bool has_active_handlers);

  // Record a change operation to a given event handler class and notify any
  // parent registry and other clients accordingly.
  void UpdateEventHandlerOfType(ChangeOperation,
                                const AtomicString& event_type,
                                const AddEventListenerOptions*,
                                EventTarget*);

  bool UpdateEventHandlerInternal(ChangeOperation,
                                  EventHandlerClass,
                                  EventTarget*);

  void UpdateAllEventHandlers(ChangeOperation, EventTarget&);

  void CheckConsistency(EventHandlerClass) const;

  Page* GetPage() const;

  void ProcessCustomWeakness(const LivenessBroker&);

  Member<LocalFrame> frame_;
  std::array<EventTargetSet, kEventHandlerClassCount> targets_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_EVENT_HANDLER_REGISTRY_H_
