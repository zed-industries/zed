//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_ALIGNED_STORAGE_H
#define _LIBCPP___TYPE_TRAITS_ALIGNED_STORAGE_H

#include <__config>
#include <__cstddef/size_t.h>
#include <__type_traits/integral_constant.h>
#include <__type_traits/type_list.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Tp>
struct __align_type {
  static const size_t value = _LIBCPP_PREFERRED_ALIGNOF(_Tp);
  typedef _Tp type;
};

struct __struct_double {
  long double __lx;
};
struct __struct_double4 {
  double __lx[4];
};

using __all_types _LIBCPP_NODEBUG =
    __type_list<__align_type<unsigned char>,
                __align_type<unsigned short>,
                __align_type<unsigned int>,
                __align_type<unsigned long>,
                __align_type<unsigned long long>,
                __align_type<double>,
                __align_type<long double>,
                __align_type<__struct_double>,
                __align_type<__struct_double4>,
                __align_type<int*> >;

template <class _TL, size_t _Len>
struct __find_max_align;

template <class _Head, size_t _Len>
struct __find_max_align<__type_list<_Head>, _Len> : public integral_constant<size_t, _Head::value> {};

template <size_t _Len, size_t _A1, size_t _A2>
struct __select_align {
private:
  static const size_t __min = _A2 < _A1 ? _A2 : _A1;
  static const size_t __max = _A1 < _A2 ? _A2 : _A1;

public:
  static const size_t value = _Len < __max ? __min : __max;
};

template <class _Head, class... _Tail, size_t _Len>
struct __find_max_align<__type_list<_Head, _Tail...>, _Len>
    : public integral_constant<
          size_t,
          __select_align<_Len, _Head::value, __find_max_align<__type_list<_Tail...>, _Len>::value>::value> {};

template <size_t _Len, size_t _Align = __find_max_align<__all_types, _Len>::value>
struct _LIBCPP_DEPRECATED_IN_CXX23 _LIBCPP_NO_SPECIALIZATIONS aligned_storage {
  union _ALIGNAS(_Align) type {
    unsigned char __data[(_Len + _Align - 1) / _Align * _Align];
  };
};

#if _LIBCPP_STD_VER >= 14

_LIBCPP_SUPPRESS_DEPRECATED_PUSH
template <size_t _Len, size_t _Align = __find_max_align<__all_types, _Len>::value>
using aligned_storage_t _LIBCPP_DEPRECATED_IN_CXX23 = typename aligned_storage<_Len, _Align>::type;
_LIBCPP_SUPPRESS_DEPRECATED_POP

#endif

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_ALIGNED_STORAGE_H
