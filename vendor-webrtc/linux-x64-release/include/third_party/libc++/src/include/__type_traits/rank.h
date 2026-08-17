//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_RANK_H
#define _LIBCPP___TYPE_TRAITS_RANK_H

#include <__config>
#include <__cstddef/size_t.h>
#include <__type_traits/integral_constant.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

#if __has_builtin(__array_rank) && !defined(_LIBCPP_COMPILER_CLANG_BASED) ||                                           \
    (defined(_LIBCPP_CLANG_VER) && _LIBCPP_CLANG_VER >= 2001)

template <class _Tp>
struct _LIBCPP_NO_SPECIALIZATIONS rank : integral_constant<size_t, __array_rank(_Tp)> {};

#else

template <class _Tp>
struct _LIBCPP_NO_SPECIALIZATIONS rank : public integral_constant<size_t, 0> {};

_LIBCPP_DIAGNOSTIC_PUSH
#  if __has_warning("-Winvalid-specialization")
_LIBCPP_CLANG_DIAGNOSTIC_IGNORED("-Winvalid-specialization")
#  endif
template <class _Tp>
struct rank<_Tp[]> : public integral_constant<size_t, rank<_Tp>::value + 1> {};
template <class _Tp, size_t _Np>
struct rank<_Tp[_Np]> : public integral_constant<size_t, rank<_Tp>::value + 1> {};
_LIBCPP_DIAGNOSTIC_POP

#endif // __has_builtin(__array_rank)

#if _LIBCPP_STD_VER >= 17
template <class _Tp>
_LIBCPP_NO_SPECIALIZATIONS inline constexpr size_t rank_v = rank<_Tp>::value;
#endif

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_RANK_H
