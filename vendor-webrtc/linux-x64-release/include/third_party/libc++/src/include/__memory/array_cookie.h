// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___MEMORY_ARRAY_COOKIE_H
#define _LIBCPP___MEMORY_ARRAY_COOKIE_H

#include <__config>
#include <__configuration/abi.h>
#include <__cstddef/size_t.h>
#include <__type_traits/integral_constant.h>
#include <__type_traits/is_trivially_destructible.h>
#include <__type_traits/negation.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

// Trait representing whether a type requires an array cookie at the start of its allocation when
// allocated as `new T[n]` and deallocated as `delete[] array`.
//
// Under the Itanium C++ ABI [1], we know that an array cookie is available unless `T` is trivially
// destructible and the call to `operator delete[]` is not a sized operator delete. Under ABIs other
// than the Itanium ABI, we assume there are no array cookies.
//
// [1]: https://itanium-cxx-abi.github.io/cxx-abi/abi.html#array-cookies
#ifdef _LIBCPP_ABI_ITANIUM
// TODO: Use a builtin instead
// TODO: We should factor in the choice of the usual deallocation function in this determination.
template <class _Tp>
struct __has_array_cookie : _Not<is_trivially_destructible<_Tp> > {};
#else
template <class _Tp>
struct __has_array_cookie : false_type {};
#endif

template <class _Tp>
// Avoid failures when -fsanitize-address-poison-custom-array-cookie is enabled
_LIBCPP_HIDE_FROM_ABI _LIBCPP_NO_SANITIZE("address") size_t __get_array_cookie(_Tp const* __ptr) {
  static_assert(
      __has_array_cookie<_Tp>::value, "Trying to access the array cookie of a type that is not guaranteed to have one");
  size_t const* __cookie = reinterpret_cast<size_t const*>(__ptr) - 1; // TODO: Use a builtin instead
  return *__cookie;
}

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___MEMORY_ARRAY_COOKIE_H
