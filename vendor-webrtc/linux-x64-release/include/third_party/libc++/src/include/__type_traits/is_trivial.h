//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_IS_TRIVIAL_H
#define _LIBCPP___TYPE_TRAITS_IS_TRIVIAL_H

#include <__config>
#include <__type_traits/integral_constant.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Tp>
struct _LIBCPP_DEPRECATED_IN_CXX26_(
    "Consider using is_trivially_copyable<T>::value && is_trivially_default_constructible<T>::value instead.")
    _LIBCPP_NO_SPECIALIZATIONS is_trivial : integral_constant<bool, __is_trivial(_Tp)> {};

#if _LIBCPP_STD_VER >= 17
template <class _Tp>
_LIBCPP_DEPRECATED_IN_CXX26_(
    "Consider using is_trivially_copyable_v<T> && is_trivially_default_constructible_v<T> instead.")
_LIBCPP_NO_SPECIALIZATIONS inline constexpr bool is_trivial_v = __is_trivial(_Tp);
#endif

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_IS_TRIVIAL_H
