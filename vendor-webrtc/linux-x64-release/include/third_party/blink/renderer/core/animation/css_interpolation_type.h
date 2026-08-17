// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_INTERPOLATION_TYPE_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/animation/css_interpolation_environment.h"
#include "third_party/blink/renderer/core/animation/interpolation_type.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class ComputedStyle;
class PropertyRegistration;
class StyleResolverState;

class CORE_EXPORT CSSInterpolationType : public InterpolationType {
 public:
  class CSSConversionChecker : public ConversionChecker {
   public:
    bool IsValid(const CSSInterpolationEnvironment& environment,
                 const InterpolationValue& underlying) const final {
      return IsValid(environment.GetState(), underlying);
    }

   protected:
    virtual bool IsValid(const StyleResolverState&,
                         const InterpolationValue& underlying) const = 0;
  };

  virtual InterpolationValue MaybeConvertNeutral(
      const InterpolationValue& underlying,
      ConversionCheckers&) const = 0;
  virtual InterpolationValue MaybeConvertInitial(const StyleResolverState&,
                                                 ConversionCheckers&) const = 0;
  virtual InterpolationValue MaybeConvertInherit(const StyleResolverState&,
                                                 ConversionCheckers&) const = 0;
  virtual InterpolationValue MaybeConvertValue(const CSSValue&,
                                               const StyleResolverState&,
                                               ConversionCheckers&) const = 0;
  virtual const CSSValue* CreateCSSValue(const InterpolableValue&,
                                         const NonInterpolableValue*,
                                         const StyleResolverState&) const {
    // TODO(alancutter): Implement this for all subclasses and make this an
    // abstract declaration so the return type can be changed to
    // const CSSValue&.
    NOTREACHED();
  }

  // The interpolation stack has an optimization where we perform compositing
  // after interpolation. This is against spec, but it works for simple addition
  // cases and halves the amount of computation needed. Some types require
  // compositing before interpolation (e.g. if their composition operator is a
  // concatenation), however, and for those we define this method that is called
  // pre-interpolation.
  // TODO(crbug.com/1009230): Revisit the post-interpolation composite
  // optimization.
  virtual InterpolationValue PreInterpolationCompositeIfNeeded(
      InterpolationValue value,
      const InterpolationValue& underlying,
      EffectModel::CompositeOperation,
      ConversionCheckers&) const {
    return value;
  }

  virtual InterpolationValue MaybeConvertCustomPropertyUnderlyingValue(
      const CSSValue&) const {
    NOTREACHED();
  }

 protected:
  explicit CSSInterpolationType(PropertyHandle,
                                const PropertyRegistration* = nullptr);

  const CSSProperty& CssProperty() const {
    return GetProperty().GetCSSProperty();
  }

  InterpolationValue MaybeConvertSingle(const PropertySpecificKeyframe&,
                                        const CSSInterpolationEnvironment&,
                                        const InterpolationValue& underlying,
                                        ConversionCheckers&) const final;

  InterpolationValue MaybeConvertUnderlyingValue(
      const CSSInterpolationEnvironment&) const override;
  virtual InterpolationValue MaybeConvertStandardPropertyUnderlyingValue(
      const ComputedStyle&) const = 0;

  void Apply(const InterpolableValue&,
             const NonInterpolableValue*,
             CSSInterpolationEnvironment&) const final;
  virtual void ApplyStandardPropertyValue(const InterpolableValue&,
                                          const NonInterpolableValue*,
                                          StyleResolverState&) const = 0;

 private:
  InterpolationValue MaybeConvertSingleInternal(
      const PropertySpecificKeyframe&,
      const CSSInterpolationEnvironment&,
      const InterpolationValue& underlying,
      ConversionCheckers&) const;

  InterpolationValue MaybeConvertCustomPropertyDeclaration(
      const CSSValue&,
      const TreeScope* keyframe_tree_scope,
      const CSSInterpolationEnvironment&,
      ConversionCheckers&) const;

  const PropertyRegistration& Registration() const {
    DCHECK(GetProperty().IsCSSCustomProperty());
    return *registration_;
  }

  void ApplyCustomPropertyValue(const InterpolableValue&,
                                const NonInterpolableValue*,
                                StyleResolverState&) const;

  WeakPersistent<const PropertyRegistration> registration_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_INTERPOLATION_TYPE_H_
