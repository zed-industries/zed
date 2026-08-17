//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_ADD_CV_H
#define _LIBCPP___TYPE_TRAITS_ADD_CV_H

#include <__config>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Tp>
struct _LIBCPP_NO_SPECIALIZATIONS add_const {
  using type _LIBCPP_NODEBUG = const _Tp;
};

#if _LIBCPP_STD_VER >= 14
template <class _Tp>
using add_const_t = typename add_const<_Tp>::type;
#endif

template <class _Tp>
struct _LIBCPP_NO_SPECIALIZATIONS add_cv {
  using type _LIBCPP_NODEBUG = const volatile _Tp;
};

#if _LIBCPP_STD_VER >= 14
template <class _Tp>
using add_cv_t = typename add_cv<_Tp>::type;
#endif

template <class _Tp>
struct _LIBCPP_NO_SPECIALIZATIONS add_volatile {
  using type _LIBCPP_NODEBUG = volatile _Tp;
};

#if _LIBCPP_STD_VER >= 14
template <class _Tp>
using add_volatile_t = typename add_volatile<_Tp>::type;
#endif

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_ADD_CV_H
