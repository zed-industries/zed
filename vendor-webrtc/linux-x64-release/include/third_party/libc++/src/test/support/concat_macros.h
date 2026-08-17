//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_CONCAT_MACROS_H
#define TEST_SUPPORT_CONCAT_MACROS_H

#include <cstdio>
#include <string>
#include <source_location>

#include "assert_macros.h"
#include "test_macros.h"

#ifndef TEST_HAS_NO_LOCALIZATION
#  include <concepts>
#  include <iterator>
#  include <sstream>
#endif

#if TEST_STD_VER > 17

#  ifndef TEST_HAS_NO_LOCALIZATION

[[nodiscard]] constexpr bool test_is_high_surrogate(char32_t value) { return value >= 0xd800 && value <= 0xdbff; }

[[nodiscard]] constexpr bool test_is_low_surrogate(char32_t value) { return value >= 0xdc00 && value <= 0xdfff; }

[[nodiscard]] constexpr bool test_is_surrogate(char32_t value) { return value >= 0xd800 && value <= 0xdfff; }

[[nodiscard]] constexpr bool test_is_code_point(char32_t value) { return value <= 0x10ffff; }

[[nodiscard]] constexpr bool test_is_scalar_value(char32_t value) {
  return test_is_code_point(value) && !test_is_surrogate(value);
}

inline constexpr char32_t test_replacement_character = U'\ufffd';

template <class InIt, class OutIt>
OutIt test_transcode() = delete;

template <class InIt, class OutIt>
  requires(std::output_iterator<OutIt, const char&> && std::same_as<std::iter_value_t<InIt>, char8_t>)
OutIt test_transcode(InIt first, InIt last, OutIt out_it) {
  return std::copy(first, last, out_it);
}

template <class OutIt>
  requires std::output_iterator<OutIt, const char&>
void test_encode(OutIt& out_it, char16_t value) {
  if (value < 0x80)
    *out_it++ = static_cast<char>(value);
  else if (value < 0x800) {
    *out_it++ = static_cast<char>(0b11000000 | (value >> 6));
    *out_it++ = static_cast<char>(0b10000000 | (value & 0b00111111));
  } else {
    *out_it++ = static_cast<char>(0b11100000 | (value >> 12));
    *out_it++ = static_cast<char>(0b10000000 | ((value) >> 6 & 0b00111111));
    *out_it++ = static_cast<char>(0b10000000 | (value & 0b00111111));
  }
}

template <class OutIt>
  requires std::output_iterator<OutIt, const char&>
void test_encode(OutIt& out_it, char32_t value) {
  if ((value & 0xffff0000) == 0)
    test_encode(out_it, static_cast<char16_t>(value));
  else {
    *out_it++ = static_cast<char>(0b11100000 | (value >> 18));
    *out_it++ = static_cast<char>(0b10000000 | ((value) >> 12 & 0b00111111));
    *out_it++ = static_cast<char>(0b10000000 | ((value) >> 6 & 0b00111111));
    *out_it++ = static_cast<char>(0b10000000 | (value & 0b00111111));
  }
}

template <class InIt, class OutIt>
  requires(std::output_iterator<OutIt, const char&> &&
           (std::same_as<std::iter_value_t<InIt>, char16_t>
#    ifndef TEST_HAS_NO_WIDE_CHARACTERS
            || (std::same_as<std::iter_value_t<InIt>, wchar_t> && sizeof(wchar_t) == 2)
#    endif
                ))
OutIt test_transcode(InIt first, InIt last, OutIt out_it) {
  while (first != last) {
    char32_t value = *first++;

    if (test_is_low_surrogate(value)) [[unlikely]] {
      test_encode(out_it, static_cast<char16_t>(test_replacement_character));
      continue;
    }

    if (!test_is_high_surrogate(value)) {
      test_encode(out_it, static_cast<char16_t>(value));
      continue;
    }

    if (first == last || !test_is_low_surrogate(static_cast<char32_t>(*first))) [[unlikely]] {
      test_encode(out_it, static_cast<char16_t>(test_replacement_character));
      continue;
    }

    value -= 0xd800;
    value <<= 10;
    value += static_cast<char32_t>(*first++) - 0xdc00;
    value += 0x10000;

    if (test_is_code_point(value)) [[likely]]
      test_encode(out_it, value);
    else
      test_encode(out_it, static_cast<char16_t>(test_replacement_character));
  }

  return out_it;
}

template <class InIt, class OutIt>
  requires(std::output_iterator<OutIt, const char&> &&
           (std::same_as<std::iter_value_t<InIt>, char32_t>
#    ifndef TEST_HAS_NO_WIDE_CHARACTERS
            || (std::same_as<std::iter_value_t<InIt>, wchar_t> && sizeof(wchar_t) == 4)
#    endif
                ))
OutIt test_transcode(InIt first, InIt last, OutIt out_it) {
  while (first != last) {
    char32_t value = *first++;
    if (test_is_code_point(value)) [[likely]]
      test_encode(out_it, value);
    else
      test_encode(out_it, static_cast<char16_t>(test_replacement_character));
  }
  return out_it;
}

std::ostream& operator<<(std::ostream& os, const std::source_location& loc) {
  return os << loc.file_name() << ':' << loc.line() << ':' << loc.column();
}

template <class T>
concept test_streamable = requires(std::stringstream& stream, T&& value) { stream << value; };

template <class R>
concept test_convertable_range = (!test_streamable<R> && requires(R&& value) {
  std::basic_string_view{std::begin(value), std::end(value)};
});

template <class T>
concept test_can_concat = test_streamable<T> || test_convertable_range<T>;

template <test_streamable T>
std::ostream& test_concat(std::ostream& stream, T&& value) {
  return stream << value;
}

template <test_convertable_range T>
std::ostream& test_concat(std::ostream& stream, T&& value) {
  auto b = std::begin(value);
  auto e = std::end(value);
  if (b != e) {
    // When T is an array it's string-literal, remove the NUL terminator.
    if constexpr (std::is_array_v<std::remove_cvref_t<T>>) {
      --e;
    }
    test_transcode(b, e, std::ostream_iterator<char>{stream});
  }
  return stream;
}
#  endif // TEST_HAS_NO_LOCALIZATION

// If possible concatenates message for the assertion function, else returns a
// default message. Not being able to stream is not considered an error. For
// example, streaming to std::wcerr doesn't work properly in the CI. Therefore
// the formatting tests should only stream to std::string.
//
// The macro TEST_WRITE_CONCATENATED can be used to evaluate the arguments
// lazily. This useful when using this function in combination with
// assert_macros.h.
template <class... Args>
std::string test_concat_message([[maybe_unused]] Args&&... args) {
#  ifndef TEST_HAS_NO_LOCALIZATION
  if constexpr ((test_can_concat<Args> && ...)) {
    std::stringstream sstr;
    ((test_concat(sstr, std::forward<Args>(args))), ...);
    return sstr.str();
  } else
#  endif // TEST_HAS_NO_LOCALIZATION
    return "Message discarded since it can't be streamed to std::cerr.\n";
}

// Writes its arguments to stderr, using the test_concat_message helper.
#  define TEST_WRITE_CONCATENATED(...) [&] { ::test_eprintf("%s", ::test_concat_message(__VA_ARGS__).c_str()); }

#else

// Fallback definition before C++20 that allows using the macro but doesn't provide a very good message.
#  define TEST_WRITE_CONCATENATED(...) [&] { ::test_eprintf("%s", TEST_STRINGIZE(__VA_ARGS__)); }

#endif // TEST_STD_VER > 17

#endif //  TEST_SUPPORT_CONCAT_MACROS_H
