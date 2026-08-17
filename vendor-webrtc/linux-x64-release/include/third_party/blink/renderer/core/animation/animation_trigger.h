// Copyright 2025 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATION_TRIGGER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATION_TRIGGER_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_animation_trigger_options.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_animation_trigger_type.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_union_string_timelinerangeoffset.h"
#include "third_party/blink/renderer/core/animation/animation_timeline.h"
#include "third_party/blink/renderer/core/animation/scroll_snapshot_timeline.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ExecutionContext;

class CORE_EXPORT AnimationTrigger : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  using RangeBoundary = V8UnionStringOrTimelineRangeOffset;
  using Type = V8AnimationTriggerType;
  using TriggerState = blink::AnimationTriggerState;

  AnimationTrigger(AnimationTimeline* timeline,
                   Type type,
                   RangeBoundary* range_start,
                   RangeBoundary* range_end,
                   RangeBoundary* exit_range_start,
                   RangeBoundary* exit_range_end);
  static AnimationTrigger* Create(ExecutionContext* execution_context,
                                  AnimationTriggerOptions* options,
                                  ExceptionState& exception_state);

  Type type() { return type_; }

  AnimationTimeline* timeline() {
    return timeline_.Get() ? timeline_.Get()->ExposedTimeline() : nullptr;
  }
  AnimationTimeline* GetTimelineInternal() { return timeline_.Get(); }

  const RangeBoundary* rangeStart(ExecutionContext* execution_context);
  const RangeBoundary* rangeEnd(ExecutionContext* execution_context);
  const RangeBoundary* exitRangeStart(ExecutionContext* execution_context);
  const RangeBoundary* exitRangeEnd(ExecutionContext* execution_context);

  void setRangeBoundariesForTest(RangeBoundary* start,
                                 RangeBoundary* exit,
                                 RangeBoundary* exit_start,
                                 RangeBoundary* exit_end) {
    range_start_ = start;
    range_end_ = exit;
    exit_range_start_ = exit_start;
    exit_range_end_ = exit_end;
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(timeline_);
    visitor->Trace(range_start_);
    visitor->Trace(range_end_);
    visitor->Trace(exit_range_start_);
    visitor->Trace(exit_range_end_);
    ScriptWrappable::Trace(visitor);
  }

  using TimelineState = ScrollSnapshotTimeline::TimelineState;
  void ActionAnimation(Animation* animation);
  bool ActionAnimationInternal(Animation* animation,
                               bool within_trigger_range,
                               bool within_exit_range);

  static Type ToV8TriggerType(EAnimationTriggerType type) {
    switch (type) {
      case EAnimationTriggerType::kOnce:
        return Type(Type::Enum::kOnce);
      case EAnimationTriggerType::kRepeat:
        return Type(Type::Enum::kRepeat);
      case EAnimationTriggerType::kAlternate:
        return Type(Type::Enum::kAlternate);
      case EAnimationTriggerType::kState:
        return Type(Type::Enum::kState);
      default:
        NOTREACHED();
    };
  }

  // Structure representing the scroll offsets (in px) corresponding to the
  // boundaries of the trigger (default) range and the exit range;
  struct TriggerBoundaries {
    // The start offset of the trigger/default range.
    double start;
    // The end offset of the trigger/default range.
    double end;
    // The start offset of the exit range.
    double exit_start;
    // The end offset of the exit range.
    double exit_end;
  };

  TriggerBoundaries ComputeTriggerBoundaries(Element& timeline_source,
                                             const ScrollTimeline& timeline);

 private:
  // It's possible we're in a range that would normally action
  // the animation but, e.g. because `animation-play-state` was 'paused'
  // when we initially entered the range (and updated |state_|) and has now
  // changed to 'running', we should unpause (and vice versa for a change
  // from 'running' to 'paused'). This function ensures that we pause or unpause
  // the the animation in these cases.
  void ProcessPendingPlayStateUpdate(Animation* animation);

  Member<AnimationTimeline> timeline_;
  Type type_;
  Member<const RangeBoundary> range_start_;
  Member<const RangeBoundary> range_end_;
  Member<const RangeBoundary> exit_range_start_;
  Member<const RangeBoundary> exit_range_end_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATION_TRIGGER_H_
