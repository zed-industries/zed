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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_KEYFRAME_EFFECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_KEYFRAME_EFFECT_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/animation/animation_effect.h"
#include "third_party/blink/renderer/core/animation/compositor_animations.h"
#include "third_party/blink/renderer/core/animation/keyframe_effect_model.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class Element;
class ExceptionState;
class KeyframeEffectModelBase;
class PaintArtifactCompositor;
class SampledEffect;
class V8UnionKeyframeEffectOptionsOrUnrestrictedDouble;

// Represents the effect of an Animation on an Element's properties.
// https://w3.org/TR/web-animations-1/#keyframe-effects
class CORE_EXPORT KeyframeEffect final : public AnimationEffect {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum Priority { kDefaultPriority, kTransitionPriority };

  // Web Animations API Bindings constructors.
  static KeyframeEffect* Create(
      ScriptState* script_state,
      Element* element,
      const ScriptValue& keyframes,
      const V8UnionKeyframeEffectOptionsOrUnrestrictedDouble* options,
      ExceptionState& exception_state);
  static KeyframeEffect* Create(ScriptState*,
                                Element*,
                                const ScriptValue&,
                                ExceptionState&);
  static KeyframeEffect* Create(ScriptState*, KeyframeEffect*, ExceptionState&);

  KeyframeEffect(Element*,
                 KeyframeEffectModelBase*,
                 const Timing&,
                 Priority = kDefaultPriority,
                 EventDelegate* = nullptr);
  ~KeyframeEffect() override;

  bool IsKeyframeEffect() const override { return true; }

  // IDL implementation.

  // Returns the target element. If the animation targets a pseudo-element,
  // this returns the originating element.
  Element* target() const { return target_element_.Get(); }
  void setTarget(Element*);
  const String& pseudoElement() const;
  void setPseudoElement(String, ExceptionState&);
  V8CompositeOperation composite() const;
  void setComposite(const V8CompositeOperation&);
  HeapVector<ScriptObject> getKeyframes(ScriptState*);
  void setKeyframes(ScriptState*,
                    const ScriptValue& keyframes,
                    ExceptionState&);

  // Returns blink's representation of the effect target.
  // This can be a blink::PseudoElement which should not be web-exposed.
  Element* EffectTarget() const { return effect_target_.Get(); }
  void SetKeyframes(StringKeyframeVector keyframes);

  bool Affects(const PropertyHandle&) const override;
  bool HasRevert() const;
  const KeyframeEffectModelBase* Model() const { return model_.Get(); }
  KeyframeEffectModelBase* Model() { return model_.Get(); }
  void SetModel(KeyframeEffectModelBase* model) {
    DCHECK(model);
    model_ = model;
  }
  Priority GetPriority() const { return priority_; }

  void NotifySampledEffectRemovedFromEffectStack();

  CompositorAnimations::FailureReasons CheckCanStartAnimationOnCompositor(
      const PaintArtifactCompositor*,
      double animation_playback_rate,
      PropertyHandleSet* unsupported_properties_for_tracing = nullptr) const;
  // Must only be called once.
  void StartAnimationOnCompositor(int group,
                                  std::optional<double> start_time,
                                  base::TimeDelta time_offset,
                                  double animation_playback_rate,
                                  CompositorAnimation* = nullptr,
                                  bool is_monotonic_timeline = true,
                                  bool is_boundary_aligned = false);
  bool HasActiveAnimationsOnCompositor() const;
  bool HasActiveAnimationsOnCompositor(const PropertyHandle&) const;
  bool CancelAnimationOnCompositor(CompositorAnimation*);
  void CancelIncompatibleAnimationsOnCompositor();
  void PauseAnimationForTestingOnCompositor(base::TimeDelta pause_time);

  void AttachCompositedLayers();

  void DowngradeToNormal() { priority_ = kDefaultPriority; }

  bool HasAnimation() const;
  bool HasPlayingAnimation() const;

  void Trace(Visitor*) const override;

  bool UpdateBoxSizeAndCheckTransformAxisAlignment(const gfx::SizeF& box_size);
  bool IsIdentityOrTranslation() const;

  ActiveInterpolationsMap InterpolationsForCommitStyles();

  // Explicitly setting the keyframes via KeyfrfameEffect.setFrames or
  // Animation.effect block subseuqent changes via CSS keyframe rules.
  bool GetIgnoreCSSKeyframes() { return ignore_css_keyframes_; }
  void SetIgnoreCSSKeyframes() { ignore_css_keyframes_ = true; }

  void SetLogicalPropertyResolutionContext(
      WritingDirectionMode writing_direction);

 private:
  EffectModel::CompositeOperation CompositeInternal() const;

  void ApplyEffects();
  void ClearEffects();
  void UpdateChildrenAndEffects() const override;
  void Attach(AnimationEffectOwner*) override;
  void Detach() override;
  void AttachTarget(Animation*);
  void DetachTarget(Animation*);
  void RefreshTarget();
  void CountAnimatedProperties() const;
  AnimationTimeDelta CalculateTimeToEffectChange(
      bool forwards,
      std::optional<AnimationTimeDelta> inherited_time,
      AnimationTimeDelta time_to_next_iteration) const override;
  std::optional<AnimationTimeDelta> TimelineDuration() const override;
  bool HasIncompatibleStyle() const;
  bool AffectsImportantProperty() const;
  void RestartRunningAnimationOnCompositor();

  Member<Element> effect_target_;
  Member<Element> target_element_;
  String target_pseudo_;
  Member<KeyframeEffectModelBase> model_;
  Member<SampledEffect> sampled_effect_;

  Priority priority_;

  // A keyframe effect with model ids has an animation on the compositor;
  // however, it may be in the process of being cancelled and the animation
  // should not be treated as if running on the compositor from the perspective
  // of paint.  Composited animations are cancelled asynchronously, to avoid
  // blocking the main thread in a protected sequence longer than necessary.
  Vector<int> compositor_keyframe_model_ids_;

  bool ignore_css_keyframes_;

  std::optional<gfx::SizeF> effect_target_size_;
};

template <>
struct DowncastTraits<KeyframeEffect> {
  static bool AllowFrom(const AnimationEffect& animationNode) {
    return animationNode.IsKeyframeEffect();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_KEYFRAME_EFFECT_H_
