//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___NEW_EXCEPTIONS_H
#define _LIBCPP___NEW_EXCEPTIONS_H

#include <__config>
#include <__exception/exception.h>
#include <__verbose_abort>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_UNVERSIONED_NAMESPACE_STD
#if !defined(_LIBCPP_ABI_VCRUNTIME)

class _LIBCPP_EXPORTED_FROM_ABI bad_alloc : public exception {
public:
  bad_alloc() _NOEXCEPT;
  _LIBCPP_HIDE_FROM_ABI bad_alloc(const bad_alloc&) _NOEXCEPT            = default;
  _LIBCPP_HIDE_FROM_ABI bad_alloc& operator=(const bad_alloc&) _NOEXCEPT = default;
  ~bad_alloc() _NOEXCEPT override;
  const char* what() const _NOEXCEPT override;
};

class _LIBCPP_EXPORTED_FROM_ABI bad_array_new_length : public bad_alloc {
public:
  bad_array_new_length() _NOEXCEPT;
  _LIBCPP_HIDE_FROM_ABI bad_array_new_length(const bad_array_new_length&) _NOEXCEPT            = default;
  _LIBCPP_HIDE_FROM_ABI bad_array_new_length& operator=(const bad_array_new_length&) _NOEXCEPT = default;
  ~bad_array_new_length() _NOEXCEPT override;
  const char* what() const _NOEXCEPT override;
};

#elif defined(_HAS_EXCEPTIONS) && _HAS_EXCEPTIONS == 0 // !_LIBCPP_ABI_VCRUNTIME

// When _HAS_EXCEPTIONS == 0, these complete definitions are needed,
// since they would normally be provided in vcruntime_exception.h
class bad_alloc : public exception {
public:
  bad_alloc() noexcept : exception("bad allocation") {}

private:
  friend class bad_array_new_length;

  bad_alloc(char const* const __message) noexcept : exception(__message) {}
};

class bad_array_new_length : public bad_alloc {
public:
  bad_array_new_length() noexcept : bad_alloc("bad array new length") {}
};

#endif // defined(_LIBCPP_ABI_VCRUNTIME) && defined(_HAS_EXCEPTIONS) && _HAS_EXCEPTIONS == 0

[[__noreturn__]] _LIBCPP_EXPORTED_FROM_ABI void __throw_bad_alloc(); // not in C++ spec

[[__noreturn__]] inline _LIBCPP_HIDE_FROM_ABI void __throw_bad_array_new_length() {
#if _LIBCPP_HAS_EXCEPTIONS
  throw bad_array_new_length();
#else
  _LIBCPP_VERBOSE_ABORT("bad_array_new_length was thrown in -fno-exceptions mode");
#endif
}
_LIBCPP_END_UNVERSIONED_NAMESPACE_STD

#endif // _LIBCPP___NEW_EXCEPTIONS_H
