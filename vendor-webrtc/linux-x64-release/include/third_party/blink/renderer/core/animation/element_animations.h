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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ELEMENT_ANIMATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ELEMENT_ANIMATIONS_H_

#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/animation/css/css_animations.h"
#include "third_party/blink/renderer/core/animation/effect_stack.h"
#include "third_party/blink/renderer/core/animation/worklet_animation_base.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/properties/css_bitset.h"
#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_counted_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/wtf/hash_counted_set.h"

namespace blink {

class CSSAnimations;

using AnimationCountedSet = HeapHashCountedSet<WeakMember<Animation>>;
using WorkletAnimationSet = HeapHashSet<WeakMember<WorkletAnimationBase>>;

class CORE_EXPORT ElementAnimations final
    : public GarbageCollected<ElementAnimations>,
      public ElementRareDataField {
 public:
  ElementAnimations();
  ElementAnimations(const ElementAnimations&) = delete;
  ElementAnimations& operator=(const ElementAnimations&) = delete;
  ~ElementAnimations();

  enum class CompositedPaintStatus {
    // A fresh compositing decision is required for an animated property.
    // Any style change for the corresponding property requires paint
    // invalidation. Even if rendered by a composited animation, we need to
    // trigger repaint in order to set up a worklet paint image. If the property
    // is animated, paint will decide if the animation is composited and will
    // update the status accordingly.
    kNeedsRepaint = 0,

    // An animation is affecting the target property, but it is not being
    // composited. Paint can short-circuit setting up a worklet paint image
    // since it is not required. Any style change affecting the target property
    // requires repaint, but no new compositing decision.
    kNotComposited = 1,

    // An animation affecting the target property is being rendered on the
    // compositor. Though repaint won't get triggered by a change to the
    // property, it can still be triggered for other reasons, in which case a
    // worklet paint image must be generated.
    kComposited = 2,

    // No animation affects the targeted property, so no paint invalidation or
    // image generation is required.
    kNoAnimation = 3
  };

  // Animations that are currently active for this element, their effects will
  // be applied during a style recalc. CSS Transitions are included in this
  // stack.
  EffectStack& GetEffectStack() { return effect_stack_; }
  const EffectStack& GetEffectStack() const { return effect_stack_; }
  // Tracks the state of active CSS Animations and Transitions. The individual
  // animations will also be part of the animation stack, but the mapping
  // between animation name and animation is kept here.
  CSSAnimations& CssAnimations() { return css_animations_; }
  const CSSAnimations& CssAnimations() const { return css_animations_; }

  // Animations which have effects targeting this element.
  AnimationCountedSet& Animations() { return animations_; }
  // Worklet Animations which have effects targeting this element.
  WorkletAnimationSet& GetWorkletAnimations() { return worklet_animations_; }

  bool IsEmpty() const {
    return effect_stack_.IsEmpty() && css_animations_.IsEmpty() &&
           animations_.empty() && worklet_animations_.empty();
  }

  void RestartAnimationOnCompositor();

  void SetAnimationStyleChange(bool animation_style_change) {
    animation_style_change_ = animation_style_change;
  }
  bool IsAnimationStyleChange() const { return animation_style_change_; }

  bool UpdateBoxSizeAndCheckTransformAxisAlignment(const gfx::SizeF& box_size);
  bool IsIdentityOrTranslation() const;

  bool HasCompositedPaintWorkletAnimation();

  void RecalcCompositedStatusForKeyframeChange(
      Element& element,
      Animation::NativePaintWorkletReasons properties);
  void RecalcCompositedStatus(Element* element);

  // TODO(crbug.com/1301961): Consider converting to an array or flat map of
  // fields for paint properties that can be composited.
  CompositedPaintStatus CompositedBackgroundColorStatus() {
    return static_cast<CompositedPaintStatus>(
        composited_background_color_status_);
  }

  bool SetCompositedBackgroundColorStatus(CompositedPaintStatus status);

  Animation* PaintWorkletClipPathAnimation() {
    return clip_path_paint_worklet_candidate_;
  }

  CompositedPaintStatus CompositedClipPathStatus() {
    return static_cast<CompositedPaintStatus>(composited_clip_path_status_);
  }

  bool SetCompositedClipPathStatus(CompositedPaintStatus status);

  void Trace(Visitor*) const override;

 private:
  EffectStack effect_stack_;
  CSSAnimations css_animations_;
  AnimationCountedSet animations_;
  WorkletAnimationSet worklet_animations_;

  // When an Element is being animated, its entire style will be dirtied every
  // frame by the running animation - even if the animation is only changing a
  // few properties. To avoid the expensive cost of recomputing the entire
  // style, we store a cached value of the 'base' computed style (e.g. with no
  // change from the running animations) and use that during style recalc,
  // applying only the animation changes on top of it.
  //
  // See also StyleBaseData.
  bool animation_style_change_ : 1;

  // The decision of whether to composite a compositable animations needs to
  // be made at Paint time and respected by the compositor.
  // The size of the bit-field must be updated if adding new
  // CompositedPaintStatus values to ensure that it can hold the value.
  unsigned composited_background_color_status_ : 2;
  unsigned composited_clip_path_status_ : 2;

  // Stores the current candidate for a composited clip-path animation. The
  // validity of this variable depends on composited_clip_path_status_. If
  // status is kNoAnimation or kNotComposited, the value will be nullptr. If the
  // status is kComposited, the value will be guaranteed to be a clip-path
  // animation that is eligible to be run on compositor. If the value is
  // kNeedsRepaint, the value is only guaranteed to be an animation on the
  // property, but may not necessarily be compositable.
  WeakMember<Animation> clip_path_paint_worklet_candidate_;

  FRIEND_TEST_ALL_PREFIXES(StyleEngineTest, PseudoElementBaseComputedStyle);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ELEMENT_ANIMATIONS_H_
