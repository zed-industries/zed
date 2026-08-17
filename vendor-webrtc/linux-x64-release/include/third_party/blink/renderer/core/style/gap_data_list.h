// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GAP_DATA_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GAP_DATA_LIST_H_

#include <algorithm>

#include "third_party/blink/renderer/core/style/gap_data.h"

namespace blink {

// These are used to store gap decorations values in the order they are
// specified. These values can be an auto repeater, an integer repeater, or a
// single value. The value could be a color, style or width. See:
// https://drafts.csswg.org/css-gaps-1/#color-style-width
template <typename T>
class CORE_EXPORT GapDataList {
  DISALLOW_NEW();

  using VectorType = ValueRepeater<T>::VectorType;

 public:
  using GapDataVector = HeapVector<GapData<T>, 1>;
  GapDataList() = default;

  static GapDataList DefaultGapColorDataList() {
    return GapDataList(StyleColor::CurrentColor());
  }

  static GapDataList DefaultGapWidthDataList() {
    constexpr int kDefaultWidth = 3;
    return GapDataList(kDefaultWidth);
  }

  static GapDataList DefaultGapStyleDataList() {
    return GapDataList(EBorderStyle::kNone);
  }

  explicit GapDataList(GapDataVector&& gap_data_list)
      : gap_data_list_(gap_data_list) {
    CHECK(!gap_data_list_.empty());
  }

  explicit GapDataList(const T& value) {
    gap_data_list_.emplace_back(GapData<T>(value));
  }

  void Trace(Visitor* visitor) const {
    visitor->Trace(gap_data_list_);
    TraceIfNeeded<VectorType>::Trace(visitor, expanded_values_);
  }

  const GapDataVector& GetGapDataList() const { return gap_data_list_; }

  bool MaybeDependsOnCurrentColor() const {
    return std::ranges::any_of(gap_data_list_, [](const GapData<T>& gap_data) {
      return gap_data.GetValue().IsCurrentColor() ||
             gap_data.GetValue().IsUnresolvedColorFunction();
    });
  }

  bool HasSingleValue() const {
    return gap_data_list_.size() == 1 && !gap_data_list_[0].IsRepeaterData();
  }

  const T GetLegacyValue() const {
    return gap_data_list_[0].GetValue();
  }

  // TODO(samomekarajr): Potential optimization. We might not need to expand
  // values. Instead, we can adopt a method similar to the grid's implementation
  // by using sets. This way, when given a particular gap index, we can
  // translate it to a specific item in the set. This approach allows us to
  // avoid expanding values and storing them in a vector, especially when an
  // integer repeater has a very large count. For this to be worthwhie, we need
  // to be able to get a given decoration value for a gap index in less than
  // O(n) time.

  // Expands `gap_data_list_` into `expanded_values_` by evaluating the
  // repeaters and storing the values in the order they are specified.
  // When an auto repeater is present, it stores the range of the auto
  // repeated values as [`auto_repeat_start_`, `auto_repeat_end_`).
  void ExpandValues() {
    expanded_values_.clear();
    auto_repeat_start_ = kNotFound;
    auto_repeat_end_ = kNotFound;

    for (const auto& gap_data : gap_data_list_) {
      if (!gap_data.IsRepeaterData()) {
        // Simple single value, add to `expanded_values_`.
        expanded_values_.push_back(gap_data.GetValue());
      } else {
        const ValueRepeater<T>* repeater = gap_data.GetValueRepeater();

        if (repeater->IsAutoRepeater()) {
          // Only one auto repeater is allowed, so store the range of the auto
          // repeated values.
          CHECK_EQ(auto_repeat_start_, kNotFound);
          CHECK_EQ(auto_repeat_end_, kNotFound);
          auto_repeat_start_ = expanded_values_.size();
          for (const auto& value : repeater->RepeatedValues()) {
            expanded_values_.push_back(value);
          }
          auto_repeat_end_ = expanded_values_.size();
        } else {
          // Integer repeater, add values `count` times.
          wtf_size_t count = repeater->RepeatCount();

          for (size_t i = 0; i < count; ++i) {
            for (const auto& value : repeater->RepeatedValues()) {
              expanded_values_.push_back(value);
            }
          }
        }
      }
    }
  }

  // Returns the gap decoration value for a given gap index. It uses the
  // `auto_repeat_start_` and `auto_repeat_end_` to partition the
  // `expanded_values_` into leading, auto repeat, and trailing sections. The
  // value at the given index is then determined based on the section it falls
  // into.
  T GetGapDecorationForGapIndex(wtf_size_t gap_index,
                                wtf_size_t total_gaps) const {
    if (auto_repeat_start_ == kNotFound) {
      // No auto repeaters are present, so we need to return the value at the
      // valid index. The size of `expanded_values_` might be less than the
      // `total_gaps`, so we apply values cyclically.
      return expanded_values_[gap_index % expanded_values_.size()];
    }

    // Leading section ends at the start of the auto repeat section.
    const wtf_size_t leading_section_end = auto_repeat_start_;

    // Trailing section starts after the auto repeat section so it'll be the
    // number of gaps minus the number of trailing values.
    const wtf_size_t trailing_section_start =
        total_gaps - (expanded_values_.size() - auto_repeat_end_);

    if (gap_index < leading_section_end) {
      // Leading values can be indexed directly.
      return expanded_values_[gap_index];
    } else if (gap_index >= trailing_section_start) {
      // Get the index of the gap relative to the trailing section and return
      // expanded value at that index. Note: `trailing_index` indicates the
      // index of the gap relative to trailing values, hence we need to add
      // `auto_repeat_end_` to get the actual index in the expanded values.
      wtf_size_t trailing_index = gap_index - trailing_section_start;
      return expanded_values_[auto_repeat_end_ + trailing_index];
    } else {
      // For auto repeat values, we get the index of the gap relative to the
      // auto section and return the expanded value at that index.
      wtf_size_t auto_section_length = auto_repeat_end_ - auto_repeat_start_;
      wtf_size_t gap_index_in_auto_section =
          (gap_index - leading_section_end) % auto_section_length;
      return expanded_values_[auto_repeat_start_ + gap_index_in_auto_section];
    }
  }

  bool operator==(const GapDataList& o) const {
    return gap_data_list_ == o.gap_data_list_;
  }

  bool operator!=(const GapDataList& o) const { return !(*this == o); }

 private:
  GapDataVector gap_data_list_;

  // Holds the expanded values of `gap_data_list_` in the order they are
  // specified. This is used to get the gap decoration value for a given gap
  // index. If the `gap_data_list_` contains an auto repeater, the repeated
  // values are stored in the order they are specified, with
  // `auto_repeat_start_` and `auto_repeat_end_` indicating the range of the
  // auto repeated values.
  VectorType expanded_values_;
  wtf_size_t auto_repeat_start_ = kNotFound;
  wtf_size_t auto_repeat_end_ = kNotFound;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GAP_DATA_LIST_H_
