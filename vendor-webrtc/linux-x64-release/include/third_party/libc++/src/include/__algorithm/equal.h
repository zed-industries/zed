// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___ALGORITHM_EQUAL_H
#define _LIBCPP___ALGORITHM_EQUAL_H

#include <__algorithm/comp.h>
#include <__algorithm/min.h>
#include <__algorithm/unwrap_iter.h>
#include <__config>
#include <__functional/identity.h>
#include <__fwd/bit_reference.h>
#include <__iterator/distance.h>
#include <__iterator/iterator_traits.h>
#include <__memory/pointer_traits.h>
#include <__string/constexpr_c_functions.h>
#include <__type_traits/desugars_to.h>
#include <__type_traits/enable_if.h>
#include <__type_traits/invoke.h>
#include <__type_traits/is_equality_comparable.h>
#include <__type_traits/is_same.h>
#include <__type_traits/is_volatile.h>
#include <__utility/move.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Cp, bool _IsConst1, bool _IsConst2>
[[__nodiscard__]] _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI bool
__equal_unaligned(__bit_iterator<_Cp, _IsConst1> __first1,
                  __bit_iterator<_Cp, _IsConst1> __last1,
                  __bit_iterator<_Cp, _IsConst2> __first2) {
  using _It             = __bit_iterator<_Cp, _IsConst1>;
  using difference_type = typename _It::difference_type;
  using __storage_type  = typename _It::__storage_type;

  const int __bits_per_word = _It::__bits_per_word;
  difference_type __n       = __last1 - __first1;
  if (__n > 0) {
    // do first word
    if (__first1.__ctz_ != 0) {
      unsigned __clz_f     = __bits_per_word - __first1.__ctz_;
      difference_type __dn = std::min(static_cast<difference_type>(__clz_f), __n);
      __n -= __dn;
      __storage_type __m   = std::__middle_mask<__storage_type>(__clz_f - __dn, __first1.__ctz_);
      __storage_type __b   = *__first1.__seg_ & __m;
      unsigned __clz_r     = __bits_per_word - __first2.__ctz_;
      __storage_type __ddn = std::min<__storage_type>(__dn, __clz_r);
      __m                  = std::__middle_mask<__storage_type>(__clz_r - __ddn, __first2.__ctz_);
      if (__first2.__ctz_ > __first1.__ctz_) {
        if (static_cast<__storage_type>(*__first2.__seg_ & __m) !=
            static_cast<__storage_type>(__b << (__first2.__ctz_ - __first1.__ctz_)))
          return false;
      } else {
        if (static_cast<__storage_type>(*__first2.__seg_ & __m) !=
            static_cast<__storage_type>(__b >> (__first1.__ctz_ - __first2.__ctz_)))
          return false;
      }
      __first2.__seg_ += (__ddn + __first2.__ctz_) / __bits_per_word;
      __first2.__ctz_ = static_cast<unsigned>((__ddn + __first2.__ctz_) % __bits_per_word);
      __dn -= __ddn;
      if (__dn > 0) {
        __m = std::__trailing_mask<__storage_type>(__bits_per_word - __n);
        if (static_cast<__storage_type>(*__first2.__seg_ & __m) !=
            static_cast<__storage_type>(__b >> (__first1.__ctz_ + __ddn)))
          return false;
        __first2.__ctz_ = static_cast<unsigned>(__dn);
      }
      ++__first1.__seg_;
      // __first1.__ctz_ = 0;
    }
    // __first1.__ctz_ == 0;
    // do middle words
    unsigned __clz_r   = __bits_per_word - __first2.__ctz_;
    __storage_type __m = std::__leading_mask<__storage_type>(__first2.__ctz_);
    for (; __n >= __bits_per_word; __n -= __bits_per_word, ++__first1.__seg_) {
      __storage_type __b = *__first1.__seg_;
      if (static_cast<__storage_type>(*__first2.__seg_ & __m) != static_cast<__storage_type>(__b << __first2.__ctz_))
        return false;
      ++__first2.__seg_;
      if (static_cast<__storage_type>(*__first2.__seg_ & static_cast<__storage_type>(~__m)) !=
          static_cast<__storage_type>(__b >> __clz_r))
        return false;
    }
    // do last word
    if (__n > 0) {
      __m                 = std::__trailing_mask<__storage_type>(__bits_per_word - __n);
      __storage_type __b  = *__first1.__seg_ & __m;
      __storage_type __dn = std::min(__n, static_cast<difference_type>(__clz_r));
      __m                 = std::__middle_mask<__storage_type>(__clz_r - __dn, __first2.__ctz_);
      if (static_cast<__storage_type>(*__first2.__seg_ & __m) != static_cast<__storage_type>(__b << __first2.__ctz_))
        return false;
      __first2.__seg_ += (__dn + __first2.__ctz_) / __bits_per_word;
      __first2.__ctz_ = static_cast<unsigned>((__dn + __first2.__ctz_) % __bits_per_word);
      __n -= __dn;
      if (__n > 0) {
        __m = std::__trailing_mask<__storage_type>(__bits_per_word - __n);
        if (static_cast<__storage_type>(*__first2.__seg_ & __m) != static_cast<__storage_type>(__b >> __dn))
          return false;
      }
    }
  }
  return true;
}

template <class _Cp, bool _IsConst1, bool _IsConst2>
[[__nodiscard__]] _LIBCPP_CONSTEXPR_SINCE_CXX20 _LIBCPP_HIDE_FROM_ABI bool
__equal_aligned(__bit_iterator<_Cp, _IsConst1> __first1,
                __bit_iterator<_Cp, _IsConst1> __last1,
                __bit_iterator<_Cp, _IsConst2> __first2) {
  using _It             = __bit_iterator<_Cp, _IsConst1>;
  using difference_type = typename _It::difference_type;
  using __storage_type  = typename _It::__storage_type;

  const int __bits_per_word = _It::__bits_per_word;
  difference_type __n       = __last1 - __first1;
  if (__n > 0) {
    // do first word
    if (__first1.__ctz_ != 0) {
      unsigned __clz       = __bits_per_word - __first1.__ctz_;
      difference_type __dn = std::min(static_cast<difference_type>(__clz), __n);
      __n -= __dn;
      __storage_type __m = std::__middle_mask<__storage_type>(__clz - __dn, __first1.__ctz_);
      if ((*__first2.__seg_ & __m) != (*__first1.__seg_ & __m))
        return false;
      ++__first2.__seg_;
      ++__first1.__seg_;
      // __first1.__ctz_ = 0;
      // __first2.__ctz_ = 0;
    }
    // __first1.__ctz_ == 0;
    // __first2.__ctz_ == 0;
    // do middle words
    for (; __n >= __bits_per_word; __n -= __bits_per_word, ++__first1.__seg_, ++__first2.__seg_)
      if (*__first2.__seg_ != *__first1.__seg_)
        return false;
    // do last word
    if (__n > 0) {
      __storage_type __m = std::__trailing_mask<__storage_type>(__bits_per_word - __n);
      if ((*__first2.__seg_ & __m) != (*__first1.__seg_ & __m))
        return false;
    }
  }
  return true;
}

template <class _Cp,
          bool _IsConst1,
          bool _IsConst2,
          class _BinaryPredicate,
          __enable_if_t<__desugars_to_v<__equal_tag, _BinaryPredicate, bool, bool>, int> = 0>
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool __equal_iter_impl(
    __bit_iterator<_Cp, _IsConst1> __first1,
    __bit_iterator<_Cp, _IsConst1> __last1,
    __bit_iterator<_Cp, _IsConst2> __first2,
    _BinaryPredicate) {
  if (__first1.__ctz_ == __first2.__ctz_)
    return std::__equal_aligned(__first1, __last1, __first2);
  return std::__equal_unaligned(__first1, __last1, __first2);
}

template <class _InputIterator1, class _InputIterator2, class _BinaryPredicate>
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool __equal_iter_impl(
    _InputIterator1 __first1, _InputIterator1 __last1, _InputIterator2 __first2, _BinaryPredicate& __pred) {
  for (; __first1 != __last1; ++__first1, (void)++__first2)
    if (!__pred(*__first1, *__first2))
      return false;
  return true;
}

template <class _Tp,
          class _Up,
          class _BinaryPredicate,
          __enable_if_t<__desugars_to_v<__equal_tag, _BinaryPredicate, _Tp, _Up> && !is_volatile<_Tp>::value &&
                            !is_volatile<_Up>::value && __libcpp_is_trivially_equality_comparable<_Tp, _Up>::value,
                        int> = 0>
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool
__equal_iter_impl(_Tp* __first1, _Tp* __last1, _Up* __first2, _BinaryPredicate&) {
  return std::__constexpr_memcmp_equal(__first1, __first2, __element_count(__last1 - __first1));
}

template <class _InputIterator1, class _InputIterator2, class _BinaryPredicate>
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool
equal(_InputIterator1 __first1, _InputIterator1 __last1, _InputIterator2 __first2, _BinaryPredicate __pred) {
  return std::__equal_iter_impl(
      std::__unwrap_iter(__first1), std::__unwrap_iter(__last1), std::__unwrap_iter(__first2), __pred);
}

template <class _InputIterator1, class _InputIterator2>
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool
equal(_InputIterator1 __first1, _InputIterator1 __last1, _InputIterator2 __first2) {
  return std::equal(__first1, __last1, __first2, __equal_to());
}

#if _LIBCPP_STD_VER >= 14

template <class _Iter1, class _Sent1, class _Iter2, class _Sent2, class _Pred, class _Proj1, class _Proj2>
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool __equal_impl(
    _Iter1 __first1, _Sent1 __last1, _Iter2 __first2, _Sent2 __last2, _Pred& __comp, _Proj1& __proj1, _Proj2& __proj2) {
  while (__first1 != __last1 && __first2 != __last2) {
    if (!std::__invoke(__comp, std::__invoke(__proj1, *__first1), std::__invoke(__proj2, *__first2)))
      return false;
    ++__first1;
    ++__first2;
  }
  return __first1 == __last1 && __first2 == __last2;
}

template <class _Tp,
          class _Up,
          class _Pred,
          class _Proj1,
          class _Proj2,
          __enable_if_t<__desugars_to_v<__equal_tag, _Pred, _Tp, _Up> && __is_identity<_Proj1>::value &&
                            __is_identity<_Proj2>::value && !is_volatile<_Tp>::value && !is_volatile<_Up>::value &&
                            __libcpp_is_trivially_equality_comparable<_Tp, _Up>::value,
                        int> = 0>
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool
__equal_impl(_Tp* __first1, _Tp* __last1, _Up* __first2, _Up*, _Pred&, _Proj1&, _Proj2&) {
  return std::__constexpr_memcmp_equal(__first1, __first2, __element_count(__last1 - __first1));
}

template <class _Cp,
          bool _IsConst1,
          bool _IsConst2,
          class _Pred,
          class _Proj1,
          class _Proj2,
          __enable_if_t<__desugars_to_v<__equal_tag, _Pred, bool, bool> && __is_identity<_Proj1>::value &&
                            __is_identity<_Proj2>::value,
                        int> = 0>
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool __equal_impl(
    __bit_iterator<_Cp, _IsConst1> __first1,
    __bit_iterator<_Cp, _IsConst1> __last1,
    __bit_iterator<_Cp, _IsConst2> __first2,
    __bit_iterator<_Cp, _IsConst2>,
    _Pred&,
    _Proj1&,
    _Proj2&) {
  if (__first1.__ctz_ == __first2.__ctz_)
    return std::__equal_aligned(__first1, __last1, __first2);
  return std::__equal_unaligned(__first1, __last1, __first2);
}

template <class _InputIterator1, class _InputIterator2, class _BinaryPredicate>
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool
equal(_InputIterator1 __first1,
      _InputIterator1 __last1,
      _InputIterator2 __first2,
      _InputIterator2 __last2,
      _BinaryPredicate __pred) {
  if constexpr (__has_random_access_iterator_category<_InputIterator1>::value &&
                __has_random_access_iterator_category<_InputIterator2>::value) {
    if (std::distance(__first1, __last1) != std::distance(__first2, __last2))
      return false;
  }
  __identity __proj;
  return std::__equal_impl(
      std::__unwrap_iter(__first1),
      std::__unwrap_iter(__last1),
      std::__unwrap_iter(__first2),
      std::__unwrap_iter(__last2),
      __pred,
      __proj,
      __proj);
}

template <class _InputIterator1, class _InputIterator2>
[[__nodiscard__]] inline _LIBCPP_HIDE_FROM_ABI _LIBCPP_CONSTEXPR_SINCE_CXX20 bool
equal(_InputIterator1 __first1, _InputIterator1 __last1, _InputIterator2 __first2, _InputIterator2 __last2) {
  return std::equal(__first1, __last1, __first2, __last2, __equal_to());
}

#endif // _LIBCPP_STD_VER >= 14

_LIBCPP_END_NAMESPACE_STD

_LIBCPP_POP_MACROS

#endif // _LIBCPP___ALGORITHM_EQUAL_H
