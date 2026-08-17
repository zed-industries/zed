//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___ALGORITHM_COPY_IF_H
#define _LIBCPP___ALGORITHM_COPY_IF_H

#include <__config>
#include <__functional/identity.h>
#include <__type_traits/invoke.h>
#include <__utility/move.h>
#include <__utility/pair.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _InIter, class _Sent, class _OutIter, class _Proj, class _Pred>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX14 pair<_InIter, _OutIter>
__copy_if(_InIter __first, _Sent __last, _OutIter __result, _Pred& __pred, _Proj& __proj) {
  for (; __first != __last; ++__first) {
    if (std::__invoke(__pred, std::__invoke(__proj, *__first))) {
      *__result = *__first;
      ++__result;
    }
  }
  return std::make_pair(std::move(__first), std::move(__result));
}

template <class _InputIterator, class _OutputIterator, class _Predicate>
inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 _OutputIterator
copy_if(_InputIterator __first, _InputIterator __last, _OutputIterator __result, _Predicate __pred) {
  __identity __proj;
  return std::__copy_if(__first, __last, __result, __pred, __proj).second;
}

_LIBCPP_END_NAMESPACE_STD

_LIBCPP_POP_MACROS

#endif // _LIBCPP___ALGORITHM_COPY_IF_H
