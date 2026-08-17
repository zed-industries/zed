//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___VECTOR_ERASE_H
#define _LIBCPP___VECTOR_ERASE_H

#include <__algorithm/remove.h>
#include <__algorithm/remove_if.h>
#include <__config>
#include <__fwd/vector.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

#if _LIBCPP_STD_VER >= 20

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Tp, class _Allocator, class _Up>
_LIBCPP_CONSTEXPR_SINCE_CXX20 inline _LIBCPP_HIDE_FROM_ABI typename vector<_Tp, _Allocator>::size_type
erase(vector<_Tp, _Allocator>& __c, const _Up& __v) {
  auto __old_size = __c.size();
  __c.erase(std::remove(__c.begin(), __c.end(), __v), __c.end());
  return __old_size - __c.size();
}

template <class _Tp, class _Allocator, class _Predicate>
_LIBCPP_CONSTEXPR_SINCE_CXX20 inline _LIBCPP_HIDE_FROM_ABI typename vector<_Tp, _Allocator>::size_type
erase_if(vector<_Tp, _Allocator>& __c, _Predicate __pred) {
  auto __old_size = __c.size();
  __c.erase(std::remove_if(__c.begin(), __c.end(), __pred), __c.end());
  return __old_size - __c.size();
}

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP_STD_VER >= 20

_LIBCPP_POP_MACROS

#endif // _LIBCPP___VECTOR_ERASE_H
