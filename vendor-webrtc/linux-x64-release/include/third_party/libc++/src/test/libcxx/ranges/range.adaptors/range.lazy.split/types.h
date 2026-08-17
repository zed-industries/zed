//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_LIBCXX_RANGES_RANGE_ADAPTORS_RANGE_LAZY_SPLIT_TYPES_H
#define TEST_LIBCXX_RANGES_RANGE_ADAPTORS_RANGE_LAZY_SPLIT_TYPES_H

#include <concepts>
#include <cstddef>
#include <ranges>
#include <string_view>
#include "test_iterators.h"

// ForwardView

struct ForwardView : std::ranges::view_base {
  constexpr explicit ForwardView() = default;
  constexpr ForwardView(ForwardView&&) = default;
  constexpr ForwardView& operator=(ForwardView&&) = default;
  constexpr forward_iterator<const char*> begin() const { return forward_iterator<const char*>(nullptr); }
  constexpr forward_iterator<const char*> end() const { return forward_iterator<const char*>(nullptr); }
};
static_assert( std::ranges::forward_range<ForwardView>);
static_assert( std::ranges::forward_range<const ForwardView>);
static_assert( std::ranges::view<ForwardView>);
static_assert(!std::is_copy_constructible_v<ForwardView>);

// InputView

struct InputView : std::ranges::view_base {
  constexpr InputView() = default;

  constexpr cpp20_input_iterator<char*> begin() { return cpp20_input_iterator<char*>(nullptr); }
  constexpr sentinel_wrapper<cpp20_input_iterator<char*>> end() {
    return sentinel_wrapper(cpp20_input_iterator<char*>(nullptr));
  }
  constexpr cpp20_input_iterator<const char*> begin() const { return cpp20_input_iterator<const char*>(nullptr); }
  constexpr sentinel_wrapper<cpp20_input_iterator<const char*>> end() const {
    return sentinel_wrapper(cpp20_input_iterator<const char*>(nullptr));
  }
};

static_assert(std::ranges::input_range<InputView>);
static_assert(std::ranges::input_range<const InputView>);
static_assert(std::ranges::view<InputView>);

// ForwardTinyView

struct ForwardTinyView : std::ranges::view_base {
  constexpr ForwardTinyView() = default;
  constexpr forward_iterator<const char*> begin() const { return forward_iterator<const char*>(nullptr); }
  constexpr forward_iterator<const char*> end() const { return forward_iterator<const char*>(nullptr); }
  constexpr static std::size_t size() { return 1; }
};
static_assert(std::ranges::forward_range<ForwardTinyView>);
static_assert(std::ranges::view<ForwardTinyView>);
LIBCPP_STATIC_ASSERT(std::ranges::__tiny_range<ForwardTinyView>);

// Aliases

using SplitViewForward = std::ranges::lazy_split_view<ForwardView, ForwardView>;
using OuterIterForward = std::ranges::iterator_t<SplitViewForward>;
using InnerIterForward = std::ranges::iterator_t<OuterIterForward::value_type>;

using SplitViewInput = std::ranges::lazy_split_view<InputView, ForwardTinyView>;
using OuterIterInput = std::ranges::iterator_t<SplitViewInput>;
using InnerIterInput = std::ranges::iterator_t<OuterIterInput::value_type>;

#endif // TEST_LIBCXX_RANGES_RANGE_ADAPTORS_RANGE_LAZY_SPLIT_TYPES_H
