// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_TRANSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_TRANSITION_H_

#include "third_party/blink/renderer/core/animation/animation.h"
#include "third_party/blink/renderer/core/animation/animation_effect.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"

namespace blink {

class CORE_EXPORT CSSTransition : public Animation {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSTransition(ExecutionContext*,
                AnimationTimeline*,
                AnimationEffect*,
                uint64_t transition_generation,
                const PropertyHandle& transition_property);

  bool IsCSSTransition() const final { return true; }

  void ClearOwningElement() final { owning_element_ = nullptr; }
  Element* OwningElement() const override { return owning_element_.Get(); }

  uint64_t TransitionGeneration() const { return transition_generation_; }
  AtomicString transitionProperty() const;
  CSSPropertyName TransitionCSSPropertyName() const {
    return transition_property_.GetCSSPropertyName();
  }

  // Animation overrides.
  // Various operations may affect the computed values of properties on
  // elements. User agents may, as an optimization, defer recomputing these
  // values until it becomes necessary; however, all operations included in the
  // programming interfaces defined in the web-animations and css-transitions
  // specifications, must produce a result consistent with having fully
  // processed any such pending changes to computed values.  Notably, setting
  // display:none must update the play state.
  // https://drafts.csswg.org/css-transitions-2/#requirements-on-pending-style-changes
  V8AnimationPlayState playState() const override;
  void Trace(blink::Visitor* visitor) const override {
    Animation::Trace(visitor);
    visitor->Trace(owning_element_);
  }

 protected:
  AnimationEffect::EventDelegate* CreateEventDelegate(
      Element* target,
      const AnimationEffect::EventDelegate* old_event_delegate) override;

 private:
  PropertyHandle transition_property_;
  // The owning element of a transition refers to the element or pseudo-element
  // to which the transition-property property was applied that generated the
  // animation.
  Member<Element> owning_element_;
  uint64_t transition_generation_;
};
template <>
struct DowncastTraits<CSSTransition> {
  static bool AllowFrom(const Animation& animation) {
    return animation.IsCSSTransition();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_TRANSITION_H_
