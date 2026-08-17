// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_GRID_LENGTH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_GRID_LENGTH_H_

#include <memory>
#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/core/css/css_to_length_conversion_data.h"
#include "third_party/blink/renderer/platform/geometry/length.h"

namespace blink {

class CSSProperty;

// Represents a blink::GridLength, converted into a form that can be
// interpolated from/to.
// This class is a representation of the <track-breadth> values:
// <length-percentage> | <flex> | min-content | max-content | auto.
class CORE_EXPORT InterpolableGridLength final : public InterpolableValue {
 public:
  // |kLength| and |kFlex| are the only types that indicate interpolation may be
  // possible.
  enum InterpolableGridLengthType {
    kLength,
    kFlex,
    kAuto,
    kMinContent,
    kMaxContent,
  };

  InterpolableGridLength(InterpolableValue* value,
                         InterpolableGridLengthType type);
  static InterpolableGridLength* Create(const Length& grid_length,
                                        const CSSProperty& property,
                                        float zoom);

  Length CreateGridLength(
      const CSSToLengthConversionData& conversion_data) const;

  // InterpolableValue implementation:
  void Interpolate(const InterpolableValue& to,
                   const double progress,
                   InterpolableValue& result) const final;
  bool IsGridLength() const final { return true; }
  bool Equals(const InterpolableValue& other) const final;
  void Scale(double scale) final;
  void Add(const InterpolableValue& other) final;
  void AssertCanInterpolateWith(const InterpolableValue& other) const final;

  void Trace(Visitor* v) const override {
    InterpolableValue::Trace(v);
    v->Trace(value_);
  }

 private:
  // An |InterpolableGridLength| is content sized when it's 'auto',
  // 'max-content' or 'min-content'.
  bool IsContentSized() const;
  // Two |InterpolableGridLength| variables are compatible when they aren't
  // content sized and their type is the same.
  bool IsCompatibleWith(const InterpolableGridLength& other) const;

  InterpolableGridLength* RawClone() const final;
  InterpolableGridLength* RawCloneAndZero() const final;

  // The form of the interpolable value varies depending on the |type_|:
  // If the type is flex, form is |InterpolableNumber|.
  // If the type is length, form is |InterpolableLength|.
  // Everything else, |value_| is nulllptr.
  Member<InterpolableValue> value_;
  InterpolableGridLengthType type_;
};

template <>
struct DowncastTraits<InterpolableGridLength> {
  static bool AllowFrom(const InterpolableValue& interpolable_value) {
    return interpolable_value.IsGridLength();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_GRID_LENGTH_H_
