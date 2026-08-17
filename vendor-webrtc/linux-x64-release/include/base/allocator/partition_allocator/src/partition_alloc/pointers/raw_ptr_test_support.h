// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_POINTERS_RAW_PTR_TEST_SUPPORT_H_
#define PARTITION_ALLOC_POINTERS_RAW_PTR_TEST_SUPPORT_H_

#include <optional>

#include "testing/gmock/include/gmock/gmock.h"

// Struct intended to be used with designated initializers and passed
// to the `CountersMatch()` matcher.
//
// TODO(tsepez): Although we only want one kind of these, the class is still
// a template to circumvent the chromium-style out-of-line constructor rule.
// Adding such a constructor would make this no longer be an aggregate and
// that would prohibit designated initiaizers.
template <int IGNORE>
struct CountingRawPtrExpectationTemplate {
  std::optional<int> wrap_raw_ptr_cnt;
  std::optional<int> release_wrapped_ptr_cnt;
  std::optional<int> get_for_dereference_cnt;
  std::optional<int> get_for_extraction_cnt;
  std::optional<int> get_for_comparison_cnt;
  std::optional<int> wrapped_ptr_swap_cnt;
  std::optional<int> wrapped_ptr_less_cnt;
  std::optional<int> pointer_to_member_operator_cnt;
  std::optional<int> wrap_raw_ptr_for_dup_cnt;
  std::optional<int> get_for_duplication_cnt;
};
using CountingRawPtrExpectations = CountingRawPtrExpectationTemplate<0>;

#define REPORT_UNEQUAL_RAW_PTR_COUNTER(member_name)                          \
  {                                                                          \
    if (arg.member_name.has_value() &&                                       \
        arg.member_name.value() !=                                           \
            base::test::RawPtrCountingImplForTest::member_name) {            \
      *result_listener << "Expected `" #member_name "` to be "               \
                       << arg.member_name.value() << " but got "             \
                       << base::test::RawPtrCountingImplForTest::member_name \
                       << "; ";                                              \
      result = false;                                                        \
    }                                                                        \
  }

#define REPORT_UNEQUAL_RAW_PTR_COUNTERS(result)                    \
  {                                                                \
    result = true;                                                 \
    REPORT_UNEQUAL_RAW_PTR_COUNTER(wrap_raw_ptr_cnt)               \
    REPORT_UNEQUAL_RAW_PTR_COUNTER(release_wrapped_ptr_cnt)        \
    REPORT_UNEQUAL_RAW_PTR_COUNTER(get_for_dereference_cnt)        \
    REPORT_UNEQUAL_RAW_PTR_COUNTER(get_for_extraction_cnt)         \
    REPORT_UNEQUAL_RAW_PTR_COUNTER(get_for_comparison_cnt)         \
    REPORT_UNEQUAL_RAW_PTR_COUNTER(wrapped_ptr_swap_cnt)           \
    REPORT_UNEQUAL_RAW_PTR_COUNTER(wrapped_ptr_less_cnt)           \
    REPORT_UNEQUAL_RAW_PTR_COUNTER(pointer_to_member_operator_cnt) \
    REPORT_UNEQUAL_RAW_PTR_COUNTER(wrap_raw_ptr_for_dup_cnt)       \
    REPORT_UNEQUAL_RAW_PTR_COUNTER(get_for_duplication_cnt)        \
  }

// Matcher used with `CountingRawPtr`. Provides slightly shorter
// boilerplate for verifying counts. This inner function is detached
// from the `MATCHER`.
inline bool CountersMatchImpl(const CountingRawPtrExpectations& arg,
                              testing::MatchResultListener* result_listener) {
  bool result = true;
  REPORT_UNEQUAL_RAW_PTR_COUNTERS(result);
  return result;
}

// Implicit `arg` has type `CountingRawPtrExpectations`, specialized for
// the specific counting impl.
MATCHER(CountersMatch, "counting impl has specified counters") {
  return CountersMatchImpl(arg, result_listener);
}

#undef REPORT_UNEQUAL_RAW_PTR_COUNTERS
#undef REPORT_UNEQUAL_RAW_PTR_COUNTER

#endif  // PARTITION_ALLOC_POINTERS_RAW_PTR_TEST_SUPPORT_H_
