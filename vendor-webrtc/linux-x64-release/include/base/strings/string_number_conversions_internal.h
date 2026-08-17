// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STRINGS_STRING_NUMBER_CONVERSIONS_INTERNAL_H_
#define BASE_STRINGS_STRING_NUMBER_CONVERSIONS_INTERNAL_H_

#include <errno.h>
#include <stdlib.h>

#include <limits>
#include <optional>
#include <string_view>

#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/logging.h"
#include "base/numerics/safe_math.h"
#include "base/strings/string_util.h"
#include "base/third_party/double_conversion/double-conversion/double-conversion.h"

namespace base {

namespace internal {

template <typename STR, typename INT>
static STR IntToStringT(INT value) {
  // log10(2) ~= 0.3 bytes needed per bit or per byte log10(2**8) ~= 2.4.
  // So round up to allocate 3 output characters per byte, plus 1 for '-'.
  const size_t kOutputBufSize =
      3 * sizeof(INT) + std::numeric_limits<INT>::is_signed;

  // Create the string in a temporary buffer, write it back to front, and
  // then return the substr of what we ended up using.
  using CHR = typename STR::value_type;
  CHR outbuf[kOutputBufSize];

  // The ValueOrDie call below can never fail, because UnsignedAbs is valid
  // for all valid inputs.
  std::make_unsigned_t<INT> res =
      CheckedNumeric<INT>(value).UnsignedAbs().ValueOrDie();

  CHR* end = UNSAFE_TODO(outbuf + kOutputBufSize);
  CHR* i = end;
  do {
    UNSAFE_TODO(--i);
    DCHECK(i != outbuf);
    *i = static_cast<CHR>((res % 10) + '0');
    res /= 10;
  } while (res != 0);
  if (IsValueNegative(value)) {
    UNSAFE_TODO(--i);
    DCHECK(i != outbuf);
    *i = static_cast<CHR>('-');
  }
  return STR(i, end);
}

// Utility to convert a character to a digit in a given base
template <int BASE, typename CHAR>
std::optional<uint8_t> CharToDigit(CHAR c) {
  static_assert(1 <= BASE && BASE <= 36, "BASE needs to be in [1, 36]");
  if (c >= '0' && c < '0' + std::min(BASE, 10)) {
    return static_cast<uint8_t>(c - '0');
  }

  if (c >= 'a' && c < 'a' + BASE - 10) {
    return static_cast<uint8_t>(c - 'a' + 10);
  }

  if (c >= 'A' && c < 'A' + BASE - 10) {
    return static_cast<uint8_t>(c - 'A' + 10);
  }

  return std::nullopt;
}

template <typename Number, int kBase>
class StringToNumberParser {
 public:
  struct Result {
    Number value = 0;
    bool valid = false;
  };

  static constexpr Number kMin = std::numeric_limits<Number>::min();
  static constexpr Number kMax = std::numeric_limits<Number>::max();

  // Sign provides:
  //  - a static function, CheckBounds, that determines whether the next digit
  //    causes an overflow/underflow
  //  - a static function, Increment, that appends the next digit appropriately
  //    according to the sign of the number being parsed.
  template <typename Sign>
  class Base {
   public:
    template <typename Iter>
    static Result Invoke(Iter begin, Iter end) {
      Number value = 0;

      if (begin == end) {
        return {value, false};
      }

      // Note: no performance difference was found when using template
      // specialization to remove this check in bases other than 16
      if (kBase == 16 && end - begin > 2 && *begin == '0' &&
          (*(begin + 1) == 'x' || *(begin + 1) == 'X')) {
        begin += 2;
      }

      for (Iter current = begin; current != end; ++current) {
        std::optional<uint8_t> new_digit = CharToDigit<kBase>(*current);

        if (!new_digit) {
          return {value, false};
        }

        if (current != begin) {
          Result result = Sign::CheckBounds(value, *new_digit);
          if (!result.valid) {
            return result;
          }

          value *= kBase;
        }

        value = Sign::Increment(value, *new_digit);
      }
      return {value, true};
    }
  };

  class Positive : public Base<Positive> {
   public:
    static Result CheckBounds(Number value, uint8_t new_digit) {
      if (value > static_cast<Number>(kMax / kBase) ||
          (value == static_cast<Number>(kMax / kBase) &&
           new_digit > kMax % kBase)) {
        return {kMax, false};
      }
      return {value, true};
    }
    static Number Increment(Number lhs, uint8_t rhs) { return lhs + rhs; }
  };

  class Negative : public Base<Negative> {
   public:
    static Result CheckBounds(Number value, uint8_t new_digit) {
      if (value < kMin / kBase ||
          (value == kMin / kBase && new_digit > 0 - kMin % kBase)) {
        return {kMin, false};
      }
      return {value, true};
    }
    static Number Increment(Number lhs, uint8_t rhs) { return lhs - rhs; }
  };
};

template <typename Number, int kBase, typename CharT>
auto StringToNumber(std::basic_string_view<CharT> input) {
  using Parser = StringToNumberParser<Number, kBase>;
  using Result = typename Parser::Result;

  bool has_leading_whitespace = false;
  auto begin = input.begin();
  auto end = input.end();

  while (begin != end && IsAsciiWhitespace(*begin)) {
    has_leading_whitespace = true;
    ++begin;
  }

  if (begin != end && *begin == '-') {
    if (!std::numeric_limits<Number>::is_signed) {
      return Result{0, false};
    }

    Result result = Parser::Negative::Invoke(begin + 1, end);
    result.valid &= !has_leading_whitespace;
    return result;
  }

  if (begin != end && *begin == '+') {
    ++begin;
  }

  Result result = Parser::Positive::Invoke(begin, end);
  result.valid &= !has_leading_whitespace;
  return result;
}

template <typename T, typename VALUE, typename CharT = typename T::value_type>
bool StringToIntImpl(T input, VALUE& output) {
  auto result = StringToNumber<VALUE, 10, CharT>(input);
  output = result.value;
  return result.valid;
}

template <typename T, typename VALUE, typename CharT = typename T::value_type>
bool HexStringToIntImpl(T input, VALUE& output) {
  auto result = StringToNumber<VALUE, 16, CharT>(input);
  output = result.value;
  return result.valid;
}

static const double_conversion::DoubleToStringConverter*
GetDoubleToStringConverter() {
  static double_conversion::DoubleToStringConverter converter(
      double_conversion::DoubleToStringConverter::EMIT_POSITIVE_EXPONENT_SIGN,
      nullptr, nullptr, 'e', -6, 12, 0, 0);
  return &converter;
}

// Converts a given (data, size) pair to a desired string type. For
// performance reasons, this dispatches to a different constructor if the
// passed-in data matches the string's value_type.
template <typename StringT>
StringT ToString(const typename StringT::value_type* data, size_t size) {
  return StringT(data, size);
}

// TODO(tsepez): should be UNSAFE_BUFFER_USAGE.
template <typename StringT, typename CharT>
StringT ToString(const CharT* data, size_t size) {
  // SAFETY: required from caller.
  return StringT(data, UNSAFE_BUFFERS(data + size));
}

template <typename StringT>
StringT DoubleToStringT(double value) {
  char buffer[32];
  double_conversion::StringBuilder builder(buffer, sizeof(buffer));
  GetDoubleToStringConverter()->ToShortest(value, &builder);
  return ToString<StringT>(buffer, static_cast<size_t>(builder.position()));
}

template <typename STRING, typename CHAR>
bool StringToDoubleImpl(STRING input, const CHAR* data, double& output) {
  static double_conversion::StringToDoubleConverter converter(
      double_conversion::StringToDoubleConverter::ALLOW_LEADING_SPACES |
          double_conversion::StringToDoubleConverter::ALLOW_TRAILING_JUNK,
      0.0, 0, nullptr, nullptr);

  int processed_characters_count;
  output = converter.StringToDouble(data, checked_cast<int>(input.size()),
                                    &processed_characters_count);

  // Cases to return false:
  //  - If the input string is empty, there was nothing to parse.
  //  - If the value saturated to HUGE_VAL.
  //  - If the entire string was not processed, there are either characters
  //    remaining in the string after a parsed number, or the string does not
  //    begin with a parseable number.
  //  - If the first character is a space, there was leading whitespace. Note
  //    that this checks using IsWhitespace(), which behaves differently for
  //    wide and narrow characters -- that is intentional and matches the
  //    behavior of the double_conversion library's whitespace-skipping
  //    algorithm.
  return !input.empty() && output != HUGE_VAL && output != -HUGE_VAL &&
         static_cast<size_t>(processed_characters_count) == input.size() &&
         !IsWhitespace(input[0]);
}

template <typename Char, typename OutIter>
static bool HexStringToByteContainer(std::string_view input, OutIter output) {
  size_t count = input.size();
  if (count == 0 || (count % 2) != 0) {
    return false;
  }
  for (uintptr_t i = 0; i < count / 2; ++i) {
    // most significant 4 bits
    std::optional<uint8_t> msb = CharToDigit<16>(input[i * 2]);
    // least significant 4 bits
    std::optional<uint8_t> lsb = CharToDigit<16>(input[i * 2 + 1]);
    if (!msb || !lsb) {
      return false;
    }
    *(output++) = static_cast<Char>((*msb << 4) | *lsb);
  }
  return true;
}

}  // namespace internal

}  // namespace base

#endif  // BASE_STRINGS_STRING_NUMBER_CONVERSIONS_INTERNAL_H_
