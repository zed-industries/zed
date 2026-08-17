// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___ALGORITHM_COUNT_IF_H
#define _LIBCPP___ALGORITHM_COUNT_IF_H

#include <__algorithm/iterator_operations.h>
#include <__config>
#include <__functional/identity.h>
#include <__iterator/iterator_traits.h>
#include <__type_traits/invoke.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _AlgPolicy, class _Iter, class _Sent, class _Proj, class _Pred>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 __policy_iter_diff_t<_AlgPolicy, _Iter>
__count_if(_Iter __first, _Sent __last, _Pred& __pred, _Proj& __proj) {
  __policy_iter_diff_t<_AlgPolicy, _Iter> __counter(0);
  for (; __first != __last; ++__first) {
    if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
      ++__counter;
  }
  return __counter;
}

template <class _InputIterator, class _Predicate>
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20
typename iterator_traits<_InputIterator>::difference_type
count_if(_InputIterator __first, _InputIterator __last, _Predicate __pred) {
  __identity __proj;
  return std::__count_if<_ClassicAlgPolicy>(__first, __last, __pred, __proj);
}

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___ALGORITHM_COUNT_IF_H
