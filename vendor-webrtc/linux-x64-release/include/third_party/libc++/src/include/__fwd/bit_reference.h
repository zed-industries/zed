//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___FWD_BIT_REFERENCE_H
#define _LIBCPP___FWD_BIT_REFERENCE_H

#include <__config>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Cp, bool _IsConst, typename _Cp::__storage_type = 0>
class __bit_iterator;

template <class _Cp>
struct __bit_array;

template <class, class = void>
struct __size_difference_type_traits;

template <class _StoragePointer>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 void
__fill_masked_range(_StoragePointer __word, unsigned __clz, unsigned __ctz, bool __fill_val);

template <class _StorageType>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 _StorageType __trailing_mask(unsigned __clz);

template <class _StorageType>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 _StorageType __leading_mask(unsigned __ctz);

template <class _StorageType>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 _StorageType __middle_mask(unsigned __clz, unsigned __ctz);

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___FWD_BIT_REFERENCE_H
