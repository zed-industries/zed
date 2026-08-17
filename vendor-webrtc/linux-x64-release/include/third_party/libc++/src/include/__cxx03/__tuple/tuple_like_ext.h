//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___CXX03___TUPLE_TUPLE_LIKE_EXT_H
#define _LIBCPP___CXX03___TUPLE_TUPLE_LIKE_EXT_H

#include <__cxx03/__config>
#include <__cxx03/__fwd/array.h>
#include <__cxx03/__fwd/pair.h>
#include <__cxx03/__fwd/tuple.h>
#include <__cxx03/__tuple/tuple_types.h>
#include <__cxx03/__type_traits/integral_constant.h>
#include <__cxx03/cstddef>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Tp>
struct __tuple_like_ext : false_type {};

template <class _Tp>
struct __tuple_like_ext<const _Tp> : public __tuple_like_ext<_Tp> {};
template <class _Tp>
struct __tuple_like_ext<volatile _Tp> : public __tuple_like_ext<_Tp> {};
template <class _Tp>
struct __tuple_like_ext<const volatile _Tp> : public __tuple_like_ext<_Tp> {};

#ifndef _LIBCPP_CXX03_LANG
template <class... _Tp>
struct __tuple_like_ext<tuple<_Tp...> > : true_type {};
#endif

template <class _T1, class _T2>
struct __tuple_like_ext<pair<_T1, _T2> > : true_type {};

template <class _Tp, size_t _Size>
struct __tuple_like_ext<array<_Tp, _Size> > : true_type {};

template <class... _Tp>
struct __tuple_like_ext<__tuple_types<_Tp...> > : true_type {};

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___CXX03___TUPLE_TUPLE_LIKE_EXT_H
