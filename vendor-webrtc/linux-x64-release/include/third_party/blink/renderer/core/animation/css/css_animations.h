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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_ANIMATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_ANIMATIONS_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/animation/css/css_animation_data.h"
#include "third_party/blink/renderer/core/animation/css/css_animation_update.h"
#include "third_party/blink/renderer/core/animation/css/css_timeline_map.h"
#include "third_party/blink/renderer/core/animation/css/css_transition_data.h"
#include "third_party/blink/renderer/core/animation/inert_effect.h"
#include "third_party/blink/renderer/core/animation/interpolation.h"
#include "third_party/blink/renderer/core/animation/timeline_offset.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_keyframes_rule.h"
#include "third_party/blink/renderer/core/css/css_property_value_set.h"
#include "third_party/blink/renderer/core/css/properties/css_bitset.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class CSSTransitionData;
class ComputedStyleBuilder;
class Element;
class StylePropertyShorthand;
class StyleResolver;
class StyleTimeline;
class WritingDirectionMode;
class AnimationTrigger;

class CORE_EXPORT CSSAnimations final {
  DISALLOW_NEW();

 public:
  CSSAnimations();
  CSSAnimations(const CSSAnimations&) = delete;
  CSSAnimations& operator=(const CSSAnimations&) = delete;

  // When |with_discrete| is set to true, this method returns not just
  // interpolable properties, but properties which can be transitioned with
  // transition-behavior:allow-discrete. |with_discrete| returns almost 3x as
  // many properties, which may result in slower style update performance, so
  // they are worth separating.
  static const StylePropertyShorthand& PropertiesForTransitionAll(
      bool with_discrete,
      const ExecutionContext* execution_context);

  static bool IsAnimationAffectingProperty(const CSSProperty&);
  static bool IsAffectedByKeyframesFromScope(const Element&, const TreeScope&);
  static bool IsAnimatingCustomProperties(const ElementAnimations*);
  static bool IsAnimatingStandardProperties(const ElementAnimations*,
                                            const CSSBitset*,
                                            KeyframeEffect::Priority);
  static bool IsAnimatingFontAffectingProperties(const ElementAnimations*);
  static bool IsAnimatingLineHeightProperty(const ElementAnimations*);
  static bool IsAnimatingRevert(const ElementAnimations*);
  static bool IsAnimatingDisplayProperty(const ElementAnimations*);
  static void CalculateTimelineUpdate(CSSAnimationUpdate&,
                                      Element& animating_element,
                                      const ComputedStyleBuilder&);
  static void CalculateAnimationUpdate(CSSAnimationUpdate&,
                                       Element& animating_element,
                                       Element&,
                                       const ComputedStyleBuilder&,
                                       const ComputedStyle* parent_style,
                                       StyleResolver*,
                                       bool can_trigger_animations);
  static void CalculateCompositorAnimationUpdate(
      CSSAnimationUpdate&,
      const Element& animating_element,
      Element&,
      const ComputedStyle&,
      const ComputedStyle* parent_style,
      bool was_viewport_changed,
      bool force_update);

  static AnimationEffect::EventDelegate* CreateEventDelegate(
      Element* element,
      const PropertyHandle& property_handle,
      const AnimationEffect::EventDelegate* old_event_delegate);

  static AnimationEffect::EventDelegate* CreateEventDelegate(
      Element* element,
      const AtomicString& animation_name,
      const AnimationEffect::EventDelegate* old_event_delegate);

  static void CalculateTransitionUpdate(
      CSSAnimationUpdate&,
      Element& animating_element,
      const ComputedStyleBuilder&,
      const ComputedStyle* old_style,
      const StyleRecalcContext& style_recalc_context,
      bool can_trigger_animations);

  static void SnapshotCompositorKeyframes(Element&,
                                          CSSAnimationUpdate&,
                                          const ComputedStyle&,
                                          const ComputedStyle* parent_style);

  static void UpdateAnimationFlags(Element& animating_element,
                                   CSSAnimationUpdate&,
                                   ComputedStyleBuilder&);

  const CSSAnimationUpdate& PendingUpdate() const { return pending_update_; }
  void SetPendingUpdate(const CSSAnimationUpdate& update) {
    ClearPendingUpdate();
    pending_update_.Copy(update);
  }
  void ClearPendingUpdate() { pending_update_.Clear(); }
  void MaybeApplyPendingUpdate(Element*);
  bool HasPreviousActiveInterpolationsForAnimations() const {
    return !previous_active_interpolations_for_animations_.empty();
  }
  bool IsEmpty() const {
    return running_animations_.empty() && transitions_.empty() &&
           pending_update_.IsEmpty();
  }
  bool HasTimelines() const { return !timeline_data_.IsEmpty(); }
  void Cancel();

  void Trace(Visitor*) const;

 private:
  friend class CSSAnimationsTest;

  class RunningAnimation final : public GarbageCollected<RunningAnimation> {
   public:
    RunningAnimation(Animation* animation, NewCSSAnimation new_animation)
        : animation(animation),
          name(new_animation.name),
          name_index(new_animation.name_index),
          specified_timing(new_animation.timing),
          style_rule(new_animation.style_rule),
          style_rule_version(new_animation.style_rule_version),
          play_state_list(new_animation.play_state_list) {}

    AnimationTimeline* Timeline() const {
      return animation->TimelineInternal();
    }
    const std::optional<TimelineOffset>& RangeStart() const {
      return animation->GetRangeStartInternal();
    }
    const std::optional<TimelineOffset>& RangeEnd() const {
      return animation->GetRangeEndInternal();
    }

    void Update(UpdatedCSSAnimation update) {
      DCHECK_EQ(update.animation, animation);
      style_rule = update.style_rule;
      style_rule_version = update.style_rule_version;
      play_state_list = update.play_state_list;
      specified_timing = update.specified_timing;
    }

    void Trace(Visitor* visitor) const {
      visitor->Trace(animation);
      visitor->Trace(style_rule);
    }

    Member<Animation> animation;
    AtomicString name;
    size_t name_index;
    Timing specified_timing;
    Member<StyleRuleKeyframes> style_rule;
    unsigned style_rule_version;
    Vector<EAnimPlayState> play_state_list;
  };

  struct RunningTransition : public GarbageCollected<RunningTransition> {
   public:
    RunningTransition(Animation* animation,
                      const ComputedStyle* from,
                      const ComputedStyle* to,
                      const ComputedStyle* reversing_adjusted_start_value,
                      double reversing_shortening_factor)
        : animation(animation),
          from(from),
          to(to),
          reversing_adjusted_start_value(reversing_adjusted_start_value),
          reversing_shortening_factor(reversing_shortening_factor) {}

    void Trace(Visitor* visitor) const {
      visitor->Trace(animation);
      visitor->Trace(from);
      visitor->Trace(to);
      visitor->Trace(reversing_adjusted_start_value);
    }

    Member<Animation> animation;
    Member<const ComputedStyle> from;
    Member<const ComputedStyle> to;
    Member<const ComputedStyle> reversing_adjusted_start_value;
    double reversing_shortening_factor;
  };

  class TimelineData {
    DISALLOW_NEW();

   public:
    void SetScrollTimeline(const ScopedCSSName& name, ScrollTimeline*);
    const CSSScrollTimelineMap& GetScrollTimelines() const {
      return scroll_timelines_;
    }
    void SetViewTimeline(const ScopedCSSName& name, ViewTimeline*);
    const CSSViewTimelineMap& GetViewTimelines() const {
      return view_timelines_;
    }

    void SetDeferredTimeline(const ScopedCSSName& name, DeferredTimeline*);
    const CSSDeferredTimelineMap& GetDeferredTimelines() const {
      return deferred_timelines_;
    }

    void SetTimelineAttachment(ScrollSnapshotTimeline*, DeferredTimeline*);
    DeferredTimeline* GetTimelineAttachment(ScrollSnapshotTimeline*);
    const TimelineAttachmentMap& GetTimelineAttachments() const {
      return timeline_attachments_;
    }

    bool IsEmpty() const {
      return scroll_timelines_.empty() && view_timelines_.empty() &&
             deferred_timelines_.empty() && timeline_attachments_.empty();
    }
    void Clear() {
      scroll_timelines_.clear();
      view_timelines_.clear();
      deferred_timelines_.clear();
      timeline_attachments_.clear();
    }
    void Trace(Visitor*) const;

   private:
    CSSScrollTimelineMap scroll_timelines_;
    CSSViewTimelineMap view_timelines_;
    CSSDeferredTimelineMap deferred_timelines_;
    TimelineAttachmentMap timeline_attachments_;
  };

  TimelineData timeline_data_;

  HeapVector<Member<RunningAnimation>> running_animations_;

  using TransitionMap = HeapHashMap<PropertyHandle, Member<RunningTransition>>;
  TransitionMap transitions_;

  CSSAnimationUpdate pending_update_;

  ActiveInterpolationsMap previous_active_interpolations_for_animations_;

  struct TransitionUpdateState {
    STACK_ALLOCATED();

   public:
    CSSAnimationUpdate& update;
    Element& animating_element;
    const ComputedStyle& old_style;
    const ComputedStyle& base_style;
    const ComputedStyle* before_change_style;
    // base_style may include animation effects for properties inherited from
    // ancestor elements. Since after-change style should not include such
    // effects, we may need to compute an ancestor chain of after-change styles
    // when calculating transition updates. This is expensive, so we only do it
    // for elements which can transition inherited properties when inherited
    // properties are being animated in an ancestor. In that case we store the
    // separately cascaded on-demand after-change style here. Otherwise, we use
    // the base_style as the after_change_style.
    const ComputedStyle* after_change_style;
    const TransitionMap* active_transitions;
    // If the performance of this HashSet shows up in profiles, we could
    // convert any non-custom CSS properties in it to use CSSBitset instead.
    HashSet<PropertyHandle>* listed_properties;
    const CSSTransitionData* transition_data;
    // The StyleRecalcContext passed to the style resolution for the element
    // which we are considering transitions for. This contains necessary try-
    // data for correct after-change style for anchored elements.
    const StyleRecalcContext& style_recalc_context;
    // @starting-style should inherited from the parent's after-change style. As
    // for base_style, when old_style is the @starting-style it will have
    // inherited from its ancestors with animation effects applied. So we need
    // to compute the before-change style for the @starting-style case correctly
    // by cascading after-change style for ancestors as necessary. This flag is
    // set to true if before_change_style has been cascaded correctly.
    bool before_change_style_is_accurate_for_starting_style = false;
  };

  static HeapHashSet<Member<const Animation>> CreateCancelledTransitionsSet(
      ElementAnimations*,
      CSSAnimationUpdate&);

  static void CalculateTransitionUpdateForProperty(
      TransitionUpdateState&,
      const CSSTransitionData::TransitionProperty&,
      wtf_size_t transition_index,
      WritingDirectionMode);

  static void CalculateTransitionUpdateForCustomProperty(
      TransitionUpdateState&,
      const CSSTransitionData::TransitionProperty&,
      wtf_size_t transition_index);

  static void CalculateTransitionUpdateForStandardProperty(
      TransitionUpdateState&,
      const CSSTransitionData::TransitionProperty&,
      wtf_size_t transition_index,
      WritingDirectionMode);

  static bool CanCalculateTransitionUpdateForProperty(
      TransitionUpdateState& state,
      const PropertyHandle& property);

  static void CalculateTransitionUpdateForPropertyHandle(
      TransitionUpdateState&,
      const CSSTransitionData::TransitionAnimationType type,
      const PropertyHandle&,
      wtf_size_t transition_index,
      bool animate_all);

  static void CalculateAnimationActiveInterpolations(
      CSSAnimationUpdate&,
      const Element& animating_element);
  static void CalculateTransitionActiveInterpolations(
      CSSAnimationUpdate&,
      const Element& animating_element);

  static void CalculateScrollTimelineUpdate(CSSAnimationUpdate&,
                                            Element& animating_element,
                                            const ComputedStyleBuilder&);
  static void CalculateViewTimelineUpdate(CSSAnimationUpdate&,
                                          Element& animating_element,
                                          const ComputedStyleBuilder&);
  static void CalculateDeferredTimelineUpdate(CSSAnimationUpdate&,
                                              Element& animating_element,
                                              const ComputedStyleBuilder&);

  static CSSScrollTimelineMap CalculateChangedScrollTimelines(
      Element& animating_element,
      const CSSScrollTimelineMap* existing_scroll_timelines,
      const ComputedStyleBuilder&);
  static CSSViewTimelineMap CalculateChangedViewTimelines(
      Element& animating_element,
      const CSSViewTimelineMap* existing_view_timelines,
      const ComputedStyleBuilder&);
  static CSSDeferredTimelineMap CalculateChangedDeferredTimelines(
      Element& animating_element,
      const CSSDeferredTimelineMap* existing_deferred_timelines,
      const ComputedStyleBuilder&);

  template <typename MapType>
  static const MapType* GetExistingTimelines(const TimelineData*);
  template <typename MapType>
  static const MapType* GetChangedTimelines(const CSSAnimationUpdate*);

  // Invokes `callback` for each timeline, taking both existing timelines
  // and pending changes into account.
  template <typename TimelineType, typename CallbackFunc>
  static void ForEachTimeline(const TimelineData*,
                              const CSSAnimationUpdate*,
                              CallbackFunc);

  template <typename TimelineType>
  static void CalculateChangedTimelineAttachments(
      Element& animating_element,
      const TimelineData*,
      const CSSAnimationUpdate&,
      const TimelineAttachmentMap* existing_attachments,
      TimelineAttachmentMap& result);

  static void CalculateTimelineAttachmentUpdate(CSSAnimationUpdate&,
                                                Element& animating_element);

  static const TimelineData* GetTimelineData(const Element&);

  static ScrollSnapshotTimeline* FindTimelineForNode(const ScopedCSSName& name,
                                                     Node*,
                                                     const CSSAnimationUpdate*);
  template <typename TimelineType>
  static TimelineType* FindTimelineForElement(const ScopedCSSName& name,
                                              const TimelineData*,
                                              const CSSAnimationUpdate*);

  static ScrollSnapshotTimeline* FindAncestorTimeline(
      const ScopedCSSName& name,
      Node*,
      const CSSAnimationUpdate*);

  static DeferredTimeline* FindDeferredTimeline(const ScopedCSSName& name,
                                                Element*,
                                                const CSSAnimationUpdate*);

  static AnimationTimeline* ComputeTimeline(
      Element*,
      const StyleTimeline& timeline_name,
      const CSSAnimationUpdate&,
      AnimationTimeline* existing_timeline);

  // The before-change style is defined as the computed values of all properties
  // on the element as of the previous style change event, except with any
  // styles derived from declarative animations updated to the current time.
  // https://drafts.csswg.org/css-transitions-1/#before-change-style
  static const ComputedStyle& CalculateBeforeChangeStyle(
      TransitionUpdateState& state,
      const PropertyHandle& transitioning_property);

  static const ComputedStyle* EnsureAfterChangeStyleIfNecessary(
      TransitionUpdateState& state,
      const ComputedStyle& base_style,
      const PropertyHandle& transitioning_property,
      bool for_starting_style);

  // Compute the after-change style for animating_element. Used by
  // CalculateAfterChangeStyle if the base style can not be used for starting
  // transitions. The after_change_root is the uppermost ancestor which may have
  // a different computed value, for the property passed into
  // CalculateAfterChangeStyle, for the after-change style than for the base
  // style.
  static const ComputedStyle& EnsureAfterChangeStyle(Element& animating_element,
                                                     Element& after_change_root,
                                                     const StyleRecalcContext&,
                                                     bool for_starting_style);

  // The after-change style is defined as values of all properties on the
  // element based on the information known at the start of that style change
  // event, but using the computed values of the animation-* properties from the
  // before-change style, excluding any styles from CSS Transitions in the
  // computation, and inheriting from the after-change style of the parent.
  // https://drafts.csswg.org/css-transitions/#after-change-style
  //
  // This method computes the after-change style for an element given by 'state'
  // if the computed after-change value for 'transitioning-property' may be
  // different from the base style value for that property. Otherwise it just
  // returns the base style.
  static const ComputedStyle& CalculateAfterChangeStyle(
      TransitionUpdateState& state,
      const PropertyHandle& transitioning_property);

  static AnimationTrigger* ComputeTrigger(Element* element,
                                          const CSSAnimationData* data,
                                          wtf_size_t animation_index,
                                          const CSSAnimationUpdate& update,
                                          AnimationTrigger* existing_trigger);

  class AnimationEventDelegate final : public AnimationEffect::EventDelegate {
   public:
    AnimationEventDelegate(
        Element* animation_target,
        const AtomicString& name,
        Timing::Phase previous_phase = Timing::kPhaseNone,
        std::optional<double> previous_iteration = std::nullopt)
        : animation_target_(animation_target),
          name_(name),
          previous_phase_(previous_phase),
          previous_iteration_(previous_iteration) {}
    bool RequiresIterationEvents(const AnimationEffect&) override;
    void OnEventCondition(const AnimationEffect&, Timing::Phase) override;

    bool IsAnimationEventDelegate() const override { return true; }
    Timing::Phase getPreviousPhase() const { return previous_phase_; }
    std::optional<double> getPreviousIteration() const {
      return previous_iteration_;
    }

    void Trace(Visitor*) const override;

   private:
    const Element& AnimationTarget() const { return *animation_target_; }
    EventTarget* GetEventTarget() const;
    Document& GetDocument() const { return animation_target_->GetDocument(); }

    void MaybeDispatch(Document::ListenerType,
                       const AtomicString& event_name,
                       const AnimationTimeDelta& elapsed_time);
    Member<Element> animation_target_;
    const AtomicString name_;
    Timing::Phase previous_phase_;
    std::optional<double> previous_iteration_;
  };

  class TransitionEventDelegate final : public AnimationEffect::EventDelegate {
   public:
    TransitionEventDelegate(Element* transition_target,
                            const PropertyHandle& property,
                            Timing::Phase previous_phase = Timing::kPhaseNone)
        : property_(property),
          transition_target_(transition_target),
          previous_phase_(previous_phase) {}
    bool RequiresIterationEvents(const AnimationEffect&) override {
      return false;
    }
    void OnEventCondition(const AnimationEffect&, Timing::Phase) override;
    bool IsTransitionEventDelegate() const override { return true; }
    Timing::Phase getPreviousPhase() const { return previous_phase_; }

    void Trace(Visitor*) const override;

   private:
    void EnqueueEvent(const WTF::AtomicString& type,
                      const AnimationTimeDelta& elapsed_time);

    const Element& TransitionTarget() const { return *transition_target_; }
    EventTarget* GetEventTarget() const;
    Document& GetDocument() const { return transition_target_->GetDocument(); }

    PropertyHandle property_;
    Member<Element> transition_target_;
    Timing::Phase previous_phase_;
  };
};

template <>
struct DowncastTraits<CSSAnimations::AnimationEventDelegate> {
  static bool AllowFrom(const AnimationEffect::EventDelegate& delegate) {
    return delegate.IsAnimationEventDelegate();
  }
};

template <>
struct DowncastTraits<CSSAnimations::TransitionEventDelegate> {
  static bool AllowFrom(const AnimationEffect::EventDelegate& delegate) {
    return delegate.IsTransitionEventDelegate();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_ANIMATIONS_H_
