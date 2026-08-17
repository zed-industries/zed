// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GAP_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GAP_DATA_H_

#include "third_party/blink/renderer/core/css/style_color.h"
#include "third_party/blink/renderer/core/style/computed_style_base_constants.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

template <typename T>
class CORE_EXPORT ValueRepeater : public GarbageCollected<ValueRepeater<T>> {
 public:
  using VectorType = typename std::conditional<WTF::IsTraceable<T>::value,
                                               HeapVector<T, 1>,
                                               Vector<T>>::type;
  ValueRepeater() = default;

  explicit ValueRepeater(VectorType repeated_values,
                         std::optional<wtf_size_t> repeat_count)
      : repeated_values_(repeated_values), repeat_count_(repeat_count) {
    CHECK(repeated_values_.size() > 0);
  }

  bool operator==(const ValueRepeater& other) const {
    return repeated_values_ == other.repeated_values_ &&
           repeat_count_ == other.repeat_count_;
  }

  bool IsAutoRepeater() const { return !repeat_count_.has_value(); }
  const VectorType& RepeatedValues() const { return repeated_values_; }
  wtf_size_t RepeatCount() const {
    CHECK(repeat_count_.has_value());
    return repeat_count_.value();
  }

  void Trace(Visitor* visitor) const {
    TraceIfNeeded<VectorType>::Trace(visitor, repeated_values_);
  }

 private:
  VectorType repeated_values_;
  std::optional<wtf_size_t> repeat_count_ = std::nullopt;
};

// A GapData is a single value or a ValueRepeater.
template <typename T>
class CORE_EXPORT GapData {
  DISALLOW_NEW();

 public:
  GapData() = default;
  explicit GapData(T value) : value_(value) {}
  explicit GapData(ValueRepeater<T>* value_repeater)
      : value_repeater_(value_repeater) {}
  void Trace(Visitor* visitor) const {
    TraceIfNeeded<T>::Trace(visitor, value_);
    visitor->Trace(value_repeater_);
  }

  bool operator==(const GapData& other) const {
    return value_ == other.value_ &&
           base::ValuesEquivalent(value_repeater_, other.value_repeater_);
  }

  const T GetValue() const {
    CHECK(!value_repeater_);
    return value_;
  }

  const ValueRepeater<T>* GetValueRepeater() const {
    CHECK(value_repeater_);
    return value_repeater_.Get();
  }

  bool IsRepeaterData() const { return value_repeater_ != nullptr; }

 private:
  T value_ = T();
  Member<ValueRepeater<T>> value_repeater_;
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::GapData<blink::StyleColor>)
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::GapData<int>)
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::GapData<blink::EBorderStyle>)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GAP_DATA_H_
