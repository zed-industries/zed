// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TRANSITION_KEYFRAME_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TRANSITION_KEYFRAME_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/animation/css/compositor_keyframe_value.h"
#include "third_party/blink/renderer/core/animation/keyframe.h"
#include "third_party/blink/renderer/core/animation/property_handle.h"
#include "third_party/blink/renderer/core/animation/typed_interpolation_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// An implementation of Keyframe specifically for CSS Transitions.
//
// TransitionKeyframes are a simple form of keyframe, which only have one
// (property, value) pair. CSS Transitions do not support SVG attributes, so the
// property will always be a CSSPropertyID (for CSS properties and presentation
// attributes) or an AtomicString (for custom CSS properties).
class CORE_EXPORT TransitionKeyframe : public Keyframe {
 public:
  class CORE_EXPORT IterableTransitionKeyframeProperty
      : public Keyframe::IterableProperties {
   public:
    explicit IterableTransitionKeyframeProperty(const PropertyHandle& property)
        : property_(property) {}
    ~IterableTransitionKeyframeProperty() override = default;
    PropertyIteratorWrapper begin() const override;
    size_t size() const override { return 1u; }
    bool IsTransitionProperties() const override { return true; }

   private:
    friend class TransitionKeyframe;

    const PropertyHandle property_;
  };

  explicit TransitionKeyframe(const PropertyHandle& property)
      : Keyframe(MakeGarbageCollected<IterableTransitionKeyframeProperty>(
            property)) {}

  TransitionKeyframe(const TransitionKeyframe& copy_from)
      : Keyframe(MakeGarbageCollected<IterableTransitionKeyframeProperty>(
                     copy_from.Property()),
                 copy_from.offset_,
                 copy_from.timeline_offset_,
                 copy_from.composite_,
                 copy_from.easing_),
        value_(copy_from.value_->Clone()),
        compositor_value_(copy_from.compositor_value_) {}

  void SetValue(TypedInterpolationValue* value) {
    // Speculative CHECK to help investigate crbug.com/826627. The theory is
    // that |SetValue| is being called with a |value| that has no underlying
    // InterpolableValue. This then would later cause a crash in the
    // TransitionInterpolation constructor.
    // TODO(crbug.com/826627): Revert once bug is fixed.
    CHECK(!!value->Value());
    value_ = value;
  }
  void SetCompositorValue(CompositorKeyframeValue*);
  void AddKeyframePropertiesToV8Object(V8ObjectBuilder&,
                                       Element*) const override;

  void Trace(Visitor*) const override;

  class PropertySpecificKeyframe : public Keyframe::PropertySpecificKeyframe {
   public:
    PropertySpecificKeyframe(double offset,
                             scoped_refptr<TimingFunction> easing,
                             EffectModel::CompositeOperation composite,
                             TypedInterpolationValue* value,
                             CompositorKeyframeValue* compositor_value)
        : Keyframe::PropertySpecificKeyframe(offset,
                                             std::move(easing),
                                             composite),
          value_(value),
          compositor_value_(compositor_value) {}

    const CompositorKeyframeValue* GetCompositorKeyframeValue() const final {
      return compositor_value_.Get();
    }

    bool IsNeutral() const final { return false; }
    bool IsRevert() const final { return false; }
    bool IsRevertLayer() const final { return false; }
    Keyframe::PropertySpecificKeyframe* NeutralKeyframe(
        double offset,
        scoped_refptr<TimingFunction> easing) const final {
      NOTREACHED();
    }
    Interpolation* CreateInterpolation(
        const PropertyHandle&,
        const Keyframe::PropertySpecificKeyframe& other) const final;

    bool IsTransitionPropertySpecificKeyframe() const final { return true; }

    const TypedInterpolationValue* GetValue() const { return value_.Get(); }

    void Trace(Visitor*) const override;

   private:
    Member<TypedInterpolationValue> value_;
    Member<CompositorKeyframeValue> compositor_value_;
  };

 private:
  bool IsTransitionKeyframe() const final { return true; }

  const PropertyHandle& Property() const {
    return To<IterableTransitionKeyframeProperty>(&Properties())->property_;
  }

  Keyframe* Clone() const final {
    return MakeGarbageCollected<TransitionKeyframe>(*this);
  }

  Keyframe::PropertySpecificKeyframe* CreatePropertySpecificKeyframe(
      const PropertyHandle&,
      EffectModel::CompositeOperation effect_composite,
      double offset) const final;

  Member<TypedInterpolationValue> value_;
  Member<CompositorKeyframeValue> compositor_value_;
};

using TransitionPropertySpecificKeyframe =
    TransitionKeyframe::PropertySpecificKeyframe;

template <>
struct DowncastTraits<TransitionKeyframe> {
  static bool AllowFrom(const Keyframe& value) {
    return value.IsTransitionKeyframe();
  }
};
template <>
struct DowncastTraits<TransitionPropertySpecificKeyframe> {
  static bool AllowFrom(const Keyframe::PropertySpecificKeyframe& value) {
    return value.IsTransitionPropertySpecificKeyframe();
  }
};
template <>
struct DowncastTraits<TransitionKeyframe::IterableTransitionKeyframeProperty> {
  static bool AllowFrom(const Keyframe::IterableProperties& properties) {
    return properties.IsTransitionProperties();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TRANSITION_KEYFRAME_H_
