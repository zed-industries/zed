/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATION_H_

#include <memory>

#include "base/gtest_prod_util.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_animation_play_state.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_replace_state.h"
#include "third_party/blink/renderer/core/animation/animation_effect.h"
#include "third_party/blink/renderer/core/animation/animation_effect_owner.h"
#include "third_party/blink/renderer/core/animation/compositor_animations.h"
#include "third_party/blink/renderer/core/animation/keyframe_effect.h"
#include "third_party/blink/renderer/core/animation/timeline_offset.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/animation/compositor_animation_client.h"
#include "third_party/blink/renderer/platform/animation/compositor_animation_delegate.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"

namespace blink {

class AnimationTimeline;
class Element;
class PaintArtifactCompositor;
class StyleChangeReasonForTracing;
class TreeScope;
class TimelineRange;
class AnimationTrigger;

// The state of the animation's trigger.
// https://drafts.csswg.org/web-animations-2/#trigger-state
enum class AnimationTriggerState {
  // The initial state of the trigger. The trigger has not yet taken any action
  // on the animation.
  kIdle,
  // The last action taken by the trigger on the animation was due to entering
  // the trigger range.
  kPrimary,
  // The last action taken by the trigger on the animation was due to exiting
  // the exit range.
  kInverse,
};

class CORE_EXPORT Animation : public EventTarget,
                              public ActiveScriptWrappable<Animation>,
                              public ExecutionContextLifecycleObserver,
                              public CompositorAnimationDelegate,
                              public CompositorAnimationClient,
                              public AnimationEffectOwner {
  DEFINE_WRAPPERTYPEINFO();
  USING_PRE_FINALIZER(Animation, Dispose);

 public:
  // Priority for sorting getAnimation by Animation class, arranged from lowest
  // priority to highest priority as per spec:
  // https://w3.org/TR/web-animations-1/#dom-document-getanimations
  enum AnimationClassPriority {
    kCssTransitionPriority,
    kCssAnimationPriority,
    kDefaultPriority
  };

  // kTreeOrder uses the order in the DOM to determine animations' relative
  // position.
  // kPointerOrder simply compares Element pointers and determine animations'
  // relative position.
  enum CompareAnimationsOrdering { kTreeOrder, kPointerOrder };

  // Only expect timing accuracy to within 1 microsecond.
  // https://w3.org/TR/web-animations-1/#precision-of-time-values.
  static constexpr double kTimeToleranceMs = 0.001;

  static Animation* Create(AnimationEffect*,
                           AnimationTimeline*,
                           ExceptionState& = ASSERT_NO_EXCEPTION);

  // Web Animations API IDL constructors.
  static Animation* Create(ExecutionContext*,
                           AnimationEffect*,
                           ExceptionState&);
  static Animation* Create(ExecutionContext*,
                           AnimationEffect*,
                           AnimationTimeline*,
                           ExceptionState&);

  Animation(ExecutionContext*,
            AnimationTimeline*,
            AnimationEffect*,
            AnimationTrigger*);
  ~Animation() override;
  void Dispose();

  virtual bool IsCSSAnimation() const { return false; }
  virtual bool IsCSSTransition() const { return false; }
  virtual Element* OwningElement() const { return nullptr; }
  virtual void ClearOwningElement() {}
  bool IsOwned() const { return OwningElement(); }

  // Returns whether the animation is finished.
  bool Update(TimingUpdateReason);

  // AnimationEffectOwner:
  void UpdateIfNecessary() override;
  void EffectInvalidated() override;
  bool IsEventDispatchAllowed() const override;
  Animation* GetAnimation() override { return this; }

  // timeToEffectChange returns:
  //  nullopt                  - if this animation is no longer in effect
  //  AnimationTimeDelta()     - if this animation requires an update on the
  //                             next frame
  //  AnimationTimeDelta() > 0 - if this animation requires an update
  //                             after 'n' units of time
  std::optional<AnimationTimeDelta> TimeToEffectChange();

  void cancel();

  V8CSSNumberish* currentTime() const;
  std::optional<AnimationTimeDelta> CurrentTimeInternal() const;
  void setCurrentTime(const V8CSSNumberish* current_time,
                      ExceptionState& exception_state);
  void SetCurrentTimeInternal(AnimationTimeDelta);

  std::optional<AnimationTimeDelta> UnlimitedCurrentTime() const;

  // https://drafts.csswg.org/web-animations-2/#the-overall-progress-of-an-animation
  std::optional<double> overallProgress() const;

  // https://w3.org/TR/web-animations-1/#play-states
  V8AnimationPlayState::Enum CalculateAnimationPlayState() const;

  // As a web exposed API, playState must update style and layout if the play
  // state may be affected by it (see CSSAnimation::playState), whereas
  // PlayStateString can be used to query the current play state.
  virtual V8AnimationPlayState playState() const;

  bool PendingInternal() const;

  // As a web exposed API, pending must update style and layout if the pending
  // status may be affected by it (see CSSAnimation::pending), whereas
  // PendingInternal can be used to query the current pending status.
  virtual bool pending() const;

  virtual void pause(ExceptionState& = ASSERT_NO_EXCEPTION);
  virtual void play(ExceptionState& = ASSERT_NO_EXCEPTION);
  virtual void reverse(ExceptionState& = ASSERT_NO_EXCEPTION);
  void finish(ExceptionState& = ASSERT_NO_EXCEPTION);
  void updatePlaybackRate(double playback_rate,
                          ExceptionState& = ASSERT_NO_EXCEPTION);

  ScriptPromise<Animation> finished(ScriptState*);
  ScriptPromise<Animation> ready(ScriptState*);

  bool Paused() const {
    return CalculateAnimationPlayState() ==
               V8AnimationPlayState::Enum::kPaused &&
           !is_paused_for_testing_;
  }

  bool Playing() const override {
    return CalculateAnimationPlayState() ==
               V8AnimationPlayState::Enum::kRunning &&
           !Limited() && !is_paused_for_testing_;
  }

  // Differs from Playing() in the case of a non-monotonic timeline outside the
  // active range. A finished animation is not Playing since no update is
  // required due to passage of time. This behavior also works for scroll-linked
  // animations since until the animation exits the finished state, no updates
  // are required.  When in the before phase, the normal passage of time will
  // trigger an effect change; however, the same is not true for scroll-linked
  // animations.
  bool EffectivelyPlaying() const;

  // Notification that the animation is entering or exiting the active phase.
  void OnActivePhaseStateChange(bool in_active_phase);

  bool Limited() const { return Limited(CurrentTimeInternal()); }
  bool FinishedInternal() const { return finished_; }

  DEFINE_ATTRIBUTE_EVENT_LISTENER(finish, kFinish)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(cancel, kCancel)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(remove, kRemove)

  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;
  bool HasPendingActivity() const final;
  void ContextDestroyed() override;

  double playbackRate() const;
  void setPlaybackRate(double, ExceptionState& = ASSERT_NO_EXCEPTION);

  AnimationTimeline* TimelineInternal() { return timeline_.Get(); }
  AnimationTimeline* TimelineInternal() const { return timeline_.Get(); }

  // Note that this function returns the *exposed* timeline, which may be
  // different from the the timeline the Animation is actually attached to.
  //
  // See AnimationTimeline::ExposedTimeline.
  AnimationTimeline* timeline();

  // Converts time to a progress measured as relative completion of the
  // animation (effect end time). This value is used to preserve progress when
  // changing timelines to prevent a discontinuity of the timeline changes while
  // in a paused state. Note that this progress measure is not the same as the
  // percentages used in the web-platform API for scroll-linked animations,
  // which are relative to the timeline duration and not the effect end time.
  std::optional<double> TimeAsAnimationProgress(AnimationTimeDelta time) const;

  virtual void setTimeline(AnimationTimeline* timeline);

  // Animation options for ScrollTimelines.
  // Setting a range boundary via rangeStart or rangeEnd overrides the
  // corresponding CSS properties and resets a "sticky" start time.
  using RangeBoundary = V8UnionStringOrTimelineRangeOffset;
  const RangeBoundary* rangeStart();
  const RangeBoundary* rangeEnd();
  virtual void setRangeStart(const RangeBoundary* range_start,
                             ExceptionState& exception_state);
  virtual void setRangeEnd(const RangeBoundary* range_end,
                           ExceptionState& exception_state);

  const std::optional<TimelineOffset>& GetRangeStartInternal() const {
    return range_start_;
  }
  const std::optional<TimelineOffset>& GetRangeEndInternal() const {
    return range_end_;
  }
  void SetRangeStartInternal(const std::optional<TimelineOffset>& range_start);
  void SetRangeEndInternal(const std::optional<TimelineOffset>& range_end);

  // This method is only called during style update of a CSS animation.
  // Preventing an endpoint from stomping a value set via the rangeStart or
  // rangeEnd API is performed by the caller in CSSAnimations.
  virtual void SetRange(const std::optional<TimelineOffset>& range_start,
                        const std::optional<TimelineOffset>& range_end);

  void UpdateBoundaryAlignment(Timing::NormalizedTiming& timing) const;

  // Called during validation of a scroll timeline to determine if a second
  // style and layout pass is required. During this validation step, we have an
  // up to date snapshot of the timeline and can initialize the start time if
  // required. If the start time or intrinsic iteration duration changes, we
  // need a second style+layout pass even if the timeline snapshot is valid.
  bool OnValidateSnapshot(bool snapshot_changed);

  void OnRangeUpdate();

  bool ResolveTimelineOffsets(const TimelineRange&);

  Document* GetDocument() const;

  V8CSSNumberish* startTime() const;
  std::optional<AnimationTimeDelta> StartTimeInternal() const {
    return start_time_;
  }
  virtual void setStartTime(const V8CSSNumberish* start_time,
                            ExceptionState& exception_state);

  const AnimationEffect* effect() const { return content_.Get(); }
  AnimationEffect* effect() { return content_.Get(); }
  void setEffect(AnimationEffect*);

  void setId(const String& id) { id_ = id; }
  const String& id() const { return id_; }

  // Pausing via this method is not reflected in the value returned by
  // paused() and must never overlap with pausing via pause().
  // Deprecated: Do not use in new tests.
  void PauseForTesting(AnimationTimeDelta pause_time);
  void DisableCompositedAnimationForTesting();

  // This should only be used for CSS
  void Unpause();
  bool ResetsCurrentTimeOnResume() const {
    return reset_current_time_on_resume_;
  }

  void SetOutdated();
  bool Outdated() { return outdated_; }

  enum class CompositorPendingReason {
    kPendingUpdate,        // Update due to an API call that may affect
                           // play state or start time.
    kPendingEffectChange,  // Update that changes the animation effect
                           // including keyframes or active interval.
    kPendingCancel,        // Animation has been canceled, but could restart
                           // conditions permitting.
    kPendingRestart,       // Animation is to be restarted.
    kPendingSafeRestart,   // Animation is to be restarted. We can be certain
                           // that the CompositorPaintStatus won't change.  A compositing decision made in PrePaint for a native-paint-worklet is still valid.
    kPaintWorkletImageCreated,  // A compositable animation was held in limbo
                                // awaiting paint of the paint worklet image. It
                                // can now be started on the compositor.
    kPendingDowngrade  // Paint is forcing the animation to downgrade to
                       // run on the main thread.
  };

  void SetCompositorPending(CompositorPendingReason reason);

  CompositorAnimations::FailureReasons CheckCanStartAnimationOnCompositor(
      const PaintArtifactCompositor* paint_artifact_compositor,
      PropertyHandleSet* unsupported_properties_for_tracing = nullptr) const;
  void StartAnimationOnCompositor(
      const PaintArtifactCompositor* paint_artifact_compositor);
  void CancelAnimationOnCompositor();
  void RestartAnimationOnCompositor(
      CompositorPendingReason reason =
          CompositorPendingReason::kPendingRestart);
  void CancelIncompatibleAnimationsOnCompositor();
  bool HasActiveAnimationsOnCompositor() const;

  void NotifyReady(AnimationTimeDelta ready_time);
  void CommitPendingPlay(AnimationTimeDelta ready_time);
  void CommitPendingPause(AnimationTimeDelta ready_time);
  // CompositorAnimationClient implementation.
  CompositorAnimation* GetCompositorAnimation() const override {
    return compositor_animation_ ? compositor_animation_->GetAnimation()
                                 : nullptr;
  }

  bool Affects(const Element&, const CSSProperty&) const;

  // Returns whether we should continue with the commit for this animation or
  // wait until next commit.
  bool PreCommit(int compositor_group,
                 const PaintArtifactCompositor*,
                 bool start_on_compositor);
  void PostCommit();

  unsigned SequenceNumber() const override { return sequence_number_; }

  int CompositorGroup() const { return compositor_group_; }

  static bool HasLowerCompositeOrdering(
      const Animation* animation1,
      const Animation* animation2,
      CompareAnimationsOrdering compare_animation_type);

  bool EffectSuppressed() const override { return effect_suppressed_; }
  void SetEffectSuppressed(bool);

  void InvalidateKeyframeEffect(const TreeScope&,
                                const StyleChangeReasonForTracing&);
  void InvalidateEffectTargetStyle();
  void InvalidateNormalizedTiming();

  void Trace(Visitor*) const override;

  bool CompositorPending() const { return compositor_pending_; }
  bool CompositorPendingCancel() const {
    return compositor_state_ &&
           compositor_state_->pending_action == CompositorAction::kCancel;
  }
  bool CompositorPendingCancelOrEffectChange() const;

  // Methods for handling removal and persistence of animations.
  bool IsReplaceable();
  void RemoveReplacedAnimation();
  void persist();
  V8ReplaceState replaceState();
  void commitStyles(ExceptionState& = ASSERT_NO_EXCEPTION);
  bool ReplaceStateRemoved() const override {
    return replace_state_ == V8ReplaceState::Enum::kRemoved;
  }
  bool ReplaceStateActive() const {
    return replace_state_ == V8ReplaceState::Enum::kActive;
  }

  // Overridden for CSS animations to force pending animation properties to be
  // applied. This step is required before any web animation API calls that
  // depends on computed values.
  virtual void FlushPendingUpdates() const {}

  bool IsInDisplayLockedSubtree();

  base::TimeDelta ComputeCompositorTimeOffset() const;

  // Updates |compositor_property_animations_have_no_effect_| and marks the
  // animation as pending if it changes.
  void MarkPendingIfCompositorPropertyAnimationChanges(
      const PaintArtifactCompositor*);
  bool CompositorPropertyAnimationsHaveNoEffectForTesting() const {
    return compositor_property_animations_have_no_effect_;
  }
  bool AnimationHasNoEffect() const { return animation_has_no_effect_; }

  // A native paint worklet animation has no visible effect until the deferred
  // paint image has been generated. If the animation is not currently
  // composited we need to restart it on the compositor.
  void OnPaintWorkletImageCreated();

  bool WaitingOnDeferredStartTime() {
    return !start_time_ && (pending_play_ || pending_pause_);
  }

  // Scroll linked animations do not initialize the start time
  // during play or pause as the start time is deferred until timeline
  // validation.
  void SetDeferredStartTimeForTesting(
      AnimationTimeDelta start_time = AnimationTimeDelta()) {
    start_time_ = start_time;
  }

  enum NativePaintWorkletProperties {
    kNoPaintWorklet = 0,
    kBackgroundColorPaintWorklet = 1,
    kClipPathPaintWorklet = 2
  };

  using NativePaintWorkletReasons = uint32_t;
  NativePaintWorkletReasons GetNativePaintWorkletReasons() const;

  static RangeBoundary* ToRangeBoundary(std::optional<TimelineOffset> offset);
  static RangeBoundary* ToRangeBoundary(TimelineOffsetOrAuto offset_or_auto);

  AnimationTrigger* trigger() {
    FlushPendingUpdates();
    return GetTriggerInternal();
  }
  AnimationTrigger* GetTriggerInternal() const { return trigger_; }
  virtual void setTrigger(AnimationTrigger* trigger);

  struct AnimationTriggerData {
    AnimationTriggerState state = blink::AnimationTriggerState::kIdle;

    // The most recent `animation-play-state` value for |animation_|. This will
    // be std::nullopt for non-CSSAnimations. When this animation's trigger
    // actions this animation, it will factor in this play state, leaving the
    // animation paused if necessary.
    std::optional<EAnimPlayState> css_play_state;

    // Whether there has been a change to |css_play_state_| value since the
    // last time this animation's trigger had an opportunity to action it.
    bool play_state_update_pending = false;
  };
  AnimationTriggerState GetTriggerState() const { return trigger_data_.state; }
  void SetTriggerState(AnimationTriggerState state) {
    trigger_data_.state = state;
  }
  std::optional<EAnimPlayState> GetTriggerActionPlayState() const {
    return trigger_data_.css_play_state;
  }
  void SetTriggerActionPlayState(std::optional<EAnimPlayState> play_state) {
    SetPendingTriggerPlayStateUpdate(play_state !=
                                     trigger_data_.css_play_state);
    trigger_data_.css_play_state = play_state;
  }
  bool PendingTriggerPlayStateUpdate() const {
    return trigger_data_.play_state_update_pending;
  }
  void SetPendingTriggerPlayStateUpdate(bool pending) {
    trigger_data_.play_state_update_pending = pending;
  }
  // Indicates if an animation is scroll-triggered and could still be played by
  // its trigger. These animations are to appear in list for getAnimations
  // calls, and must not be garbage-collected.
  bool CanBeTriggered() const;

 protected:
  DispatchEventResult DispatchEventInternal(Event&) override;
  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;
  virtual AnimationEffect::EventDelegate* CreateEventDelegate(
      Element* target,
      const AnimationEffect::EventDelegate* old_event_delegate) {
    return nullptr;
  }

 private:
  void ClearOutdated();
  void ForceServiceOnNextFrame();

  AnimationTimeDelta EffectEnd() const;
  bool Limited(std::optional<AnimationTimeDelta> current_time) const;

  // Playback rate that will take effect once any pending tasks are resolved.
  // If there are no pending tasks, then the effective playback rate equals the
  // active playback rate.
  double EffectivePlaybackRate() const;
  void ApplyPendingPlaybackRate();

  std::optional<AnimationTimeDelta> CalculateStartTime(
      AnimationTimeDelta current_time) const;
  std::optional<AnimationTimeDelta> CalculateCurrentTime() const;

  V8CSSNumberish* ConvertTimeToCSSNumberish(
      std::optional<AnimationTimeDelta>) const;
  // Failure to convert results in a thrown exception and returning false.
  bool ConvertCSSNumberishToTime(const V8CSSNumberish* numberish,
                                 std::optional<AnimationTimeDelta>& time,
                                 String variable_name,
                                 ExceptionState& exception_state);

  void BeginUpdatingState();
  void EndUpdatingState();

  CompositorAnimations::FailureReasons
  CheckCanStartAnimationOnCompositorInternal() const;
  void CreateCompositorAnimation(std::optional<int> replaced_cc_animation_id);
  void DestroyCompositorAnimation();
  void AttachCompositorTimeline();
  void DetachCompositorTimeline();
  void AttachCompositedLayers();
  void DetachCompositedLayers();
  // CompositorAnimationDelegate implementation.
  void NotifyAnimationStarted(base::TimeDelta monotonic_time,
                              int group) override;
  void NotifyAnimationFinished(base::TimeDelta monotonic_time,
                               int group) override {}
  void NotifyAnimationAborted(base::TimeDelta monotonic_time,
                              int group) override {}

  using AnimationPromise = ScriptPromiseProperty<Animation, DOMException>;
  void ResolvePromiseMaybeAsync(AnimationPromise*);
  void RejectAndResetPromise(AnimationPromise*);
  void RejectAndResetPromiseMaybeAsync(AnimationPromise*);

  // Updates the finished state of the animation. If the update is the result of
  // a discontinuous time change then the value for current time is not bound by
  // the limits of the animation. The finished notification may be synchronous
  // or asynchronous. A synchronous notification is used in the case of
  // explicitly calling finish on an animation.
  enum class UpdateType { kContinuous, kDiscontinuous };
  enum class NotificationType { kAsync, kSync };
  void UpdateFinishedState(UpdateType update_context,
                           NotificationType notification_type);
  void QueueFinishedEvent();

  // Plays an animation. When auto_rewind is enabled, the current time can be
  // adjusted to accommodate reversal of an animation or snapping to an
  // endpoint.
  enum class AutoRewind { kDisabled, kEnabled };
  void PlayInternal(AutoRewind auto_rewind, ExceptionState& exception_state);

  void ResetPendingTasks();
  std::optional<AnimationTimeDelta> TimelineTime() const;

  void ScheduleAsyncFinish();
  void AsyncFinishMicrotask();
  void CommitFinishNotification();

  // Tracking the state of animations in dev tools.
  void NotifyProbe();

  // Reset the cached value for the status of a possible background color
  // animation if required. Any time an animation affecting background color
  // changes we need to reset the flag so that Paint can make a fresh
  // compositing decision and create a fresh paint worklet image from the
  // keyframes.
  // TODO(crbug.com/1310961): Investigate if we need a similar fix for
  // non-native paint worklets.
  void UpdateCompositedPaintStatus();

  // Updates the start time for a running animation that is linked to a scroll
  // timeline. As the animation is linked to a timeline range, we don't
  // necessarily know the start time when calling play or pause. Instead, we
  // calculate the start time and iteration duration once the timeline has been
  // validated or the animation is ready (if no validation required). The start
  // time must also be updated if changing the animation range on a running or
  // finished animation. If a start time was explicitly set, it is treated as
  // sticky and not updated.
  void UpdateAutoAlignedStartTime();

  // Conversion between V8 representation of an animation range boundary and the
  // internal representation.
  std::optional<TimelineOffset> GetEffectiveTimelineOffset(
      const RangeBoundary* boundary,
      double default_percent,
      ExceptionState& exception_state);

  String id_;

  // Extended play state reported to dev tools. This play state has an
  // additional pending state that is not part of the spec by expected by dev
  // tools.
  V8AnimationPlayState::Enum reported_play_state_ =
      V8AnimationPlayState::Enum::kIdle;
  double playback_rate_;
  // The pending playback rate is not currently in effect. It typically takes
  // effect when running a scheduled task in response to the animation being
  // ready.
  std::optional<double> pending_playback_rate_;
  std::optional<AnimationTimeDelta> start_time_;
  std::optional<AnimationTimeDelta> hold_time_;
  std::optional<AnimationTimeDelta> previous_current_time_;
  // Timeline duration is non-null when using a scroll timeline. The value is
  // tracked in order to update a hold time if the timeline duration changes.
  std::optional<AnimationTimeDelta> timeline_duration_;
  bool reset_current_time_on_resume_ = false;

  // Indicates if the animation should auto-align it's start time to the
  // animation range if attached to a ScrollTimeline.  Explicit calls to
  // set the current or start time override auto-alignment, effectively making
  // the start time "sticky", until play or pause are called to un-stick the
  // start time.
  bool auto_align_start_time_ = true;

  unsigned sequence_number_;

  Member<AnimationPromise> finished_promise_;
  Member<AnimationPromise> ready_promise_;

  Member<AnimationEffect> content_;
  // Document refers to the timeline's document if there is a timeline.
  // Otherwise it refers to the document for the execution context.
  Member<Document> document_;
  Member<AnimationTimeline> timeline_;

  std::optional<TimelineOffset> range_start_;
  std::optional<TimelineOffset> range_end_;

  Member<CSSValue> style_dependent_range_start_;
  Member<CSSValue> style_dependent_range_end_;

  V8ReplaceState::Enum replace_state_ = V8ReplaceState::Enum::kActive;

  // Testing flags.
  bool is_paused_for_testing_;
  bool is_composited_animation_disabled_for_testing_;

  // Pending micro-tasks. These flags are used for tracking purposes only for
  // the Animation.pending attribute, and do not otherwise affect internal flow
  // control.
  bool pending_pause_;
  bool pending_play_;
  // Indicates finish notification queued but not processed.
  bool pending_finish_notification_;
  bool has_queued_microtask_;

  // This indicates timing information relevant to the animation's effect
  // has changed by means other than the ordinary progression of time
  bool outdated_;

  // Indicates the animation is no longer active. Cancelled animation is marked
  // as finished_.
  bool finished_;
  // Indicates finish notification has been handled.
  bool committed_finish_notification_;
  // Holds a 'finished' event queued for asynchronous dispatch via the
  // ScriptedAnimationController. This object remains active until the
  // event is actually dispatched.
  Member<Event> pending_finished_event_;

  Member<Event> pending_cancelled_event_;

  Member<Event> pending_remove_event_;

  // Cache whether animation can potentially have native paint worklets.
  // In the event of the keyframes changing, we need a new evaluation, of
  // the composited status for native paint worklet eligible properties.
  // A change in the playState can also necessitate a composited style update.
  mutable std::optional<NativePaintWorkletReasons>
      native_paint_worklet_reasons_;
  mutable std::optional<NativePaintWorkletReasons>
      prior_native_paint_worklet_reasons_;
  Member<Element> prior_native_paint_worklet_target_;

  // TODO(crbug.com/960944): Consider reintroducing kPause and cleanup use of
  // mutually exclusive pending_play_ and pending_pause_ flags.
  enum class CompositorAction { kNone, kStart, kCancel };

  class CompositorState {
    USING_FAST_MALLOC(CompositorState);

   public:
    explicit CompositorState(Animation& animation)
        : start_time(animation.start_time_),
          hold_time(animation.hold_time_),
          playback_rate(animation.EffectivePlaybackRate()),
          pending_action(animation.start_time_ ? CompositorAction::kNone
                                               : CompositorAction::kStart) {}
    CompositorState(const CompositorState&) = delete;
    CompositorState& operator=(const CompositorState&) = delete;

    std::optional<AnimationTimeDelta> start_time;
    std::optional<AnimationTimeDelta> hold_time;
    double playback_rate;
    bool effect_changed = false;
    CompositorAction pending_action;
  };

  // CompositorAnimation objects need to eagerly sever their connection to their
  // Animation delegate; use a separate 'holder' on-heap object to accomplish
  // that.
  class CompositorAnimationHolder final
      : public GarbageCollected<CompositorAnimationHolder> {
    USING_PRE_FINALIZER(CompositorAnimationHolder, Dispose);

   public:
    static CompositorAnimationHolder* Create(
        Animation*,
        std::optional<int> replaced_cc_animation_id);

    explicit CompositorAnimationHolder(
        Animation*,
        std::optional<int> replaced_cc_animation_id);

    void Detach();

    void Trace(Visitor* visitor) const { visitor->Trace(animation_); }

    CompositorAnimation* GetAnimation() const {
      return compositor_animation_.get();
    }

   private:
    void Dispose();

    std::unique_ptr<CompositorAnimation> compositor_animation_;
    Member<Animation> animation_;
  };

  // This mirrors the known compositor state. It is created when a compositor
  // animation is started. Updated once the start time is known and each time
  // modifications are pushed to the compositor.
  std::unique_ptr<CompositorState> compositor_state_;
  bool compositor_pending_;
  int compositor_group_;

  Member<CompositorAnimationHolder> compositor_animation_;

  bool effect_suppressed_;

  // Animations with an owning element stop ticking if there is an active
  // display lock on an ancestor element.  Cache the status to minimize the
  // number of tree walks.
  base::TimeTicks last_display_lock_update_time_ = base::TimeTicks();
  bool is_in_display_locked_subtree_ = false;

  // True if we animate compositor properties but they would have no effect due
  // to being optimized out on the compositor. Updated in |Animation::PreCommit|
  // and |MarkPendingIfCompositorPropertyAnimationChanges|.
  bool compositor_property_animations_have_no_effect_;
  // True if the only reason for not running the animation on the compositor is
  // that the animation would have no effect. Updated in |Animation::PreCommit|.
  bool animation_has_no_effect_;

  Member<AnimationTrigger> trigger_;
  AnimationTriggerData trigger_data_;

  FRIEND_TEST_ALL_PREFIXES(AnimationAnimationTestCompositing,
                           NoCompositeWithoutCompositedElementId);
  FRIEND_TEST_ALL_PREFIXES(AnimationAnimationTestNoCompositing,
                           PendingActivityWithFinishedEventListener);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATION_H_
