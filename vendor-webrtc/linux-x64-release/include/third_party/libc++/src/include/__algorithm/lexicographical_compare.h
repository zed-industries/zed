//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___ALGORITHM_LEXICOGRAPHICAL_COMPARE_H
#define _LIBCPP___ALGORITHM_LEXICOGRAPHICAL_COMPARE_H

#include <__algorithm/comp.h>
#include <__algorithm/min.h>
#include <__algorithm/mismatch.h>
#include <__algorithm/simd_utils.h>
#include <__algorithm/unwrap_iter.h>
#include <__config>
#include <__functional/identity.h>
#include <__iterator/iterator_traits.h>
#include <__string/constexpr_c_functions.h>
#include <__type_traits/desugars_to.h>
#include <__type_traits/enable_if.h>
#include <__type_traits/invoke.h>
#include <__type_traits/is_equality_comparable.h>
#include <__type_traits/is_integral.h>
#include <__type_traits/is_trivially_lexicographically_comparable.h>
#include <__type_traits/is_volatile.h>

#if _LIBCPP_HAS_WIDE_CHARACTERS
#  include <cwchar>
#endif

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Iter1, class _Sent1, class _Iter2, class _Sent2, class _Proj1, class _Proj2, class _Comp>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool __lexicographical_compare(
    _Iter1 __first1, _Sent1 __last1, _Iter2 __first2, _Sent2 __last2, _Comp& __comp, _Proj1& __proj1, _Proj2& __proj2) {
  while (__first2 != __last2) {
    if (__first1 == __last1 ||
        std::__invoke(__comp, std::__invoke(__proj1, *__first1), std::__invoke(__proj2, *__first2)))
      return true;
    if (std::__invoke(__comp, std::__invoke(__proj2, *__first2), std::__invoke(__proj1, *__first1)))
      return false;
    ++__first1;
    ++__first2;
  }
  return false;
}

#if _LIBCPP_STD_VER >= 14

// If the comparison operation is equivalent to < and that is a total order, we know that we can use equality comparison
// on that type instead to extract some information. Furthermore, if equality comparison on that type is trivial, the
// user can't observe that we're calling it. So instead of using the user-provided total order, we use std::mismatch,
// which uses equality comparison (and is vertorized). Additionally, if the type is trivially lexicographically
// comparable, we can go one step further and use std::memcmp directly instead of calling std::mismatch.
template <class _Tp,
          class _Proj1,
          class _Proj2,
          class _Comp,
          __enable_if_t<__desugars_to_v<__totally_ordered_less_tag, _Comp, _Tp, _Tp> && !is_volatile<_Tp>::value &&
                            __libcpp_is_trivially_equality_comparable<_Tp, _Tp>::value &&
                            __is_identity<_Proj1>::value && __is_identity<_Proj2>::value,
                        int> = 0>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool
__lexicographical_compare(_Tp* __first1, _Tp* __last1, _Tp* __first2, _Tp* __last2, _Comp&, _Proj1&, _Proj2&) {
  if constexpr (__is_trivially_lexicographically_comparable_v<_Tp, _Tp>) {
    auto __res =
        std::__constexpr_memcmp(__first1, __first2, __element_count(std::min(__last1 - __first1, __last2 - __first2)));
    if (__res == 0)
      return __last1 - __first1 < __last2 - __first2;
    return __res < 0;
  }
#  if _LIBCPP_HAS_WIDE_CHARACTERS
  else if constexpr (is_same<__remove_cv_t<_Tp>, wchar_t>::value) {
    auto __res = std::__constexpr_wmemcmp(__first1, __first2, std::min(__last1 - __first1, __last2 - __first2));
    if (__res == 0)
      return __last1 - __first1 < __last2 - __first2;
    return __res < 0;
  }
#  endif // _LIBCPP_HAS_WIDE_CHARACTERS
  else {
    auto __res = std::mismatch(__first1, __last1, __first2, __last2);
    if (__res.second == __last2)
      return false;
    if (__res.first == __last1)
      return true;
    return *__res.first < *__res.second;
  }
}

#endif // _LIBCPP_STD_VER >= 14

template <class _InputIterator1, class _InputIterator2, class _Compare>
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool lexicographical_compare(
    _InputIterator1 __first1,
    _InputIterator1 __last1,
    _InputIterator2 __first2,
    _InputIterator2 __last2,
    _Compare __comp) {
  __identity __proj;
  return std::__lexicographical_compare(
      std::__unwrap_iter(__first1),
      std::__unwrap_iter(__last1),
      std::__unwrap_iter(__first2),
      std::__unwrap_iter(__last2),
      __comp,
      __proj,
      __proj);
}

template <class _InputIterator1, class _InputIterator2>
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool lexicographical_compare(
    _InputIterator1 __first1, _InputIterator1 __last1, _InputIterator2 __first2, _InputIterator2 __last2) {
  return std::lexicographical_compare(__first1, __last1, __first2, __last2, __less<>());
}

_LIBCPP_END_NAMESPACE_STD

_LIBCPP_POP_MACROS

#endif // _LIBCPP___ALGORITHM_LEXICOGRAPHICAL_COMPARE_H
