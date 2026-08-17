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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_COMPOSITOR_ANIMATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_COMPOSITOR_ANIMATIONS_H_

#include <memory>
#include <optional>

#include "base/time/time.h"
#include "cc/animation/keyframe_model.h"
#include "third_party/blink/renderer/core/animation/effect_model.h"
#include "third_party/blink/renderer/core/animation/keyframe.h"
#include "third_party/blink/renderer/core/animation/timing.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/animation/timing_function.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Animation;
class CompositorAnimation;
class Element;
class KeyframeEffectModelBase;
class Node;
class PaintArtifactCompositor;
class SVGElement;

class CORE_EXPORT CompositorAnimations {
  STATIC_ONLY(CompositorAnimations);

 public:
  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  using FailureReasons = uint32_t;
  enum FailureReason : uint32_t {
    kNoFailure = 0,

    // Cases where the compositing is disabled by an exterior cause.
    kAcceleratedAnimationsDisabled = 1 << 0,
    kEffectSuppressedByDevtools = 1 << 1,

    // There are many cases where an animation may not be valid (e.g. it is not
    // playing, or has no effect, etc). In these cases we would never composite
    // it in any world, so we lump them together.
    kInvalidAnimationOrEffect = 1 << 2,

    // The compositor is not able to support all setups of timing values; see
    // CompositorAnimations::ConvertTimingForCompositor.
    kEffectHasUnsupportedTimingParameters = 1 << 3,

    // Currently the compositor does not support any composite mode other than
    // 'replace'.
    kEffectHasNonReplaceCompositeMode = 1 << 4,

    // Cases where the target element isn't in a valid compositing state.
    kTargetHasInvalidCompositingState = 1 << 5,

    // Cases where the target is invalid (but that we could feasibly address).
    kTargetHasIncompatibleAnimations = 1 << 6,
    kTargetHasCSSOffset = 1 << 7,

    // This failure reason is no longer used, as multiple transform-related
    // animations are allowed on the same target provided they target different
    // transform properties (e.g. rotate vs scale).
    kObsoleteTargetHasMultipleTransformProperties = 1 << 8,

    // Cases relating to the properties being animated.
    kAnimationAffectsNonCSSProperties = 1 << 9,
    kTransformRelatedPropertyCannotBeAcceleratedOnTarget = 1 << 10,
    kFilterRelatedPropertyMayMovePixels = 1 << 12,
    kUnsupportedCSSProperty = 1 << 13,

    // This failure reason is no longer used, as multiple transform-related
    // animations are allowed on the same target provided they target different
    // transform properties (e.g. rotate vs scale).
    kObsoleteMultipleTransformAnimationsOnSameTarget = 1 << 14,

    kMixedKeyframeValueTypes = 1 << 15,

    // Cases where the scroll timeline source is not composited.
    kTimelineSourceHasInvalidCompositingState = 1 << 16,

    // Cases where there is an animation that has no visible change through the
    // active phase. This could be due to optimizing out an off-screen
    // composited animation or due to having only constant valued properties.
    kAnimationHasNoVisibleChange = 1 << 17,

    // Cases where we are animating a property that is marked important.
    kAffectsImportantProperty = 1 << 18,

    kSVGTargetHasIndependentTransformProperty = 1 << 19,

    // When adding new values, update the count below *and* add a description
    // of the value to CompositorAnimationsFailureReason in
    // tools/metrics/histograms/enums.xml .

    // The maximum number of flags in this enum (excluding itself). New flags
    // should increment this number but it should never be decremented because
    // the values are used in UMA histograms. It should also be noted that it
    // excludes the kNoFailure value.
    kFailureReasonCount = 20,
  };

  static FailureReasons CheckCanStartAnimationOnCompositor(
      const Timing&,
      const Timing::NormalizedTiming&,
      const Element&,
      const Animation*,
      const EffectModel&,
      const PaintArtifactCompositor*,
      double animation_playback_rate,
      PropertyHandleSet* unsupported_properties_for_tracing = nullptr);
  static bool CompositorPropertyAnimationsHaveNoEffect(
      const Element& target_element,
      const Animation* animation,
      const EffectModel& effect,
      const PaintArtifactCompositor*);
  static void CancelIncompatibleAnimationsOnCompositor(const Element&,
                                                       const Animation&,
                                                       const EffectModel&);
  static void StartAnimationOnCompositor(
      const Element&,
      int group,
      std::optional<double> start_time,
      base::TimeDelta time_offset,
      const Timing&,
      const Timing::NormalizedTiming&,
      const Animation*,
      CompositorAnimation&,
      const EffectModel&,
      Vector<int>& started_keyframe_model_ids,
      double animation_playback_rate,
      bool is_monotonic_timeline,
      bool is_boundary_aligned);
  static void CancelAnimationOnCompositor(const Element&,
                                          CompositorAnimation*,
                                          int id,
                                          const EffectModel& model);
  static void PauseAnimationForTestingOnCompositor(const Element&,
                                                   const Animation&,
                                                   int id,
                                                   base::TimeDelta pause_time,
                                                   const EffectModel&);

  static void AttachCompositedLayers(Element&, CompositorAnimation*);

  struct CompositorTiming {
    Timing::PlaybackDirection direction;
    AnimationTimeDelta scaled_duration;
    base::TimeDelta scaled_time_offset;
    double adjusted_iteration_count;
    double playback_rate;
    Timing::FillMode fill_mode;
    double iteration_start;
  };

  static bool ConvertTimingForCompositor(const Timing&,
                                         const Timing::NormalizedTiming&,
                                         base::TimeDelta time_offset,
                                         CompositorTiming& out,
                                         double animation_playback_rate,
                                         bool is_monotonic_timeline = true,
                                         bool is_boundary_aligned = false);

  static void GetAnimationOnCompositor(
      const Element&,
      const Timing&,
      const Timing::NormalizedTiming&,
      int group,
      std::optional<double> start_time,
      base::TimeDelta time_offset,
      const KeyframeEffectModelBase&,
      Vector<std::unique_ptr<cc::KeyframeModel>>& animations,
      double animation_playback_rate,
      bool is_monotonic_timeline,
      bool is_boundary_aligned);

  static CompositorElementIdNamespace CompositorElementNamespaceForProperty(
      CSSPropertyID property);

  static bool CanStartScrollTimelineOnCompositor(Node* target);

  static bool CanStartTransformAnimationOnCompositorForSVG(const SVGElement&);

  static bool CompositedPropertyRequiresSnapshot(
      const PropertyHandle& property);

 private:
  static FailureReasons CheckCanStartEffectOnCompositor(
      const Timing&,
      const Timing::NormalizedTiming&,
      const Element&,
      const Animation*,
      const EffectModel&,
      const PaintArtifactCompositor*,
      double animation_playback_rate,
      PropertyHandleSet* unsupported_properties_for_tracing = nullptr);
  static FailureReasons CheckCanStartElementOnCompositor(
      const Element& element,
      const EffectModel& model);
  static FailureReasons CheckCanStartSVGElementOnCompositor(const SVGElement&);
  // This doesn't include the reasons returned from the above function.
  static FailureReasons CheckCanStartTransformAnimationOnCompositorForSVG(
      const SVGElement&);

  friend class AnimationCompositorAnimationsTest;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_COMPOSITOR_ANIMATIONS_H_
