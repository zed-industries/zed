//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_IS_REFERENCEABLE_H
#define _LIBCPP___TYPE_TRAITS_IS_REFERENCEABLE_H

#include <__config>
#include <__type_traits/void_t.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Tp, class = void>
inline const bool __is_referenceable_v = false;

template <class _Tp>
inline const bool __is_referenceable_v<_Tp, __void_t<_Tp&> > = true;

#if _LIBCPP_STD_VER >= 20
template <class _Tp>
concept __referenceable = __is_referenceable_v<_Tp>;
#endif

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_IS_REFERENCEABLE_H
