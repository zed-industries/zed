//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_EXTENT_H
#define _LIBCPP___TYPE_TRAITS_EXTENT_H

#include <__config>
#include <__cstddef/size_t.h>
#include <__type_traits/integral_constant.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

#if __has_builtin(__array_extent)

template <class _Tp, size_t _Dim = 0>
struct _LIBCPP_NO_SPECIALIZATIONS extent : integral_constant<size_t, __array_extent(_Tp, _Dim)> {};

#  if _LIBCPP_STD_VER >= 17
template <class _Tp, unsigned _Ip = 0>
_LIBCPP_NO_SPECIALIZATIONS inline constexpr size_t extent_v = __array_extent(_Tp, _Ip);
#  endif

#else // __has_builtin(__array_extent)

template <class _Tp, unsigned _Ip = 0>
struct extent : public integral_constant<size_t, 0> {};
template <class _Tp>
struct extent<_Tp[], 0> : public integral_constant<size_t, 0> {};
template <class _Tp, unsigned _Ip>
struct extent<_Tp[], _Ip> : public integral_constant<size_t, extent<_Tp, _Ip - 1>::value> {};
template <class _Tp, size_t _Np>
struct extent<_Tp[_Np], 0> : public integral_constant<size_t, _Np> {};
template <class _Tp, size_t _Np, unsigned _Ip>
struct extent<_Tp[_Np], _Ip> : public integral_constant<size_t, extent<_Tp, _Ip - 1>::value> {};

#  if _LIBCPP_STD_VER >= 17
template <class _Tp, unsigned _Ip = 0>
inline constexpr size_t extent_v = extent<_Tp, _Ip>::value;
#  endif

#endif // __has_builtin(__array_extent)

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_EXTENT_H
