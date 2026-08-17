//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_UNWRAP_REF_H
#define _LIBCPP___TYPE_TRAITS_UNWRAP_REF_H

#include <__config>
#include <__fwd/functional.h>
#include <__type_traits/decay.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Tp>
struct __unwrap_reference {
  using type _LIBCPP_NODEBUG = _Tp;
};

template <class _Tp>
struct __unwrap_reference<reference_wrapper<_Tp> > {
  using type _LIBCPP_NODEBUG = _Tp&;
};

template <class _Tp>
using __unwrap_ref_decay_t _LIBCPP_NODEBUG = typename __unwrap_reference<__decay_t<_Tp> >::type;

#if _LIBCPP_STD_VER >= 20
template <class _Tp>
struct _LIBCPP_NO_SPECIALIZATIONS unwrap_reference : __unwrap_reference<_Tp> {};

template <class _Tp>
using unwrap_reference_t = typename unwrap_reference<_Tp>::type;

template <class _Tp>
struct _LIBCPP_NO_SPECIALIZATIONS unwrap_ref_decay : unwrap_reference<__decay_t<_Tp> > {};

template <class _Tp>
using unwrap_ref_decay_t = __unwrap_ref_decay_t<_Tp>;
#endif // _LIBCPP_STD_VER >= 20

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_UNWRAP_REF_H
