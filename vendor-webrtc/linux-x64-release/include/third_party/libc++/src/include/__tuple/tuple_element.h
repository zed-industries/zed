//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TUPLE_TUPLE_ELEMENT_H
#define _LIBCPP___TUPLE_TUPLE_ELEMENT_H

#include <__config>
#include <__cstddef/size_t.h>
#include <__tuple/tuple_indices.h>
#include <__tuple/tuple_types.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <size_t _Ip, class _Tp>
struct tuple_element;

template <size_t _Ip, class _Tp>
struct tuple_element<_Ip, const _Tp> {
  using type _LIBCPP_NODEBUG = const typename tuple_element<_Ip, _Tp>::type;
};

template <size_t _Ip, class _Tp>
struct tuple_element<_Ip, volatile _Tp> {
  using type _LIBCPP_NODEBUG = volatile typename tuple_element<_Ip, _Tp>::type;
};

template <size_t _Ip, class _Tp>
struct tuple_element<_Ip, const volatile _Tp> {
  using type _LIBCPP_NODEBUG = const volatile typename tuple_element<_Ip, _Tp>::type;
};

#ifndef _LIBCPP_CXX03_LANG

template <size_t _Ip, class... _Types>
struct tuple_element<_Ip, __tuple_types<_Types...> > {
  static_assert(_Ip < sizeof...(_Types), "tuple_element index out of range");
  using type _LIBCPP_NODEBUG = __type_pack_element<_Ip, _Types...>;
};

#  if _LIBCPP_STD_VER >= 14
template <size_t _Ip, class... _Tp>
using tuple_element_t _LIBCPP_NODEBUG = typename tuple_element<_Ip, _Tp...>::type;
#  endif

#endif // _LIBCPP_CXX03_LANG

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TUPLE_TUPLE_ELEMENT_H
