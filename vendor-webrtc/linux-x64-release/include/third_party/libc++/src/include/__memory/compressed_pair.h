// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___MEMORY_COMPRESSED_PAIR_H
#define _LIBCPP___MEMORY_COMPRESSED_PAIR_H

#include <__config>
#include <__cstddef/size_t.h>
#include <__type_traits/datasizeof.h>
#include <__type_traits/is_empty.h>
#include <__type_traits/is_final.h>
#include <__type_traits/is_reference.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

// ================================================================================================================== //
// The utilites here are for staying ABI compatible with the legacy `__compressed_pair`. They should not be used      //
// for new data structures. Use `_LIBCPP_NO_UNIQUE_ADDRESS` for new data structures instead (but make sure you        //
// understand how it works).                                                                                          //
// ================================================================================================================== //

// The first member is aligned to the alignment of the second member to force padding in front of the compressed pair
// in case there are members before it.
//
// For example:
// (assuming x86-64 linux)
// class SomeClass {
//   uint32_t member1;
//   _LIBCPP_COMPRESSED_PAIR(uint32_t, member2, uint64_t, member3);
// }
//
// The layout with __compressed_pair is:
// member1 - offset: 0,  size: 4
// padding - offset: 4,  size: 4
// member2 - offset: 8,  size: 4
// padding - offset: 12, size: 4
// member3 - offset: 16, size: 8
//
// If the [[gnu::aligned]] wasn't there, the layout would instead be:
// member1 - offset: 0, size: 4
// member2 - offset: 4, size: 4
// member3 - offset: 8, size: 8
//
// Furthermore, that alignment must be the same as what was used in the old __compressed_pair layout, so we must
// handle reference types specially since alignof(T&) == alignof(T).
// See https://github.com/llvm/llvm-project/issues/118559.

#ifndef _LIBCPP_ABI_NO_COMPRESSED_PAIR_PADDING

template <class _Tp>
inline const size_t __compressed_pair_alignment = _LIBCPP_ALIGNOF(_Tp);

template <class _Tp>
inline const size_t __compressed_pair_alignment<_Tp&> = _LIBCPP_ALIGNOF(void*);

template <class _ToPad,
          bool _Empty = ((is_empty<_ToPad>::value && !__libcpp_is_final<_ToPad>::value) ||
                         is_reference<_ToPad>::value || sizeof(_ToPad) == __datasizeof_v<_ToPad>)>
class __compressed_pair_padding {
  char __padding_[sizeof(_ToPad) - __datasizeof_v<_ToPad>] = {};
};

template <class _ToPad>
class __compressed_pair_padding<_ToPad, true> {};

#  define _LIBCPP_COMPRESSED_PAIR(T1, Initializer1, T2, Initializer2)                                                  \
    _LIBCPP_NO_UNIQUE_ADDRESS __attribute__((__aligned__(::std::__compressed_pair_alignment<T2>))) T1 Initializer1;    \
    _LIBCPP_NO_UNIQUE_ADDRESS ::std::__compressed_pair_padding<T1> _LIBCPP_CONCAT3(__padding1_, __LINE__, _);          \
    _LIBCPP_NO_UNIQUE_ADDRESS T2 Initializer2;                                                                         \
    _LIBCPP_NO_UNIQUE_ADDRESS ::std::__compressed_pair_padding<T2> _LIBCPP_CONCAT3(__padding2_, __LINE__, _)

#  define _LIBCPP_COMPRESSED_TRIPLE(T1, Initializer1, T2, Initializer2, T3, Initializer3)                              \
    _LIBCPP_NO_UNIQUE_ADDRESS                                                                                          \
    __attribute__((__aligned__(::std::__compressed_pair_alignment<T2>),                                                \
                   __aligned__(::std::__compressed_pair_alignment<T3>))) T1 Initializer1;                              \
    _LIBCPP_NO_UNIQUE_ADDRESS ::std::__compressed_pair_padding<T1> _LIBCPP_CONCAT3(__padding1_, __LINE__, _);          \
    _LIBCPP_NO_UNIQUE_ADDRESS T2 Initializer2;                                                                         \
    _LIBCPP_NO_UNIQUE_ADDRESS ::std::__compressed_pair_padding<T2> _LIBCPP_CONCAT3(__padding2_, __LINE__, _);          \
    _LIBCPP_NO_UNIQUE_ADDRESS T3 Initializer3;                                                                         \
    _LIBCPP_NO_UNIQUE_ADDRESS ::std::__compressed_pair_padding<T3> _LIBCPP_CONCAT3(__padding3_, __LINE__, _)

#else
#  define _LIBCPP_COMPRESSED_PAIR(T1, Name1, T2, Name2)                                                                \
    _LIBCPP_NO_UNIQUE_ADDRESS T1 Name1;                                                                                \
    _LIBCPP_NO_UNIQUE_ADDRESS T2 Name2

#  define _LIBCPP_COMPRESSED_TRIPLE(T1, Name1, T2, Name2, T3, Name3)                                                   \
    _LIBCPP_NO_UNIQUE_ADDRESS T1 Name1;                                                                                \
    _LIBCPP_NO_UNIQUE_ADDRESS T2 Name2;                                                                                \
    _LIBCPP_NO_UNIQUE_ADDRESS T3 Name3
#endif // _LIBCPP_ABI_NO_COMPRESSED_PAIR_PADDING

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___MEMORY_COMPRESSED_PAIR_H
