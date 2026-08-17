// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_PRIMITIVE_INTERPOLATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_PRIMITIVE_INTERPOLATION_H_

#include <cmath>
#include <limits>
#include <memory>
#include <utility>

#include "base/check_op.h"
#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/core/animation/typed_interpolation_value.h"
#include "third_party/blink/renderer/platform/geometry/blend.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Represents an animation's effect between an adjacent pair of
// PropertySpecificKeyframes after converting the keyframes to an internal
// format with respect to the animation environment and underlying values.
class PrimitiveInterpolation : public GarbageCollected<PrimitiveInterpolation> {
 public:
  PrimitiveInterpolation(const PrimitiveInterpolation&) = delete;
  PrimitiveInterpolation& operator=(const PrimitiveInterpolation&) = delete;
  virtual ~PrimitiveInterpolation() = default;

  virtual void InterpolateValue(
      double fraction,
      Member<TypedInterpolationValue>& result) const = 0;
  virtual double InterpolateUnderlyingFraction(double start,
                                               double end,
                                               double fraction) const = 0;
  virtual bool IsFlip() const { return false; }

  virtual void Trace(Visitor*) const {}

 protected:
  PrimitiveInterpolation() = default;
};

// Represents a pair of keyframes that are compatible for "smooth" interpolation
// eg. "0px" and "100px".
class PairwisePrimitiveInterpolation : public PrimitiveInterpolation {
 public:
  PairwisePrimitiveInterpolation(
      const InterpolationType& type,
      InterpolableValue* start,
      InterpolableValue* end,
      const NonInterpolableValue* non_interpolable_value)
      : type_(type),
        start_(start),
        end_(end),
        non_interpolable_value_(non_interpolable_value) {
    DCHECK(start_);
    DCHECK(end_);
  }

  ~PairwisePrimitiveInterpolation() override = default;

  const InterpolationType& GetType() const { return type_; }

  TypedInterpolationValue* InitialValue() const {
    return MakeGarbageCollected<TypedInterpolationValue>(
        type_, start_->Clone(), non_interpolable_value_);
  }

  void Trace(Visitor* v) const override {
    PrimitiveInterpolation::Trace(v);
    v->Trace(start_);
    v->Trace(end_);
    v->Trace(non_interpolable_value_);
  }

 private:
  void InterpolateValue(double fraction,
                        Member<TypedInterpolationValue>& result) const final {
    DCHECK(result);
    DCHECK_EQ(&result->GetType(), &type_);
    DCHECK_EQ(result->GetNonInterpolableValue(), non_interpolable_value_.Get());
    start_->AssertCanInterpolateWith(*end_);
    start_->Interpolate(*end_, fraction,
                        *result->MutableValue().interpolable_value);
  }

  double InterpolateUnderlyingFraction(double start,
                                       double end,
                                       double fraction) const final {
    return Blend(start, end, fraction);
  }

  const InterpolationType& type_;
  Member<InterpolableValue> start_;
  Member<InterpolableValue> end_;
  Member<const NonInterpolableValue> non_interpolable_value_;
};

// Represents a pair of incompatible keyframes that fall back to 50% flip
// behaviour eg. "auto" and "0px".
class FlipPrimitiveInterpolation : public PrimitiveInterpolation {
 public:
  FlipPrimitiveInterpolation(TypedInterpolationValue* start,
                             TypedInterpolationValue* end)
      : start_(start),
        end_(end),
        last_fraction_(std::numeric_limits<double>::quiet_NaN()) {}

  ~FlipPrimitiveInterpolation() override = default;

  void Trace(Visitor* v) const override {
    PrimitiveInterpolation::Trace(v);
    v->Trace(start_);
    v->Trace(end_);
  }

 private:
  void InterpolateValue(double fraction,
                        Member<TypedInterpolationValue>& result) const final {
    if (!std::isnan(last_fraction_) &&
        (fraction < 0.5) == (last_fraction_ < 0.5)) {
      return;
    }
    const TypedInterpolationValue* side =
        ((fraction < 0.5) ? start_ : end_).Get();
    result = side ? side->Clone() : nullptr;
    last_fraction_ = fraction;
  }

  double InterpolateUnderlyingFraction(double start,
                                       double end,
                                       double fraction) const final {
    return fraction < 0.5 ? start : end;
  }

  bool IsFlip() const final { return true; }

  Member<TypedInterpolationValue> start_;
  Member<TypedInterpolationValue> end_;
  mutable double last_fraction_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_PRIMITIVE_INTERPOLATION_H_
