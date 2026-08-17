// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___ALGORITHM_ALL_OF_H
#define _LIBCPP___ALGORITHM_ALL_OF_H

#include <__config>
#include <__functional/identity.h>
#include <__type_traits/invoke.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Iter, class _Sent, class _Proj, class _Pred>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 bool
__all_of(_Iter __first, _Sent __last, _Pred& __pred, _Proj& __proj) {
  for (; __first != __last; ++__first) {
    if (!std::__invoke(__pred, std::__invoke(__proj, *__first)))
      return false;
  }
  return true;
}

template <class _InputIterator, class _Predicate>
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool
all_of(_InputIterator __first, _InputIterator __last, _Predicate __pred) {
  __identity __proj;
  return std::__all_of(__first, __last, __pred, __proj);
}

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___ALGORITHM_ALL_OF_H
