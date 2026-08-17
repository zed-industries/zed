//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_TYPE_LIST_H
#define _LIBCPP___TYPE_TRAITS_TYPE_LIST_H

#include <__config>
#include <__cstddef/size_t.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class... _Types>
struct __type_list {};

template <class>
struct __type_list_head;

template <class _Head, class... _Tail>
struct __type_list_head<__type_list<_Head, _Tail...> > {
  using type _LIBCPP_NODEBUG = _Head;
};

template <class _TypeList, size_t _Size, bool = _Size <= sizeof(typename __type_list_head<_TypeList>::type)>
struct __find_first;

template <class _Head, class... _Tail, size_t _Size>
struct __find_first<__type_list<_Head, _Tail...>, _Size, true> {
  using type _LIBCPP_NODEBUG = _Head;
};

template <class _Head, class... _Tail, size_t _Size>
struct __find_first<__type_list<_Head, _Tail...>, _Size, false> {
  using type _LIBCPP_NODEBUG = typename __find_first<__type_list<_Tail...>, _Size>::type;
};

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_TYPE_LIST_H
