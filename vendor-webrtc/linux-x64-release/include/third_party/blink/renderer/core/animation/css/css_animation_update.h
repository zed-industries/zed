// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_ANIMATION_UPDATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_ANIMATION_UPDATE_H_

#include <optional>

#include "third_party/blink/renderer/core/animation/animation_timeline.h"
#include "third_party/blink/renderer/core/animation/animation_trigger.h"
#include "third_party/blink/renderer/core/animation/css/css_timeline_map.h"
#include "third_party/blink/renderer/core/animation/deferred_timeline.h"
#include "third_party/blink/renderer/core/animation/effect_stack.h"
#include "third_party/blink/renderer/core/animation/inert_effect.h"
#include "third_party/blink/renderer/core/animation/interpolation.h"
#include "third_party/blink/renderer/core/animation/keyframe_effect_model.h"
#include "third_party/blink/renderer/core/animation/scroll_timeline.h"
#include "third_party/blink/renderer/core/animation/timeline_offset.h"
#include "third_party/blink/renderer/core/animation/view_timeline.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_keyframes_rule.h"
#include "third_party/blink/renderer/core/css/css_property_equality.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/core/style/scoped_css_name.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Animation;
class ComputedStyle;

class NewCSSAnimation {
  DISALLOW_NEW();

 public:
  NewCSSAnimation(AtomicString name,
                  size_t name_index,
                  wtf_size_t position_index,
                  const InertEffect& effect,
                  Timing timing,
                  StyleRuleKeyframes* style_rule,
                  AnimationTimeline* timeline,
                  const Vector<EAnimPlayState>& play_state_list,
                  const std::optional<TimelineOffset>& range_start,
                  const std::optional<TimelineOffset>& range_end,
                  AnimationTrigger* trigger)
      : name(name),
        name_index(name_index),
        position_index(position_index),
        effect(effect),
        timing(timing),
        style_rule(style_rule),
        style_rule_version(this->style_rule->Version()),
        timeline(timeline),
        play_state_list(play_state_list),
        range_start(range_start),
        range_end(range_end),
        trigger(trigger) {}

  void Trace(Visitor* visitor) const {
    visitor->Trace(effect);
    visitor->Trace(style_rule);
    visitor->Trace(timeline);
    visitor->Trace(trigger);
  }

  AtomicString name;
  size_t name_index;
  wtf_size_t position_index;
  Member<const InertEffect> effect;
  Timing timing;
  Member<StyleRuleKeyframes> style_rule;
  unsigned style_rule_version;
  Member<AnimationTimeline> timeline;
  Vector<EAnimPlayState> play_state_list;
  std::optional<TimelineOffset> range_start;
  std::optional<TimelineOffset> range_end;
  Member<AnimationTrigger> trigger;
};

class UpdatedCSSAnimation {
  DISALLOW_NEW();

 public:
  UpdatedCSSAnimation(wtf_size_t index,
                      Animation* animation,
                      const InertEffect& effect,
                      Timing specified_timing,
                      StyleRuleKeyframes* style_rule,
                      AnimationTimeline* timeline,
                      const Vector<EAnimPlayState>& play_state_list,
                      const std::optional<TimelineOffset>& range_start,
                      const std::optional<TimelineOffset>& range_end,
                      AnimationTrigger* trigger)
      : specified_timing(specified_timing),
        index(index),
        animation(animation),
        effect(&effect),
        style_rule(style_rule),
        style_rule_version(this->style_rule->Version()),
        timeline(timeline),
        play_state_list(play_state_list),
        range_start(range_start),
        range_end(range_end),
        trigger(trigger) {}

  void Trace(Visitor* visitor) const {
    visitor->Trace(animation);
    visitor->Trace(effect);
    visitor->Trace(style_rule);
    visitor->Trace(timeline);
    visitor->Trace(trigger);
  }

  Timing specified_timing;
  wtf_size_t index;
  Member<Animation> animation;
  Member<const InertEffect> effect;
  Member<StyleRuleKeyframes> style_rule;
  unsigned style_rule_version;
  Member<AnimationTimeline> timeline;
  Vector<EAnimPlayState> play_state_list;
  std::optional<TimelineOffset> range_start;
  std::optional<TimelineOffset> range_end;
  Member<AnimationTrigger> trigger;
};

}  // namespace blink

WTF_ALLOW_INIT_WITH_MEM_FUNCTIONS(blink::NewCSSAnimation)
WTF_ALLOW_INIT_WITH_MEM_FUNCTIONS(blink::UpdatedCSSAnimation)

namespace blink {

// This class stores the CSS Animations/Transitions information we use during a
// style recalc. This includes updates to animations/transitions as well as the
// Interpolations to be applied.
class CORE_EXPORT CSSAnimationUpdate final {
  DISALLOW_NEW();

 public:
  CSSAnimationUpdate();
  CSSAnimationUpdate(const CSSAnimationUpdate&) = delete;
  CSSAnimationUpdate& operator=(const CSSAnimationUpdate&) = delete;
  ~CSSAnimationUpdate();

  void Copy(const CSSAnimationUpdate&);
  void Clear();

  void StartAnimation(const AtomicString& animation_name,
                      size_t name_index,
                      wtf_size_t position_index,
                      const InertEffect& effect,
                      const Timing& timing,
                      StyleRuleKeyframes* style_rule,
                      AnimationTimeline* timeline,
                      const Vector<EAnimPlayState>& play_state_list,
                      const std::optional<TimelineOffset>& range_start,
                      const std::optional<TimelineOffset>& range_end,
                      AnimationTrigger* trigger) {
    new_animations_.push_back(NewCSSAnimation(
        animation_name, name_index, position_index, effect, timing, style_rule,
        timeline, play_state_list, range_start, range_end, trigger));
  }
  void CancelAnimation(wtf_size_t index, const Animation& animation) {
    cancelled_animation_indices_.push_back(index);
    suppressed_animations_.insert(&animation);
  }
  void ToggleAnimationIndexPaused(wtf_size_t index) {
    animation_indices_with_pause_toggled_.push_back(index);
  }
  void UpdateAnimation(wtf_size_t index,
                       Animation* animation,
                       const InertEffect& effect,
                       const Timing& specified_timing,
                       StyleRuleKeyframes* style_rule,
                       AnimationTimeline* timeline,
                       const Vector<EAnimPlayState>& play_state_list,
                       const std::optional<TimelineOffset>& range_start,
                       const std::optional<TimelineOffset>& range_end,
                       AnimationTrigger* trigger) {
    animations_with_updates_.push_back(UpdatedCSSAnimation(
        index, animation, effect, specified_timing, style_rule, timeline,
        play_state_list, range_start, range_end, trigger));
    suppressed_animations_.insert(animation);
  }
  void UpdateCompositorKeyframes(Animation* animation) {
    updated_compositor_keyframes_.push_back(animation);
  }

  void StartTransition(const PropertyHandle&,
                       const ComputedStyle* from,
                       const ComputedStyle* to,
                       const ComputedStyle* reversing_adjusted_start_value,
                       double reversing_shortening_factor,
                       const InertEffect&);
  void UnstartTransition(const PropertyHandle&);
  void CancelTransition(const PropertyHandle& property) {
    cancelled_transitions_.insert(property);
  }
  void FinishTransition(const PropertyHandle& property) {
    finished_transitions_.insert(property);
  }

  void SetChangedScrollTimelines(CSSScrollTimelineMap timelines) {
    changed_scroll_timelines_ = std::move(timelines);
  }

  void SetChangedViewTimelines(CSSViewTimelineMap timelines) {
    changed_view_timelines_ = std::move(timelines);
  }

  void SetChangedDeferredTimelines(CSSDeferredTimelineMap timelines) {
    changed_deferred_timelines_ = std::move(timelines);
  }

  void SetChangedTimelineAttachments(TimelineAttachmentMap attachments) {
    changed_timeline_attachments_ = std::move(attachments);
  }

  const HeapVector<NewCSSAnimation>& NewAnimations() const {
    return new_animations_;
  }
  const Vector<wtf_size_t>& CancelledAnimationIndices() const {
    return cancelled_animation_indices_;
  }
  const HeapHashSet<Member<const Animation>>& SuppressedAnimations() const {
    return suppressed_animations_;
  }
  const Vector<wtf_size_t>& AnimationIndicesWithPauseToggled() const {
    return animation_indices_with_pause_toggled_;
  }
  const HeapVector<UpdatedCSSAnimation>& AnimationsWithUpdates() const {
    return animations_with_updates_;
  }
  const HeapVector<Member<Animation>>& UpdatedCompositorKeyframes() const {
    return updated_compositor_keyframes_;
  }

  struct NewTransition : public GarbageCollected<NewTransition> {
   public:
    NewTransition(const PropertyHandle& property,
                  const ComputedStyle* from,
                  const ComputedStyle* to,
                  const ComputedStyle* reversing_adjusted_start_value,
                  double reversing_shortening_factor,
                  const InertEffect* effect)
        : property(property),
          from(from),
          to(to),
          reversing_adjusted_start_value(reversing_adjusted_start_value),
          reversing_shortening_factor(reversing_shortening_factor),
          effect(effect) {}
    void Trace(Visitor* visitor) const {
      visitor->Trace(from);
      visitor->Trace(to);
      visitor->Trace(reversing_adjusted_start_value);
      visitor->Trace(effect);
    }

    PropertyHandle property = HashTraits<blink::PropertyHandle>::EmptyValue();
    Member<const ComputedStyle> from;
    Member<const ComputedStyle> to;
    Member<const ComputedStyle> reversing_adjusted_start_value;
    double reversing_shortening_factor;
    Member<const InertEffect> effect;
  };
  using NewTransitionMap = HeapHashMap<PropertyHandle, Member<NewTransition>>;
  const NewTransitionMap& NewTransitions() const { return new_transitions_; }
  const HashSet<PropertyHandle>& CancelledTransitions() const {
    return cancelled_transitions_;
  }
  const HashSet<PropertyHandle>& FinishedTransitions() const {
    return finished_transitions_;
  }

  // A "changed timelines map" (returned by Changed[Scroll,View]Timelines)
  // contains en entry for each timeline (name) that was created, updated
  // or removed. An entry with a non-nullptr value either means that a new
  // timeline was created, or that an existing timeline was updated. An entry
  // with an explicit nullptr value means that the timeline was removed.
  const CSSScrollTimelineMap& ChangedScrollTimelines() const {
    return changed_scroll_timelines_;
  }
  const CSSViewTimelineMap& ChangedViewTimelines() const {
    return changed_view_timelines_;
  }
  const CSSDeferredTimelineMap& ChangedDeferredTimelines() const {
    return changed_deferred_timelines_;
  }
  const TimelineAttachmentMap& ChangedTimelineAttachments() const {
    return changed_timeline_attachments_;
  }

  void AdoptActiveInterpolationsForAnimations(
      ActiveInterpolationsMap& new_map) {
    new_map.swap(active_interpolations_for_animations_);
  }
  void AdoptActiveInterpolationsForTransitions(
      ActiveInterpolationsMap& new_map) {
    new_map.swap(active_interpolations_for_transitions_);
  }
  const ActiveInterpolationsMap& ActiveInterpolationsForAnimations() const {
    return active_interpolations_for_animations_;
  }
  ActiveInterpolationsMap& ActiveInterpolationsForAnimations() {
    return active_interpolations_for_animations_;
  }
  const ActiveInterpolationsMap& ActiveInterpolationsForTransitions() const {
    return active_interpolations_for_transitions_;
  }

  bool HasActiveInterpolationsForProperty(
      const PropertyHandle& property) const {
    return active_interpolations_for_animations_.Contains(property) ||
           active_interpolations_for_transitions_.Contains(property);
  }

  bool IsEmpty() const { return !HasUpdates() && !HasActiveInterpolations(); }

  bool HasUpdates() const {
    return !new_animations_.empty() || !cancelled_animation_indices_.empty() ||
           !suppressed_animations_.empty() ||
           !animation_indices_with_pause_toggled_.empty() ||
           !animations_with_updates_.empty() || !new_transitions_.empty() ||
           !cancelled_transitions_.empty() || !finished_transitions_.empty() ||
           !updated_compositor_keyframes_.empty() ||
           !changed_scroll_timelines_.empty() ||
           !changed_view_timelines_.empty() ||
           !changed_deferred_timelines_.empty() ||
           !changed_timeline_attachments_.empty();
  }

  void Trace(Visitor* visitor) const {
    visitor->Trace(new_transitions_);
    visitor->Trace(new_animations_);
    visitor->Trace(suppressed_animations_);
    visitor->Trace(animations_with_updates_);
    visitor->Trace(updated_compositor_keyframes_);
    visitor->Trace(active_interpolations_for_animations_);
    visitor->Trace(active_interpolations_for_transitions_);
    visitor->Trace(changed_scroll_timelines_);
    visitor->Trace(changed_view_timelines_);
    visitor->Trace(changed_deferred_timelines_);
    visitor->Trace(changed_timeline_attachments_);
  }

 private:
  bool HasActiveInterpolations() const {
    return !active_interpolations_for_animations_.empty() ||
           !active_interpolations_for_transitions_.empty();
  }

  // Order is significant since it defines the order in which new animations
  // will be started. Note that there may be multiple animations present
  // with the same name, due to the way in which we split up animations with
  // incomplete keyframes.
  HeapVector<NewCSSAnimation> new_animations_;
  Vector<wtf_size_t> cancelled_animation_indices_;
  HeapHashSet<Member<const Animation>> suppressed_animations_;
  Vector<wtf_size_t> animation_indices_with_pause_toggled_;
  HeapVector<UpdatedCSSAnimation> animations_with_updates_;
  HeapVector<Member<Animation>> updated_compositor_keyframes_;

  NewTransitionMap new_transitions_;
  HashSet<PropertyHandle> cancelled_transitions_;
  HashSet<PropertyHandle> finished_transitions_;

  CSSScrollTimelineMap changed_scroll_timelines_;
  CSSViewTimelineMap changed_view_timelines_;
  CSSDeferredTimelineMap changed_deferred_timelines_;
  TimelineAttachmentMap changed_timeline_attachments_;

  ActiveInterpolationsMap active_interpolations_for_animations_;
  ActiveInterpolationsMap active_interpolations_for_transitions_;

  friend class PendingAnimationUpdate;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_ANIMATION_UPDATE_H_
