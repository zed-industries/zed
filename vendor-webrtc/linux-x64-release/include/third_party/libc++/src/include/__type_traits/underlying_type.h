//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_UNDERLYING_TYPE_H
#define _LIBCPP___TYPE_TRAITS_UNDERLYING_TYPE_H

#include <__config>
#include <__type_traits/is_enum.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Tp, bool>
struct __underlying_type_impl;

template <class _Tp>
struct __underlying_type_impl<_Tp, false> {};

template <class _Tp>
struct __underlying_type_impl<_Tp, true> {
  typedef __underlying_type(_Tp) type;
};

template <class _Tp>
struct _LIBCPP_NO_SPECIALIZATIONS underlying_type : __underlying_type_impl<_Tp, is_enum<_Tp>::value> {};

// GCC doesn't SFINAE away when using __underlying_type directly
#if !defined(_LIBCPP_COMPILER_GCC)
template <class _Tp>
using __underlying_type_t _LIBCPP_NODEBUG = __underlying_type(_Tp);
#else
template <class _Tp>
using __underlying_type_t _LIBCPP_NODEBUG = typename underlying_type<_Tp>::type;
#endif

#if _LIBCPP_STD_VER >= 14
template <class _Tp>
using underlying_type_t = __underlying_type_t<_Tp>;
#endif

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_UNDERLYING_TYPE_H
