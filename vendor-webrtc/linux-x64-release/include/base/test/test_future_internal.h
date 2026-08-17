// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_FUTURE_INTERNAL_H_
#define BASE_TEST_TEST_FUTURE_INTERNAL_H_

#include <tuple>
#include <type_traits>

namespace base::test::internal {

// Helper to only implement a method if the future holds one or more values.
template <typename Tuple>
concept IsNonEmptyTuple = std::tuple_size<Tuple>::value > 0;

// Helper to only implement a method if the future holds a single value.
template <typename Tuple>
concept IsSingleValuedTuple = std::tuple_size<Tuple>::value == 1;

// Helper to only implement a method if the future holds multiple values.
template <typename Tuple>
concept IsMultiValuedTuple = std::tuple_size<Tuple>::value > 1;

}  // namespace base::test::internal

#endif  // BASE_TEST_TEST_FUTURE_INTERNAL_H_
