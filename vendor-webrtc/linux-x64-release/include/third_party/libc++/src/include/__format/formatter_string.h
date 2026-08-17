// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___FORMAT_FORMATTER_STRING_H
#define _LIBCPP___FORMAT_FORMATTER_STRING_H

#include <__config>
#include <__format/concepts.h>
#include <__format/format_parse_context.h>
#include <__format/formatter.h>
#include <__format/formatter_output.h>
#include <__format/parser_std_format_spec.h>
#include <__format/write_escaped.h>
#include <string>
#include <string_view>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

#if _LIBCPP_STD_VER >= 20

template <__fmt_char_type _CharT>
struct __formatter_string {
public:
  template <class _ParseContext>
  _LIBCPP_HIDE_FROM_ABI constexpr typename _ParseContext::iterator parse(_ParseContext& __ctx) {
    typename _ParseContext::iterator __result = __parser_.__parse(__ctx, __format_spec::__fields_string);
    __format_spec::__process_display_type_string(__parser_.__type_);
    return __result;
  }

  template <class _FormatContext>
  _LIBCPP_HIDE_FROM_ABI typename _FormatContext::iterator
  format(basic_string_view<_CharT> __str, _FormatContext& __ctx) const {
#  if _LIBCPP_STD_VER >= 23
    if (__parser_.__type_ == __format_spec::__type::__debug)
      return __formatter::__format_escaped_string(__str, __ctx.out(), __parser_.__get_parsed_std_specifications(__ctx));
#  endif

    return __formatter::__write_string(__str, __ctx.out(), __parser_.__get_parsed_std_specifications(__ctx));
  }

#  if _LIBCPP_STD_VER >= 23
  _LIBCPP_HIDE_FROM_ABI constexpr void set_debug_format() { __parser_.__type_ = __format_spec::__type::__debug; }
#  endif

  __format_spec::__parser<_CharT> __parser_{.__alignment_ = __format_spec::__alignment::__left};
};

// Formatter const char*.
template <__fmt_char_type _CharT>
struct formatter<const _CharT*, _CharT> : public __formatter_string<_CharT> {
  using _Base _LIBCPP_NODEBUG = __formatter_string<_CharT>;

  template <class _FormatContext>
  _LIBCPP_HIDE_FROM_ABI typename _FormatContext::iterator format(const _CharT* __str, _FormatContext& __ctx) const {
    _LIBCPP_ASSERT_INTERNAL(__str, "The basic_format_arg constructor should have prevented an invalid pointer.");
    // Converting the input to a basic_string_view means the data is looped over twice;
    // - once to determine the length, and
    // - once to process the data.
    //
    // This sounds slower than writing the output directly. However internally
    // the output algorithms have optimizations for "bulk" operations, which
    // makes this faster than a single-pass character-by-character output.
    return _Base::format(basic_string_view<_CharT>(__str), __ctx);
  }
};

// Formatter char*.
template <__fmt_char_type _CharT>
struct formatter<_CharT*, _CharT> : public formatter<const _CharT*, _CharT> {
  using _Base _LIBCPP_NODEBUG = formatter<const _CharT*, _CharT>;

  template <class _FormatContext>
  _LIBCPP_HIDE_FROM_ABI typename _FormatContext::iterator format(_CharT* __str, _FormatContext& __ctx) const {
    return _Base::format(__str, __ctx);
  }
};

// Formatter char[].
template <__fmt_char_type _CharT, size_t _Size>
struct formatter<_CharT[_Size], _CharT> : public __formatter_string<_CharT> {
  using _Base _LIBCPP_NODEBUG = __formatter_string<_CharT>;

  template <class _FormatContext>
  _LIBCPP_HIDE_FROM_ABI typename _FormatContext::iterator
  format(const _CharT (&__str)[_Size], _FormatContext& __ctx) const {
    return _Base::format(basic_string_view<_CharT>(__str, _Size), __ctx);
  }
};

// Formatter std::string.
template <__fmt_char_type _CharT, class _Traits, class _Allocator>
struct formatter<basic_string<_CharT, _Traits, _Allocator>, _CharT> : public __formatter_string<_CharT> {
  using _Base _LIBCPP_NODEBUG = __formatter_string<_CharT>;

  template <class _FormatContext>
  _LIBCPP_HIDE_FROM_ABI typename _FormatContext::iterator
  format(const basic_string<_CharT, _Traits, _Allocator>& __str, _FormatContext& __ctx) const {
    // Drop _Traits and _Allocator to have one std::basic_string formatter.
    return _Base::format(basic_string_view<_CharT>(__str.data(), __str.size()), __ctx);
  }
};

// Formatter std::string_view.
template <__fmt_char_type _CharT, class _Traits>
struct formatter<basic_string_view<_CharT, _Traits>, _CharT> : public __formatter_string<_CharT> {
  using _Base _LIBCPP_NODEBUG = __formatter_string<_CharT>;

  template <class _FormatContext>
  _LIBCPP_HIDE_FROM_ABI typename _FormatContext::iterator
  format(basic_string_view<_CharT, _Traits> __str, _FormatContext& __ctx) const {
    // Drop _Traits to have one std::basic_string_view formatter.
    return _Base::format(basic_string_view<_CharT>(__str.data(), __str.size()), __ctx);
  }
};

#  if _LIBCPP_HAS_WIDE_CHARACTERS
template <>
struct formatter<char*, wchar_t> : __disabled_formatter {};
template <>
struct formatter<const char*, wchar_t> : __disabled_formatter {};
template <size_t _Size>
struct formatter<char[_Size], wchar_t> : __disabled_formatter {};
template <class _Traits, class _Allocator>
struct formatter<basic_string<char, _Traits, _Allocator>, wchar_t> : __disabled_formatter {};
template <class _Traits>
struct formatter<basic_string_view<char, _Traits>, wchar_t> : __disabled_formatter {};
#  endif // _LIBCPP_HAS_WIDE_CHARACTERS

#  if _LIBCPP_STD_VER >= 23
template <>
inline constexpr bool enable_nonlocking_formatter_optimization<char*> = true;
template <>
inline constexpr bool enable_nonlocking_formatter_optimization<const char*> = true;
template <size_t _Size>
inline constexpr bool enable_nonlocking_formatter_optimization<char[_Size]> = true;
template <class _Traits, class _Allocator>
inline constexpr bool enable_nonlocking_formatter_optimization<basic_string<char, _Traits, _Allocator>> = true;
template <class _Traits>
inline constexpr bool enable_nonlocking_formatter_optimization<basic_string_view<char, _Traits>> = true;

#    if _LIBCPP_HAS_WIDE_CHARACTERS
template <>
inline constexpr bool enable_nonlocking_formatter_optimization<wchar_t*> = true;
template <>
inline constexpr bool enable_nonlocking_formatter_optimization<const wchar_t*> = true;
template <size_t _Size>
inline constexpr bool enable_nonlocking_formatter_optimization<wchar_t[_Size]> = true;
template <class _Traits, class _Allocator>
inline constexpr bool enable_nonlocking_formatter_optimization<basic_string<wchar_t, _Traits, _Allocator>> = true;
template <class _Traits>
inline constexpr bool enable_nonlocking_formatter_optimization<basic_string_view<wchar_t, _Traits>> = true;
#    endif // _LIBCPP_HAS_WIDE_CHARACTERS
#  endif   // _LIBCPP_STD_VER >= 23
#endif     // _LIBCPP_STD_VER >= 20

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___FORMAT_FORMATTER_STRING_H
