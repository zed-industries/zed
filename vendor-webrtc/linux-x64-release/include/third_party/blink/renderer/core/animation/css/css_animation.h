// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_ANIMATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_ANIMATION_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/animation/animation.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"

namespace blink {

class AnimationTrigger;

class CORE_EXPORT CSSAnimation : public Animation {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSAnimation(ExecutionContext*,
               AnimationTimeline*,
               AnimationEffect*,
               wtf_size_t animation_index,
               const String& animation_name,
               AnimationTrigger* trigger);

  bool IsCSSAnimation() const final { return true; }

  void ClearOwningElement() final { owning_element_ = nullptr; }
  Element* OwningElement() const override { return owning_element_.Get(); }

  // Animation effect owner implementation.
  bool IsEventDispatchAllowed() const override;

  const String& animationName() const { return animation_name_; }
  wtf_size_t AnimationIndex() const { return animation_index_; }
  void SetAnimationIndex(wtf_size_t absolute_position) {
    animation_index_ = absolute_position;
  }

  // Animation overrides.
  // Various operations may affect the computed values of properties on
  // elements. User agents may, as an optimization, defer recomputing these
  // values until it becomes necessary; however, all operations included in the
  // programming interfaces defined in the web-animations and css-animations
  // specifications, must produce a result consistent with having fully
  // processed any such pending changes to computed values.  Notably, changes
  // to animation-play-state and display:none must update the play state.
  // https://drafts.csswg.org/css-animations-2/#requirements-on-pending-style-changes
  V8AnimationPlayState playState() const override;
  bool pending() const override;

  // Explicit calls to the web-animation API that update the play state are
  // conditionally sticky and override the animation-play-state style.
  void pause(ExceptionState& = ASSERT_NO_EXCEPTION) override;
  void play(ExceptionState& = ASSERT_NO_EXCEPTION) override;
  void reverse(ExceptionState& = ASSERT_NO_EXCEPTION) override;
  void setTimeline(AnimationTimeline*) override;
  void setStartTime(const V8CSSNumberish* start_time,
                    ExceptionState& exception_state) override;
  void setRangeStart(const RangeBoundary* range_start,
                     ExceptionState& exception_state) override;
  void setRangeEnd(const RangeBoundary* range_end,
                   ExceptionState& exception_state) override;

  // Conditionally updates both boundaries of the animation range.
  // If the corresponding boundary has been explicitly set via WAAPI
  // the new value will be ignored.
  void SetRange(const std::optional<TimelineOffset>& range_start,
                const std::optional<TimelineOffset>& range_end) override;

  // When set, subsequent changes to animation-<property> no longer affect
  // <property>.
  // https://drafts.csswg.org/css-animations-2/#interaction-between-animation-play-state-and-web-animations-API
  bool GetIgnoreCSSPlayState() { return ignore_css_play_state_; }
  void SetIgnoreCSSPlayState(bool ignore) { ignore_css_play_state_ = ignore; }
  void ResetIgnoreCSSPlayState() { ignore_css_play_state_ = false; }
  bool GetIgnoreCSSTimeline() const { return ignore_css_timeline_; }
  void SetIgnoreCSSTimeline(bool ignore) { ignore_css_timeline_ = ignore; }
  void ResetIgnoreCSSTimeline() { ignore_css_timeline_ = false; }
  bool GetIgnoreCSSRangeStart() { return ignore_css_range_start_; }
  void SetIgnoreCSSRangeStart(bool ignore) { ignore_css_range_start_ = ignore; }
  void ResetIgnoreCSSRangeStart() { ignore_css_range_start_ = false; }
  bool GetIgnoreCSSRangeEnd() { return ignore_css_range_end_; }
  void SetIgnoreCSSRangeEnd(bool ignore) { ignore_css_range_end_ = ignore; }
  void ResetIgnoreCSSRangeEnd() { ignore_css_range_end_ = false; }

  void setTrigger(AnimationTrigger* trigger) override;
  bool GetIgnoreCSSTrigger() { return ignore_css_trigger_; }
  void SetIgnoreCSSTrigger(bool ignore) { ignore_css_trigger_ = ignore; }
  void ResetIgnoreCSSTrigger() { ignore_css_trigger_ = false; }

  class ScopedResetIgnoreCSSProperties {
    STACK_ALLOCATED();

   public:
    explicit ScopedResetIgnoreCSSProperties(CSSAnimation* animation);
    ~ScopedResetIgnoreCSSProperties();

   private:
    bool ignore_css_play_state_ = false;
    bool ignore_css_timeline_ = false;
    bool ignore_css_range_start_ = false;
    bool ignore_css_range_end_ = false;
    bool ignore_css_trigger_ = false;
    CSSAnimation* animation_ = nullptr;
  };

  void Trace(blink::Visitor* visitor) const override {
    Animation::Trace(visitor);
    visitor->Trace(owning_element_);
  }

  // Force pending animation properties to be applied, as these may alter the
  // animation. This step is required before any web animation API calls that
  // depends on computed values.
  void FlushPendingUpdates() const override { FlushStyles(); }

 protected:
  AnimationEffect::EventDelegate* CreateEventDelegate(
      Element* target,
      const AnimationEffect::EventDelegate* old_event_delegate) override;

 private:
  void FlushStyles() const;

  class PlayStateTransitionScope {
    STACK_ALLOCATED();

   public:
    explicit PlayStateTransitionScope(CSSAnimation& animation);
    ~PlayStateTransitionScope();

   private:
    CSSAnimation& animation_;
    bool was_paused_;
  };

  // animation_index_ represents the absolute position of an animation within
  // the same owning element. This index helps resolve the animation ordering
  // when comparing two animations with the same owning element.
  wtf_size_t animation_index_;
  AtomicString animation_name_;

  // When set, the web-animation API is overruling the animation-play-state
  // style.
  bool ignore_css_play_state_ = false;
  // When set, changes to the 'animation-timeline' property will be ignored.
  bool ignore_css_timeline_ = false;
  // When set changes to 'animation-range-*' will be ignored.
  bool ignore_css_range_start_ = false;
  bool ignore_css_range_end_ = false;
  // When set, changes to 'animation-trigger*' will be ignored.
  bool ignore_css_trigger_ = false;

  // The owning element of an animation refers to the element or pseudo-element
  // whose animation-name property was applied that generated the animation
  // The spec: https://drafts.csswg.org/css-animations-2/#owning-element-section
  Member<Element> owning_element_;
};

template <>
struct DowncastTraits<CSSAnimation> {
  static bool AllowFrom(const Animation& animation) {
    return animation.IsCSSAnimation();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_ANIMATION_H_
