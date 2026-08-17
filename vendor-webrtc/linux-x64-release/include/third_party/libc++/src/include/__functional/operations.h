// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___FUNCTIONAL_OPERATIONS_H
#define _LIBCPP___FUNCTIONAL_OPERATIONS_H

#include <__config>
#include <__functional/binary_function.h>
#include <__functional/unary_function.h>
#include <__fwd/functional.h>
#include <__type_traits/desugars_to.h>
#include <__type_traits/is_integral.h>
#include <__utility/forward.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

// Arithmetic operations

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct plus : __binary_function<_Tp, _Tp, _Tp> {
  typedef _Tp __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI _Tp operator()(const _Tp& __x, const _Tp& __y) const {
    return __x + __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(plus);

// The non-transparent std::plus specialization is only equivalent to a raw plus
// operator when we don't perform an implicit conversion when calling it.
template <class _Tp>
inline const bool __desugars_to_v<__plus_tag, plus<_Tp>, _Tp, _Tp> = true;

template <class _Tp, class _Up>
inline const bool __desugars_to_v<__plus_tag, plus<void>, _Tp, _Up> = true;

#if _LIBCPP_STD_VER >= 14
template <>
struct plus<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) + std::forward<_T2>(__u))) //
      -> decltype(std::forward<_T1>(__t) + std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) + std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};
#endif

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct minus : __binary_function<_Tp, _Tp, _Tp> {
  typedef _Tp __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI _Tp operator()(const _Tp& __x, const _Tp& __y) const {
    return __x - __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(minus);

#if _LIBCPP_STD_VER >= 14
template <>
struct minus<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) - std::forward<_T2>(__u))) //
      -> decltype(std::forward<_T1>(__t) - std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) - std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};
#endif

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct multiplies : __binary_function<_Tp, _Tp, _Tp> {
  typedef _Tp __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI _Tp operator()(const _Tp& __x, const _Tp& __y) const {
    return __x * __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(multiplies);

#if _LIBCPP_STD_VER >= 14
template <>
struct multiplies<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) * std::forward<_T2>(__u))) //
      -> decltype(std::forward<_T1>(__t) * std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) * std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};
#endif

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct divides : __binary_function<_Tp, _Tp, _Tp> {
  typedef _Tp __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI _Tp operator()(const _Tp& __x, const _Tp& __y) const {
    return __x / __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(divides);

#if _LIBCPP_STD_VER >= 14
template <>
struct divides<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) / std::forward<_T2>(__u))) //
      -> decltype(std::forward<_T1>(__t) / std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) / std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};
#endif

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct modulus : __binary_function<_Tp, _Tp, _Tp> {
  typedef _Tp __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI _Tp operator()(const _Tp& __x, const _Tp& __y) const {
    return __x % __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(modulus);

#if _LIBCPP_STD_VER >= 14
template <>
struct modulus<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) % std::forward<_T2>(__u))) //
      -> decltype(std::forward<_T1>(__t) % std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) % std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};
#endif

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct negate : __unary_function<_Tp, _Tp> {
  typedef _Tp __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI _Tp operator()(const _Tp& __x) const { return -__x; }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(negate);

#if _LIBCPP_STD_VER >= 14
template <>
struct negate<void> {
  template <class _Tp>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_Tp&& __x) const
      noexcept(noexcept(-std::forward<_Tp>(__x))) //
      -> decltype(-std::forward<_Tp>(__x)) {
    return -std::forward<_Tp>(__x);
  }
  typedef void is_transparent;
};
#endif

// Bitwise operations

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct bit_and : __binary_function<_Tp, _Tp, _Tp> {
  typedef _Tp __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI _Tp operator()(const _Tp& __x, const _Tp& __y) const {
    return __x & __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(bit_and);

#if _LIBCPP_STD_VER >= 14
template <>
struct bit_and<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) &
                        std::forward<_T2>(__u))) -> decltype(std::forward<_T1>(__t) & std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) & std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};
#endif

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
struct bit_not : __unary_function<_Tp, _Tp> {
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI _Tp operator()(const _Tp& __x) const { return ~__x; }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(bit_not);

template <>
struct bit_not<void> {
  template <class _Tp>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_Tp&& __x) const
      noexcept(noexcept(~std::forward<_Tp>(__x))) //
      -> decltype(~std::forward<_Tp>(__x)) {
    return ~std::forward<_Tp>(__x);
  }
  typedef void is_transparent;
};
#endif

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct bit_or : __binary_function<_Tp, _Tp, _Tp> {
  typedef _Tp __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI _Tp operator()(const _Tp& __x, const _Tp& __y) const {
    return __x | __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(bit_or);

#if _LIBCPP_STD_VER >= 14
template <>
struct bit_or<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) | std::forward<_T2>(__u))) //
      -> decltype(std::forward<_T1>(__t) | std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) | std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};
#endif

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct bit_xor : __binary_function<_Tp, _Tp, _Tp> {
  typedef _Tp __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI _Tp operator()(const _Tp& __x, const _Tp& __y) const {
    return __x ^ __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(bit_xor);

#if _LIBCPP_STD_VER >= 14
template <>
struct bit_xor<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) ^ std::forward<_T2>(__u))) //
      -> decltype(std::forward<_T1>(__t) ^ std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) ^ std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};
#endif

// Comparison operations

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct equal_to : __binary_function<_Tp, _Tp, bool> {
  typedef bool __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI bool operator()(const _Tp& __x, const _Tp& __y) const {
    return __x == __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(equal_to);

#if _LIBCPP_STD_VER >= 14
template <>
struct equal_to<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) == std::forward<_T2>(__u))) //
      -> decltype(std::forward<_T1>(__t) == std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) == std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};
#endif

// The non-transparent std::equal_to specialization is only equivalent to a raw equality
// comparison when we don't perform an implicit conversion when calling it.
template <class _Tp>
inline const bool __desugars_to_v<__equal_tag, equal_to<_Tp>, _Tp, _Tp> = true;

// In the transparent case, we do not enforce that
template <class _Tp, class _Up>
inline const bool __desugars_to_v<__equal_tag, equal_to<void>, _Tp, _Up> = true;

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct not_equal_to : __binary_function<_Tp, _Tp, bool> {
  typedef bool __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI bool operator()(const _Tp& __x, const _Tp& __y) const {
    return __x != __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(not_equal_to);

#if _LIBCPP_STD_VER >= 14
template <>
struct not_equal_to<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) != std::forward<_T2>(__u))) //
      -> decltype(std::forward<_T1>(__t) != std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) != std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};
#endif

template <class _Tp>
struct less : __binary_function<_Tp, _Tp, bool> {
  typedef bool __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI bool operator()(const _Tp& __x, const _Tp& __y) const {
    return __x < __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(less);

template <class _Tp>
inline const bool __desugars_to_v<__less_tag, less<_Tp>, _Tp, _Tp> = true;

template <class _Tp>
inline const bool __desugars_to_v<__totally_ordered_less_tag, less<_Tp>, _Tp, _Tp> = is_integral<_Tp>::value;

#if _LIBCPP_STD_VER >= 14
template <>
struct less<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) < std::forward<_T2>(__u))) //
      -> decltype(std::forward<_T1>(__t) < std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) < std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};

template <class _Tp, class _Up>
inline const bool __desugars_to_v<__less_tag, less<>, _Tp, _Up> = true;

template <class _Tp>
inline const bool __desugars_to_v<__totally_ordered_less_tag, less<>, _Tp, _Tp> = is_integral<_Tp>::value;
#endif

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct less_equal : __binary_function<_Tp, _Tp, bool> {
  typedef bool __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI bool operator()(const _Tp& __x, const _Tp& __y) const {
    return __x <= __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(less_equal);

#if _LIBCPP_STD_VER >= 14
template <>
struct less_equal<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) <= std::forward<_T2>(__u))) //
      -> decltype(std::forward<_T1>(__t) <= std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) <= std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};
#endif

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct greater_equal : __binary_function<_Tp, _Tp, bool> {
  typedef bool __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI bool operator()(const _Tp& __x, const _Tp& __y) const {
    return __x >= __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(greater_equal);

#if _LIBCPP_STD_VER >= 14
template <>
struct greater_equal<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) >=
                        std::forward<_T2>(__u))) -> decltype(std::forward<_T1>(__t) >= std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) >= std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};
#endif

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct greater : __binary_function<_Tp, _Tp, bool> {
  typedef bool __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI bool operator()(const _Tp& __x, const _Tp& __y) const {
    return __x > __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(greater);

template <class _Tp>
inline const bool __desugars_to_v<__greater_tag, greater<_Tp>, _Tp, _Tp> = true;

#if _LIBCPP_STD_VER >= 14
template <>
struct greater<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) > std::forward<_T2>(__u))) //
      -> decltype(std::forward<_T1>(__t) > std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) > std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};

template <class _Tp, class _Up>
inline const bool __desugars_to_v<__greater_tag, greater<>, _Tp, _Up> = true;
#endif

// Logical operations

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct logical_and : __binary_function<_Tp, _Tp, bool> {
  typedef bool __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI bool operator()(const _Tp& __x, const _Tp& __y) const {
    return __x && __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(logical_and);

#if _LIBCPP_STD_VER >= 14
template <>
struct logical_and<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) && std::forward<_T2>(__u))) //
      -> decltype(std::forward<_T1>(__t) && std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) && std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};
#endif

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct logical_not : __unary_function<_Tp, bool> {
  typedef bool __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI bool operator()(const _Tp& __x) const { return !__x; }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(logical_not);

#if _LIBCPP_STD_VER >= 14
template <>
struct logical_not<void> {
  template <class _Tp>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_Tp&& __x) const
      noexcept(noexcept(!std::forward<_Tp>(__x))) //
      -> decltype(!std::forward<_Tp>(__x)) {
    return !std::forward<_Tp>(__x);
  }
  typedef void is_transparent;
};
#endif

#if _LIBCPP_STD_VER >= 14
template <class _Tp = void>
#else
template <class _Tp>
#endif
struct logical_or : __binary_function<_Tp, _Tp, bool> {
  typedef bool __result_type; // used by valarray
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI bool operator()(const _Tp& __x, const _Tp& __y) const {
    return __x || __y;
  }
};
_LIBCPP_CTAD_SUPPORTED_FOR_TYPE(logical_or);

#if _LIBCPP_STD_VER >= 14
template <>
struct logical_or<void> {
  template <class _T1, class _T2>
  _LIBCPP_CONSTEXPR_SINCE_CXX14 _LIBCPP_HIDE_FROM_ABI auto operator()(_T1&& __t, _T2&& __u) const
      noexcept(noexcept(std::forward<_T1>(__t) || std::forward<_T2>(__u))) //
      -> decltype(std::forward<_T1>(__t) || std::forward<_T2>(__u)) {
    return std::forward<_T1>(__t) || std::forward<_T2>(__u);
  }
  typedef void is_transparent;
};
#endif

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___FUNCTIONAL_OPERATIONS_H
