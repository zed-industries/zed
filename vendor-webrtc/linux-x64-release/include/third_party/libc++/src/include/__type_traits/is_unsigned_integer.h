//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_IS_UNSIGNED_INTEGER_H
#define _LIBCPP___TYPE_TRAITS_IS_UNSIGNED_INTEGER_H

#include <__config>
#include <__type_traits/integral_constant.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

// clang-format off
template <class _Tp> struct __libcpp_is_unsigned_integer                     : false_type {};
template <>          struct __libcpp_is_unsigned_integer<unsigned char>      : true_type {};
template <>          struct __libcpp_is_unsigned_integer<unsigned short>     : true_type {};
template <>          struct __libcpp_is_unsigned_integer<unsigned int>       : true_type {};
template <>          struct __libcpp_is_unsigned_integer<unsigned long>      : true_type {};
template <>          struct __libcpp_is_unsigned_integer<unsigned long long> : true_type {};
#if _LIBCPP_HAS_INT128
template <>          struct __libcpp_is_unsigned_integer<__uint128_t>        : true_type {};
#endif
// clang-format on

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_IS_UNSIGNED_INTEGER_H
