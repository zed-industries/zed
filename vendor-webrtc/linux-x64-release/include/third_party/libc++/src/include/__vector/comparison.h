//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___VECTOR_COMPARISON_H
#define _LIBCPP___VECTOR_COMPARISON_H

#include <__algorithm/equal.h>
#include <__algorithm/lexicographical_compare.h>
#include <__algorithm/lexicographical_compare_three_way.h>
#include <__compare/synth_three_way.h>
#include <__config>
#include <__fwd/vector.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Tp, class _Allocator>
_LIBCPP_CONSTEXPR_SINCE_CXX20 inline _LIBCPP_HIDE_FROM_ABI bool
operator==(const vector<_Tp, _Allocator>& __x, const vector<_Tp, _Allocator>& __y) {
  const typename vector<_Tp, _Allocator>::size_type __sz = __x.size();
  return __sz == __y.size() && std::equal(__x.begin(), __x.end(), __y.begin());
}

#if _LIBCPP_STD_VER <= 17

template <class _Tp, class _Allocator>
inline _LIBCPP_HIDE_FROM_ABI bool operator!=(const vector<_Tp, _Allocator>& __x, const vector<_Tp, _Allocator>& __y) {
  return !(__x == __y);
}

template <class _Tp, class _Allocator>
inline _LIBCPP_HIDE_FROM_ABI bool operator<(const vector<_Tp, _Allocator>& __x, const vector<_Tp, _Allocator>& __y) {
  return std::lexicographical_compare(__x.begin(), __x.end(), __y.begin(), __y.end());
}

template <class _Tp, class _Allocator>
inline _LIBCPP_HIDE_FROM_ABI bool operator>(const vector<_Tp, _Allocator>& __x, const vector<_Tp, _Allocator>& __y) {
  return __y < __x;
}

template <class _Tp, class _Allocator>
inline _LIBCPP_HIDE_FROM_ABI bool operator>=(const vector<_Tp, _Allocator>& __x, const vector<_Tp, _Allocator>& __y) {
  return !(__x < __y);
}

template <class _Tp, class _Allocator>
inline _LIBCPP_HIDE_FROM_ABI bool operator<=(const vector<_Tp, _Allocator>& __x, const vector<_Tp, _Allocator>& __y) {
  return !(__y < __x);
}

#else // _LIBCPP_STD_VER <= 17

template <class _Tp, class _Allocator>
_LIBCPP_HIDE_FROM_ABI constexpr __synth_three_way_result<_Tp>
operator<=>(const vector<_Tp, _Allocator>& __x, const vector<_Tp, _Allocator>& __y) {
  return std::lexicographical_compare_three_way(__x.begin(), __x.end(), __y.begin(), __y.end(), std::__synth_three_way);
}

#endif // _LIBCPP_STD_VER <= 17

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___VECTOR_COMPARISON_H
