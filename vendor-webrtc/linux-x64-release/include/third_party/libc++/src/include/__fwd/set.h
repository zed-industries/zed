//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___FWD_SET_H
#define _LIBCPP___FWD_SET_H

#include <__config>
#include <__fwd/functional.h>
#include <__fwd/memory.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Key, class _Compare = less<_Key>, class _Allocator = allocator<_Key> >
class set;

template <class _Key, class _Compare = less<_Key>, class _Allocator = allocator<_Key> >
class multiset;

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___FWD_SET_H
