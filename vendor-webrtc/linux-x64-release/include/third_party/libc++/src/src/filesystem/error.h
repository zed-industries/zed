//===----------------------------------------------------------------------===////
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===////

#ifndef FILESYSTEM_ERROR_H
#define FILESYSTEM_ERROR_H

#include <__assert>
#include <__config>
#include <cerrno>
#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <filesystem>
#include <string>
#include <system_error>
#include <utility> // __libcpp_unreachable

#include "format_string.h"

#if defined(_LIBCPP_WIN32API)
#  define WIN32_LEAN_AND_MEAN
#  define NOMINMAX
#  include <windows.h> // ERROR_* macros
#endif

_LIBCPP_BEGIN_NAMESPACE_FILESYSTEM

namespace detail {

// On windows, libc functions use errno, but system functions use GetLastError.
// So, callers need to be careful which of these next functions they call!

inline error_code capture_errno() {
  _LIBCPP_ASSERT_INTERNAL(errno != 0, "Expected errno to be non-zero");
  return error_code(errno, generic_category());
}

inline error_code get_last_error() {
#if defined(_LIBCPP_WIN32API)
  return std::error_code(GetLastError(), std::system_category());
#else
  return capture_errno();
#endif
}

template <class T>
T error_value();
template <>
inline constexpr void error_value<void>() {}
template <>
inline bool error_value<bool>() {
  return false;
}
#if __SIZEOF_SIZE_T__ != __SIZEOF_LONG_LONG__
template <>
inline size_t error_value<size_t>() {
  return size_t(-1);
}
#endif
template <>
inline uintmax_t error_value<uintmax_t>() {
  return uintmax_t(-1);
}
template <>
inline constexpr file_time_type error_value<file_time_type>() {
  return file_time_type::min();
}
template <>
inline path error_value<path>() {
  return {};
}

template <class T>
struct ErrorHandler {
  const char* func_name_;
  error_code* ec_ = nullptr;
  const path* p1_ = nullptr;
  const path* p2_ = nullptr;

  ErrorHandler(const char* fname, error_code* ec, const path* p1 = nullptr, const path* p2 = nullptr)
      : func_name_(fname), ec_(ec), p1_(p1), p2_(p2) {
    if (ec_)
      ec_->clear();
  }

  T report(const error_code& ec) const {
    if (ec_) {
      *ec_ = ec;
      return error_value<T>();
    }
    string what = string("in ") + func_name_;
    switch (bool(p1_) + bool(p2_)) {
    case 0:
      filesystem::__throw_filesystem_error(what, ec);
    case 1:
      filesystem::__throw_filesystem_error(what, *p1_, ec);
    case 2:
      filesystem::__throw_filesystem_error(what, *p1_, *p2_, ec);
    }
    __libcpp_unreachable();
  }

  _LIBCPP_ATTRIBUTE_FORMAT(__printf__, 3, 0)
  void report_impl(const error_code& ec, const char* msg, va_list ap) const {
    if (ec_) {
      *ec_ = ec;
      return;
    }
    string what = string("in ") + func_name_ + ": " + detail::vformat_string(msg, ap);
    switch (bool(p1_) + bool(p2_)) {
    case 0:
      filesystem::__throw_filesystem_error(what, ec);
    case 1:
      filesystem::__throw_filesystem_error(what, *p1_, ec);
    case 2:
      filesystem::__throw_filesystem_error(what, *p1_, *p2_, ec);
    }
    __libcpp_unreachable();
  }

  _LIBCPP_ATTRIBUTE_FORMAT(__printf__, 3, 4)
  T report(const error_code& ec, const char* msg, ...) const {
    va_list ap;
    va_start(ap, msg);
#if _LIBCPP_HAS_EXCEPTIONS
    try {
#endif // _LIBCPP_HAS_EXCEPTIONS
      report_impl(ec, msg, ap);
#if _LIBCPP_HAS_EXCEPTIONS
    } catch (...) {
      va_end(ap);
      throw;
    }
#endif // _LIBCPP_HAS_EXCEPTIONS
    va_end(ap);
    return error_value<T>();
  }

  T report(errc const& err) const { return report(make_error_code(err)); }

  _LIBCPP_ATTRIBUTE_FORMAT(__printf__, 3, 4)
  T report(errc const& err, const char* msg, ...) const {
    va_list ap;
    va_start(ap, msg);
#if _LIBCPP_HAS_EXCEPTIONS
    try {
#endif // _LIBCPP_HAS_EXCEPTIONS
      report_impl(make_error_code(err), msg, ap);
#if _LIBCPP_HAS_EXCEPTIONS
    } catch (...) {
      va_end(ap);
      throw;
    }
#endif // _LIBCPP_HAS_EXCEPTIONS
    va_end(ap);
    return error_value<T>();
  }

private:
  ErrorHandler(ErrorHandler const&)            = delete;
  ErrorHandler& operator=(ErrorHandler const&) = delete;
};

} // namespace detail

_LIBCPP_END_NAMESPACE_FILESYSTEM

#endif // FILESYSTEM_ERROR_H
