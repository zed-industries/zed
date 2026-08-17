/*
 * Copyright (C) 2011 Google Inc. All Rights Reserved.
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
 *  THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCRIPTED_ANIMATION_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCRIPTED_ANIMATION_CONTROLLER_H_

#include "base/functional/callback.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/frame_request_callback_collection.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_state_observer.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/string_impl.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Event;
class EventTarget;
class LocalDOMWindow;
class MediaQueryListListener;
class PageAnimator;

class CORE_EXPORT ScriptedAnimationController
    : public GarbageCollected<ScriptedAnimationController>,
      public ExecutionContextLifecycleStateObserver,
      public NameClient {
 public:
  explicit ScriptedAnimationController(LocalDOMWindow*);
  ~ScriptedAnimationController() override = default;

  void Trace(Visitor*) const override;
  const char* NameInHeapSnapshot() const override {
    return "ScriptedAnimationController";
  }

  // Runs all the video.requestVideoFrameCallback() callbacks associated with
  // one HTMLVideoElement. |double| is the current frame time in milliseconds
  // (e.g. |current_frame_time_ms_|), to be passed as the "now" parameter
  // when running the callbacks.
  using ExecuteVfcCallback = base::OnceCallback<void(double)>;

  // Animation frame callbacks are used for requestAnimationFrame().
  typedef int CallbackId;
  CallbackId RegisterFrameCallback(FrameCallback*);
  void CancelFrameCallback(CallbackId);
  // Returns true if any callback is currently registered.
  bool HasFrameCallback() const;

  // Queues up the execution of video.requestVideoFrameCallback() callbacks for
  // a specific HTMLVideoELement, as part of the next rendering steps.
  void ScheduleVideoFrameCallbacksExecution(ExecuteVfcCallback);

  // Animation frame events are used for resize events, scroll events, etc.
  void EnqueueEvent(Event*);
  void EnqueuePerFrameEvent(Event*);

  // Animation frame tasks are used for Fullscreen.
  void EnqueueTask(base::OnceClosure);

  // Used for the MediaQueryList change event.
  void EnqueueMediaQueryChangeListeners(
      HeapVector<Member<MediaQueryListListener>>&);

  void ContextLifecycleStateChanged(mojom::FrameLifecycleState) final;
  void ContextDestroyed() final {}

  void DispatchEventsAndCallbacksForPrinting();

  LocalDOMWindow* GetWindow() const;
  void ScheduleAnimationIfNeeded();

  void RunTasks();
  using DispatchFilter = base::RepeatingCallback<bool(Event*)>;
  bool DispatchEvents(DispatchFilter filter = DispatchFilter{});
  void ExecuteFrameCallbacks();
  void ExecuteVideoFrameCallbacks();
  void CallMediaQueryListListeners();
  void SetCurrentFrameTimeMs(double time_ms) {
    current_frame_time_ms_ = time_ms;
  }
  void SetCurrentFrameLegacyTimeMs(double time_ms) {
    current_frame_legacy_time_ms_ = time_ms;
  }
  // A helper function that is called by more than one callsite.
  PageAnimator* GetPageAnimator();
  bool HasScheduledFrameTasks() const;

 private:
  ALWAYS_INLINE bool InsertToPerFrameEventsMap(const Event* event);
  ALWAYS_INLINE void EraseFromPerFrameEventsMap(const Event* event);

  FrameRequestCallbackCollection callback_collection_;
  Vector<base::OnceClosure> task_queue_;
  Vector<ExecuteVfcCallback> vfc_execution_queue_;
  HeapVector<Member<Event>> event_queue_;
  using PerFrameEventsMap =
      HeapHashMap<Member<const EventTarget>, HashSet<const StringImpl*>>;
  PerFrameEventsMap per_frame_events_;
  using MediaQueryListListeners = HeapVector<Member<MediaQueryListListener>>;
  MediaQueryListListeners media_query_list_listeners_;
  // This is used to quickly lookup if a listener exists in
  // media_query_list_listeners_. The contents should be exactly the same.
  HeapHashSet<Member<MediaQueryListListener>> media_query_list_listeners_set_;
  double current_frame_time_ms_ = 0.0;
  double current_frame_legacy_time_ms_ = 0.0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCRIPTED_ANIMATION_CONTROLLER_H_
