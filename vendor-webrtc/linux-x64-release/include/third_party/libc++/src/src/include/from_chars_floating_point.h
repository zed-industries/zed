//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP_SRC_INCLUDE_FROM_CHARS_FLOATING_POINT_H
#define _LIBCPP_SRC_INCLUDE_FROM_CHARS_FLOATING_POINT_H

// These headers are in the shared LLVM-libc header library.
#include "shared/fp_bits.h"
#include "shared/str_to_float.h"
#include "shared/str_to_integer.h"

#include <__assert>
#include <__config>
#include <cctype>
#include <charconv>
#include <concepts>
#include <limits>

// Included for the _Floating_type_traits class
#include "to_chars_floating_point.h"

_LIBCPP_BEGIN_NAMESPACE_STD

// Parses an infinity string.
// Valid strings are case insensitive and contain INF or INFINITY.
//
// - __first is the first argument to std::from_chars. When the string is invalid
//   this value is returned as ptr in the result.
// - __last is the last argument of std::from_chars.
// - __value is the value argument of std::from_chars,
// - __ptr is the current position is the input string. This is points beyond
//   the initial I character.
// - __negative whether a valid string represents -inf or +inf.
template <floating_point _Fp>
__from_chars_result<_Fp>
__from_chars_floating_point_inf(const char* const __first, const char* __last, const char* __ptr, bool __negative) {
  if (__last - __ptr < 2) [[unlikely]]
    return {_Fp{0}, 0, errc::invalid_argument};

  if (std::tolower(__ptr[0]) != 'n' || std::tolower(__ptr[1]) != 'f') [[unlikely]]
    return {_Fp{0}, 0, errc::invalid_argument};

  __ptr += 2;

  // At this point the result is valid and contains INF.
  // When the remaining part contains INITY this will be consumed. Otherwise
  // only INF is consumed. For example INFINITZ will consume INF and ignore
  // INITZ.

  if (__last - __ptr >= 5              //
      && std::tolower(__ptr[0]) == 'i' //
      && std::tolower(__ptr[1]) == 'n' //
      && std::tolower(__ptr[2]) == 'i' //
      && std::tolower(__ptr[3]) == 't' //
      && std::tolower(__ptr[4]) == 'y')
    __ptr += 5;

  if constexpr (numeric_limits<_Fp>::has_infinity) {
    if (__negative)
      return {-std::numeric_limits<_Fp>::infinity(), __ptr - __first, std::errc{}};

    return {std::numeric_limits<_Fp>::infinity(), __ptr - __first, std::errc{}};
  } else {
    return {_Fp{0}, __ptr - __first, errc::result_out_of_range};
  }
}

// Parses a nan string.
// Valid strings are case insensitive and contain INF or INFINITY.
//
// - __first is the first argument to std::from_chars. When the string is invalid
//   this value is returned as ptr in the result.
// - __last is the last argument of std::from_chars.
// - __value is the value argument of std::from_chars,
// - __ptr is the current position is the input string. This is points beyond
//   the initial N character.
// - __negative whether a valid string represents -nan or +nan.
template <floating_point _Fp>
__from_chars_result<_Fp>
__from_chars_floating_point_nan(const char* const __first, const char* __last, const char* __ptr, bool __negative) {
  if (__last - __ptr < 2) [[unlikely]]
    return {_Fp{0}, 0, errc::invalid_argument};

  if (std::tolower(__ptr[0]) != 'a' || std::tolower(__ptr[1]) != 'n') [[unlikely]]
    return {_Fp{0}, 0, errc::invalid_argument};

  __ptr += 2;

  // At this point the result is valid and contains NAN. When the remaining
  // part contains ( n-char-sequence_opt ) this will be consumed. Otherwise
  // only NAN is consumed. For example NAN(abcd will consume NAN and ignore
  // (abcd.
  if (__last - __ptr >= 2 && __ptr[0] == '(') {
    size_t __offset = 1;
    do {
      if (__ptr[__offset] == ')') {
        __ptr += __offset + 1;
        break;
      }
      if (__ptr[__offset] != '_' && !std::isalnum(__ptr[__offset]))
        break;
      ++__offset;
    } while (__ptr + __offset != __last);
  }

  if (__negative)
    return {-std::numeric_limits<_Fp>::quiet_NaN(), __ptr - __first, std::errc{}};

  return {std::numeric_limits<_Fp>::quiet_NaN(), __ptr - __first, std::errc{}};
}

template <class _Tp>
struct __fractional_constant_result {
  size_t __offset{size_t(-1)};
  _Tp __mantissa{0};
  int __exponent{0};
  bool __truncated{false};
  bool __is_valid{false};
};

// Parses the hex constant part of the hexadecimal floating-point value.
// - input start of buffer given to from_chars
// - __n the number of elements in the buffer
// - __offset where to start parsing. The input can have an optional sign, the
//   offset starts after this sign.
template <class _Tp>
__fractional_constant_result<_Tp> __parse_fractional_hex_constant(const char* __input, size_t __n, size_t __offset) {
  __fractional_constant_result<_Tp> __result;

  const _Tp __mantissa_truncate_threshold = numeric_limits<_Tp>::max() / 16;
  bool __fraction                         = false;
  for (; __offset < __n; ++__offset) {
    if (std::isxdigit(__input[__offset])) {
      __result.__is_valid = true;

      uint32_t __digit = __input[__offset] - '0';
      switch (std::tolower(__input[__offset])) {
      case 'a':
        __digit = 10;
        break;
      case 'b':
        __digit = 11;
        break;
      case 'c':
        __digit = 12;
        break;
      case 'd':
        __digit = 13;
        break;
      case 'e':
        __digit = 14;
        break;
      case 'f':
        __digit = 15;
        break;
      }

      if (__result.__mantissa < __mantissa_truncate_threshold) {
        __result.__mantissa = (__result.__mantissa * 16) + __digit;
        if (__fraction)
          __result.__exponent -= 4;
      } else {
        if (__digit > 0)
          __result.__truncated = true;
        if (!__fraction)
          __result.__exponent += 4;
      }
    } else if (__input[__offset] == '.') {
      if (__fraction)
        break; // this means that __input[__offset] points to a second decimal point, ending the number.

      __fraction = true;
    } else
      break;
  }

  __result.__offset = __offset;
  return __result;
}

struct __exponent_result {
  size_t __offset{size_t(-1)};
  int __value{0};
  bool __present{false};
};

// When the exponent is not present the result of the struct contains
// __offset, 0, false. This allows using the results unconditionally, the
// __present is important for the scientific notation, where the value is
// mandatory.
__exponent_result __parse_exponent(const char* __input, size_t __n, size_t __offset, char __marker) {
  if (__offset + 1 < __n &&                          // an exponent always needs at least one digit.
      std::tolower(__input[__offset]) == __marker && //
      !std::isspace(__input[__offset + 1])           // leading whitespace is not allowed.
  ) {
    ++__offset;
    LIBC_NAMESPACE::shared::StrToNumResult<int32_t> __e =
        LIBC_NAMESPACE::shared::strtointeger<int32_t>(__input + __offset, 10, __n - __offset);
    // __result.error contains the errno value, 0 or ERANGE these are not interesting.
    // If the number of characters parsed is 0 it means there was no number.
    if (__e.parsed_len != 0)
      return {__offset + __e.parsed_len, __e.value, true};
    else
      --__offset; // the assumption of a valid exponent was not true, undo eating the exponent character.
  }

  return {__offset, 0, false};
}

// Here we do this operation as int64 to avoid overflow.
int32_t __merge_exponents(int64_t __fractional, int64_t __exponent, int __max_biased_exponent) {
  int64_t __sum = __fractional + __exponent;

  if (__sum > __max_biased_exponent)
    return __max_biased_exponent;

  if (__sum < -__max_biased_exponent)
    return -__max_biased_exponent;

  return __sum;
}

template <class _Fp, class _Tp>
__from_chars_result<_Fp>
__calculate_result(_Tp __mantissa, int __exponent, bool __negative, __from_chars_result<_Fp> __result) {
  auto __r = LIBC_NAMESPACE::shared::FPBits<_Fp>();
  __r.set_mantissa(__mantissa);
  __r.set_biased_exponent(__exponent);

  // C17 7.12.1/6
  // The result underflows if the magnitude of the mathematical result is so
  // small that the mathematical result cannot be represented, without
  // extraordinary roundoff error, in an object of the specified type.237) If
  // the result underflows, the function returns an implementation-defined
  // value whose magnitude is no greater than the smallest normalized positive
  // number in the specified type; if the integer expression math_errhandling
  // & MATH_ERRNO is nonzero, whether errno acquires the value ERANGE is
  // implementation-defined; if the integer expression math_errhandling &
  // MATH_ERREXCEPT is nonzero, whether the "underflow" floating-point
  // exception is raised is implementation-defined.
  //
  // LLVM-LIBC sets ERAGNE for subnormal values
  //
  // [charconv.from.chars]/1
  //   ... If the parsed value is not in the range representable by the type of
  //   value, value is unmodified and the member ec of the return value is
  //   equal to errc::result_out_of_range. ...
  //
  // Undo the ERANGE for subnormal values.
  if (__result.__ec == errc::result_out_of_range && __r.is_subnormal() && !__r.is_zero())
    __result.__ec = errc{};

  if (__negative)
    __result.__value = -__r.get_val();
  else
    __result.__value = __r.get_val();

  return __result;
}

// Implements from_chars for decimal floating-point values.
// __first forwarded from from_chars
// __last forwarded from from_chars
// __value forwarded from from_chars
// __fmt forwarded from from_chars
// __ptr the start of the buffer to parse. This is after the optional sign character.
// __negative should __value be set to a negative value?
//
// This function and __from_chars_floating_point_decimal are similar. However
// the similar parts are all in helper functions. So the amount of code
// duplication is minimal.
template <floating_point _Fp>
__from_chars_result<_Fp>
__from_chars_floating_point_hex(const char* const __first, const char* __last, const char* __ptr, bool __negative) {
  size_t __n         = __last - __first;
  ptrdiff_t __offset = __ptr - __first;

  auto __fractional =
      std::__parse_fractional_hex_constant<typename _Floating_type_traits<_Fp>::_Uint_type>(__first, __n, __offset);
  if (!__fractional.__is_valid)
    return {_Fp{0}, 0, errc::invalid_argument};

  auto __parsed_exponent = std::__parse_exponent(__first, __n, __fractional.__offset, 'p');
  __offset               = __parsed_exponent.__offset;
  int __exponent         = std::__merge_exponents(
      __fractional.__exponent, __parsed_exponent.__value, LIBC_NAMESPACE::shared::FPBits<_Fp>::MAX_BIASED_EXPONENT);

  __from_chars_result<_Fp> __result{_Fp{0}, __offset, {}};
  LIBC_NAMESPACE::shared::ExpandedFloat<_Fp> __expanded_float = {0, 0};
  if (__fractional.__mantissa != 0) {
    auto __temp = LIBC_NAMESPACE::shared::binary_exp_to_float<_Fp>(
        {__fractional.__mantissa, __exponent},
        __fractional.__truncated,
        LIBC_NAMESPACE::shared::RoundDirection::Nearest);
    __expanded_float = __temp.num;
    if (__temp.error == ERANGE) {
      __result.__ec = errc::result_out_of_range;
    }
  }

  return std::__calculate_result<_Fp>(__expanded_float.mantissa, __expanded_float.exponent, __negative, __result);
}

// Parses the hex constant part of the decimal float value.
// - input start of buffer given to from_chars
// - __n the number of elements in the buffer
// - __offset where to start parsing. The input can have an optional sign, the
//   offset starts after this sign.
template <class _Tp>
__fractional_constant_result<_Tp>
__parse_fractional_decimal_constant(const char* __input, ptrdiff_t __n, ptrdiff_t __offset) {
  __fractional_constant_result<_Tp> __result;

  const _Tp __mantissa_truncate_threshold = numeric_limits<_Tp>::max() / 10;
  bool __fraction                         = false;
  for (; __offset < __n; ++__offset) {
    if (std::isdigit(__input[__offset])) {
      __result.__is_valid = true;

      uint32_t __digit = __input[__offset] - '0';
      if (__result.__mantissa < __mantissa_truncate_threshold) {
        __result.__mantissa = (__result.__mantissa * 10) + __digit;
        if (__fraction)
          --__result.__exponent;
      } else {
        if (__digit > 0)
          __result.__truncated = true;
        if (!__fraction)
          ++__result.__exponent;
      }
    } else if (__input[__offset] == '.') {
      if (__fraction)
        break; // this means that __input[__offset] points to a second decimal point, ending the number.

      __fraction = true;
    } else
      break;
  }

  __result.__offset = __offset;
  return __result;
}

// Implements from_chars for decimal floating-point values.
// __first forwarded from from_chars
// __last forwarded from from_chars
// __value forwarded from from_chars
// __fmt forwarded from from_chars
// __ptr the start of the buffer to parse. This is after the optional sign character.
// __negative should __value be set to a negative value?
template <floating_point _Fp>
__from_chars_result<_Fp> __from_chars_floating_point_decimal(
    const char* const __first, const char* __last, chars_format __fmt, const char* __ptr, bool __negative) {
  ptrdiff_t __n      = __last - __first;
  ptrdiff_t __offset = __ptr - __first;

  auto __fractional =
      std::__parse_fractional_decimal_constant<typename _Floating_type_traits<_Fp>::_Uint_type>(__first, __n, __offset);
  if (!__fractional.__is_valid)
    return {_Fp{0}, 0, errc::invalid_argument};

  __offset = __fractional.__offset;

  // LWG3456 Pattern used by std::from_chars is underspecified
  // This changes fixed to ignore a possible exponent instead of making its
  // existance an error.
  int __exponent;
  if (__fmt == chars_format::fixed) {
    __exponent =
        std::__merge_exponents(__fractional.__exponent, 0, LIBC_NAMESPACE::shared::FPBits<_Fp>::MAX_BIASED_EXPONENT);
  } else {
    auto __parsed_exponent = std::__parse_exponent(__first, __n, __offset, 'e');
    if (__fmt == chars_format::scientific && !__parsed_exponent.__present) {
      // [charconv.from.chars]/6.2 if fmt has chars_format::scientific set but not chars_format::fixed,
      // the otherwise optional exponent part shall appear;
      return {_Fp{0}, 0, errc::invalid_argument};
    }

    __offset   = __parsed_exponent.__offset;
    __exponent = std::__merge_exponents(
        __fractional.__exponent, __parsed_exponent.__value, LIBC_NAMESPACE::shared::FPBits<_Fp>::MAX_BIASED_EXPONENT);
  }

  __from_chars_result<_Fp> __result{_Fp{0}, __offset, {}};
  LIBC_NAMESPACE::shared::ExpandedFloat<_Fp> __expanded_float = {0, 0};
  if (__fractional.__mantissa != 0) {
    // This function expects to parse a positive value. This means it does not
    // take a __first, __n as arguments, since __first points to '-' for
    // negative values.
    auto __temp = LIBC_NAMESPACE::shared::decimal_exp_to_float<_Fp>(
        {__fractional.__mantissa, __exponent},
        __fractional.__truncated,
        LIBC_NAMESPACE::shared::RoundDirection::Nearest,
        __ptr,
        __last - __ptr);
    __expanded_float = __temp.num;
    if (__temp.error == ERANGE) {
      __result.__ec = errc::result_out_of_range;
    }
  }

  return std::__calculate_result(__expanded_float.mantissa, __expanded_float.exponent, __negative, __result);
}

template <floating_point _Fp>
__from_chars_result<_Fp>
__from_chars_floating_point_impl(const char* const __first, const char* __last, chars_format __fmt) {
  if (__first == __last) [[unlikely]]
    return {_Fp{0}, 0, errc::invalid_argument};

  const char* __ptr = __first;
  bool __negative   = *__ptr == '-';
  if (__negative) {
    ++__ptr;
    if (__ptr == __last) [[unlikely]]
      return {_Fp{0}, 0, errc::invalid_argument};
  }

  // [charconv.from.chars]
  //   [Note 1: If the pattern allows for an optional sign, but the string has
  //   no digit characters following the sign, no characters match the pattern.
  //   -- end note]
  // This is true for integrals, floating point allows -.0

  // [charconv.from.chars]/6.2
  //   if fmt has chars_format::scientific set but not chars_format::fixed, the
  //   otherwise optional exponent part shall appear;
  // Since INF/NAN do not have an exponent this value is not valid.
  //
  // LWG3456 Pattern used by std::from_chars is underspecified
  // Does not address this point, but proposed option B does solve this issue,
  // Both MSVC STL and libstdc++ implement this this behaviour.
  switch (std::tolower(*__ptr)) {
  case 'i':
    return std::__from_chars_floating_point_inf<_Fp>(__first, __last, __ptr + 1, __negative);
  case 'n':
    if constexpr (numeric_limits<_Fp>::has_quiet_NaN)
      // NOTE: The pointer passed here will be parsed in the default C locale.
      // This is standard behavior (see https://eel.is/c++draft/charconv.from.chars), but may be unexpected.
      return std::__from_chars_floating_point_nan<_Fp>(__first, __last, __ptr + 1, __negative);
    return {_Fp{0}, 0, errc::invalid_argument};
  }

  if (__fmt == chars_format::hex)
    return std::__from_chars_floating_point_hex<_Fp>(__first, __last, __ptr, __negative);

  return std::__from_chars_floating_point_decimal<_Fp>(__first, __last, __fmt, __ptr, __negative);
}

_LIBCPP_END_NAMESPACE_STD

#endif //_LIBCPP_SRC_INCLUDE_FROM_CHARS_FLOATING_POINT_H
