/*
 * Copyright (C) 2008 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_TIME_CONTAINER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_TIME_CONTAINER_H_

#include "base/dcheck_is_on.h"
#include "base/time/time.h"
#include "third_party/blink/public/mojom/webpreferences/web_preferences.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/svg/animation/priority_queue.h"
#include "third_party/blink/renderer/core/svg/animation/smil_time.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_counted_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class Document;
class SVGElement;
class SVGSMILElement;
class SVGSVGElement;

class CORE_EXPORT SMILTimeContainer final
    : public GarbageCollected<SMILTimeContainer> {
 public:
  explicit SMILTimeContainer(SVGSVGElement& owner);
  ~SMILTimeContainer();

  void Schedule(SVGSMILElement*);
  void Reschedule(SVGSMILElement*, SMILTime interval_time);
  void Unschedule(SVGSMILElement*);

  // Returns the current animation time.
  SMILTime Elapsed() const;
  // Returns the time that we last updated timed elements to. This differs from
  // the above in that it only moves during animation update steps.
  SMILTime LatestUpdatePresentationTime() const;

  bool IsPaused() const;
  bool IsStarted() const;

  void Start();
  void Pause();
  void Unpause();
  void SetElapsed(SMILTime);

  // True if an animation frame is successfully scheduled.
  bool ServiceAnimations();
  bool HasAnimations() const;

  void ResetDocumentTime();
  void SetDocumentOrderIndexesDirty() { document_order_indexes_dirty_ = true; }

  // Advance the animation timeline a single frame.
  void AdvanceFrameForTesting();
  bool EventsDisabled() const { return !should_dispatch_events_; }

  void Trace(Visitor*) const;

  void DidAttachLayoutObject();

 private:
  enum FrameSchedulingState {
    // No frame scheduled.
    kIdle,
    // Scheduled a wakeup to update the animation values.
    kSynchronizeAnimations,
    // Scheduled a wakeup to trigger an animation frame.
    kFutureAnimationFrame,
    // Scheduled a animation frame for continuous update.
    kAnimationFrame
  };

  bool IsTimelineRunning() const;
  void SynchronizeToDocumentTimeline();
  void ScheduleAnimationFrame(base::TimeDelta delay_time,
                              bool disable_throttling);
  void CancelAnimationFrame();
  void WakeupTimerFired(TimerBase*);
  mojom::blink::ImageAnimationPolicy AnimationPolicy() const;
  bool AnimationsDisabled() const;
  class TimingUpdate;
  bool UpdateAnimationsAndScheduleFrameIfNeeded(TimingUpdate&);
  void PrepareSeek(TimingUpdate&);
  void ResetIntervals();
  void UpdateIntervals(TimingUpdate&);
  void UpdateTimedElements(TimingUpdate&);
  bool ApplyTimedEffects(SMILTime elapsed);
  SMILTime NextProgressTime(SMILTime presentation_time,
                            bool disable_throttling) const;
  void ServiceOnNextFrame();
  void ScheduleWakeUp(base::TimeDelta delay_time, FrameSchedulingState);
  bool HasPendingSynchronization() const;
  void SetPresentationTime(SMILTime new_presentation_time);
  SMILTime ClampPresentationTime(SMILTime presentation_time) const;

  void UpdateDocumentOrderIndexes();

  SVGSVGElement& OwnerSVGElement() const;
  Document& GetDocument() const;

  // The latest "restart" time for the time container's timeline. If the
  // timeline has not been manipulated (seeked, paused) this will be zero.
  SMILTime presentation_time_;
  // The maximum possible presentation time. When this time is reached
  // animations will stop.
  SMILTime max_presentation_time_;
  // The state all SVGSMILElements should be at.
  SMILTime latest_update_time_;
  // The time on the document timeline corresponding to |presentation_time_|.
  base::TimeDelta reference_time_;

  FrameSchedulingState frame_scheduling_state_;
  bool started_ : 1;  // The timeline has been started.
  bool paused_ : 1;   // The timeline is paused.

  const bool should_dispatch_events_ : 1;
  bool document_order_indexes_dirty_ : 1;
  bool is_updating_intervals_;

  HeapTaskRunnerTimer<SMILTimeContainer> wakeup_timer_;

  using AnimatedTargets = HeapHashCountedSet<WeakMember<SVGElement>>;
  AnimatedTargets animated_targets_;

  PriorityQueue<SMILTime, SVGSMILElement> priority_queue_;

  Member<SVGSVGElement> owner_svg_element_;

#if DCHECK_IS_ON()
  friend class AnimationTargetsMutationsForbidden;
  // This boolean will catch any attempts to mutate (schedule/unschedule)
  // |scheduled_animations_| when it is set to true.
  bool prevent_animation_targets_changes_ = false;
#endif

  bool AnimationTargetsMutationsAllowed() const {
#if DCHECK_IS_ON()
    return !prevent_animation_targets_changes_;
#else
    return true;
#endif
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_TIME_CONTAINER_H_
