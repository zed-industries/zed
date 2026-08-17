//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_INPUTOUTPUT_STRINGSTREAMS_HELPER_TYPES_H
#define TEST_STD_INPUTOUTPUT_STRINGSTREAMS_HELPER_TYPES_H

#include <string_view>
#include <concepts>

#include "test_macros.h"

template <typename CharT, class Traits = std::char_traits<CharT>>
class ConstConvertibleStringView {
public:
  explicit ConstConvertibleStringView(const CharT* cs) : cs_{cs} {}

  operator std::basic_string_view<CharT, Traits>() = delete;
  operator std::basic_string_view<CharT, Traits>() const { return std::basic_string_view<CharT, Traits>(cs_); }

private:
  const CharT* cs_;
};

static_assert(!std::constructible_from<std::basic_string_view<char>, ConstConvertibleStringView<char>>);
static_assert(!std::convertible_to<ConstConvertibleStringView<char>, std::basic_string_view<char>>);

static_assert(std::constructible_from<std::basic_string_view<char>, const ConstConvertibleStringView<char>>);
static_assert(std::convertible_to<const ConstConvertibleStringView<char>, std::basic_string_view<char>>);

#ifndef TEST_HAS_NO_WIDE_CHARACTERS
static_assert(!std::constructible_from<std::basic_string_view<wchar_t>, ConstConvertibleStringView<wchar_t>>);
static_assert(!std::convertible_to<ConstConvertibleStringView<wchar_t>, std::basic_string_view<wchar_t>>);

static_assert(std::constructible_from<std::basic_string_view<wchar_t>, const ConstConvertibleStringView<wchar_t>>);
static_assert(std::convertible_to<const ConstConvertibleStringView<wchar_t>, std::basic_string_view<wchar_t>>);
#endif

template <typename CharT, class Traits = std::char_traits<CharT>>
class NonConstConvertibleStringView {
public:
  explicit NonConstConvertibleStringView(const CharT* cs) : cs_{cs} {}

  operator std::basic_string_view<CharT, Traits>() { return std::basic_string_view<CharT, Traits>(cs_); }
  operator std::basic_string_view<CharT, Traits>() const = delete;

private:
  const CharT* cs_;
};

static_assert(std::constructible_from<std::basic_string_view<char>, NonConstConvertibleStringView<char>>);
static_assert(std::convertible_to<NonConstConvertibleStringView<char>, std::basic_string_view<char>>);

static_assert(!std::constructible_from<std::basic_string_view<char>, const NonConstConvertibleStringView<char>>);
static_assert(!std::convertible_to<const NonConstConvertibleStringView<char>, std::basic_string_view<char>>);

#ifndef TEST_HAS_NO_WIDE_CHARACTERS
static_assert(std::constructible_from<std::basic_string_view<wchar_t>, NonConstConvertibleStringView<wchar_t>>);
static_assert(std::convertible_to<NonConstConvertibleStringView<wchar_t>, std::basic_string_view<wchar_t>>);

static_assert(!std::constructible_from<std::basic_string_view<wchar_t>, const NonConstConvertibleStringView<wchar_t>>);
static_assert(!std::convertible_to<const NonConstConvertibleStringView<wchar_t>, std::basic_string_view<wchar_t>>);
#endif

struct SomeObject {};

struct NonMode {};

struct NonAllocator {};

#endif // TEST_STD_INPUTOUTPUT_STRINGSTREAMS_HELPER_TYPES_H
