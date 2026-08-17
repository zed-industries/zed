// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_FILTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_FILTER_H_

#include <memory>
#include "base/notreached.h"
#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/filter_operation.h"

namespace blink {

class CSSProperty;
class CSSValue;
class StyleResolverState;

// Represents a blink::FilterOperation, converted into a form that can be
// interpolated from/to.
class CORE_EXPORT InterpolableFilter final : public InterpolableValue {
 public:
  InterpolableFilter(InterpolableValue* value,
                     FilterOperation::OperationType type)
      : value_(value), type_(type) {
    static_assert(std::is_trivially_destructible_v<InterpolableFilter>,
                  "Require trivial destruction for faster sweeping");
    DCHECK(value_);
  }

  static InterpolableFilter* MaybeCreate(
      const FilterOperation&,
      const CSSProperty& property,
      double zoom,
      mojom::blink::ColorScheme color_scheme,
      const ui::ColorProvider* color_provider);
  static InterpolableFilter* MaybeConvertCSSValue(const CSSValue&,
                                                  const StyleResolverState&);

  // Create an InterpolableFilter representing the 'initial value for
  // interpolation' for the given OperationType.
  static InterpolableFilter* CreateInitialValue(FilterOperation::OperationType);

  FilterOperation::OperationType GetType() const { return type_; }

  // Convert this InterpolableFilter back into a FilterOperation class, usually
  // to be applied to the style after interpolating |this|.
  FilterOperation* CreateFilterOperation(const StyleResolverState&) const;

  // InterpolableValue implementation:
  void Interpolate(const InterpolableValue& to,
                   const double progress,
                   InterpolableValue& result) const final;
  bool IsFilter() const final { return true; }
  bool Equals(const InterpolableValue& other) const final { NOTREACHED(); }
  void Scale(double scale) final { NOTREACHED(); }
  void Add(const InterpolableValue& other) final;
  void AssertCanInterpolateWith(const InterpolableValue& other) const final;

  void Trace(Visitor* v) const override {
    InterpolableValue::Trace(v);
    v->Trace(value_);
  }

 private:
  InterpolableFilter* RawClone() const final {
    return MakeGarbageCollected<InterpolableFilter>(value_->Clone(), type_);
  }
  InterpolableFilter* RawCloneAndZero() const final {
    return MakeGarbageCollected<InterpolableFilter>(value_->CloneAndZero(),
                                                    type_);
  }

  // Stores the interpolable data for the filter. The form varies depending on
  // the |type_|; see the implementation file for details of the mapping.
  Member<InterpolableValue> value_;

  FilterOperation::OperationType type_;
};

template <>
struct DowncastTraits<InterpolableFilter> {
  static bool AllowFrom(const InterpolableValue& interpolable_value) {
    return interpolable_value.IsFilter();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_FILTER_H_
