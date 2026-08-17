// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_LIST_INTERPOLATION_FUNCTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_LIST_INTERPOLATION_FUNCTIONS_H_

#include <memory>
#include "base/functional/function_ref.h"
#include "third_party/blink/renderer/core/animation/interpolation_value.h"
#include "third_party/blink/renderer/core/animation/pairwise_interpolation_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class UnderlyingValue;
class UnderlyingValueOwner;
class InterpolationType;

class CORE_EXPORT ListInterpolationFunctions {
  STACK_ALLOCATED();

 public:
  template <typename CreateItemCallback>
  static InterpolationValue CreateList(wtf_size_t length, CreateItemCallback);
  static InterpolationValue CreateEmptyList() {
    return InterpolationValue(MakeGarbageCollected<InterpolableList>(0));
  }

  enum class LengthMatchingStrategy {
    kEqual,
    kLowestCommonMultiple,
    kPadToLargest
  };

  using MergeSingleItemConversionsCallback =
      base::FunctionRef<PairwiseInterpolationValue(InterpolationValue&&,
                                                   InterpolationValue&&)>;

  static PairwiseInterpolationValue MaybeMergeSingles(
      InterpolationValue&& start,
      InterpolationValue&& end,
      LengthMatchingStrategy,
      MergeSingleItemConversionsCallback);

  using EqualNonInterpolableValuesCallback =
      bool (*)(const NonInterpolableValue*, const NonInterpolableValue*);
  static bool EqualValues(const InterpolationValue&,
                          const InterpolationValue&,
                          EqualNonInterpolableValuesCallback);

  using InterpolableValuesAreCompatibleCallback =
      base::FunctionRef<bool(const InterpolableValue*,
                             const InterpolableValue*)>;
  using NonInterpolableValuesAreCompatibleCallback =
      base::FunctionRef<bool(const NonInterpolableValue*,
                             const NonInterpolableValue*)>;
  using CompositeItemCallback =
      base::FunctionRef<void(UnderlyingValue&,
                             double underlying_fraction,
                             const InterpolableValue&,
                             const NonInterpolableValue*)>;
  static void Composite(UnderlyingValueOwner&,
                        double underlying_fraction,
                        const InterpolationType&,
                        const InterpolationValue&,
                        LengthMatchingStrategy,
                        InterpolableValuesAreCompatibleCallback,
                        NonInterpolableValuesAreCompatibleCallback,
                        CompositeItemCallback);

  // Used when the interpolable values are known to always be compatible.
  static bool InterpolableValuesKnownCompatible(const InterpolableValue* a,
                                                const InterpolableValue* b) {
    return true;
  }

  // We are moving towards elimination of |NonInterpolableValue|, and expect
  // more clients to assert no more usage with this function.
  static bool VerifyNoNonInterpolableValues(const NonInterpolableValue* a,
                                            const NonInterpolableValue* b);
};

class CORE_EXPORT NonInterpolableList final : public NonInterpolableValue {
 public:
  NonInterpolableList() = default;
  explicit NonInterpolableList(
      HeapVector<Member<const NonInterpolableValue>>&& list)
      : list_(list) {}
  ~NonInterpolableList() final = default;

  void Trace(Visitor* visitor) const override {
    NonInterpolableValue::Trace(visitor);
    visitor->Trace(list_);
  }

  wtf_size_t length() const { return list_.size(); }
  const NonInterpolableValue* Get(wtf_size_t index) const {
    return list_[index].Get();
  }

  // This class can update the NonInterpolableList of an UnderlyingValue with
  // a series of mutations. The actual update of the list is delayed until the
  // AutoBuilder object goes out of scope, to avoid creating a new list for
  // every call to Set().
  class CORE_EXPORT AutoBuilder {
    STACK_ALLOCATED();

   public:
    // The UnderlyingValue provided here is assumed to contain a
    // non-nullptr NonInterpolableList.
    AutoBuilder(UnderlyingValue&);
    ~AutoBuilder();

    void Set(wtf_size_t index, const NonInterpolableValue*);

   private:
    UnderlyingValue& underlying_value_;
    HeapVector<Member<const NonInterpolableValue>> list_;
  };

  DECLARE_NON_INTERPOLABLE_VALUE_TYPE();

 private:
  HeapVector<Member<const NonInterpolableValue>> list_;
};

template <>
struct DowncastTraits<NonInterpolableList> {
  static bool AllowFrom(const NonInterpolableValue* value) {
    return value && AllowFrom(*value);
  }
  static bool AllowFrom(const NonInterpolableValue& value) {
    return value.GetType() == NonInterpolableList::static_type_;
  }
};

template <typename CreateItemCallback>
InterpolationValue ListInterpolationFunctions::CreateList(
    wtf_size_t length,
    CreateItemCallback create_item) {
  if (length == 0)
    return CreateEmptyList();
  auto* interpolable_list = MakeGarbageCollected<InterpolableList>(length);
  HeapVector<Member<const NonInterpolableValue>> non_interpolable_values(
      length);
  for (wtf_size_t i = 0; i < length; i++) {
    InterpolationValue item = create_item(i);
    if (!item)
      return nullptr;
    interpolable_list->Set(i, std::move(item.interpolable_value));
    non_interpolable_values[i] = std::move(item.non_interpolable_value);
  }
  return InterpolationValue(interpolable_list,
                            MakeGarbageCollected<NonInterpolableList>(
                                std::move(non_interpolable_values)));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_LIST_INTERPOLATION_FUNCTIONS_H_
