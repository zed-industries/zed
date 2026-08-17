//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_IS_UNBOUNDED_ARRAY_H
#define _LIBCPP___TYPE_TRAITS_IS_UNBOUNDED_ARRAY_H

#include <__config>
#include <__type_traits/integral_constant.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class>
inline const bool __is_unbounded_array_v = false;
template <class _Tp>
inline const bool __is_unbounded_array_v<_Tp[]> = true;

#if _LIBCPP_STD_VER >= 20

template <class>
struct _LIBCPP_NO_SPECIALIZATIONS is_unbounded_array : false_type {};

_LIBCPP_DIAGNOSTIC_PUSH
#  if __has_warning("-Winvalid-specialization")
_LIBCPP_CLANG_DIAGNOSTIC_IGNORED("-Winvalid-specialization")
#  endif
template <class _Tp>
struct is_unbounded_array<_Tp[]> : true_type {};
_LIBCPP_DIAGNOSTIC_POP

template <class _Tp>
_LIBCPP_NO_SPECIALIZATIONS inline constexpr bool is_unbounded_array_v = is_unbounded_array<_Tp>::value;

#endif

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_IS_UNBOUNDED_ARRAY_H
