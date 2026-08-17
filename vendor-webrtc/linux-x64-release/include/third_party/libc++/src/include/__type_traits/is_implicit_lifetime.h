//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_IS_IMPLICIT_LIFETIME_H
#define _LIBCPP___TYPE_TRAITS_IS_IMPLICIT_LIFETIME_H

#include <__config>
#include <__type_traits/integral_constant.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

#if _LIBCPP_STD_VER >= 23
#  if __has_builtin(__builtin_is_implicit_lifetime)

template <class _Tp>
struct _LIBCPP_NO_SPECIALIZATIONS is_implicit_lifetime : bool_constant<__builtin_is_implicit_lifetime(_Tp)> {};

template <class _Tp>
_LIBCPP_NO_SPECIALIZATIONS inline constexpr bool is_implicit_lifetime_v = __builtin_is_implicit_lifetime(_Tp);

#  endif
#endif

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_IS_IMPLICIT_LIFETIME_H
