// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_NATIVE_CSS_PAINT_DEFINITION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_NATIVE_CSS_PAINT_DEFINITION_H_

#include "third_party/blink/renderer/core/css/cssom/paint_worklet_input.h"
#include "third_party/blink/renderer/modules/csspaint/nativepaint/native_paint_definition.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "ui/gfx/animation/keyframe/timing_function.h"

namespace blink {

class LocalFrame;
class CSSProperty;
class Element;

// See README.md for how this class fits in to the overall design
class MODULES_EXPORT NativeCssPaintDefinition : public NativePaintDefinition {
 public:
  ~NativeCssPaintDefinition() override = default;

  // Validation function for determining if a value / interpolable_value is
  // supported on the compositor.
  using ValueFilter = bool (*)(const Element* element,
                               const CSSValue* value,
                               const InterpolableValue* interpolable_value);

  // Returns an animation for the given property, if it is compositable
  // excepting additional checks in CheckCanStartAnimationOnCompositor. Used
  // as a base for NativePaintImageGenerator::GetAnimationIfCompositable methods
  // which are implemented in the different Native Paint Definitions
  static Animation* GetAnimationForProperty(
      const Element* element,
      const CSSProperty& property,
      ValueFilter filter = DefaultValueFilter);

  // Used by GetAnimationForProperty and others to verify that the given
  // animation meets compositable paint-worklet animation criteria, excluding
  // additional checks in CheckCanStartAnimationOnCompositor which are the
  // caller's responsibility to verify.
  static bool AnimationIsValidForPaintWorklets(
      Animation* compositable_animation,
      const Element* element,
      const CSSProperty& property,
      NativeCssPaintDefinition::ValueFilter filter);

  // Used by GetAnimationForProperty to check the individual keyframe values.
  static bool CanGetValueFromKeyframe(const Element* element,
                                      const PropertySpecificKeyframe* frame,
                                      const KeyframeEffectModelBase* model,
                                      ValueFilter filter);

  // Default validator for a keyframe value, which accepts any non-null value
  // as being supported. Replace with a property specific validator as needed.
  static bool DefaultValueFilter(const Element* element,
                                 const CSSValue* value,
                                 const InterpolableValue* interpolable_value);

  struct BaseKeyframe {
    BaseKeyframe(double offset, std::unique_ptr<gfx::TimingFunction>& tf)
        : offset(offset), timing_function(tf.release()) {}
    double offset;
    std::unique_ptr<gfx::TimingFunction> timing_function;
  };

  template <typename T>
  struct TypedKeyframe : public BaseKeyframe {
    TypedKeyframe(double offset, std::unique_ptr<gfx::TimingFunction>& tf, T v)
        : BaseKeyframe(offset, tf), value(v) {}
    T value;
  };

  struct KeyframeIndexAndProgress {
    unsigned index;
    double progress;
  };

 protected:
  NativeCssPaintDefinition(LocalFrame*,
                           PaintWorkletInput::PaintWorkletInputType);
  NativeCssPaintDefinition() = default;

  // TODO(crbug.com/381126162): Unify this with the implementation in composited
  // clip path animations.
  // Computes the correct keyframe index and intra-keyframe progress given the
  // global progress. Used to ensure that custom timing functions are handled
  // correctly.
  template <typename T>
  KeyframeIndexAndProgress ComputeKeyframeIndexAndProgress(
      const std::optional<double>& main_thread_progress,
      const CompositorPaintWorkletJob::AnimatedPropertyValues&
          animated_property_values,
      const Vector<TypedKeyframe<T>>& keyframes) {
    DCHECK_GT(keyframes.size(), 1u);

    // TODO(crbug.com/40173432): We should handle the case when the animation is
    // inactive due to being in the before or after phase. When inactive,
    // progress is null. For now, clamping to the start of the animation, which
    // is incorrect. The worklet input will need the underlying property value.
    // Currently, animations with a positive input delay are run on the main
    // thread. This issue will need to be addressed in order to run the
    // animation on the compositor thread.
    double progress =
        Progress(main_thread_progress, animated_property_values).value_or(0);

    // Get the bounding keyframes based on the progress and offsets.
    // TODO(kevers): avoid a linear walk when the number of keyframes is large.
    unsigned result_index = keyframes.size() - 1;
    if (progress <= 0) {
      result_index = 0;
    } else if (progress > 0 && progress < 1) {
      for (unsigned i = 0; i < keyframes.size() - 1; i++) {
        if (progress <= keyframes[i + 1].offset) {
          result_index = i;
          break;
        }
      }
    }
    if (result_index == keyframes.size() - 1) {
      result_index = keyframes.size() - 2;
    }
    // Because the progress is a global one, we need to adjust it with offsets.
    double local_progress =
        (progress - keyframes[result_index].offset) /
        (keyframes[result_index + 1].offset - keyframes[result_index].offset);

    // TODO(crbug.com/347958668): Fix limit direction to account for phase and
    // direction. Important for making the correct decision at the boundary when
    // using a step timing function. Currently blocked on lack of support for a
    // start delay.
    double transformed_progress =
        keyframes[result_index].timing_function
            ? keyframes[result_index].timing_function->GetValue(
                  local_progress, TimingFunction::LimitDirection::RIGHT)
            : local_progress;

    return {result_index, transformed_progress};
  }

  static double Interpolate(double from, double to, double progress) {
    return from * (1 - progress) + to * progress;
  }

 private:
  std::optional<double> Progress(
      const std::optional<double>& main_thread_progress,
      const CompositorPaintWorkletJob::AnimatedPropertyValues&
          animated_property_values);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_NATIVE_CSS_PAINT_DEFINITION_H_
