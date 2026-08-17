// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_VALUE_H_

#include <array>
#include <memory>
#include <utility>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_math_expression_node.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

// Represents the components of a PropertySpecificKeyframe's value that change
// smoothly as it interpolates to an adjacent value.
class CORE_EXPORT InterpolableValue
    : public GarbageCollected<InterpolableValue> {
 public:
  // Interpolates from |this| InterpolableValue towards |to| at the given
  // |progress|, placing the output in |result|. That is:
  //
  //   result = this * (1 - progress) + to * progress
  //
  // Callers must make sure that |this|, |to|, and |result| are all of the same
  // concrete subclass.
  virtual void Interpolate(const InterpolableValue& to,
                           const double progress,
                           InterpolableValue& result) const = 0;

  virtual bool IsDouble() const { return false; }
  virtual bool IsNumber() const { return false; }
  virtual bool IsBool() const { return false; }
  virtual bool IsColor() const { return false; }
  virtual bool IsStyleColor() const { return false; }
  virtual bool IsScrollbarColor() const { return false; }
  virtual bool IsList() const { return false; }
  virtual bool IsLength() const { return false; }
  virtual bool IsAspectRatio() const { return false; }
  virtual bool IsShadow() const { return false; }
  virtual bool IsFilter() const { return false; }
  virtual bool IsTransformList() const { return false; }
  virtual bool IsGridLength() const { return false; }
  virtual bool IsGridTrackList() const { return false; }
  virtual bool IsGridTrackRepeater() const { return false; }
  virtual bool IsGridTrackSize() const { return false; }
  virtual bool IsFontPalette() const { return false; }
  virtual bool IsDynamicRangeLimit() const { return false; }

  // TODO(alancutter): Remove Equals().
  virtual bool Equals(const InterpolableValue&) const = 0;
  virtual void Scale(double scale) = 0;
  virtual void Add(const InterpolableValue& other) = 0;
  // The default implementation should be sufficient for most types, but
  // subclasses can override this to be more efficient if they chose.
  virtual void ScaleAndAdd(double scale, const InterpolableValue& other) {
    Scale(scale);
    Add(other);
  }
  virtual void AssertCanInterpolateWith(
      const InterpolableValue& other) const = 0;

  // Clone this value, optionally zeroing out the components at the same time.
  // These are not virtual to allow for covariant return types; see
  // documentation on RawClone/RawCloneAndZero.
  InterpolableValue* Clone() const { return RawClone(); }
  InterpolableValue* CloneAndZero() const { return RawCloneAndZero(); }

  virtual void Trace(Visitor*) const {}

 private:
  // Helper methods to allow covariant Clone/CloneAndZero methods. Concrete
  // subclasses should not expose these methods publically, but instead should
  // declare their own version of Clone/CloneAndZero with a concrete return type
  // if it is useful for their clients.
  virtual InterpolableValue* RawClone() const = 0;
  virtual InterpolableValue* RawCloneAndZero() const = 0;
};

template <typename T>
struct ThreadingTrait<
    T,
    std::enable_if_t<std::is_base_of_v<InterpolableValue, T>>> {
  static constexpr ThreadAffinity kAffinity = kMainThreadOnly;
};

class CORE_EXPORT InterpolableNumber final : public InterpolableValue {
 public:
  InterpolableNumber() = default;
  explicit InterpolableNumber(double value,
                              CSSPrimitiveValue::UnitType unit_type =
                                  CSSPrimitiveValue::UnitType::kNumber);
  explicit InterpolableNumber(const CSSMathExpressionNode& expression);
  explicit InterpolableNumber(const CSSPrimitiveValue& value);

  // TODO(crbug.com/1521261): Remove this, once the bug is fixed.
  double Value() const { return value_; }
  double Value(const CSSLengthResolver& length_resolver) const;

  static double Interpolate(double from, double to, double progress);

  // InterpolableValue
  void Interpolate(const InterpolableValue& to,
                   const double progress,
                   InterpolableValue& result) const final;
  bool IsNumber() const final { return true; }
  bool Equals(const InterpolableValue& other) const final;
  void Scale(double scale) final;
  void Scale(const InterpolableNumber& other);
  void Add(const InterpolableValue& other) final;
  void AssertCanInterpolateWith(const InterpolableValue& other) const final;

  InterpolableNumber* Clone() const { return RawClone(); }
  InterpolableNumber* CloneAndZero() const { return RawCloneAndZero(); }

  void Trace(Visitor* v) const override {
    InterpolableValue::Trace(v);
    v->Trace(expression_);
  }

 private:
  InterpolableNumber* RawClone() const final {
    if (IsDoubleValue()) {
      return MakeGarbageCollected<InterpolableNumber>(value_, unit_type_);
    }
    return MakeGarbageCollected<InterpolableNumber>(*expression_);
  }
  InterpolableNumber* RawCloneAndZero() const final {
    return MakeGarbageCollected<InterpolableNumber>(0, unit_type_);
  }

  bool IsDoubleValue() const { return type_ == Type::kDouble; }
  bool IsExpression() const { return type_ == Type::kExpression; }

  void SetDouble(double value, CSSPrimitiveValue::UnitType unit_type);
  void SetExpression(const CSSMathExpressionNode& expression);
  const CSSMathExpressionNode& AsExpression() const;
  CSSPrimitiveValue::UnitType ResolvedUnitType() const {
    return IsDouble() ? unit_type_ : expression_->ResolvedUnitType();
  }

  enum class Type { kDouble, kExpression };
  Type type_;
  double value_;
  CSSPrimitiveValue::UnitType unit_type_;
  Member<const CSSMathExpressionNode> expression_;
};

static_assert(std::is_trivially_destructible_v<InterpolableNumber>,
              "Require trivial destruction for faster sweeping");

class CORE_EXPORT InterpolableList final : public InterpolableValue {
 public:
  explicit InterpolableList(wtf_size_t size) : values_(size) {
    static_assert(std::is_trivially_destructible_v<InterpolableList>,
                  "Require trivial destruction for faster sweeping");
  }

  explicit InterpolableList(HeapVector<Member<InterpolableValue>>&& values)
      : values_(std::move(values)) {}

  InterpolableList(const InterpolableList&) = delete;
  InterpolableList& operator=(const InterpolableList&) = delete;
  InterpolableList(InterpolableList&&) = default;
  InterpolableList& operator=(InterpolableList&&) = default;

  const InterpolableValue* Get(wtf_size_t position) const {
    return values_[position];
  }
  Member<InterpolableValue>& GetMutable(wtf_size_t position) {
    return values_[position];
  }
  wtf_size_t length() const { return values_.size(); }
  void Set(wtf_size_t position, InterpolableValue* value) {
    values_[position] = std::move(value);
  }

  InterpolableList* Clone() const { return RawClone(); }
  InterpolableList* CloneAndZero() const { return RawCloneAndZero(); }

  // InterpolableValue
  void Interpolate(const InterpolableValue& to,
                   const double progress,
                   InterpolableValue& result) const final;
  bool IsList() const final { return true; }
  bool Equals(const InterpolableValue& other) const final;
  void Scale(double scale) final;
  void Add(const InterpolableValue& other) final;
  // We override this to avoid two passes on the list from the base version.
  void ScaleAndAdd(double scale, const InterpolableValue& other) final;
  void AssertCanInterpolateWith(const InterpolableValue& other) const final;

  void Trace(Visitor* v) const override {
    InterpolableValue::Trace(v);
    v->Trace(values_);
  }

 private:
  InterpolableList* RawClone() const final {
    auto* result = MakeGarbageCollected<InterpolableList>(length());
    for (wtf_size_t i = 0; i < length(); i++) {
      result->Set(i, values_[i]->Clone());
    }
    return result;
  }
  InterpolableList* RawCloneAndZero() const final;

  HeapVector<Member<InterpolableValue>> values_;
};

template <>
struct DowncastTraits<InterpolableNumber> {
  static bool AllowFrom(const InterpolableValue& value) {
    return value.IsNumber();
  }
};
template <>
struct DowncastTraits<InterpolableList> {
  static bool AllowFrom(const InterpolableValue& value) {
    return value.IsList();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_VALUE_H_
